use super::ExtType;
use crate::{
    decode::{read_be_i64, read_be_u32, read_be_u64, Error as DeError},
    encode::{write_be_i64, write_be_u32, write_be_u64, Error as SerError, SerializeIntoSlice},
    Ext,
};
use core::convert::{TryFrom, TryInto};

const EXT_TIMESTAMP: ExtType = ExtType(-1);

pub(crate) const TYPE_NAME: &'static str = "$Timestamp";
pub(crate) const FIELD_SECONDS_NAME: &'static str = "seconds";
pub(crate) const FIELD_NANOSECONDS_NAME: &'static str = "nanoseconds";

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "derive-debug"), derive(core::fmt::Debug))]
pub struct Timestamp {
    seconds: i64,
    nanoseconds: u32,
}

impl Timestamp {
    pub const fn new(seconds: i64, nanoseconds: u32) -> Result<Timestamp, DeError> {
        let mut seconds = seconds;
        let mut nanoseconds = nanoseconds;
        while nanoseconds >= 1_000_000_000 {
            // This will be at most 4 times, so it can't become a performance problem
            seconds += 1;
            nanoseconds -= 1_000_000_000;
        }
        Ok(Timestamp { seconds, nanoseconds })
    }

    pub const fn seconds(&self) -> i64 { self.seconds }
    pub const fn nanoseconds(&self) -> u32 { self.nanoseconds }

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn to_ext<'a>(&self, data: &'a mut [u8]) -> Result<Ext<'a>, SerError> {
        if self.seconds >> 34 == 0 {
            let x = (u64::from(self.nanoseconds) << 34) | self.seconds as u64;
            if x & 0xffff_ffff_0000_0000_u64 == 0 {
                // timestamp 32
                if data.len() < 4 {
                    return Err(SerError::EndOfBuffer);
                }
                write_be_u32(data, x as u32);
                Ok(Ext::new(-1, &data[0..4]))
            } else {
                // timestamp 64
                if data.len() < 8 {
                    return Err(SerError::EndOfBuffer);
                }
                write_be_u64(data, x);
                Ok(Ext::new(-1, &data[0..8]))
            }
        } else {
            // timestamp 96
            #[cfg(feature = "timestamp96")]
            {
                if data.len() < 12 {
                    return Err(SerError::EndOfBuffer);
                }
                write_be_u32(data, self.nanoseconds);
                write_be_i64(&mut data[4..], self.seconds);
                return Ok(Ext::new(-1, &data[0..12]));
            }
            #[cfg(not(feature = "timestamp96"))]
            return Err(SerError::InvalidType);
        }
    }
}

impl SerializeIntoSlice for Timestamp {
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, SerError> {
        let mut tmp = [0; 12];
        let ext = self.to_ext(&mut tmp)?;
        crate::ext::serialize_ext(&ext, buf)
    }
}

pub fn try_deserialize(buf: &[u8]) -> Result<Timestamp, DeError> { crate::ext::try_deserialize_ext(&buf)?.try_into() }

impl<'a> TryFrom<Ext<'a>> for Timestamp {
    type Error = DeError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(ext: Ext<'a>) -> Result<Self, Self::Error> {
        if ext.typ == EXT_TIMESTAMP {
            match ext.data.len() {
                4 => {
                    // timestamp 32 stores the number of seconds that have elapsed since 1970-01-01 00:00:00 UTC
                    // in an 32-bit unsigned integer:
                    // +--------+--------+--------+--------+--------+--------+
                    // |  0xd6  |   -1   |   seconds in 32-bit unsigned int  |
                    // +--------+--------+--------+--------+--------+--------+
                    Timestamp::new(i64::from(read_be_u32(&ext.data)), 0)
                }
                #[allow(clippy::cast_possible_wrap)]
                8 => {
                    // timestamp 64 stores the number of seconds and nanoseconds that have elapsed since 1970-01-01 00:00:00 UTC
                    // in 32-bit unsigned integers:
                    // +--------+--------+--------+--------+--------+------|-+--------+--------+--------+--------+
                    // |  0xd7  |   -1   | nanosec. in 30-bit unsigned int |   seconds in 34-bit unsigned int    |
                    // +--------+--------+--------+--------+--------+------^-+--------+--------+--------+--------+
                    let value = read_be_u64(&ext.data);
                    Timestamp::new((value & 0x0000_0003_ffff_ffff_u64) as i64, (value >> 34) as u32)
                }
                #[cfg(feature = "timestamp96")]
                12 => {
                    // timestamp 96 stores the number of seconds and nanoseconds that have elapsed since 1970-01-01 00:00:00 UTC
                    // in 64-bit signed integer and 32-bit unsigned integer:
                    // +--------+--------+--------+--------+--------+--------+--------+
                    // |  0xc7  |   12   |   -1   |nanoseconds in 32-bit unsigned int |
                    // +--------+--------+--------+--------+--------+--------+--------+
                    // +--------+--------+--------+--------+--------+--------+--------+--------+
                    // |                   seconds in 64-bit signed int                        |
                    // +--------+--------+--------+--------+--------+--------+--------+--------+
                    let nanos = read_be_u32(&ext.data[0..4]);
                    let s = read_be_i64(&ext.data[4..12]);
                    Timestamp::new(s, nanos)
                }
                _ => Err(DeError::InvalidType),
            }
        } else {
            Err(DeError::InvalidType)
        }
    }
}

#[cfg(feature = "serde")]
impl ::serde::ser::Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct(TYPE_NAME, 2)?;
        s.serialize_field(FIELD_SECONDS_NAME, &self.seconds)?;
        s.serialize_field(FIELD_NANOSECONDS_NAME, &self.nanoseconds)?;
        s.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::de::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: ::serde::de::Deserializer<'de> {
        use crate::encode::Binary;
        struct TimestampVisitor<'a>(core::marker::PhantomData<&'a ()>);

        impl<'de: 'a, 'a> ::serde::de::Visitor<'de> for TimestampVisitor<'a> {
            type Value = Timestamp;
            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { formatter.write_str("a MsgPack ext data") }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where A: serde::de::SeqAccess<'de> {
                // This will be called from the MsgPack deserializer
                // if it detects an ext tag or an array tag
                let typ: Option<ExtType> = seq.next_element()?;
                let data: Option<Binary> = seq.next_element()?;
                match (typ, data) {
                    (Some(typ), Some(data)) => Ok(Ext::new(typ.0, &data).try_into().map_err(::serde::de::Error::custom)?),
                    (Some(_), None) => Err(::serde::de::Error::custom("ext data not found")),
                    _ => Err(::serde::de::Error::custom("ext type field not found")),
                }
            }
            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where V: ::serde::de::MapAccess<'de> {
                // This will probably be called from other deserializers
                // or if the MsgPack deserializer sees a map tag

                enum Field {
                    Seconds,
                    Nanoseconds,
                    Type,
                    Data,
                }

                impl<'de> ::serde::de::Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where D: serde::Deserializer<'de> {
                        struct FieldVisitor;
                        impl<'de> ::serde::de::Visitor<'de> for FieldVisitor {
                            type Value = Field;
                            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                                formatter.write_str("`seconds`, `secs`, `s`, `nanoseconds`, `nanos`, `ns`, `type` or `data`")
                            }
                            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                            where E: serde::de::Error {
                                match v {
                                    "seconds" | "secs" | "s" => Ok(Field::Seconds),
                                    "nanoseconds" | "nanos" | "ns" => Ok(Field::Nanoseconds),
                                    "type" => Ok(Field::Type),
                                    "data" => Ok(Field::Data),
                                    _ => Err(::serde::de::Error::unknown_field(
                                        v,
                                        &["seconds", "secs", "s", "nanoseconds", "nanos", "ns", "type", "data"],
                                    )),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                let mut typ = None;
                let mut data = None;
                let mut seconds = None;
                let mut nanoseconds = None;

                loop {
                    match map.next_key::<Field>() {
                        Ok(Some(Field::Type)) => typ = Some(map.next_value::<ExtType>()?),
                        Ok(Some(Field::Data)) => data = Some(map.next_value::<Binary>()?),
                        Ok(Some(Field::Seconds)) => seconds = Some(map.next_value::<i64>()?),
                        Ok(Some(Field::Nanoseconds)) => nanoseconds = Some(map.next_value::<u32>()?),
                        Ok(None) => break, // no more fields
                        Err(_e) => {
                            // Error, could be an unknown field name
                            // println!("{:?}", e);
                            map.next_value::<()>()?;
                        }
                    }
                }

                match (typ, data, seconds, nanoseconds) {
                    (Some(typ), Some(data), None, None) => Ok(Ext::new(typ.0, &data).try_into().map_err(::serde::de::Error::custom)?),
                    (None, None, Some(seconds), Some(nanoseconds)) => {
                        Ok(Timestamp::new(seconds, nanoseconds).map_err(::serde::de::Error::custom)?)
                    }
                    (None, None, Some(seconds), None) => Ok(Timestamp::new(seconds, 0).map_err(::serde::de::Error::custom)?),
                    (None, None, None, Some(nanoseconds)) => Ok(Timestamp::new(0, nanoseconds).map_err(::serde::de::Error::custom)?),
                    (Some(_), None, _, _) => Err(::serde::de::Error::custom("ext data not found")),
                    _ => Err(::serde::de::Error::custom("ext type field not found")),
                }
            }
        }

        static FIELDS: [&str; 2] = [FIELD_SECONDS_NAME, FIELD_NANOSECONDS_NAME];
        deserializer.deserialize_struct(TYPE_NAME, &FIELDS, TimestampVisitor(core::marker::PhantomData))
    }
}

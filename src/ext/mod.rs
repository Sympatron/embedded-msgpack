#[cfg(feature = "timestamp")]
pub mod timestamp;

use crate::encode::{Binary, Error, SerializeIntoSlice};
#[allow(unused_imports)]
use crate::marker::Marker;
#[allow(unused_imports)]
use byteorder::{BigEndian, ByteOrder};
use core::{convert::TryInto, fmt::Display, marker::PhantomData};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "debug-impls"), derive(core::fmt::Debug))]
#[serde(transparent)]
struct ExtType(i8);

impl Display for ExtType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0 == -1 {
            f.write_str("-1 (Timestamp)")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(PartialEq, Eq)]
#[cfg_attr(any(test, feature = "debug-impls"), derive(core::fmt::Debug))]
pub struct Ext<'a> {
    typ: ExtType,
    data: Binary<'a>,
}

impl<'a> Ext<'a> {
    pub const fn new_from_binary(typ: i8, data: Binary<'a>) -> Self {
        Ext {
            typ: ExtType(typ),
            data: data,
        }
    }
    pub const fn new(typ: i8, data: &'a [u8]) -> Self {
        Ext {
            typ: ExtType(typ),
            data: Binary::new(data),
        }
    }
    #[inline(always)]
    pub const fn get_type(&self) -> i8 { self.typ.0 }
    #[inline(always)]
    pub const fn get_data(&self) -> &Binary<'a> { &self.data }
}

#[inline]
pub(crate) fn get_ext_start(data_len: usize) -> Result<(Marker, usize), Error> {
    let (marker, header_len) = match data_len {
        #[cfg(feature = "fixext")]
        1 | 2 | 4 | 8 | 16 => {
            let header_len = 2;
            let marker = match data_len {
                1 => Marker::FixExt1,
                2 => Marker::FixExt2,
                4 => Marker::FixExt4,
                8 => Marker::FixExt8,
                16 => Marker::FixExt16,
                _ => unreachable!(),
            };
            (marker, header_len)
        }
        #[cfg(feature = "ext8")]
        0..=0xff => (Marker::Ext8, 3),
        #[cfg(feature = "ext16")]
        0x100..=0xffff => (Marker::Ext16, 4),
        #[cfg(feature = "ext32")]
        0x1_0000..=0xffff_ffff => (Marker::Ext32, 6),
        _ => return Err(Error::OutOfBounds),
    };
    Ok((marker, header_len))
}

pub(crate) fn read_ext_len<B: zerocopy::ByteSlice>(buf: B) -> Result<(usize, usize), crate::decode::Error> {
    if buf.len() < 2 {
        return Err(crate::decode::Error::EndOfBuffer);
    }
    let marker: Marker = buf[0].try_into().unwrap();
    let (header_len, data_len) = match marker {
        #[cfg(feature = "fixext")]
        Marker::FixExt1 => (2, 1),
        #[cfg(feature = "fixext")]
        Marker::FixExt2 => (2, 2),
        #[cfg(feature = "fixext")]
        Marker::FixExt4 => (2, 4),
        #[cfg(feature = "fixext")]
        Marker::FixExt8 => (2, 8),
        #[cfg(feature = "fixext")]
        Marker::FixExt16 => (2, 16),
        #[cfg(feature = "ext8")]
        Marker::Ext8 => (3, buf[1] as usize),
        #[cfg(feature = "ext16")]
        Marker::Ext16 => {
            if buf.len() < 4 {
                return Err(crate::decode::Error::EndOfBuffer);
            }
            (4, BigEndian::read_u16(&buf[1..3]) as usize)
        }
        #[cfg(feature = "ext32")]
        Marker::Ext32 => {
            if buf.len() < 6 {
                return Err(crate::decode::Error::EndOfBuffer);
            }
            (6, BigEndian::read_u32(&buf[1..5]) as usize)
        }
        _ => return Err(crate::decode::Error::InvalidType),
    };
    // let typ = buf[header_len - 1] as i8;
    if buf.len() >= header_len + data_len {
        Ok((header_len, data_len))
    } else {
        Err(crate::decode::Error::EndOfBuffer)
    }
}

pub fn serialize_ext<'a>(value: &Ext<'a>, buf: &mut [u8]) -> Result<usize, Error> {
    let typ = value.get_type();
    let data = value.get_data();

    let (marker, header_len) = get_ext_start(data.len())?;
    if buf.len() < data.len() + header_len {
        return Err(Error::EndOfBuffer);
    }
    buf[0] = marker.to_u8();
    if header_len > 2 {
        #[cfg(all(feature = "ext8", not(any(feature = "ext16", feature = "ext32"))))]
        {
            buf[1] = data.len() as u8;
        }
        #[cfg(any(feature = "ext16", feature = "ext32"))]
        {
            BigEndian::write_uint(&mut buf[1..], data.len() as u64, header_len - 2);
        }
    }
    buf[header_len - 1] = typ as u8;
    buf[header_len..data.len() + header_len].clone_from_slice(&data);
    Ok(data.len() + header_len)
}

pub fn try_deserialize_ext<'a>(buf: &'a [u8]) -> Result<Ext<'a>, crate::decode::Error> {
    if buf.len() < 3 {
        return Err(crate::decode::Error::EndOfBuffer);
    }
    let (header_len, data_len) = read_ext_len(&buf[..])?;
    if buf.len() < header_len + data_len {
        return Err(crate::decode::Error::EndOfBuffer);
    }
    let typ = buf[header_len - 1] as i8;
    return Ok(Ext::new(typ, &buf[header_len..header_len + data_len]));
}

impl<'a> SerializeIntoSlice for &Ext<'a> {
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_ext(self, buf) }
}

pub(crate) const TYPE_NAME: &'static str = "$Ext";
pub(crate) const FIELD_TYPE_NAME: &'static str = "type";
pub(crate) const FIELD_DATA_NAME: &'static str = "data";

#[cfg(feature = "serde")]
impl<'a> ::serde::ser::Serialize for Ext<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut s = serializer.serialize_struct(TYPE_NAME, 2)?;
        s.serialize_field(FIELD_TYPE_NAME, &self.typ)?;
        s.serialize_field(FIELD_DATA_NAME, &self.data)?;
        s.end()
    }
}

#[cfg(feature = "serde")]
impl<'de: 'a, 'a> ::serde::de::Deserialize<'de> for Ext<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: ::serde::de::Deserializer<'de> {
        struct ExtVisitor<'a>(PhantomData<&'a ()>);

        impl<'de: 'a, 'a> ::serde::de::Visitor<'de> for ExtVisitor<'a> {
            type Value = Ext<'a>;
            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { formatter.write_str("a MsgPack ext data") }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where A: serde::de::SeqAccess<'de> {
                // This will be called from the MsgPack deserializer
                // if it detects an ext tag or an array tag
                let typ: Option<ExtType> = seq.next_element()?;
                let data: Option<Binary> = seq.next_element()?;
                match (typ, data) {
                    (Some(typ), Some(data)) => Ok(Ext::new_from_binary(typ.0, data)),
                    (Some(_), None) => Err(::serde::de::Error::custom("ext data not found")),
                    _ => Err(::serde::de::Error::custom("ext type field not found")),
                }
            }
            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where V: ::serde::de::MapAccess<'de> {
                // This will probably be called from other deserializers
                // or if the MsgPack deserializer sees a map tag

                enum Field {
                    // Seconds,
                    // Nanoseconds,
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
                                // formatter.write_str("`seconds`, `secs`, `s`, `nanoseconds`, `nanos` or `ns`")
                                formatter.write_str("`type` or `data`")
                            }
                            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                            where E: serde::de::Error {
                                match v {
                                    // "seconds" | "secs" | "s" => Ok(Field::Seconds),
                                    // "nanoseconds" | "nanos" | "ns" => Ok(Field::Nanoseconds),
                                    "type" => Ok(Field::Type),
                                    "data" => Ok(Field::Data),
                                    _ => Err(::serde::de::Error::unknown_field(
                                        v,
                                        &["type", "data"],
                                        // &["seconds", "secs", "s", "nanoseconds", "nanos", "ns"],
                                    )),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                // if let Ok(typ) = map.next_key::<Field>() {
                //     let typ = match typ {
                //         Some(v) => v,
                //         None => return Err(::serde::de::Error::custom("ext type not found")),
                //     };
                //     let v: Binary = map.next_value()?;
                //     Ok(Ext::new(typ.0, &v))
                // } else {
                let mut typ = None;
                let mut data = None;
                // let mut seconds = None;
                // let mut nanoseconds = None;

                loop {
                    match map.next_key::<Field>() {
                        Ok(Some(Field::Type)) => typ = Some(map.next_value::<ExtType>()?),
                        Ok(Some(Field::Data)) => data = Some(map.next_value::<Binary>()?),
                        Ok(None) => break, // no more fields
                        Err(_e) => {
                            // Error, could be an unknown field name
                            // println!("{:?}", e);
                            map.next_value()?;
                        }
                    }
                }

                match (typ, data) {
                    (Some(typ), Some(data)) => Ok(Ext::new_from_binary(typ.0, data)),
                    (Some(_), None) => Err(::serde::de::Error::custom("ext data not found")),
                    _ => Err(::serde::de::Error::custom("ext type field not found")),
                }
                // }
            }
        }

        static FIELDS: [&str; 2] = [FIELD_TYPE_NAME, FIELD_DATA_NAME];
        deserializer.deserialize_struct(TYPE_NAME, &FIELDS, ExtVisitor(PhantomData))
    }
}

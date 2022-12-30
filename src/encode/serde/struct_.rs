use super::{Error, Serializer, Unreachable};
use serde::ser::{self, Serialize};

pub struct SerializeStruct<'a, 'b> {
    ser: &'a mut Serializer<'b>,
    #[cfg(feature = "timestamp")]
    ts_ser: TimestampSerializer,
}

impl<'a, 'b> SerializeStruct<'a, 'b> {
    pub(crate) fn new(ser: &'a mut Serializer<'b>) -> Self {
        SerializeStruct {
            ser,
            #[cfg(feature = "timestamp")]
            ts_ser: TimestampSerializer {
                seconds: 0,
                nanoseconds: 0,
            },
        }
    }
}

impl<'a, 'b> ser::SerializeStruct for SerializeStruct<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where T: ser::Serialize {
        match self.ser.state {
            super::State::Normal => {
                key.serialize(&mut *self.ser)?;
                value.serialize(&mut *self.ser)?;
            }
            #[cfg(feature = "ext")]
            super::State::Ext(_) => match key {
                // Special handling to support serializing MsgPack Ext type.
                // In this case keys are not serialized. Only the values for Ext type and data
                // are written without markers.
                crate::ext::FIELD_TYPE_NAME => return value.serialize(&mut *self.ser),
                crate::ext::FIELD_DATA_NAME => return value.serialize(&mut *self.ser),
                _ => {} //TODO ignore all other fields or error?
            },
            #[cfg(feature = "timestamp")]
            super::State::Timestamp(s, ns) => {
                match key {
                    crate::timestamp::FIELD_SECONDS_NAME => {
                        value.serialize(&mut self.ts_ser)?;
                        self.ser.state = super::State::Timestamp(Some(self.ts_ser.seconds), ns)
                    }
                    "nanoseconds" => {
                        value.serialize(&mut self.ts_ser)?;
                        self.ser.state = super::State::Timestamp(s, Some(self.ts_ser.nanoseconds))
                    }
                    _ => {} //TODO ignore all other fields or error?
                }
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.ser.state {
            super::State::Normal => Ok(()),
            #[cfg(feature = "ext")]
            super::State::Ext(_) => {
                self.ser.state = super::State::Normal;
                Ok(())
            }
            #[cfg(feature = "timestamp")]
            super::State::Timestamp(Some(s), Some(ns)) => {
                let ts = crate::timestamp::Timestamp::new(s, ns).unwrap();
                let mut buf = [0; 12];
                let ext = ts.to_ext(&mut buf)?;
                self.ser.state = super::State::Normal;
                ext.serialize(&mut *self.ser)
            }
            #[cfg(feature = "timestamp")]
            super::State::Timestamp(_, _) => {
                // This must not happen
                unreachable!()
            }
        }
    }
}

#[cfg(feature = "timestamp")]
struct TimestampSerializer {
    seconds: i64,
    nanoseconds: u32,
}

#[cfg(feature = "timestamp")]
impl<'a> ::serde::ser::Serializer for &'a mut TimestampSerializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Unreachable;
    type SerializeTuple = Unreachable;
    type SerializeTupleStruct = Unreachable;
    type SerializeTupleVariant = Unreachable;
    type SerializeMap = Unreachable;
    type SerializeStruct = Unreachable;
    type SerializeStructVariant = Unreachable;

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.seconds = v;
        Ok(())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.nanoseconds = v;
        Ok(())
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where T: Serialize {
        unreachable!()
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
    where T: Serialize {
        unreachable!()
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unreachable!() }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> { unreachable!() }
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unreachable!() }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unreachable!()
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { unreachable!() }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> { unreachable!() }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unreachable!()
    }
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where T: core::fmt::Display {
        unreachable!()
    }
}

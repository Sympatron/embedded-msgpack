use self::{map::SerializeMap, seq::SerializeSeq, struct_::SerializeStruct};
use super::Error;
use crate::encode::SerializeIntoSlice;

mod map;
mod seq;
mod struct_;

pub(crate) struct Serializer<'a> {
    buf: &'a mut [u8],
    pos: usize,
    state: State,
}

enum State {
    Normal,
    #[cfg(feature = "ext")]
    Ext(Option<i8>),
    #[cfg(feature = "timestamp")]
    Timestamp(Option<i64>, Option<u32>),
}

impl<'a> Serializer<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        Serializer {
            buf,
            pos: 0,
            state: State::Normal,
        }
    }
    #[allow(clippy::clippy::needless_pass_by_value)]
    fn append<S: SerializeIntoSlice>(&mut self, value: S) -> Result<(), Error> {
        self.pos += value.write_into_slice(&mut self.buf[self.pos..])?;
        Ok(())
    }
}

impl<'a, 'b> ::serde::ser::Serializer for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeSeq<'a, 'b>;
    type SerializeTuple = SerializeSeq<'a, 'b>;
    type SerializeTupleStruct = Unreachable;
    type SerializeTupleVariant = &'a mut Serializer<'b>;
    type SerializeMap = SerializeMap<'a, 'b>;
    type SerializeStruct = SerializeStruct<'a, 'b>;
    type SerializeStructVariant = Unreachable;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> { self.append(v) }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        match self.state {
            #[cfg(feature = "ext")]
            State::Ext(_) => {
                // serialize Ext type
                self.state = State::Ext(Some(v));
                Ok(())
            }
            _ => self.append(v),
        }
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> { self.append(v) }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> { self.append(v) }
    #[cfg(feature = "i64")]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> { self.append(v) }
    #[cfg(not(feature = "i64"))]
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> { self.append(v) }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> { self.append(v) }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> { self.append(v) }
    #[cfg(feature = "u64")]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> { self.append(v) }
    #[cfg(not(feature = "u64"))]
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> { self.append(v) }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> { self.append(v) }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut b = [0; 4];
        let v: &str = v.encode_utf8(&mut b);
        self.append(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> { self.append(v) }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.state {
            #[cfg(feature = "ext")]
            State::Ext(typ) => {
                let typ = typ.ok_or(Error::InvalidType)?;
                self.state = State::Normal;
                let ext = crate::Ext::new(typ, v);
                self.pos += (&ext).write_into_slice(&mut self.buf[self.pos..])?;
                Ok(())
            }
            _ => {
                let v = super::Binary::new(v);
                self.append(v)
            }
        }
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let x: Option<()> = None;
        self.append(x)
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where T: ::serde::ser::Serialize {
        v.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unreachable!() }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> { unreachable!() }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, v: &T) -> Result<Self::Ok, Self::Error>
    where T: ::serde::ser::Serialize {
        v.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ::serde::ser::Serialize,
    {
        unreachable!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.pos += crate::encode::serialize_array_start(len.ok_or(Error::InvalidType)?, &mut self.buf[self.pos..])?;
        Ok(SerializeSeq::new(self))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> { self.serialize_seq(Some(len)) }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unreachable!() }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.pos += super::serialize_map_start(len.ok_or(Error::InvalidType)?, &mut self.buf[self.pos..])?;
        Ok(SerializeMap::new(self))
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        match name {
            #[cfg(feature = "ext")]
            crate::ext::TYPE_NAME => {
                // special handling to support serializing MsgPack Ext type
                // SerializeStruct will reset `self.serializing_ext` when `end()` is called on it
                self.state = State::Ext(None);
            }
            #[cfg(feature = "timestamp")]
            crate::timestamp::TYPE_NAME => {
                self.state = State::Timestamp(None, None);
            }
            _ => {
                self.pos += super::serialize_map_start(len, &mut self.buf[self.pos..])?;
            }
        }
        Ok(SerializeStruct::new(self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unreachable!()
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where T: core::fmt::Display {
        unreachable!()
    }
}

/// Serializes the given data structure as a JSON byte vector
pub fn to_array<T>(value: &T, buf: &mut [u8]) -> Result<usize, Error>
where T: ::serde::ser::Serialize + ?Sized {
    let mut ser = Serializer::new(buf);
    value.serialize(&mut ser)?;
    Ok(ser.pos)
}

impl ::serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where T: core::fmt::Display {
        unreachable!()
    }
}

#[cfg(not(feature = "std"))]
impl ::serde::ser::StdError for Error {}

impl<'a, 'b> ::serde::ser::SerializeTupleVariant for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where T: ?Sized + serde::Serialize {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        self.state = State::Normal;
        Ok(())
    }
}

// impl<'a, 'b> ser::SerializeStruct for &'a mut Serializer<'b> {
//     type Ok = ();
//     type Error = Error;

//     fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
//     where T: ::serde::ser::Serialize {
//         key.serialize(&mut **self)?;
//         value.serialize(&mut **self)?;
//         Ok(())
//     }

//     fn end(self) -> Result<(), Error> {
//         self.serializing_ext = false;
//         Ok(())
//     }
// }

pub(crate) enum Unreachable {}

impl ::serde::ser::SerializeTupleStruct for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok, Self::Error> { unreachable!() }

    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl ::serde::ser::SerializeTupleVariant for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok, Self::Error> { unreachable!() }

    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl ::serde::ser::SerializeMap for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<Self::Ok, Self::Error>
    where T: ::serde::ser::Serialize {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok, Self::Error>
    where T: ::serde::ser::Serialize {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl ::serde::ser::SerializeStructVariant for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
    where T: ::serde::ser::Serialize {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl ::serde::ser::SerializeTuple for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where T: serde::Serialize {
        unreachable!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl ::serde::ser::SerializeSeq for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where T: serde::Serialize {
        unreachable!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl ::serde::ser::SerializeStruct for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where T: serde::Serialize {
        unreachable!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

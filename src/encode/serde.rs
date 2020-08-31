use serde::ser;

// use heapless::{consts::*, String, Vec};

use self::map::SerializeMap;
use self::seq::SerializeSeq;
use self::struct_::SerializeStruct;

mod map;
mod seq;
mod struct_;

use crate::encode::Serializable;
use crate::Error;

pub(crate) struct Serializer<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> Serializer<'a> {
    // fn new(buf: &'a mut [u8]) -> Self {
    //     Serializer { buf: buf, pos: 0 }
    // }
    fn append<S: Serializable>(&mut self, value: S) -> Result<(), Error> {
        self.pos += value.write_into(&mut self.buf[self.pos..])?;
        Ok(())
    }
}

impl<'a> ser::Serializer for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeSeq<'a>;
    type SerializeTuple = SerializeSeq<'a>;
    type SerializeTupleStruct = Unreachable;
    type SerializeTupleVariant = Unreachable;
    type SerializeMap = SerializeMap<'a, 'a>;
    type SerializeStruct = SerializeStruct<'a>;
    type SerializeStructVariant = Unreachable;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut b = [0; 4];
        let v: &str = v.encode_utf8(&mut b);
        self.append(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let v = super::Binary::new(v);
        self.append(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let x: Option<()> = None;
        self.append(x)
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        v.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
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
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.pos += crate::encode::serialize_array_start(
            len.ok_or(Error::InvalidType)?,
            &mut self.buf[self.pos..],
        )?;
        Ok(SerializeSeq::new(self))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.pos += crate::encode::serialize_array_start(len, &mut self.buf[self.pos..])?;
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unreachable!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unreachable!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
        // self.buf.push(b'{')?;
        // Ok(SerializeMap::new(self))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
        // self.buf.push(b'{')?;
        // Ok(SerializeStruct::new(self))
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
    where
        T: core::fmt::Display,
    {
        unreachable!()
    }
}

/// Serializes the given data structure as a string of JSON text
// pub fn to_string<B, T>(value: &T) -> Result<String<B>>
// where
//     B: heapless::ArrayLength<u8>,
//     T: ser::Serialize + ?Sized,
// {
//     let mut ser = Serializer::new();
//     value.serialize(&mut ser)?;
//     Ok(unsafe { String::from_utf8_unchecked(ser.buf) })
// }

// /// Serializes the given data structure as a JSON byte vector
// pub fn to_vec<B, T>(value: &T) -> Result<Vec<u8, B>>
// where
//     B: heapless::ArrayLength<u8>,
//     T: ser::Serialize + ?Sized,
// {
//     let mut ser = Serializer::new();
//     value.serialize(&mut ser)?;
//     Ok(ser.buf)
// }

impl ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        unreachable!()
    }
}

pub(crate) enum Unreachable {}

impl ser::SerializeTupleStruct for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl ser::SerializeTupleVariant for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl ser::SerializeMap for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl ser::SerializeStructVariant for Unreachable {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

// #[cfg(test)]
// mod tests {
//     use serde_derive::Serialize;

//     use heapless::consts::U128;

//     type N = U128;

//     #[test]
//     fn array() {
//         assert_eq!(&*crate::to_string::<N, _>(&[0, 1, 2]).unwrap(), "[0,1,2]");
//     }

//     #[test]
//     fn bool() {
//         assert_eq!(&*crate::to_string::<N, _>(&true).unwrap(), "true");
//     }

//     #[test]
//     fn enum_() {
//         #[derive(Serialize)]
//         enum Type {
//             #[serde(rename = "boolean")]
//             Boolean,
//             #[serde(rename = "number")]
//             Number,
//         }

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Type::Boolean).unwrap(),
//             r#""boolean""#
//         );

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Type::Number).unwrap(),
//             r#""number""#
//         );
//     }

//     #[test]
//     fn str() {
//         assert_eq!(&*crate::to_string::<N, _>("hello").unwrap(), r#""hello""#);
//     }

//     #[test]
//     fn struct_bool() {
//         #[derive(Serialize)]
//         struct Led {
//             led: bool,
//         }

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Led { led: true }).unwrap(),
//             r#"{"led":true}"#
//         );
//     }

//     #[test]
//     fn struct_i8() {
//         #[derive(Serialize)]
//         struct Temperature {
//             temperature: i8,
//         }

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Temperature { temperature: 127 }).unwrap(),
//             r#"{"temperature":127}"#
//         );

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Temperature { temperature: 20 }).unwrap(),
//             r#"{"temperature":20}"#
//         );

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Temperature { temperature: -17 }).unwrap(),
//             r#"{"temperature":-17}"#
//         );

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Temperature { temperature: -128 }).unwrap(),
//             r#"{"temperature":-128}"#
//         );
//     }

//     #[test]
//     fn struct_f32() {
//         #[derive(Serialize)]
//         struct Temperature {
//             temperature: f32,
//         }

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Temperature { temperature: -20. }).unwrap(),
//             r#"{"temperature":-2e1}"#
//         );

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Temperature {
//                 temperature: -20345.
//             })
//             .unwrap(),
//             r#"{"temperature":-2.0345e4}"#
//         );

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Temperature {
//                 temperature: -2.3456789012345e-23
//             })
//             .unwrap(),
//             r#"{"temperature":-2.3456788e-23}"#
//         );
//     }

//     #[test]
//     fn struct_option() {
//         #[derive(Serialize)]
//         struct Property<'a> {
//             description: Option<&'a str>,
//         }

//         assert_eq!(
//             crate::to_string::<N, _>(&Property {
//                 description: Some("An ambient temperature sensor"),
//             })
//             .unwrap(),
//             r#"{"description":"An ambient temperature sensor"}"#
//         );

//         // XXX Ideally this should produce "{}"
//         assert_eq!(
//             crate::to_string::<N, _>(&Property { description: None }).unwrap(),
//             r#"{"description":null}"#
//         );
//     }

//     #[test]
//     fn struct_u8() {
//         #[derive(Serialize)]
//         struct Temperature {
//             temperature: u8,
//         }

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Temperature { temperature: 20 }).unwrap(),
//             r#"{"temperature":20}"#
//         );
//     }

//     #[test]
//     fn struct_() {
//         #[derive(Serialize)]
//         struct Empty {}

//         assert_eq!(&*crate::to_string::<N, _>(&Empty {}).unwrap(), r#"{}"#);

//         #[derive(Serialize)]
//         struct Tuple {
//             a: bool,
//             b: bool,
//         }

//         assert_eq!(
//             &*crate::to_string::<N, _>(&Tuple { a: true, b: false }).unwrap(),
//             r#"{"a":true,"b":false}"#
//         );
//     }
// }

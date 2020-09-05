use crate::marker::Marker;
use core::fmt;
use paste::paste;
use serde::de::{self, Visitor};

use self::enum_::UnitVariantAccess;
use self::map::MapAccess;
use self::seq::SeqAccess;

mod enum_;
mod map;
mod seq;

use super::Error;

#[cfg(debug_assertions)]
fn print_debug<T>(function_name: &str, de: &Deserializer) {
    extern crate std;
    use std::println;
    println!(
        "{}<{}> ({:?})",
        function_name,
        core::any::type_name::<T>(),
        &de.slice[de.index..core::cmp::min(de.slice.len(), de.index + 10)]
    );
}
#[cfg(debug_assertions)]
fn print_debug_value<T, V: core::fmt::Debug>(function_name: &str, de: &Deserializer, value: &V) {
    extern crate std;
    use std::println;
    println!(
        "{}<{}> => {:?}   ({:?})",
        function_name,
        core::any::type_name::<T>(),
        value,
        &de.slice[de.index..core::cmp::min(de.slice.len(), de.index + 10)]
    );
}
#[cfg(not(debug_assertions))]
fn print_debug<T>(_function_name: &str, _de: &Deserializer) {}
#[cfg(not(debug_assertions))]
fn print_debug_value<T, V: core::fmt::Debug>(_function_name: &str, _de: &Deserializer, _value: &V) {
}

pub(crate) struct Deserializer<'b> {
    slice: &'b [u8],
    index: usize,
}

impl<'a> Deserializer<'a> {
    pub fn new(slice: &'a [u8]) -> Deserializer<'_> {
        Deserializer { slice, index: 0 }
    }

    fn eat_byte(&mut self) {
        self.index += 1;
    }

    fn peek(&mut self) -> Option<Marker> {
        Some(Marker::from_u8(*self.slice.get(self.index)?))
    }
}

// NOTE(deserialize_*signed) we avoid parsing into u64 and then casting to a smaller integer, which
// is what upstream does, to avoid pulling in 64-bit compiler intrinsics, which waste a few KBs of
// Flash, when targeting non 64-bit architectures
macro_rules! deserialize_type {
    ($self:ident, $visitor:ident, $ty:ident) => {{
        let (value, len) = paste! { super::[<read_ $ty>](&$self.slice[$self.index..])? };
        $self.index += len;
        print_debug_value::<$ty, $ty>(
            stringify!(concat_idents!(Deserializer::deserialize_, $ty)),
            &$self,
            &value,
        );
        paste! { $visitor.[<visit_ $ty>](value) }
    }};
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    /// Unsupported. Can’t parse a value without knowing its expected type.
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_any", &self);
        unreachable!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_bool", &self);
        let peek = self.peek();
        if let Some(marker) = peek {
            match marker {
                Marker::True => visitor.visit_bool(true),
                Marker::False => visitor.visit_bool(false),
                _ => Err(Error::InvalidType),
            }
        } else {
            Err(Error::EndOfBuffer)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_i8", &self);
        deserialize_type!(self, visitor, i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_i16", &self);
        deserialize_type!(self, visitor, i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_i32", &self);
        deserialize_type!(self, visitor, i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_i64", &self);
        deserialize_type!(self, visitor, i64)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_u8", &self);
        deserialize_type!(self, visitor, u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_u16", &self);
        deserialize_type!(self, visitor, u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_u32", &self);
        deserialize_type!(self, visitor, u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_u64", &self);
        deserialize_type!(self, visitor, u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_f32", &self);
        deserialize_type!(self, visitor, f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_f64", &self);
        deserialize_type!(self, visitor, f64)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_char", &self);
        unreachable!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_str", &self);
        let (s, len) = crate::decode::read_str(&self.slice[self.index..])?;
        self.index += len;
        visitor.visit_borrowed_str(s)
    }

    /// Unsupported. String is not available in no-std.
    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_string", &self);
        unreachable!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_bytes", &self);
        let (value, len) = super::read_bin(&self.slice[self.index..])?;
        self.index += len;
        visitor.visit_borrowed_bytes(value)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_byte_buf", &self);
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_option", &self);
        let marker = self.peek().ok_or(Error::EndOfBuffer)?;
        match marker {
            Marker::Null => {
                self.eat_byte();
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    /// Unsupported. Use a more specific deserialize_* method
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_unit", &self);
        unreachable!()
    }

    /// Unsupported. Use a more specific deserialize_* method
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_unit_struct", &self);
        unreachable!()
    }

    /// Unsupported. We can’t parse newtypes because we don’t know the underlying type.
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_newtype_struct", &self);
        unreachable!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_seq", &self);
        let (len, header_len) = crate::decode::read_array_len(&self.slice[self.index..])?;
        self.index += header_len;
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_tuple", &self);
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_tuple_struct", &self);
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_map", &self);
        let (len, header_len) = crate::decode::read_map_len(&self.slice[self.index..])?;
        self.index += header_len;
        visitor.visit_map(MapAccess::new(self, len))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_struct", &self);
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_enum", &self);
        visitor.visit_enum(UnitVariantAccess::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_identifier", &self);
        self.deserialize_str(visitor)
    }

    /// Used to throw out fields from JSON objects that we don’t want to
    /// keep in our structs.
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        print_debug::<V>("Deserializer::deserialize_ignored_any", &self);
        todo!();
        // match self.peek() {
        //     Some(Marker::I8) => self.deserialize_i8(visitor),
        //     //TODO Remaining
        //     _ => visitor.visit_unit(),
        // }

        // match self.parse_whitespace().ok_or(Error::EofWhileParsingValue)? {
        //     b'"' => self.deserialize_str(visitor),
        //     b'[' => self.deserialize_seq(visitor),
        //     b'{' => self.deserialize_struct("ignored", &[], visitor),
        //     b',' | b'}' | b']' => Err(Error::ExpectedSomeValue),
        //     // If it’s something else then we chomp until we get to an end delimiter.
        //     // This does technically allow for illegal JSON since we’re just ignoring
        //     // characters rather than parsing them.
        //     _ => loop {
        //         match self.peek() {
        //             // The visitor is expected to be UnknownAny’s visitor, which
        //             // implements visit_unit to return its unit Ok result.
        //             Some(b',') | Some(b'}') | Some(b']') => break visitor.visit_unit(),
        //             Some(_) => self.eat_byte(),
        //             None => break Err(Error::EofWhileParsingString),
        //         }
        //     },
        // }
    }
}

impl de::Error for Error {
    #[cfg_attr(not(feature = "custom-error-messages"), allow(unused_variables))]
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        todo!()
        // #[cfg(not(feature = "custom-error-messages"))]
        // {
        //     Error::CustomError
        // }
        // #[cfg(feature = "custom-error-messages")]
        // {
        //     use core::fmt::Write;

        //     let mut string = heapless::String::new();
        //     write!(string, "{:.64}", msg).unwrap();
        //     Error::CustomErrorWithMessage(string)
        // }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
        // write!(
        //     f,
        //     "{}",
        //     match self {
        //         Error::EofWhileParsingList => "EOF while parsing a list.",
        //         Error::EofWhileParsingObject => "EOF while parsing an object.",
        //         Error::EofWhileParsingString => "EOF while parsing a string.",
        //         Error::EofWhileParsingValue => "EOF while parsing a JSON value.",
        //         Error::ExpectedColon => "Expected this character to be a `':'`.",
        //         Error::ExpectedListCommaOrEnd => {
        //             "Expected this character to be either a `','` or\
        //              a \
        //              `']'`."
        //         }
        //         Error::ExpectedObjectCommaOrEnd => {
        //             "Expected this character to be either a `','` \
        //              or a \
        //              `'}'`."
        //         }
        //         Error::ExpectedSomeIdent => {
        //             "Expected to parse either a `true`, `false`, or a \
        //              `null`."
        //         }
        //         Error::ExpectedSomeValue => "Expected this character to start a JSON value.",
        //         Error::InvalidNumber => "Invalid number.",
        //         Error::InvalidType => "Invalid type",
        //         Error::InvalidUnicodeCodePoint => "Invalid unicode code point.",
        //         Error::KeyMustBeAString => "Object key is not a string.",
        //         Error::TrailingCharacters => {
        //             "JSON has non-whitespace trailing characters after \
        //              the \
        //              value."
        //         }
        //         Error::TrailingComma => "JSON has a comma after the last value in an array or map.",
        //         Error::CustomError => "JSON does not match deserializer’s expected format.",
        //         #[cfg(feature = "custom-error-messages")]
        //         Error::CustomErrorWithMessage(msg) => msg.as_str(),
        //         _ => "Invalid JSON",
        //     }
        // )
    }
}

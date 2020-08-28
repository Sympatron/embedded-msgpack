use crate::marker::Marker;
use crate::Error;

use ::byteorder::{BigEndian, ByteOrder};
use ::core::convert::From;
use ::core::ops::Deref;
use ::num_traits::cast::FromPrimitive;

pub trait Serializable {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error>;
}

pub fn serialize_u8(value: u8, buf: &mut [u8]) -> Result<usize, Error> {
    if value < 0x80 {
        if buf.len() < 1 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = value;
        return Ok(1);
    } else {
        if buf.len() < 2 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::U8.to_u8();
        buf[1] = value;
        return Ok(2);
    }
}
pub fn serialize_u16(value: u16, buf: &mut [u8]) -> Result<usize, Error> {
    if value <= u8::max_value() as u16 {
        return serialize_u8(value as u8, buf);
    } else {
        if buf.len() < 3 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::U16.to_u8();
        BigEndian::write_u16(&mut buf[1..], value);
        return Ok(3);
    }
}
pub fn serialize_u32(value: u32, buf: &mut [u8]) -> Result<usize, Error> {
    if value <= u16::max_value() as u32 {
        return serialize_u16(value as u16, buf);
    } else {
        if buf.len() < 5 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::U32.to_u8();
        BigEndian::write_u32(&mut buf[1..], value);
        return Ok(5);
    }
}
#[cfg(feature = "u64")]
pub fn serialize_u64(value: u64, buf: &mut [u8]) -> Result<usize, Error> {
    if value <= u32::max_value() as u64 {
        return serialize_u32(value as u32, buf);
    } else {
        if buf.len() < 9 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::U64.to_u8();
        BigEndian::write_u64(&mut buf[1..], value);
        return Ok(9);
    }
}
pub fn serialize_i8(value: i8, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 1 {
        return Err(Error::EndOfBuffer);
    }
    match value {
        -32..=-1 => {
            buf[0] = Marker::FixNeg(value).to_u8();
            Ok(1)
        }
        0..=0x7f => {
            buf[0] = Marker::FixPos(value as u8).to_u8();
            Ok(1)
        }
        // -32..0x7f => {
        //     if buf.len() < 1 {
        //         return Err(Error::EndOfBuffer);
        //     }
        //     buf[0] = value as u8;
        //     Ok(1)
        // }
        _ => {
            if buf.len() < 2 {
                return Err(Error::EndOfBuffer);
            }
            buf[0] = Marker::I8.to_u8();
            buf[1] = value as u8;
            Ok(2)
        }
    }
}
pub fn serialize_i16(value: i16, buf: &mut [u8]) -> Result<usize, Error> {
    // if value <= i8::max_value() as i16 && value >= i8::min_value() as i16 {
    if let Some(value) = i8::from_i16(value) {
        return serialize_i8(value as i8, buf);
    } else {
        if buf.len() < 3 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::I16.to_u8();
        BigEndian::write_i16(&mut buf[1..], value);
        return Ok(3);
    }
}
pub fn serialize_i32(value: i32, buf: &mut [u8]) -> Result<usize, Error> {
    // if value <= i16::max_value() as i32 && value >= i16::min_value() as i32 {
    if let Some(value) = i16::from_i32(value) {
        return serialize_i16(value as i16, buf);
    } else {
        if buf.len() < 5 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::I32.to_u8();
        BigEndian::write_i32(&mut buf[1..], value);
        return Ok(5);
    }
}
#[cfg(feature = "i64")]
pub fn serialize_i64(value: i64, buf: &mut [u8]) -> Result<usize, Error> {
    if let Some(value) = i32::from_i64(value) {
        return serialize_i32(value, buf);
    } else {
        if buf.len() < 9 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::I64.to_u8();
        BigEndian::write_i64(&mut buf[1..], value);
        return Ok(9);
    }
}

impl Serializable for u8 {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        serialize_u8(*self, buf)
    }
}
impl Serializable for u16 {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        serialize_u16(*self, buf)
    }
}
impl Serializable for u32 {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        serialize_u32(*self, buf)
    }
}
#[cfg(feature = "u64")]
impl Serializable for u64 {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        serialize_u64(*self, buf)
    }
}
impl Serializable for i8 {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        serialize_i8(*self, buf)
    }
}
impl Serializable for i16 {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        serialize_i16(*self, buf)
    }
}
impl Serializable for i32 {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        serialize_i32(*self, buf)
    }
}
#[cfg(feature = "i64")]
impl Serializable for i64 {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        serialize_i64(*self, buf)
    }
}

impl Serializable for bool {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.len() < 1 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = if *self {
            Marker::True.to_u8()
        } else {
            Marker::False.to_u8()
        };
        Ok(1)
    }
}

impl<T> Serializable for Option<T>
where
    T: Serializable,
{
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if let Some(value) = self {
            Serializable::write_into(value, buf)
        } else {
            if buf.len() < 1 {
                return Err(Error::EndOfBuffer);
            }
            buf[0] = Marker::Null.to_u8();
            Ok(1)
        }
    }
}

#[repr(transparent)]
pub struct Binary<'a>(&'a [u8]);
impl<'a> Binary<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Binary(slice)
    }
}
impl<'a> Deref for Binary<'a> {
    type Target = &'a [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> From<&'a [u8]> for Binary<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Binary(slice)
    }
}

impl<'a> Serializable for Binary<'a> {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let n = self.len();
        match n {
            0..=0xff => {
                if buf.len() < 2 + n {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = Marker::Bin8.to_u8();
                buf[1] = n as u8;
                buf[2..(2 + n)].clone_from_slice(self);
                Ok(2 + n)
            }
            0x100..=0xffff => {
                if buf.len() < 3 + n {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = Marker::Bin16.to_u8();
                BigEndian::write_u16(&mut buf[1..], n as u16);
                buf[3..(3 + n)].clone_from_slice(self);
                Ok(3 + n)
            }
            #[cfg(feature = "bin32")]
            0x10000..=0xffffffff => {
                if buf.len() < 5 + n {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = Marker::Bin32.to_u8();
                BigEndian::write_u32(&mut buf[1..], n as u32);
                buf[5..(5 + n)].clone_from_slice(self);
                Ok(5 + n)
            }
            _ => unimplemented!(),
        }
    }
}

impl<K, V> Serializable for &(K, V)
where
    K: Serializable,
    V: Serializable,
{
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let index = serialize_map_kay_value(&self.0, &self.1, buf)?;
        Ok(index)
    }
}

impl Serializable for &str {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let n = self.len();
        match n {
            // FIXSTR_SIZE
            0..=0x1f => {
                let header_len = 1;
                if buf.len() < header_len + n {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = Marker::FixStr(n as u8).to_u8();
                buf[header_len..(header_len + n)].clone_from_slice(self.as_bytes());
                Ok(header_len + n)
            }
            0x20..=0xff => {
                let header_len = 2;
                if buf.len() < header_len + n {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = Marker::Str8.to_u8();
                buf[1] = n as u8;
                buf[header_len..(header_len + n)].clone_from_slice(self.as_bytes());
                Ok(header_len + n)
            }
            #[cfg(feature = "str16")]
            0x100..=0xffff => {
                let header_len = 3;
                if buf.len() < header_len + n {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = Marker::Str8.to_u8();
                BigEndian::write_u16(&mut buf[1..], n as u16);
                buf[header_len..(header_len + n)].clone_from_slice(self.as_bytes());
                Ok(header_len + n)
            }
            #[cfg(feature = "str32")]
            0x10000..=0xffffffff => {
                let header_len = 5;
                if buf.len() < header_len + n {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = Marker::Str8.to_u8();
                BigEndian::write_u32(&mut buf[1..], n as u32);
                buf[header_len..(header_len + n)].clone_from_slice(self.as_bytes());
                Ok(header_len + n)
            }
            _ => unimplemented!(),
        }
    }
}

impl<K, V> Serializable for &[(K, V)]
where
    K: Serializable,
    V: Serializable,
{
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        // serialize_sequence(self, SeuqenceType::Map, buf)
        let mut index = serialize_map_start(self.len(), buf)?;
        for kv in self.iter() {
            index += kv.write_into(&mut buf[index..])?;
        }
        Ok(index)
    }
}

impl<T> Serializable for &[T]
where
    T: Serializable,
{
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        // serialize_sequence(self, SeuqenceType::Array, buf)
        let mut index = serialize_array_start(self.len(), buf)?;
        for i in self.iter() {
            index += Serializable::write_into(i, &mut buf[index..])?;
        }
        Ok(index)
    }
}

pub enum SeuqenceType {
    Array,
    Map,
}
impl SeuqenceType {
    pub fn serialize_start(&self, n: usize, buf: &mut [u8]) -> Result<usize, Error> {
        match *self {
            SeuqenceType::Array => serialize_array_start(n, buf),
            SeuqenceType::Map => serialize_map_start(n, buf),
        }
    }
}

pub fn serialize_sequence<T: Serializable>(
    seq: &[T],
    typ: SeuqenceType,
    buf: &mut [u8],
) -> Result<usize, Error> {
    let mut index = typ.serialize_start(seq.len(), buf)?;
    for i in seq.iter() {
        index += Serializable::write_into(i, &mut buf[index..])?;
    }
    Ok(index)
}

pub fn serialize_array_start(n: usize, buf: &mut [u8]) -> Result<usize, Error> {
    if n <= crate::marker::FIXARRAY_SIZE as usize {
        if buf.len() < 1 + n {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::FixArray(n as u8).to_u8();
        Ok(1)
    } else {
        #[cfg(feature = "array16")]
        if n <= u16::max_value() as usize {
            buf[0] = Marker::Array16.to_u8();
            BigEndian::write_u16(&mut buf[1..], n as u16);
            return Ok(3);
        }
        #[cfg(feature = "array32")]
        if n <= u32::max_value() as usize {
            buf[0] = Marker::Array32.to_u8();
            BigEndian::write_u32(&mut buf[1..], n as u32);
            return Ok(5);
        }
        unimplemented!()
    }
}

pub fn serialize_map_start(n: usize, buf: &mut [u8]) -> Result<usize, Error> {
    if n <= crate::marker::FIXMAP_SIZE as usize {
        if buf.len() < 1 + n {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::FixMap(n as u8).to_u8();
        Ok(1)
    } else {
        #[cfg(feature = "map16")]
        if n <= u16::max_value() as usize {
            buf[0] = Marker::Map16.to_u8();
            BigEndian::write_u16(&mut buf[1..], n as u16);
            return Ok(3);
        }
        #[cfg(feature = "map32")]
        if n <= u32::max_value() as usize {
            buf[0] = Marker::Map32.to_u8();
            BigEndian::write_u32(&mut buf[1..], n as u32);
            return Ok(5);
        }
        unimplemented!()
    }
}
pub fn serialize_map_kay_value<K: Serializable, V: Serializable>(
    key: &K,
    value: &V,
    buf: &mut [u8],
) -> Result<usize, Error> {
    let mut index = 0;
    index += Serializable::write_into(key, &mut buf[index..])?;
    index += Serializable::write_into(value, &mut buf[index..])?;
    Ok(index)
}

#[cfg(feature = "serde")]
mod serde;

use crate::marker::Marker;

use ::byteorder::{BigEndian, ByteOrder};
use ::zerocopy::ByteSlice;
// use ::zerocopy::byteorder::{U16, U32, U64, I16, I32, I64};
use ::num_traits::cast::FromPrimitive;
// use core::convert::TryInto;

#[derive(Debug)]
pub enum Error {
    EndOfBuffer,
    OutOfBounds,
    InvalidType,
}

#[cfg(feature = "serde")]
// #[inline(never)]
pub fn from_slice<'a, T: ::serde::de::Deserialize<'a>>(buf: &'a [u8]) -> Result<T, Error> {
    let mut de = serde::Deserializer::new(buf);
    let value = ::serde::de::Deserialize::deserialize(&mut de)?;

    Ok(value)
}

pub fn read_int<B: ByteSlice, T: FromPrimitive>(buf: B) -> Result<(T, usize), Error> {
    match read_u64(buf) {
        Ok((v, len)) => {
            if let Some(v) = T::from_u64(v) {
                Ok((v, len))
            } else {
                Err(Error::OutOfBounds)
            }
        }
        Err(kind) => Err(kind),
    }
}
pub fn read_sint<B: ByteSlice, T: FromPrimitive>(buf: B) -> Result<(T, usize), Error> {
    match read_i64(buf) {
        Ok((v, len)) => {
            if let Some(v) = T::from_i64(v) {
                Ok((v, len))
            } else {
                Err(Error::OutOfBounds)
            }
        }
        Err(kind) => Err(kind),
    }
}

#[inline(always)]
pub fn read_u8<B: ByteSlice>(buf: B) -> Result<(u8, usize), Error> {
    read_int(buf)
}
#[inline(always)]
pub fn read_u16<B: ByteSlice>(buf: B) -> Result<(u16, usize), Error> {
    read_int(buf)
}
#[inline(always)]
pub fn read_u32<B: ByteSlice>(buf: B) -> Result<(u32, usize), Error> {
    read_int(buf)
}

#[inline(always)]
pub fn read_i8<B: ByteSlice>(buf: B) -> Result<(i8, usize), Error> {
    read_sint(buf)
}
#[inline(always)]
pub fn read_i16<B: ByteSlice>(buf: B) -> Result<(i16, usize), Error> {
    read_sint(buf)
}
#[inline(always)]
pub fn read_i32<B: ByteSlice>(buf: B) -> Result<(i32, usize), Error> {
    read_sint(buf)
}

pub fn read_u64<B: ByteSlice>(buf: B) -> Result<(u64, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        // Nur u64 muss hier gesondert behandelt werden, weil es der einzige Typ ist, der potentiell nicht in i64 passt
        Marker::U64 => {
            if buf.len() >= 9 {
                Ok((BigEndian::read_u64(&buf[1..9]) as u64, 9))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => match read_i64(buf) {
            Ok((i, l)) => {
                if let Some(u) = u64::from_i64(i) {
                    Ok((u, l))
                } else {
                    Err(Error::OutOfBounds)
                }
            }
            Err(kind) => Err(kind),
        },
    }
}

pub fn read_i64<B: ByteSlice>(buf: B) -> Result<(i64, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        Marker::FixPos(val) => Ok((val as i64, 1)),
        Marker::FixNeg(val) => Ok((val as i64, 1)),

        Marker::U8 => {
            if buf.len() >= 2 {
                Ok((buf[1] as i64, 2))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::U16 => {
            if buf.len() >= 3 {
                Ok((BigEndian::read_u16(&buf[1..3]) as i64, 3))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::U32 => {
            if buf.len() >= 5 {
                Ok((BigEndian::read_u32(&buf[1..5]) as i64, 5))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::U64 => {
            if buf.len() >= 9 {
                let u = BigEndian::read_u64(&buf[1..9]);
                if let Some(i) = i64::from_u64(u) {
                    Ok((i, 9))
                } else {
                    Err(Error::OutOfBounds)
                }
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::I8 => {
            if buf.len() >= 2 {
                Ok((buf[1] as i64, 2))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::I16 => {
            if buf.len() >= 3 {
                Ok((BigEndian::read_i16(&buf[1..3]) as i64, 3))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::I32 => {
            if buf.len() >= 5 {
                Ok((BigEndian::read_i32(&buf[1..5]) as i64, 5))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::I64 => {
            if buf.len() >= 9 {
                Ok((BigEndian::read_i64(&buf[1..9]) as i64, 9))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => Err(Error::EndOfBuffer),
    }
}

pub fn read_f32<B: ByteSlice>(buf: B) -> Result<(f32, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        Marker::F32 => {
            if buf.len() >= 5 {
                Ok((BigEndian::read_f32(&buf[1..5]), 5))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => Err(Error::EndOfBuffer),
    }
}
pub fn read_f64<B: ByteSlice>(buf: B) -> Result<(f64, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        Marker::F32 => {
            if buf.len() >= 5 {
                let v = BigEndian::read_f32(&buf[1..5]);
                Ok((v as f64, 5))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::F64 => {
            if buf.len() >= 9 {
                Ok((BigEndian::read_f64(&buf[1..9]), 9))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => Err(Error::EndOfBuffer),
    }
}

pub fn read_bin<'a, B: ByteSlice>(buf: B) -> Result<(B, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        Marker::FixStr(len) => {
            let header_len = 1;
            let len = len as usize;
            if buf.len() >= header_len + len {
                let (_head, rest) = buf.split_at(header_len);
                let (bin, _rest) = rest.split_at(len);
                Ok((bin, header_len + len))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::Bin8 | Marker::Str8 => {
            let header_len = 2;
            if let Some(&len) = buf.get(1) {
                let len = len as usize;
                if buf.len() >= header_len + len {
                    let (_head, rest) = buf.split_at(header_len);
                    let (bin, _rest) = rest.split_at(len);
                    Ok((bin, header_len + len))
                } else {
                    Err(Error::EndOfBuffer)
                }
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::Bin16 | Marker::Str16 => {
            let header_len = 3;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u16(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                let (_head, rest) = buf.split_at(header_len);
                let (bin, _rest) = rest.split_at(len);
                Ok((bin, header_len + len))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        #[cfg(feature = "bin32")]
        Marker::Bin32 | Marker::Str32 => {
            let header_len = 5;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u32(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                let (head, rest) = buf.split_at(header_len);
                let (bin, rest) = rest.split_at(len);
                Ok((bin, header_len + len))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => Err(Error::InvalidType),
    }
}

pub fn read_str<'a>(buf: &'a [u8]) -> Result<(&'a str, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    let (header_len, len) = match marker {
        Marker::FixStr(len) => {
            let header_len = 1;
            let len = len as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        Marker::Str8 => {
            let header_len = 2;
            if let Some(&len) = buf.get(1) {
                let len = len as usize;
                if buf.len() >= header_len + len {
                    (header_len, len)
                } else {
                    return Err(Error::EndOfBuffer);
                }
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        Marker::Str16 => {
            let header_len = 3;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u16(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        #[cfg(feature = "str32")]
        Marker::Str32 => {
            let header_len = 5;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u32(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        _ => return Err(Error::InvalidType),
    };
    let buf = &buf[header_len..header_len + len];
    let s = if buf.is_ascii() {
        // This is safe because all ASCII characters are valid UTF-8 characters
        unsafe { core::str::from_utf8_unchecked(buf) }
    } else {
        return Err(Error::InvalidType);
    };
    Ok((s, header_len + len))
}

pub fn read_array_len<B: ByteSlice>(buf: B) -> Result<(usize, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    let (header_len, len) = match marker {
        Marker::FixArray(len) => {
            let header_len = 1;
            let len = len as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        #[cfg(feature = "array16")]
        Marker::Array16 => {
            let header_len = 3;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u16(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        #[cfg(feature = "array32")]
        Marker::Array32 => {
            let header_len = 5;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u32(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        _ => return Err(Error::InvalidType),
    };
    Ok((len, header_len))
}

pub fn read_map_len<B: ByteSlice>(buf: B) -> Result<(usize, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    let (header_len, len) = match marker {
        Marker::FixMap(len) => {
            let header_len = 1;
            let len = len as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        #[cfg(feature = "map16")]
        Marker::Map16 => {
            let header_len = 3;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u16(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        #[cfg(feature = "map32")]
        Marker::Map32 => {
            let header_len = 5;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u32(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        _ => return Err(Error::InvalidType),
    };
    Ok((len, header_len))
}

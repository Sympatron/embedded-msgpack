use crate::marker::Marker;
use crate::Error;

use ::byteorder::{BigEndian, ByteOrder};
use ::zerocopy::ByteSlice;
// use ::zerocopy::byteorder::{U16, U32, U64, I16, I32, I64};
use ::num_traits::cast::FromPrimitive;
// use core::convert::TryInto;

#[derive(Debug, PartialEq, Eq)]
pub enum Value<'a> {
    Nil,
    Bool(bool),

    U8(u8),
    U16(&'a [u8]),
    U32(&'a [u8]),
    U64(&'a [u8]),

    I8(i8),
    I16(&'a [u8]),
    I32(&'a [u8]),
    I64(&'a [u8]),

    //TODO F32(f32),
    //TODO F64(f64),
    Bin(&'a [u8]),
    Str(&'a [u8]),
    // Ext(u32, &'a [u8]),
    // Array(&'a [Value<'a>]),
    // Map(&'a [(Value<'a>, Value<'a>)]),
}

// fn to_arr16<'a, B: ByteSlice>(buf: &'a B) -> &'a [u8; 2] {
//     let ptr = buf.as_ptr();
//     unsafe { ptr as &[u8; 2] }
// }
fn to_slice<'a, B: ByteSlice>(buf: &'a B, offset: usize, n: usize) -> Option<&'a [u8]> {
    if buf.len() < n + offset {
        return None;
    }
    Some(&buf[offset..offset + n])
    // let ptr = buf.as_ptr();
    // unsafe { Some(core::slice::from_raw_parts(ptr.offset(offset as isize), n)) }
}

impl<'a> Value<'a> {
    pub fn read<B: ByteSlice>(buf: &'a B) -> Result<(Value<'a>, usize), Error> {
        if buf.len() == 0 {
            return Err(Error::EndOfBuffer);
        }

        let marker = Marker::from(buf[0]);
        match marker {
            Marker::FixPos(val) => Ok((Value::U8(val), 1)),
            Marker::FixNeg(val) => Ok((Value::I8(val), 1)),

            Marker::Null => unimplemented!(),

            Marker::True => unimplemented!(),
            Marker::False => unimplemented!(),

            Marker::U8 => Ok((Value::U8(buf[1]), 2)),
            Marker::U16 => Ok((Value::U16(to_slice(buf, 1, 2).unwrap()), 3)),
            Marker::U32 => unimplemented!(),
            Marker::U64 => unimplemented!(),

            Marker::I8 => unimplemented!(),
            Marker::I16 => unimplemented!(),
            Marker::I32 => unimplemented!(),
            Marker::I64 => unimplemented!(),

            Marker::F32 => unimplemented!(),
            Marker::F64 => unimplemented!(),

            Marker::FixStr(_len) => unimplemented!(),
            Marker::Str8 => unimplemented!(),
            Marker::Str16 => unimplemented!(),
            Marker::Str32 => unimplemented!(),

            Marker::Bin8 => unimplemented!(),
            Marker::Bin16 => unimplemented!(),
            Marker::Bin32 => unimplemented!(),

            Marker::FixArray(_len) => unimplemented!(),
            Marker::Array16 => unimplemented!(),
            Marker::Array32 => unimplemented!(),

            Marker::FixMap(_len) => unimplemented!(),
            Marker::Map16 => unimplemented!(),
            Marker::Map32 => unimplemented!(),

            Marker::FixExt1 => unimplemented!(),
            Marker::FixExt2 => unimplemented!(),
            Marker::FixExt4 => unimplemented!(),
            Marker::FixExt8 => unimplemented!(),
            Marker::FixExt16 => unimplemented!(),
            Marker::Ext8 => unimplemented!(),
            Marker::Ext16 => unimplemented!(),
            Marker::Ext32 => unimplemented!(),

            Marker::Reserved => unimplemented!(),
        }
    }
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

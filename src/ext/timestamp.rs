use byteorder::{BigEndian, ByteOrder};
use core::{
    convert::{TryFrom, TryInto},
    marker::PhantomData,
};

use crate::{encode::Serializable, Error, Ext};

const EXT_TIMESTAMP: i8 = -1;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanoseconds: u32,
    prevent_new: PhantomData<u8>,
}

impl Timestamp {
    pub fn new(seconds: i64, nanoseconds: u32) -> Result<Timestamp, Error> {
        if nanoseconds >= 1000000000 {
            return Err(Error::OutOfBounds);
        }
        Ok(Timestamp {
            seconds,
            nanoseconds,
            prevent_new: PhantomData::default(),
        })
    }
    pub fn into_ext<'a>(&self, buf: &'a mut [u8]) -> Result<Ext<'a>, Error> {
        if self.seconds >> 34 == 0 {
            let x = ((self.nanoseconds as u64) << 34) | self.seconds as u64;
            if x & 0xffffffff00000000u64 == 0 {
                // timestamp 32
                if buf.len() < 4 {
                    return Err(Error::EndOfBuffer);
                }
                BigEndian::write_u32(buf, x as u32);
                Ok(Ext::new(-1, &buf[0..4]))
            } else {
                // timestamp 64
                if buf.len() < 8 {
                    return Err(Error::EndOfBuffer);
                }
                BigEndian::write_u64(buf, x);
                Ok(Ext::new(-1, &buf[0..8]))
            }
        } else {
            // timestamp 96
            #[cfg(feature = "timestamp96")]
            {
                if buf.len() < 12 {
                    return Err(Error::EndOfBuffer);
                }
                BigEndian::write_u32(&mut buf[..], self.nanoseconds);
                BigEndian::write_i64(&mut buf[4..], self.seconds);
                return Ok(Ext::new(-1, &buf[0..12]));
                // serialize(0xc7, 12, -1, time.tv_nsec, time.tv_sec)
            }
            #[cfg(not(feature = "timestamp96"))]
            return Err(Error::InvalidType);
        }
    }
}

impl Serializable for Timestamp {
    fn write_into(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.seconds >> 34 == 0 {
            let x = ((self.nanoseconds as u64) << 34) | self.seconds as u64;
            if x & 0xffffffff00000000u64 == 0 {
                // timestamp 32
                if buf.len() < 6 {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = crate::marker::Marker::FixExt4.to_u8();
                buf[1] = -1i8 as u8;
                BigEndian::write_u32(&mut buf[2..], x as u32);
                Ok(6)
            // serialize(0xd6, -1, x as u32)
            } else {
                // timestamp 64
                if buf.len() < 10 {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = crate::marker::Marker::FixExt8.to_u8();
                buf[1] = -1i8 as u8;
                BigEndian::write_u64(&mut buf[2..], x);
                Ok(10)
                // serialize(0xd7, -1, x)
            }
        } else {
            #[cfg(feature = "timestamp96")]
            return {
                // timestamp 96
                if buf.len() < 12 {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = crate::marker::Marker::Ext8.to_u8();
                buf[1] = 12;
                buf[2] = -1i8 as u8;
                BigEndian::write_u32(&mut buf[3..], self.nanoseconds);
                BigEndian::write_i64(&mut buf[7..], self.seconds);
                Ok(15)
                // serialize(0xc7, 12, -1, self.nanoseconds, self.seconds)}
            };
            #[cfg(not(feature = "timestamp96"))]
            return Err(Error::InvalidType);
        }
    }
}

impl<'a> TryFrom<Ext<'a>> for Timestamp {
    type Error = Error;

    fn try_from(ext: Ext<'a>) -> Result<Self, Self::Error> {
        if ext.typ == EXT_TIMESTAMP {
            match ext.data.len() {
                4 => Timestamp::new(BigEndian::read_u32(ext.data) as i64, 0),
                8 => {
                    let value = BigEndian::read_u64(ext.data);
                    Timestamp::new((value & 0x00000003ffffffffu64) as i64, (value >> 34) as u32)
                }
                #[cfg(feature = "timestamp96")]
                12 => {
                    let nanos = BigEndian::read_u32(&ext.data[0..4]);
                    let s = BigEndian::read_i64(&ext.data[4..12]);
                    Timestamp::new(s, nanos)
                }
                _ => Err(Error::InvalidType),
            }
        } else {
            Err(Error::InvalidType)
        }
    }
}

// Does not make that much sense since it will be as big or bigger then Timestamp in memory...
pub struct TimestampRef<'a> {
    ext: Ext<'a>,
}

impl<'a> TryFrom<Ext<'a>> for TimestampRef<'a> {
    type Error = ();

    #[inline]
    fn try_from(ext: Ext<'a>) -> Result<Self, Self::Error> {
        if ext.typ == EXT_TIMESTAMP {
            Ok(TimestampRef { ext: ext })
        } else {
            Err(())
        }
    }
}

impl<'a> From<TimestampRef<'a>> for Timestamp {
    #[inline]
    fn from(ts: TimestampRef<'a>) -> Self {
        match ts.ext.try_into() {
            Ok(x) => x,
            _ => unreachable!(),
        }
    }
}
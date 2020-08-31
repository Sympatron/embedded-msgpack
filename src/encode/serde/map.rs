use serde::ser;

// use heapless::ArrayLength;

use super::{Error, Serializer};

pub struct SerializeMap<'ser, 'a> {
    ser: &'a mut Serializer<'ser>,
}

impl<'ser, 'a> SerializeMap<'ser, 'a> {
    pub(crate) fn new(ser: &'a mut Serializer<'ser>) -> Self {
        SerializeMap { ser }
    }
}

impl<'ser, 'a> ser::SerializeMap for SerializeMap<'ser, 'a> {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        // key.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        // value.serialize(&mut *self.ser)?;
        Ok(())
    }
}

use serde::ser::{self, Serialize};

// use heapless::ArrayLength;

use super::{Error, Serializer};

pub struct SerializeStruct<'a> {
    ser: &'a mut Serializer<'a>,
}

impl<'a> SerializeStruct<'a> {
    pub(crate) fn new(ser: &'a mut Serializer<'a>) -> Self {
        SerializeStruct { ser }
    }
}

impl<'a> ser::SerializeStruct for SerializeStruct<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        // key.serialize(&mut self.ser)?;
        // value.serialize(&mut self.ser)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

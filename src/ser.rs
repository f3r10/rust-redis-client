use serde::{ser, Serialize};

use super::error::RedisError;
use super::error::Result;

pub struct RedisSerializer {
    output: String
}


pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = RedisSerializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)

}
const CRLF_BYTES: &str = "\r\n";
impl<'a> ser::Serializer for &'a mut RedisSerializer {
    type Ok = ();
    type Error = RedisError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += &format!("+{}{}", &v.to_string(), CRLF_BYTES);
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output += &format!(":{}{}", &v.to_string(), CRLF_BYTES);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += &format!("${}\r\n{}\r\n", &v.len().to_string(), &v);
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        self.output += "$-1\r\n";
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        match _len {
            None => self.output += "*0\r\n",
            Some(l) => {
                let val = format!("*{}\r\n", l.to_string());
                self.output += &val;
            }
        }
        Ok(self)
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
}
impl<'a> ser::SerializeSeq for &'a mut RedisSerializer{
    type Ok = ();
    type Error = RedisError;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)

    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}
impl<'a> ser::SerializeTuple for &'a mut RedisSerializer{
    type Ok = ();
    type Error = RedisError;
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(RedisError::Message("unsupported type".to_string()))

    }
    fn end(self) -> Result<()> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
}
impl<'a> ser::SerializeTupleStruct for &'a mut RedisSerializer{

    type Ok = ();
    type Error = RedisError;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(RedisError::Message("unsupported type".to_string()))

    }
    fn end(self) -> Result<()> {
        Err(RedisError::Message("unsupported type".to_string()))
    }

}
impl<'a> ser::SerializeTupleVariant for &'a mut RedisSerializer{

    type Ok = ();
    type Error = RedisError;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(RedisError::Message("unsupported type".to_string()))

    }
    fn end(self) -> Result<()> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
}
impl<'a> ser::SerializeMap for &'a mut RedisSerializer{
    type Ok = ();
    type Error = RedisError;
    fn serialize_key<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(RedisError::Message("unsupported type".to_string()))

    }
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(RedisError::Message("unsupported type".to_string()))

    }
    fn end(self) -> Result<()> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
}
impl<'a> ser::SerializeStruct for &'a mut RedisSerializer{

    type Ok = ();
    type Error = RedisError;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(RedisError::Message("unsupported type".to_string()))

    }
    fn end(self) -> Result<()> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
}
impl<'a> ser::SerializeStructVariant for &'a mut RedisSerializer{

    type Ok = ();
    type Error = RedisError;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(RedisError::Message("unsupported type".to_string()))

    }
    fn end(self) -> Result<()> {
        Err(RedisError::Message("unsupported type".to_string()))
    }
}

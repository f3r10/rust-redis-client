use serde::Deserialize;
use serde::de::{self, DeserializeSeed, SeqAccess, Visitor};

use super::error::{RedisError, Result};

const CRLF_BYTES: &str = "\r\n";

pub enum RedisArray {
    String(String),
    Int(i64),
}

pub struct Deserializer<'de> {
    input: &'de str,
    pos: usize,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        let pos = 0;
        Deserializer {input, pos}
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    while deserializer.next_char()? != '\n' {
        deserializer.pos +=1
    }
    Ok(t)
    // if deserializer.input.is_empty() {
    // } else {
    //     Err(RedisError::Message("TraillingCharacters".to_owned()))
    // }
}

impl<'de> Deserializer<'de> {
    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(RedisError::Message("Peek EOF".to_owned()))
    }

    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn parse_simple_string(&mut self) -> Result<&'de str> {
        match self.input.find("\r\n") {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => Err(RedisError::Message("Eof string".to_owned()))
        }
    }


    fn parse_bulk_string(&mut self) -> Result<&'de str> {
        while self.next_char()? != '\n' {
            self.pos += 1;
        }

        match self.input.find("\r\n") {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => Err(RedisError::Message("Eof string".to_owned()))
        }
    }

    fn parse_integer(&mut self) -> Result<i64> {
        match self.input.find("\r\n") {
            Some(len) => {
                let s = &self.input[..len];
                let si64: i64 = s.parse::<i64>().unwrap(); //TODO handle the error //str::parse(s).unwrap();
                self.input = &self.input[len + 1..];
                self.pos += len;
                Ok(si64)
            }
            None => Err(RedisError::Message("Eof intege".to_owned()))
        }
    }
   
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = RedisError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>
    {
        match self.peek_char()? {
            '+' => {
                self.deserialize_string(visitor)
            }
            '-' => {
                self.deserialize_str(visitor)
            }
            ':' => {
                self.deserialize_i64(visitor)
            }
            '$' => {
                self.deserialize_str(visitor)
            }
            '*' => {
                self.deserialize_seq(visitor)
            }
            _ => Err(RedisError::Message("Syntax".to_owned())),
        }

    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.next_char()? != ':' {
            return Err(RedisError::Message("InvalidInteger".to_owned()));
        }
        visitor.visit_i64(self.parse_integer()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {

        Err(RedisError::Message("unsupported type".to_string()))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        while self.next_char()? != '\n' {
            self.pos += 1
        }
        if self.next_char()? != '$' {
            return Err(RedisError::Message("InvalidBulkString".to_owned()));
        }
        self.pos += 1;
        let _size = self.parse_integer()?;
        visitor.visit_borrowed_str(self.parse_bulk_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        match self.peek_char()? {
            '+' => {
                if self.next_char()? != '+' {
                    return Err(RedisError::Message("Invalid String".to_owned()));
                }
                visitor.visit_string(self.parse_simple_string()?.to_string())
            }
            '$' => {
                if self.next_char()? != '$' {
                    return Err(RedisError::Message("Invalid String".to_owned()));
                }
                let _size = self.parse_integer()?;
                visitor.visit_borrowed_str(self.parse_bulk_string()?)
            }
            _ => {
                self.deserialize_str(visitor)
            }
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        if self.next_char()? == '*' {
            let num_elements = self.parse_integer()?;
            visitor.visit_seq(CRLFSeparated::new(&mut self, num_elements))
        } else {
            Err(RedisError::Message("ExpectedArray".to_owned()))
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de> {
        unimplemented!()
    }

}
// each CRLF is like a comma in a json format.
// the first CRLF represents the number of items that the array will contain.
// after the first CRLF it is necessary to check the first character in order to parse the correct format.
// *5\r\n
// :1\r\n
// :2\r\n
// :3\r\n
// :4\r\n
// $6\r\n
// foobar\r\n
struct CRLFSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    first: bool,
    elements: i64,
}


impl<'a, 'de> CRLFSeparated<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, num_elements: i64) -> Self {
        CRLFSeparated {
            de,
            first: true,
            elements: num_elements
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for CRLFSeparated<'a, 'de> {
    type Error = RedisError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where T: DeserializeSeed<'de> {

        if self.elements == 0 {
            return Ok(None);
        }

        self.first = false;
        self.elements -= 1;
        seed.deserialize(&mut *self.de).map(Some)

    }
   
}

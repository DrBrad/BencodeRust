use std::collections::HashMap;
use std::hash::Hash;
use crate::BencodeVariables;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::bencode_object::{BencodeObject, PutObject};
use crate::variables::from_bencode::FromBencode;
use crate::variables::inter::bencode_type::BencodeType;
use crate::variables::to_bencode::ToBencode;

#[derive(Debug)]
pub struct BencodeArray<'a>(pub Vec<BencodeVariables<'a>>);

pub trait AddArray<'a, V> {

    fn add(&mut self, value: V);
}

impl<'a> BencodeArray<'a> {//: ToBencode + FromBencode

    const TYPE: BencodeType = BencodeType::ARRAY;

    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get_number(&'a self, index: usize) -> Result<i32, ()> {
        match self.0.get(index).unwrap() {
            BencodeVariables::NUMBER(num) => Ok(num.parse()),
            _ => Err(())
        }
    }

    pub fn get_array(&'a self, index: usize) -> Result<&BencodeArray, ()> {
        match self.0.get(index).unwrap() {
            BencodeVariables::ARRAY(arr) => Ok(arr),
            _ => Err(())
        }
    }

    pub fn get_object(&'a self, index: usize) -> Result<&BencodeObject, ()> {
        match self.0.get(index).unwrap() {
            BencodeVariables::OBJECT(obj) => Ok(obj),
            _ => Err(())
        }
    }

    pub fn get_bytes(&'a self, index: usize) -> Result<&[u8], ()> {
        match self.0.get(index).unwrap() {
            BencodeVariables::BYTES(bytes) => Ok(bytes.0),
            _ => Err(())
        }
    }

    pub fn get_string(&'a self, index: usize) -> Result<&str, ()> {
        match self.0.get(index).unwrap() {
            BencodeVariables::BYTES(bytes) => Ok(bytes.as_string()),
            _ => Err(())
        }
    }
}

impl<'a, const N: usize> AddArray<'a, &'a [u8; N]> for BencodeArray<'a> {

    fn add(&mut self, value: &'a [u8; N]) {
        self.0.push(BencodeVariables::BYTES(BencodeBytes::from(value)));
    }
}

impl<'a> AddArray<'a, &'a str> for BencodeArray<'a> {

    fn add(&mut self, value: &'a str) {
        self.0.push(BencodeVariables::BYTES(BencodeBytes::from(value)));
    }
}

impl<'a> AddArray<'a, String> for BencodeArray<'a> {

    fn add(&mut self, value: String) {
        self.0.push(BencodeVariables::BYTES(BencodeBytes::from(value)));
    }
}

impl<'a> AddArray<'a, BencodeArray<'a>> for BencodeArray<'a> {

    fn add(&mut self, value: BencodeArray<'a>) {
        self.0.push(BencodeVariables::ARRAY(value));
    }
}

impl<'a> AddArray<'a, BencodeObject<'a>> for BencodeArray<'a> {

    fn add(&mut self, value: BencodeObject<'a>) {
        self.0.push(BencodeVariables::OBJECT(value));
    }
}

macro_rules! impl_array_number {
    ($($type:ty)*) => {
        $(
            impl<'a> AddArray<'a, $type> for BencodeArray<'a> {

                fn add(&mut self, value: $type) {
                    self.0.push(BencodeVariables::NUMBER(BencodeNumber::from(value)));
                }

                /*
                fn get(&'a self, key: &'a str) -> Result<$type, ()> {
                    let key = BencodeBytes::from(key);

                    match self.0.get(&key).unwrap() {
                        BencodeVariables::NUMBER(num) => Ok(num.parse()),
                        _ => Err(())
                    }
                }*/
            }
        )*
    }
}

impl_array_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl<'a> FromBencode<'a> for BencodeArray<'a> {

    fn from_bencode(buf: &'a Vec<u8>, off: &mut usize) -> Self {
        let mut res = Vec::<BencodeVariables>::new();

        if BencodeType::type_by_prefix(buf[*off] as char) != Self::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        *off += 1;

        let mut res = Vec::new();

        while buf[*off] != Self::TYPE.suffix() as u8 {
            let type_ = BencodeType::type_by_prefix(buf[*off] as char);

            //let item = V::from_bencode(buf, off);

            let item = match type_ {
                BencodeType::NUMBER => BencodeVariables::NUMBER(BencodeNumber::from_bencode(buf, off)),
                BencodeType::ARRAY => BencodeVariables::ARRAY(BencodeArray::from_bencode(buf, off)),
                BencodeType::OBJECT => BencodeVariables::OBJECT(BencodeObject::from_bencode(buf, off)),
                BencodeType::BYTES => BencodeVariables::BYTES(BencodeBytes::from_bencode(buf, off)),
                _ => unimplemented!()
            };
            /*
            let item = match type_ {
                BencodeType::ARRAY => ,
                BencodeType::NUMBER => V::from_bencode(buf, off),
                BencodeType::BYTES => V::from_bencode(buf, off),
                _ => unimplemented!()
            };
            */

            res.push(item);
        }

        *off += 1;

        Self(res)
    }
}

impl<'a> ToBencode for BencodeArray<'a> {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(Self::TYPE.prefix() as u8);

        for item in &self.0 {
            let item = match item {
                BencodeVariables::NUMBER(num) => num.to_bencode(),
                BencodeVariables::ARRAY(arr) => arr.to_bencode(),
                BencodeVariables::OBJECT(obj) => obj.to_bencode(),
                BencodeVariables::BYTES(byt) => byt.to_bencode(),
                _ => unimplemented!()
            };
            buf.extend_from_slice(&item);
        }

        buf.push(Self::TYPE.suffix() as u8);
        buf
    }
}

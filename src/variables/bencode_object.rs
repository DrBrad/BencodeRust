//use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use crate::utils::ordered_map::OrderedMap;
use crate::BencodeVariables;
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::from_bencode::FromBencode;
use crate::variables::inter::bencode_type::BencodeType;
use crate::variables::to_bencode::ToBencode;

#[derive(Debug)]
pub struct BencodeObject<'a>(pub OrderedMap<BencodeBytes<'a>, BencodeVariables<'a>>);

pub trait PutObject<'a, V> {

    fn put(&mut self, key: &'a str, value: V);
}

impl<'a> BencodeObject<'a> {//: ToBencode + FromBencode

    const TYPE: BencodeType = BencodeType::OBJECT;

    pub fn new() -> Self {
        Self(OrderedMap::<BencodeBytes, BencodeVariables>::new())
    }

    pub fn get_number<V>(&'a self, key: &'a str) -> Result<V, ()> where V: FromStr {
        let key = BencodeBytes::from(key);

        match self.0.get(&key).unwrap() {
            BencodeVariables::NUMBER(num) => Ok(num.parse::<V>()),
            _ => Err(())
        }
    }

    pub fn get_array(&'a self, key: &'a str) -> Result<&BencodeArray, ()> {
        let key = BencodeBytes::from(key);

        match self.0.get(&key).unwrap() {
            BencodeVariables::ARRAY(arr) => Ok(arr),
            _ => Err(())
        }
    }

    pub fn get_object(&'a self, key: &'a str) -> Result<&BencodeObject, ()> {
        let key = BencodeBytes::from(key);

        match self.0.get(&key).unwrap() {
            BencodeVariables::OBJECT(obj) => Ok(obj),
            _ => Err(())
        }
    }

    pub fn get_bytes(&'a self, key: &'a str) -> Result<&[u8], ()> {
        let key = BencodeBytes::from(key);

        match self.0.get(&key).unwrap() {
            BencodeVariables::BYTES(bytes) => Ok(bytes.0),
            _ => Err(())
        }
    }

    pub fn get_string(&'a self, key: &'a str) -> Result<&str, ()> {
        let key = BencodeBytes::from(key);

        match self.0.get(&key).unwrap() {
            BencodeVariables::BYTES(bytes) => Ok(bytes.as_string()),
            _ => Err(())
        }
    }
}

/*
impl<'a> GetObject<f32> for BencodeObject<'a> {
    fn get(&self, key: &str) -> f32 {
        let key = BencodeBytes::from(key);

        match self.0.get(&key).unwrap() {
            BencodeVariables::NUMBER(num) => num.parse(),
            _ => 0.0
        }
    }
}

impl<'a> GetObject<i32> for BencodeObject<'a> {
    fn get(&self, key: &str) -> i32 {
        let key = BencodeBytes::from(key);

        match self.0.get(&key).unwrap() {
            BencodeVariables::NUMBER(num) => num.parse(),
            _ => 0
        }
    }
}


impl<'a> GetObject<&'a str> for BencodeObject<'a> {

    fn get<&'a str>(&self, key: &str) -> &'a str {
        &"asdasd"
    }
}
*/
impl<'a, const N: usize> PutObject<'a, &'a [u8; N]> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: &'a [u8; N]) {
        self.0.insert(BencodeBytes::from(key), BencodeVariables::BYTES(BencodeBytes::from(value)));
    }
}

impl<'a> PutObject<'a, &'a str> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: &'a str) {
        self.0.insert(BencodeBytes::from(key), BencodeVariables::BYTES(BencodeBytes::from(value)));
    }
}

impl<'a> PutObject<'a, String> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: String) {
        self.0.insert(BencodeBytes::from(key), BencodeVariables::BYTES(BencodeBytes::from(value)));
    }
}

impl<'a> PutObject<'a, BencodeArray<'a>> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: BencodeArray<'a>) {
        self.0.insert(BencodeBytes::from(key), BencodeVariables::ARRAY(value));
    }
}

impl<'a> PutObject<'a, BencodeObject<'a>> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: BencodeObject<'a>) {
        self.0.insert(BencodeBytes::from(key), BencodeVariables::OBJECT(value));
    }
}

macro_rules! impl_object_number {
    ($($type:ty)*) => {
        $(
            impl<'a> PutObject<'a, $type> for BencodeObject<'a> {

                fn put(&mut self, key: &'a str, value: $type) {
                    self.0.insert(BencodeBytes::from(key), BencodeVariables::NUMBER(BencodeNumber::from(value)));
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

impl_object_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl<'a> FromBencode<'a> for BencodeObject<'a> {

    fn from_bencode(buf: &'a Vec<u8>, off: &mut usize) -> Self {
        if BencodeType::type_by_prefix(buf[*off] as char) != Self::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        *off += 1;

        let mut res = OrderedMap::<BencodeBytes, BencodeVariables>::new();//::with_hasher(Default::default());

        while buf[*off] != Self::TYPE.suffix() as u8 {
            let key = BencodeBytes::from_bencode(buf, off);

            let type_ = BencodeType::type_by_prefix(buf[*off] as char);

            //println!("{:?}", type_);

            let value = match type_ {
                BencodeType::NUMBER => BencodeVariables::NUMBER(BencodeNumber::from_bencode(buf, off)),
                BencodeType::ARRAY => BencodeVariables::ARRAY(BencodeArray::from_bencode(buf, off)),
                BencodeType::OBJECT => BencodeVariables::OBJECT(BencodeObject::from_bencode(buf, off)),
                BencodeType::BYTES => BencodeVariables::BYTES(BencodeBytes::from_bencode(buf, off)),
                _ => unimplemented!()
            };

            res.insert(key, value);
        }

        *off += 1;


        Self(res)
    }
}

impl<'a> ToBencode for BencodeObject<'a> {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(Self::TYPE.prefix() as u8);

        for (key, value) in self.0.iter() {
            buf.extend_from_slice(&key.to_bencode());
            let value = match value {
                BencodeVariables::NUMBER(num) => num.to_bencode(),
                BencodeVariables::ARRAY(arr) => arr.to_bencode(),
                BencodeVariables::OBJECT(obj) => obj.to_bencode(),
                BencodeVariables::BYTES(byt) => byt.to_bencode(),
                _ => Vec::new()
            };
            buf.extend_from_slice(&value);
        }

        buf.push(Self::TYPE.suffix() as u8);
        buf
    }
}

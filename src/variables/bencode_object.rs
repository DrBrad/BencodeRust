use std::collections::HashMap;
use std::hash::Hash;
use crate::BencodeVariables;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::from_bencode::FromBencode;
use crate::variables::inter::bencode_type::BencodeType;
use crate::variables::to_bencode::ToBencode;

//#[derive(Debug, PartialEq)]
pub struct BencodeObject<'a>(pub HashMap<BencodeBytes<'a>, BencodeVariables<'a>>);

pub trait Object<'a, V> {

    fn put(&mut self, key: &'a str, value: V);
}

impl<'a> BencodeObject<'a> {//: ToBencode + FromBencode

    const TYPE: BencodeType = BencodeType::OBJECT;

    pub fn new() -> Self {
        Self(HashMap::<BencodeBytes, BencodeVariables>::new())
    }

    pub fn get_number(&'a self, key: &'a str) -> i32 {
        let key = BencodeBytes::from(key);
        let p = self.0.get(&key).unwrap();

        match p {
            BencodeVariables::NUMBER(num) => num.parse(),
            _ => panic!("Requested key is not a number")
        }
    }

    pub fn get_bytes(&'a self, key: &'a str) -> &[u8] {
        let key = BencodeBytes::from(key);
        let p = self.0.get(&key).unwrap();

        match p {
            BencodeVariables::NUMBER(_) => &[0u8; 0],
            BencodeVariables::OBJECT(_) => &[0u8; 0],
            BencodeVariables::ARRAY(_) => &[0u8; 0],
            BencodeVariables::BYTES(bytes) => bytes.0
        }
        //&[0u8]

    }

    pub fn get_string(&'a self, key: &'a str) -> &str {
        let key = BencodeBytes::from(key);
        let p = self.0.get(&key).unwrap();

        match p {
            BencodeVariables::NUMBER(_) => &"",
            BencodeVariables::OBJECT(_) => &"",
            BencodeVariables::ARRAY(_) => &"",
            BencodeVariables::BYTES(bytes) => bytes.as_string()
        }
        //&[0u8]

    }
}

impl<'a> Object<'a, &'a str> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: &'a str) {
        self.0.insert(BencodeBytes::from(key), BencodeVariables::BYTES(BencodeBytes::from(value)));
    }
}

impl<'a> Object<'a, String> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: String) {
        self.0.insert(BencodeBytes::from(key), BencodeVariables::BYTES(BencodeBytes::from(value)));
    }
}

macro_rules! impl_object_number {
    ($($type:ty)*) => {
        $(
            impl<'a> Object<'a, $type> for BencodeObject<'a> {

                fn put(&mut self, key: &'a str, value: $type) {
                    self.0.insert(BencodeBytes::from(key), BencodeVariables::NUMBER(BencodeNumber::from(value)));
                }
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

        let mut res = HashMap::<BencodeBytes, BencodeVariables>::with_hasher(Default::default());

        while buf[*off] != Self::TYPE.suffix() as u8 {
            let key = BencodeBytes::from_bencode(buf, off);

            let type_ = BencodeType::type_by_prefix(buf[*off] as char);

            println!("{:?}", type_);

            let value = match type_ {
                BencodeType::NUMBER => BencodeVariables::NUMBER(BencodeNumber::from_bencode(buf, off)),
                BencodeType::BYTES => BencodeVariables::BYTES(BencodeBytes::from_bencode(buf, off)),
                _ => unimplemented!()
            };

            res.insert(key, value);
        }


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
                //BencodeVariables::OBJECT(obj) => &obj.to_bencode(),
                //BencodeVariables::ARRAY(arr) => &arr.to_bencode(),
                BencodeVariables::BYTES(byt) => byt.to_bencode(),
                _ => Vec::new()
            };
            buf.extend_from_slice(&value);
        }

        buf.push(Self::TYPE.suffix() as u8);
        buf
    }
}

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

impl<'a> BencodeObject<'a> {//: ToBencode + FromBencode

    const TYPE: BencodeType = BencodeType::OBJECT;

    pub fn new() -> Self {
        Self(HashMap::<BencodeBytes, BencodeVariables>::new())
    }

    pub fn put(&mut self, key: &'a str, value: &'a str) {
        self.0.insert(BencodeBytes::from(key), BencodeVariables::BYTES(BencodeBytes::from(value)));
    }

    pub fn put_int(&mut self, key: &'a str, value: i32) {
        self.0.insert(BencodeBytes::from(key), BencodeVariables::NUMBER(BencodeNumber::from(value)));
    }

    pub fn get_number(&'a self, key: &'a str) -> i32 {
        let key = BencodeBytes::from(key);
        let p = self.0.get(&key).unwrap();

        let z: i32 = match p {
            BencodeVariables::NUMBER(num) => num.parse(),
            BencodeVariables::OBJECT(_) => 0,
            BencodeVariables::ARRAY(_) => 0,
            BencodeVariables::BYTES(_) => 0
        };

        z
        //&[0u8]

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

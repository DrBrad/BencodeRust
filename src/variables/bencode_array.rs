use std::collections::HashMap;
use std::hash::Hash;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::from_bencode::FromBencode;
use crate::variables::inter::bencode_type::BencodeType;
use crate::variables::to_bencode::ToBencode;

//#[derive(Debug, PartialEq)]
pub struct BencodeArray<V>(pub Vec<V>);

impl<V> BencodeArray<V> {//: ToBencode + FromBencode

    const TYPE: BencodeType = BencodeType::ARRAY;

    pub fn new() -> Self {
        Self(Vec::<V>::new())
    }
}

impl<'a, V> FromBencode<'a> for BencodeArray<V> where V: FromBencode<'a> {

    fn from_bencode(buf: &'a Vec<u8>, off: &mut usize) -> Self {
        let mut res = Vec::<V>::new();

        if BencodeType::type_by_prefix(buf[*off] as char) != Self::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        *off += 1;

        let mut res = Vec::new();

        while buf[*off] != Self::TYPE.suffix() as u8 {
            let type_ = BencodeType::type_by_prefix(buf[*off] as char);

            let item = V::from_bencode(buf, off);

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

        Self(res)
    }
}

impl<V> ToBencode for BencodeArray<V> where V: ToBencode {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(Self::TYPE.prefix() as u8);

        for item in &self.0 {
            buf.extend_from_slice(&item.to_bencode());
        }

        buf.push(Self::TYPE.suffix() as u8);
        buf
    }
}

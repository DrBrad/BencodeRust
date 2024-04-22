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

}

impl<V> FromBencode for BencodeArray<V> where V: FromBencode {

    fn from_bencode(buf: &Vec<u8>, off: &mut usize) -> Self {
        let mut res = Vec::<V>::new();


        BencodeArray(res)
    }
}

impl<V> ToBencode for BencodeArray<V> where V: ToBencode {

    fn to_bencode(&self) -> Vec<u8> {
        Vec::new()
    }
}

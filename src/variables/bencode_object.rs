use std::collections::HashMap;
use std::hash::Hash;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::from_bencode::FromBencode;
use crate::variables::inter::bencode_type::BencodeType;
use crate::variables::to_bencode::ToBencode;

//#[derive(Debug, PartialEq)]
pub struct BencodeObject<'a, V>(pub HashMap<BencodeBytes<'a>, V>);

impl<'a, V> BencodeObject<'a, V> {//: ToBencode + FromBencode

    const TYPE: BencodeType = BencodeType::OBJECT;

    pub fn new() -> Self {
        Self(HashMap::<BencodeBytes, V>::new())
    }
}

impl<'a, V> FromBencode<'a> for BencodeObject<'a, V> where V: FromBencode<'a> {

    fn from_bencode(buf: &'a Vec<u8>, off: &mut usize) -> Self {
        if BencodeType::type_by_prefix(buf[*off] as char) != Self::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        *off += 1;

        let mut res = HashMap::<BencodeBytes, V>::with_hasher(Default::default());

        while buf[*off] != Self::TYPE.suffix() as u8 {
            let key = BencodeBytes::from_bencode(buf, off);

            let type_ = BencodeType::type_by_prefix(buf[*off] as char);

            let value = match type_ {
                BencodeType::NUMBER => V::from_bencode(buf, off),
                BencodeType::BYTES => V::from_bencode(buf, off),
                _ => unimplemented!()
            };

            res.insert(key, value);
        }


        Self(res)
    }
}

impl<'a, V> ToBencode for BencodeObject<'a, V> where V: ToBencode {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(Self::TYPE.prefix() as u8);

        for (key, value) in self.0.iter() {
            buf.extend_from_slice(&key.to_bencode());
            buf.extend_from_slice(&value.to_bencode());
        }

        buf.push(Self::TYPE.suffix() as u8);
        buf
    }
}

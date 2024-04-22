use std::collections::HashMap;
use std::hash::Hash;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::from_bencode::FromBencode;
use crate::variables::inter::bencode_type::BencodeType;
use crate::variables::to_bencode::ToBencode;

//#[derive(Debug, PartialEq)]
pub struct BencodeObject<V>(pub HashMap<BencodeBytes, V>);

impl<V> BencodeObject<V> {//: ToBencode + FromBencode

    const TYPE: BencodeType = BencodeType::OBJECT;

}

impl<V> FromBencode for BencodeObject<V> where V: FromBencode {

    fn from_bencode(buf: &Vec<u8>, off: &mut usize) -> Self {
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


        BencodeObject(res)
    }
}

impl<V> ToBencode for BencodeObject<V> where V: ToBencode {

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

use crate::variables::decoder::decode_string;

pub trait FromBencode {

    fn from_bencode(b: &Vec<u8>) -> Self;
}

impl FromBencode for String {

    fn from_bencode(b: &Vec<u8>) -> Self {
        decode_string(b).to_string()
    }
}

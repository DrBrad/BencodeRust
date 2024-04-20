use super::encoder::encode_string;

pub trait ToBencode {

    fn to_bencode(&self) -> Vec<u8>;
}

impl ToBencode for String {

    fn to_bencode(&self) -> Vec<u8> {
        encode_string(self)
    }
}

impl ToBencode for u32 {

    fn to_bencode(&self) -> Vec<u8> {
        Vec::new()
    }
}







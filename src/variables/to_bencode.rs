pub trait ToBencode {

    fn to_bencode(&self) -> Vec<u8>;
}

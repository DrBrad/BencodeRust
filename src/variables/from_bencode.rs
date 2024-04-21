
pub trait FromBencode {

    fn from_bencode(&self, b: &[u8]) -> Self;
}

impl FromBencode for String {

    fn from_bencode(&self, b: &[u8]) -> Self {
        "asdasd".to_string()
    }
}

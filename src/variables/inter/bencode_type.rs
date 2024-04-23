#[derive(Debug, PartialEq)]
pub enum BencodeType {

    NUMBER,
    OBJECT,
    ARRAY,
    BYTES,
    INVALID
}

impl BencodeType {

    pub fn is_prefix(&self, c: u8) -> bool {
        match self {
            BencodeType::BYTES => c >= b'0' && c <= b'9',
            _ => c == self.prefix()
        }
    }

    pub fn prefix(&self) -> u8 {
        match self {
            BencodeType::NUMBER => b'i',
            BencodeType::OBJECT => b'd',
            BencodeType::ARRAY => b'l',
            _ => unimplemented!()
        }
    }

    pub fn suffix(&self) -> u8 {
        match self {
            BencodeType::NUMBER => b'e',
            BencodeType::ARRAY => b'e',
            BencodeType::OBJECT => b'e',
            _ => unimplemented!()
        }
    }

    pub fn delimiter(&self) -> u8 {
        match self {
            BencodeType::BYTES => b':',
            _ => unimplemented!()
        }
    }

    pub fn type_by_prefix(c: u8) -> Self {
        for btype in [BencodeType::NUMBER, BencodeType::ARRAY, BencodeType::OBJECT, BencodeType::BYTES] {
            if btype.is_prefix(c) {
                return btype;
            }
        }

        BencodeType::INVALID
    }
}

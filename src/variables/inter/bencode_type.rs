#[derive(Debug, PartialEq)]
pub enum BencodeType {

    NUMBER,
    OBJECT,
    ARRAY,
    BYTES,
    INVALID
}

impl BencodeType {

    pub fn is_prefix(&self, c: char) -> bool {
        match self {
            BencodeType::BYTES => c >= '0' && c <= '9',
            _ => c == self.prefix(),
        }
    }

    pub fn prefix(&self) -> char {
        match self {
            BencodeType::NUMBER => 'i',
            BencodeType::OBJECT => 'd',
            BencodeType::ARRAY => 'l',
            BencodeType::BYTES => '\0',
            _ => '\0'
        }
    }

    pub fn suffix(&self) -> char {
        match self {
            BencodeType::NUMBER => 'e',
            BencodeType::ARRAY => 'e',
            BencodeType::OBJECT => 'e',
            _ => '\0',
        }
    }

    pub fn delimiter(&self) -> char {
        match self {
            BencodeType::BYTES => ':',
            _ => '\0',
        }
    }

    pub fn type_by_prefix(c: char) -> Self {
        for btype in [BencodeType::NUMBER, BencodeType::ARRAY, BencodeType::OBJECT, BencodeType::BYTES] {
            if btype.is_prefix(c) {
                return btype;
            }
        }

        BencodeType::INVALID
    }
}

pub enum BencodeType {

    NUMBER,
    OBJECT,
    ARRAY,
    BYTES
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
        }
    }

    pub fn suffix(&self) -> char {
        match self {
            BencodeType::NUMBER => 'e',
            _ => 'e',
        }
    }

    pub fn delimiter(&self) -> char {
        match self {
            BencodeType::BYTES => ':',
            _ => '\0',
        }
    }
}
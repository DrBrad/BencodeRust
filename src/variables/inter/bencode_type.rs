pub enum BencodeType {

    NUMBER,
    OBJECT,
    ARRAY,
    BYTES
}

impl BencodeType {

    pub fn is_prefix(&self, c: char) -> bool {
        match self {
            BencodeType::NUMBER => {
                c == self.prefix()
            },
            BencodeType::OBJECT => {
                c == self.prefix()
            },
            BencodeType::ARRAY => {
                c == self.prefix()
            },
            BencodeType::BYTES => {
                c >= '0' && c <= '9'
            }
        }
    }

    pub fn prefix(&self) -> char {
        match self {
            BencodeType::NUMBER => 'e',
            BencodeType::OBJECT => 'i',
            BencodeType::ARRAY => 'l',
            BencodeType::BYTES => 0x00 as char
        }
    }

    pub fn suffix(&self) -> char {
        match self {
            BencodeType::NUMBER => 'e',
            BencodeType::OBJECT => 'e',
            BencodeType::ARRAY => 'e',
            BencodeType::BYTES => 0x00 as char
        }
    }

    pub fn delimiter(&self) -> char {
        match self {
            BencodeType::NUMBER => 0x00 as char,
            BencodeType::OBJECT => 0x00 as char,
            BencodeType::ARRAY => 0x00 as char,
            BencodeType::BYTES => ':'
        }
    }
}
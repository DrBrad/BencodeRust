use crate::variables::from_bencode::FromBencode;
use crate::variables::inter::bencode_type::BencodeType;
use crate::variables::to_bencode::ToBencode;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct BencodeBytes(pub Vec<u8>);

impl BencodeBytes {

    const TYPE: BencodeType = BencodeType::BYTES;

    pub fn as_string(&self) -> String {
        String::from_utf8(self.0.clone()).unwrap_or_else(|_| panic!("Failed to parse UTF-8 string"))
    }
}

impl From<String> for BencodeBytes {

    fn from(value: String) -> Self {
        BencodeBytes(value.as_bytes().to_vec())
    }
}

impl FromBencode for BencodeBytes {

    fn from_bencode(buf: &Vec<u8>, off: &mut usize) -> Self {
        if BencodeType::type_by_prefix(buf[*off] as char) != Self::TYPE {
            panic!("Buffer is not a bencode bytes / string.");
        }

        let mut len_bytes = [0; 8];
        let start = *off;

        while buf[*off] != Self::TYPE.delimiter() as u8 {
            len_bytes[*off - start] = buf[*off];
            *off += 1;
        }

        let length = len_bytes.iter().take(*off - start).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        let bytes = buf[*off + 1..*off + 1 + length].to_vec();

        *off += 1+length;

        BencodeBytes(bytes)
    }
}

impl ToBencode for BencodeBytes {

    fn to_bencode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();

        r.extend_from_slice(self.0.len().to_string().as_bytes());
        r.push(Self::TYPE.delimiter() as u8);
        r.extend(self.0.clone());
        r
    }
}

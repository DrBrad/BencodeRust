use crate::variables::inter::bencode_variable::BencodeVariable;

pub struct BencodeNumber {
    s: u32,
    pub n: usize
}

impl BencodeVariable for BencodeNumber {

    fn byte_size(&self) -> u32 {
        self.s
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn decode(&self, buf: Vec<u8>) {
        todo!()
    }
}

impl From<usize> for BencodeNumber {

    fn from(n: usize) -> Self {
        Self {
            s: 0,
            n
        }
    }
}

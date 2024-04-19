use crate::variables::bencode_number::BencodeNumber;
use crate::variables::inter::bencode_variable::BencodeVariable;
use super::inter::bencode_variable;

pub struct BencodeArray {
    s: u32,
    pub l: Vec<Box<dyn BencodeVariable>>
}

impl BencodeVariable for BencodeArray {

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

impl BencodeArray {

    pub fn new() -> Self {
        Self {
            s: 0,
            l: Vec::new()
        }
    }

    pub fn contains(&self, v: &dyn BencodeVariable) {

    }

    pub fn add(&mut self, v: BencodeNumber) {
        self.l.push(Box::new(v));
    }

    pub fn remove(&mut self, v: &dyn BencodeVariable) {

    }
}

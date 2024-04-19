use crate::variables::inter::bencode_variable::BencodeVariable;
use super::inter::bencode_variable;

pub struct BencodeArray {
    s: u32,
    pub l: Vec<BencodeVariable>
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

    fn contains(&self, v: BencodeVariable) {

    }

    fn add(&mut self, v: BencodeVariable) {

    }

    fn remove(&mut self, v: BencodeVariable) {

    }
}

/*
pub enum BencodeElem {
    String(String),
    Bytes(Vec<u8>),
    Integer(i64),
    List(Vec<BencodeElem>),
    //Dictionary(HashMap<String, BencodeElem>),
    //RawDictionary(HashMap<Vec<u8>, BencodeElem>),
}
*/

/*

impl BencodeArray {

    pub fn new() -> Self {
        Self {
            s: 2,
            l: Vec::new()
        }
    }

    pub fn add(&mut self, v: &dyn BencodeVariable) {
        self.l.push(v);
    }
}
*/

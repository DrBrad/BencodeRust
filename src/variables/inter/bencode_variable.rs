use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::bencode_object::BencodeObject;

#[derive(Debug, Clone)]
pub enum BencodeVariable<'a> {
    Number(BencodeNumber<'a>),
    Array(BencodeArray<'a>),
    Object(BencodeObject<'a>),
    Bytes(BencodeBytes)
}

pub trait Bencode<'a> {

    fn decode(buf: &'a [u8]) -> Self where Self: Sized {
        Self::decode_with_offset(buf, 0)
    }

    fn decode_with_offset(buf: &'a [u8], off: usize) -> Self;

    fn encode(&self) -> Vec<u8>;//&[u8];

    fn byte_size(&self) -> usize;
}

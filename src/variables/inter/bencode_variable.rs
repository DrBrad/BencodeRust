use std::any::Any;
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::bencode_object::BencodeObject;

/*
#[derive(Debug, Clone, PartialEq)]
pub enum BencodeVariable<'a> {
    Number(BencodeNumber<'a>),
    Array(BencodeArray<'a>),
    Object(BencodeObject<'a>),
    Bytes(BencodeBytes)
}
*/
pub trait Bencode {

    /*
    fn decode(buf: &[u8]) -> Self where Self: Sized {
        Self::decode_with_offset(buf, 0)
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Self;
    */

    fn encode(&self) -> Vec<u8>;//&[u8];

    fn decode(buf: &[u8]) -> Self where Self: Sized {
        Self::decode_with_offset(buf, 0)
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Self where Self: Sized;

    fn as_any(&self) -> &dyn Any;

    fn byte_size(&self) -> usize;
}



pub trait Bencode2 {

    /*
    fn decode(buf: &[u8]) -> Self where Self: Sized {
        Self::decode_with_offset(buf, 0)
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Self;
    */

    fn encode(&self) -> Vec<u8>;//&[u8];

    fn decode(buf: &[u8]) -> Self where Self: Sized {
        Self::decode_with_offset(buf, 0)
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Self where Self: Sized;

    fn as_any(&'static self) -> &dyn Any;

    fn byte_size(&self) -> usize;
}

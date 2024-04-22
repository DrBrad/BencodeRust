use std::collections::{LinkedList, VecDeque};
//use crate::variables::decoder::{Decoder};
use crate::variables::to_bencode::ToBencode;


pub struct BencodeBytes {
    pub(crate) size: usize,
    pub(crate) buf: Vec<u8>
}


impl BencodeBytes {

    pub fn new() -> Self {
        Self {
            size: 0,
            buf: Vec::new()
        }
    }

    pub fn as_string(&self) -> String {
        String::from_utf8_lossy(&self.buf).into_owned()
    }
}







pub trait FromBencode {

    fn from_bencode(b: &Vec<u8>) -> Self;
}

impl FromBencode for BencodeBytes {

    fn from_bencode(buf: &Vec<u8>) -> Self {
        let mut len_bytes = [0; 8];
        let start = 0;
        let mut off = 0;

        while buf[off] != b':' {
            len_bytes[off - start] = buf[off];
            off += 1;
        }

        let length = len_bytes.iter().take(off - start).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        let string_bytes = &buf[off + 1..off + 1 + length];

        off += 1+length;

        Self {
            size: off,
            buf: string_bytes.to_vec()
        }
    }
}

/*
pub trait FromBencode {

    const size: usize;

    fn from_bencode(b: &Vec<u8>) -> Self;
}

impl FromBencode for String {
    const size;

    fn from_bencode(buf: &Vec<u8>) -> Self {
        Self::size = 100;
        Decoder::new().decode_string(buf)
    }
}

macro_rules! impl_decodable_number {
    ($($type:ty)*) => {$(
        impl FromBencode for $type {

            fn from_bencode(buf: &Vec<u8>) -> Self {
                Decoder::new().decode_number(buf)
            }
        }
    )*}
}

impl_decodable_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);



macro_rules! impl_decodable_iterable {
    ($($type:ident)*) => {
        $(
            impl<ContentT> FromBencode for $type<ContentT> where ContentT: FromBencode {

                fn from_bencode(buf: &Vec<u8>) -> Self {
                    if buf[0] != b'l' {
                        panic!("Buffer is not a bencode array.");
                    }

                    println!("{}", stringify!($type));
                    println!("ContentT: {}", stringify!(ContentT));

                    let mut decoder = Decoder::new();
                    let response = $type::new()

                    for i in 1..buf.len() {
                    //for (index, &byte) in buf.iter().enumerate() {
                        println!("{}", buf[i] as char);
                    }

                    response
                }
            }
        )*
    };
}

impl_decodable_iterable!(Vec VecDeque LinkedList);
*/

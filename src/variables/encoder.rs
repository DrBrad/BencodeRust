use std::fmt::Display;
use crate::variables::to_bencode::ToBencode;

pub fn encode_string(v: &str) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    let z = v.as_bytes();

    r.extend_from_slice(z.len().to_string().as_bytes());
    r.push(b':');
    r.extend_from_slice(z);
    r
}

pub fn encode_number<T: Display>(v: &T) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    r.push(b'i');
    r.extend_from_slice(v.to_string().as_bytes());
    r.push(b'e');
    r
}
/*
pub fn encode_list<E: ToBencode>(v: E) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    r.push(b'l');

    for item in v {
        //println!("{:?}", item);
    }

    r.push(b'e');
    r
}
*/

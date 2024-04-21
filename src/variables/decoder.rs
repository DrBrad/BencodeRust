use std::fmt::Display;
use std::str::{FromStr, from_utf8};

pub struct Decoder {
    pub(crate) off: usize
}

impl Decoder {

    pub fn new() -> Self {
        Self {
            off: 0
        }
    }

    pub fn decode_string(&mut self, buf: &Vec<u8>) -> String {
        let mut len_bytes = [0; 8];
        let start = self.off;

        while buf[self.off] != b':' {
            len_bytes[self.off - start] = buf[self.off];
            self.off += 1;
        }

        let length = len_bytes.iter().take(self.off - start).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        let string_bytes = &buf[self.off + 1..self.off + 1 + length];

        self.off += 1+length;

        String::from_utf8_lossy(string_bytes).into_owned()
    }
}

/*
pub fn decode_string(buf: &Vec<u8>, mut off: usize) -> String {
    let mut len_bytes = [0; 8];
    let start = off;

    //USE DELIMITER
    while buf[off] != b':' {
        len_bytes[off - start] = buf[off];
        off += 1;
    }

    let length = len_bytes.iter().take(off - start).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
    let string_bytes = &buf[off + 1..off + 1 + length];

    String::from_utf8_lossy(string_bytes).into_owned()
}

pub fn decode_number<T: FromStr>(buf: &Vec<u8>, mut off: usize) -> T {
    let mut c = [0 as char; 32];
    let mut off = off + 1;
    let s = off;

    //type.get_suffix()
    while buf[off] != b'e' {
        c[off - s] = buf[off] as char;
        off += 1;
    }

    let number_str = from_utf8(&buf[s..off]).unwrap_or_else(|_| panic!("Failed to parse UTF-8 string"));

    match number_str.parse::<T>() {
        Ok(number) => number,
        Err(_) => panic!("Number is invalid."),
    }
}
*/

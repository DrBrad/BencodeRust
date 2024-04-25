use std::str::from_utf8;
use std::slice::from_raw_parts;
use std::mem::forget;

use crate::variables::inter::bencode_variable::Bencode;
use crate::variables::inter::bencode_type::BencodeType;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct BencodeBytes {
    b: Vec<u8>,
    s: usize
}//(&'a [u8]);

impl BencodeBytes {

    const TYPE: BencodeType = BencodeType::Bytes;

    pub fn as_bytes(&self) -> &[u8] {
        &self.b
    }

    pub fn as_str(&self) -> &str {
        from_utf8(&self.b).unwrap_or_else(|_| panic!("Failed to parse UTF-8 string"))
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.b).to_string()
    }
}

impl<'a, const N: usize> From<[u8; N]> for BencodeBytes {

    fn from(value: [u8; N]) -> Self {
        Self {
            b: value.to_vec(),
            s: value.len()+value.len().to_string().len()+1
        }
    }
}

impl<'a> From<&'a str> for BencodeBytes {

    fn from(value: &'a str) -> Self {
        //let value = value.as_bytes();

        Self {
            b: value.as_bytes().to_vec(),
            s: value.len()+value.len().to_string().len()+1
        }
    }
}

impl<'a> From<String> for BencodeBytes {

    fn from(value: String) -> Self {
        let value = value.into_bytes();
        let s = value.len()+value.len().to_string().len()+1;
        Self {
            b: value,//from_raw_parts(bytes, len),
            s
        }
        /*
        let bytes = value.as_ptr();
        let len = value.len();
        forget(value);

        unsafe {
            let value = from_raw_parts(bytes, len);

            Self {
                b: value,//from_raw_parts(bytes, len),
                s: value.len()+value.len().to_string().len()+1
            }
        }*/
    }
}

impl<'a> Bencode<'a> for BencodeBytes {

    fn decode_with_offset(buf: &'a [u8], off: usize) -> Self {
        if BencodeType::type_by_prefix(buf[off]) != Self::TYPE {
            panic!("Buffer is not a bencode bytes / string.");
        }

        let mut off = off;
        let mut len_bytes = [0u8; 8];
        let mut s = off;

        while buf[off] != Self::TYPE.delimiter() {
            len_bytes[off - s] = buf[off];
            off += 1;
        }

        let length = len_bytes.iter().take(off - s).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        let bytes = buf[off + 1..off + 1 + length].to_vec();

        off += 1+length;
        s = off-s;

        Self {
            b: bytes,
            s
        }
    }

    /*
    fn to_bencode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();

        r.extend_from_slice(self.b.len().to_string().as_bytes());
        r.push(Self::TYPE.delimiter());
        r.extend_from_slice(&self.b);
        r
    }
    */
    fn encode(&self) -> &[u8] {
        let mut data = vec![0u8; self.s];

        let len_str = self.b.len().to_string();
        let len_bytes = len_str.as_bytes();

        let mut index = 0;
        for &byte in len_bytes {
            data[index] = byte;
            index += 1;
        }
        data[index] = Self::TYPE.delimiter();
        index += 1;

        for &byte in self.b.iter() {
            data[index] = byte;
            index += 1;
        }

        let ptr = data.as_ptr();
        let len = data.len();

        forget(data);

        unsafe {
            from_raw_parts(ptr, len)
        }
    }

    fn byte_size(&self) -> usize {
        self.s
    }
}

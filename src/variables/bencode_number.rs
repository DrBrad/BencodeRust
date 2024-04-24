use std::str::{from_utf8, FromStr};
use std::slice::from_raw_parts;
use std::mem::forget;

use crate::variables::inter::bencode_variable::Bencode;
use crate::variables::inter::bencode_type::BencodeType;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct BencodeNumber<'a>{
    n: &'a [u8],
    s: usize
}

impl<'a> BencodeNumber<'a> {

    const TYPE: BencodeType = BencodeType::NUMBER;

    pub fn parse<V>(&self) -> V where V: FromStr {
        let str = from_utf8(&self.n).unwrap_or_else(|_| panic!("Failed to parse UTF-8 string"));
        str.parse::<V>().unwrap_or_else(|_| panic!("Failed to parse to Number"))
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.n).to_string()
    }
}

macro_rules! impl_decodable_number {
    ($($type:ty)*) => {
        $(
            impl<'a> From<$type> for BencodeNumber<'a> {

                fn from(value: $type) -> Self {
                    let value = value.to_string();
                    let size = value.len()+2;

                    let bytes = value.as_ptr();
                    let len = value.len();
                    forget(value);

                    unsafe {
                        Self {
                            n: from_raw_parts(bytes, len),
                            s: size
                        }
                    }
                }
            }
        )*
    }
}

impl_decodable_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl<'a> Bencode<'a> for BencodeNumber<'a> {

    fn decode_with_offset(buf: &'a [u8], off: usize) -> Self {
        if BencodeType::type_by_prefix(buf[off]) != Self::TYPE {
            panic!("Buffer is not a bencode bytes / string.");
        }

        let mut off = off+1;

        let mut c = [0u8; 32];
        let mut s = off;

        while buf[off] != Self::TYPE.suffix() {
            c[off - s] = buf[off];
            off += 1;
        }

        let bytes = &buf[s..off];

        off += 1;
        s = off+1-s;

        Self {
            n: bytes,
            s
        }
    }

    /*
    fn to_bencode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();

        r.push(Self::TYPE.prefix());
        r.extend_from_slice(self.n);
        r.push(Self::TYPE.suffix());
        r
    }
    */
    fn encode(&self) -> &[u8] {
        let mut data = vec![0u8; self.s];

        data[0] = Self::TYPE.prefix();
        data[1..=self.n.len()].copy_from_slice(self.n);
        data[self.n.len() + 1] = Self::TYPE.suffix();

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

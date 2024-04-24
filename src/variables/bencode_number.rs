use std::str::{from_utf8, FromStr};
use std::slice::from_raw_parts;

use crate::variables::inter::bencode::Bencode;
use crate::variables::inter::bencode_type::BencodeType;

#[derive(Debug, Eq, Hash, PartialEq)]
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

                    let bytes = value.as_ptr();
                    let len = value.len();
                    std::mem::forget(value);

                    unsafe {
                        Self {
                            n: from_raw_parts(bytes, len),
                            s: 10
                        }
                    }
                }
            }
        )*
    }
}

impl_decodable_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);



impl<'a> Bencode<'a> for BencodeNumber<'a> {

    fn from_bencode(buf: &'a Vec<u8>, off: &mut usize) -> Self {
        if BencodeType::type_by_prefix(buf[*off]) != Self::TYPE {
            panic!("Buffer is not a bencode bytes / string.");
        }

            *off += 1;

        let mut c = [0 as char; 32];
        let s = *off;

        while buf[*off] != Self::TYPE.suffix() as u8 {
            c[*off - s] = buf[*off] as char;
            *off += 1;
        }

        let bytes = &buf[s..*off];

        *off += 1;

        Self {
            n: bytes,
            s: 10
        }
    }

    fn to_bencode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();

        r.push(Self::TYPE.prefix());
        r.extend_from_slice(self.n);
        r.push(Self::TYPE.suffix());
        r
    }

    fn byte_size(&self) -> usize {
        self.s
    }
}

use crate::variables::decoder::{decode_number, decode_string};

pub trait FromBencode {

    fn from_bencode(b: &Vec<u8>) -> Self;
}

impl FromBencode for String {

    fn from_bencode(b: &Vec<u8>) -> Self {
        decode_string(b, 0).to_string()
    }
}

macro_rules! impl_decodable_number {
    ($($type:ty)*) => {$(
        impl FromBencode for $type {

            fn from_bencode(b: &Vec<u8>) -> Self {
                decode_number::<$type>(b, 0)
            }
        }
    )*}
}

impl_decodable_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);





/*
macro_rules! impl_decodabl_iterable {
    ($($type:ident)*) => {$(
        impl <ContentT> ToBencode for $type<ContentT> where ContentT: ToBencode {

            fn to_bencode(&self) -> Vec<u8> {
                let mut r: Vec<u8> = Vec::new();
                r.push(b'l');

                for item in self {
                    r.extend_from_slice(&item.to_bencode());
                }

                r.push(b'e');
                r
            }
        }
    )*}
}

impl_decodabl_iterable!(Vec VecDeque LinkedList);
*/


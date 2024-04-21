use crate::variables::decoder::decode_string;

pub trait FromBencode {

    fn from_bencode(b: &Vec<u8>) -> Self;
}

impl FromBencode for String {

    fn from_bencode(b: &Vec<u8>) -> Self {
        decode_string(b, 0).to_string()
    }
}

macro_rules! impl_decodable_integer {
    ($($type:ty)*) => {$(
        impl FromBencode for $type {

            fn from_bencode(b: &Vec<u8>) -> Self {
                /*
                let content = object.try_into_integer()?;
                let number = content.parse::<$type>()?;

                number
                */
                100 as $type
            }
        }
    )*}
}

impl_decodable_integer!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

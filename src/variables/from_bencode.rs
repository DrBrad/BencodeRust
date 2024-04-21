use crate::variables::decoder::decode_string;

pub trait FromBencode {

    fn from_bencode(b: &Vec<u8>) -> Self;
}

impl FromBencode for String {

    fn from_bencode(b: &Vec<u8>) -> Self {
        decode_string(b, 0).to_string()
    }
}

impl FromBencode for f32 {

    fn from_bencode(b: &Vec<u8>) -> Self {
        5.05
        //decode_string(b, 0).to_string()
    }
}


/*
macro_rules! impl_decodable_integer {
    ($($type:ty)*) => {$(
        impl FromBencode for $type {

            fn from_bencode(b: &Vec<u8>) -> Self
    where
        Self: Sized,
    {
                println!("{:?}", b);

                5.05
                //decode_number(self)
            }
        }
    )*}
}

impl_decodable_integer!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);
*/

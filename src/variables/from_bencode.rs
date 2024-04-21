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
                //CHECK PREFIX

                let mut off = 0;

                let mut c = [0 as char; 32];
                let mut off = off + 1;
                let s = off;

                //type.get_suffix()
                while b[off] != b'e' {
                    c[off - s] = b[off] as char;
                    off += 1;
                }


                // Convert the byte slice to string
                let number_str = std::str::from_utf8(&b[s..off])
                    .unwrap_or_else(|_| panic!("Failed to parse UTF-8 string"));

                // Parse the string into the desired number type
                match number_str.parse::<$type>() {
                    Ok(number) => number,
                    Err(_) => panic!("Number is invalid."),
                }

                //number

                /*
                let content = object.try_into_integer()?;
                let number = content.parse::<$type>()?;

                number
                */
                //100 as $type
            }
        }
    )*}
}

impl_decodable_integer!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

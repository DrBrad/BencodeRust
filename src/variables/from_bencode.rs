use std::collections::{HashMap, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};
use std::str::from_utf8;
use crate::variables::inter::bencode_type::BencodeType;
use crate::variables::to_bencode::ToBencode;

pub trait FromBencode {

    const TYPE: BencodeType;

    fn from_bencode(buf: &Vec<u8>, off: &mut usize) -> Self;
}

impl FromBencode for String {

    const TYPE: BencodeType = BencodeType::BYTES;

    fn from_bencode(buf: &Vec<u8>, off: &mut usize) -> Self {
        if BencodeType::type_by_prefix(buf[*off] as char) != <String as FromBencode>::TYPE {
            panic!("Buffer is not a bencode bytes / string.");
        }

        let mut len_bytes = [0; 8];
        let start = *off;

        while buf[*off] != <String as FromBencode>::TYPE.delimiter() as u8 {
            len_bytes[*off - start] = buf[*off];
            *off += 1;
        }

        let length = len_bytes.iter().take(*off - start).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);

        let string_bytes = &buf[*off + 1..*off + 1 + length];

        *off += 1+length;

        String::from_utf8_lossy(string_bytes).into_owned()
    }
}


macro_rules! impl_decodable_number {
    ($($type:ty)*) => {
        $(
            impl FromBencode for $type {

                const TYPE: BencodeType = BencodeType::NUMBER;

                fn from_bencode(buf: &Vec<u8>, off: &mut usize) -> Self {
                    if BencodeType::type_by_prefix(buf[*off] as char) != <$type as FromBencode>::TYPE {
                        panic!("Buffer is not a bencode bytes / string.");
                    }

                    *off += 1;

                    let mut c = [0 as char; 32];
                    let s = *off;

                    while buf[*off] != <$type as FromBencode>::TYPE.suffix() as u8 {
                        c[*off - s] = buf[*off] as char;
                        *off += 1;
                    }

                    let number_str = from_utf8(&buf[s..*off]).unwrap_or_else(|_| panic!("Failed to parse UTF-8 string"));

                    *off += 1;
                    let num = match number_str.parse::<$type>() {
                        Ok(number) => number,
                        Err(_) => panic!("Number is invalid."),
                    };

                    num
                }
            }
        )*
    }
}

impl_decodable_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);





macro_rules! impl_decodable_iterable {
    ($($type:ident)*) => {
        $(
            impl<T> FromBencode for $type<T> where T: FromBencode {

                const TYPE: BencodeType = BencodeType::ARRAY;

                fn from_bencode(buf: &Vec<u8>, off: &mut usize) -> Self {
                    if BencodeType::type_by_prefix(buf[*off] as char) != <$type<T> as FromBencode>::TYPE {
                        panic!("Buffer is not a bencode array.");
                    }

                    *off += 1;

                    let mut res = $type::new();

                    //println!("{}", type_);
                    //println!("{:?}", type_);
                    //println!("{}", buf[*off] as char);

                    while buf[*off] != <$type<T> as FromBencode>::TYPE.suffix() as u8 {
                    //for off in 1..buf.len()-1 {


                        let type_ = BencodeType::type_by_prefix(buf[*off] as char);

                        let item = match type_ {
                            BencodeType::NUMBER => T::from_bencode(buf, off),
                            BencodeType::BYTES => T::from_bencode(buf, off),
                            _ => unimplemented!()
                        };

                        //MOVE BELOW INTO A DIFFERENT FUNCTION...

                        res.push(item);
                    }


                    res
                }
            }
        )*
    };
}

impl_decodable_iterable!(Vec);// VecDeque LinkedList);

impl<K, V, S> FromBencode for HashMap<K, V, S> where K: FromBencode + Eq + Hash, V: FromBencode, S: BuildHasher + Default {

    const TYPE: BencodeType = BencodeType::OBJECT;

    fn from_bencode(buf: &Vec<u8>, off: &mut usize) -> Self {
        if BencodeType::type_by_prefix(buf[*off] as char) != <HashMap<K, V, S> as FromBencode>::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        *off += 1;

        let mut res = HashMap::<K, V, S>::with_hasher(Default::default());

        while buf[*off] != <HashMap<K, V, S> as FromBencode>::TYPE.suffix() as u8 {
            //for off in 1..buf.len()-1 {

            let key = K::from_bencode(buf, off);

            let type_ = BencodeType::type_by_prefix(buf[*off] as char);

            let value = match type_ {
                BencodeType::NUMBER => V::from_bencode(buf, off),
                BencodeType::BYTES => V::from_bencode(buf, off),
                _ => unimplemented!()
            };


            //MOVE BELOW INTO A DIFFERENT FUNCTION...

            res.insert(key, value);
        }

        res
    }
}

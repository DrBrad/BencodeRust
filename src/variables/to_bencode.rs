use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};
use super::encoder::{encode_number, encode_string};

pub trait ToBencode {

    fn to_bencode(&self) -> Vec<u8>;
}

impl ToBencode for String {

    fn to_bencode(&self) -> Vec<u8> {
        encode_string(self)
    }
}

impl ToBencode for &str {

    fn to_bencode(&self) -> Vec<u8> {
        encode_string(self)
    }
}

macro_rules! impl_encodable_integer {
    ($($type:ty)*) => {$(
        impl ToBencode for $type {

            fn to_bencode(&self) -> Vec<u8> {
                encode_number(self)
            }
        }
    )*}
}

impl_encodable_integer!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

macro_rules! impl_encodable_iterable {
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

impl_encodable_iterable!(Vec VecDeque LinkedList);




impl<K: AsRef<[u8]>, V: ToBencode> ToBencode for BTreeMap<K, V> {

    fn to_bencode(&self) -> Vec<u8> {
        //encode_string(self)
        Vec::new()
    }
}

impl<K, V, S> ToBencode for HashMap<K, V, S> where K: AsRef<[u8]> + Eq + Hash, V: ToBencode, S: BuildHasher, {

    fn to_bencode(&self) -> Vec<u8> {
        //encode_string(self)
        Vec::new()
    }
}







/*
impl <E: ToBencode> ToBencode for Box<E> {
//impl<'a, ContentT> ToBencode for &'a [ContentT] where ContentT: ToBencode {

    fn to_bencode(&self) -> Vec<u8> {
        Vec::new()
    }
}
*/




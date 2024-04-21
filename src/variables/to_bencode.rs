use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std::fmt::Display;
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

impl_encodable_integer!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

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




impl<K, V> ToBencode for BTreeMap<K, V> where K: ToBencode, V: ToBencode {

    fn to_bencode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();
        r.push(b'd');

        for (key, value) in self {
            r.extend_from_slice(&key.to_bencode());
            r.extend_from_slice(&value.to_bencode());
        }

        r.push(b'e');
        r
    }
}

impl<K, V, S> ToBencode for HashMap<K, V, S> where K: ToBencode, V: ToBencode, S: BuildHasher, {

    fn to_bencode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();
        r.push(b'd');

        for (key, value) in self {
            r.extend_from_slice(&key.to_bencode());
            r.extend_from_slice(&value.to_bencode());
        }

        r.push(b'e');
        r
    }
}


pub enum Value<T> {
    STRING(String),
    NUMBER(T)
}

impl<T: Display> ToBencode for Value<T> {

    fn to_bencode(&self) -> Vec<u8> {
        match self {
            Value::STRING(v) => encode_string(v),
            Value::NUMBER(v) => encode_number(v)
        }
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




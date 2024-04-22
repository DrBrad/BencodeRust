use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
//use std::fmt::Display;
use std::hash::BuildHasher;
use crate::variables::inter::bencode_type::BencodeType;

pub trait ToBencode {

    const TYPE: BencodeType;

    fn to_bencode(&self) -> Vec<u8>;
}

impl ToBencode for String {
    const TYPE: BencodeType = BencodeType::BYTES;

    fn to_bencode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();
        let z = self.as_bytes();

        r.extend_from_slice(z.len().to_string().as_bytes());
        r.push(Self::TYPE.delimiter() as u8);
        r.extend_from_slice(z);
        r
    }
}

impl ToBencode for &str {

    const TYPE: BencodeType = BencodeType::BYTES;

    fn to_bencode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();
        let z = self.as_bytes();

        r.extend_from_slice(z.len().to_string().as_bytes());
        r.push(Self::TYPE.delimiter() as u8);
        r.extend_from_slice(z);
        r
    }
}

macro_rules! impl_encodable_number {
    ($($type:ty)*) => {
        $(
            impl ToBencode for $type {

                const TYPE: BencodeType = BencodeType::NUMBER;

                fn to_bencode(&self) -> Vec<u8> {
                    format!("{}{}{}", Self::TYPE.prefix(), self, Self::TYPE.suffix()).into_bytes()
                }
            }
        )*
    }
}

impl_encodable_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

macro_rules! impl_encodable_iterable {
    ($($type:ident)*) => {
        $(
            impl <ContentT> ToBencode for $type<ContentT> where ContentT: ToBencode {

                const TYPE: BencodeType = BencodeType::ARRAY;

                fn to_bencode(&self) -> Vec<u8> {
                    let mut buf: Vec<u8> = Vec::new();
                    buf.push(Self::TYPE.prefix() as u8);

                    for item in self {
                        buf.extend_from_slice(&item.to_bencode());
                    }

                    buf.push(Self::TYPE.suffix() as u8);
                    buf
                }
            }
        )*
    }
}

impl_encodable_iterable!(Vec VecDeque LinkedList);




impl<K, V> ToBencode for BTreeMap<K, V> where K: ToBencode, V: ToBencode {

    const TYPE: BencodeType = BencodeType::OBJECT;

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(Self::TYPE.prefix() as u8);
        buf.push(b'd');

        for (key, value) in self {
            buf.extend_from_slice(&key.to_bencode());
            buf.extend_from_slice(&value.to_bencode());
        }

        buf.push(Self::TYPE.suffix() as u8);
        buf
    }
}

impl<K, V, S> ToBencode for HashMap<K, V, S> where K: ToBencode, V: ToBencode, S: BuildHasher {

    const TYPE: BencodeType = BencodeType::OBJECT;

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(b'd');

        for (key, value) in self {
            buf.extend_from_slice(&key.to_bencode());
            buf.extend_from_slice(&value.to_bencode());
        }

        buf.push(b'e');
        buf
    }
}



/*
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




impl <E: ToBencode> ToBencode for Box<E> {
//impl<'a, ContentT> ToBencode for &'a [ContentT] where ContentT: ToBencode {

    fn to_bencode(&self) -> Vec<u8> {
        Vec::new()
    }
}
*/




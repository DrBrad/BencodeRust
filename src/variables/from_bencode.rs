use std::collections::{LinkedList, VecDeque};
use crate::variables::decoder::{Decoder};
use crate::variables::to_bencode::ToBencode;

pub trait FromBencode {

    fn from_bencode(b: &Vec<u8>) -> Self;
}

impl FromBencode for String {

    fn from_bencode(buf: &Vec<u8>) -> Self {
        Decoder::new().decode_string(buf)
    }
}

macro_rules! impl_decodable_number {
    ($($type:ty)*) => {$(
        impl FromBencode for $type {

            fn from_bencode(buf: &Vec<u8>) -> Self {
                Decoder::new().decode_number(buf)
            }
        }
    )*}
}

impl_decodable_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);


/*
macro_rules! impl_decodable_iterable {
    ($($type:ident)*) => {
        $(
            impl<ContentT> FromBencode for $type<ContentT> where ContentT: FromBencode {

                fn from_bencode(b: &Vec<u8>) -> Self {
                    $type<ContentT>
                    /.*
                    let mut off = 0;

                    // Ensure the buffer starts with 'l'
                    if b.get(0) != Some(&b'l') {
                        panic!("Invalid Bencode list");
                    }

                    off += 1; // Move past the 'l'

                    let mut decoded_list = $type::new(); // Create the collection

                    // Continue until we find the list terminator 'e'
                    while let Some(&next) = b.get(off) {
                        if next == b'e' {
                            return decoded_list; // End of list, return the decoded collection
                        }

                        let item = ContentT::from_bencode(&b[off..].to_vec()); // Decode the item
                        decoded_list.push(item); // Add the item to the collection
                        off += item.to_bencode().len(); // Move past the decoded item
                    }

                    panic!("Invalid Bencode list"); // If we reach here, the list is malformed
                    *./
                }
            }
        )*
    };
}

impl_decodable_iterable!(Vec VecDeque LinkedList);
*/

macro_rules! impl_decodable_iterable {
    ($($type:ident)*) => {
        $(
            impl<ContentT> FromBencode for $type<ContentT> where ContentT: FromBencode {

                fn from_bencode(buf: &Vec<u8>) -> Self {
                    if buf[0] != b'l' {
                        panic!("Buffer is not a bencode array.");
                    }

                    println!("{}", stringify!($type));
                    println!("ContentT: {}", stringify!(ContentT));

                    let mut decoder = Decoder::new();

                    for i in 1..buf.len() {
                    //for (index, &byte) in buf.iter().enumerate() {
                        println!("{}", buf[i] as char);
                    }

                    $type::new()
                }
            }
        )*
    };
}

impl_decodable_iterable!(Vec VecDeque LinkedList);


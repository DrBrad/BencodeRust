use std::collections::HashMap;
use crate::variables::to_bencode::ToBencode;
use crate::variables::from_bencode::FromBencode;
use crate::variables::inter::bencode_type::BencodeType;

mod variables;
mod utils;



fn main() {
    //STRING TEST

    let encoded = "blank test".to_bencode();
    println!("{:?}", encoded);
    let mut off = 0;
    let decoded = String::from_bencode(&encoded, &mut off);
    println!("{}", decoded);
    //println!("{}", off);



    //NUMBER TEST

    let encoded = 100.56.to_bencode();
    println!("{:?}", encoded);
    let mut off = 0;
    let decoded = f32::from_bencode(&encoded, &mut off);
    println!("{}", decoded);
    //println!("{}", off);



    //LIST TEST

    let mut vec = Vec::new();
    vec.push("asdasd");
    vec.push("bloop");
    let encoded = vec.to_bencode();
    println!("{:?}", encoded);
    let mut off = 0;
    let decoded = Vec::<String>::from_bencode(&encoded, &mut off);
    //println!("{}", decoded.get(0));
    //println!("{}", off);

    for item in decoded {
        println!("{}", item);
    }



    //DICTIONARY TEST

    let mut dic = HashMap::new();
    dic.insert("hello", "123123");
    dic.insert("bloop", "poop");
    let encoded = dic.to_bencode();
    println!("{:?}", encoded);
    let mut off = 0;
    let decoded = HashMap::<String, String>::from_bencode(&encoded, &mut off);

    for (key, value) in decoded.iter() {
        println!("{}: {}", key, value);
    }

    //let stringify = std::str::from_utf8(&encoded).unwrap();
    //println!("{}", stringify);



    //LIST TEST

    let mut vec = Vec::new();
    vec.push(Poopie::STRING("asdasd".to_string()));
    vec.push(Poopie::STRING("asdasd3".to_string()));
    vec.push(Poopie::NUMBER(3123));
    let encoded = vec.to_bencode();
    println!("{:?}", encoded);
    //let string = String::from_utf8(encoded).expect("Invalid UTF-8 data");
    //println!("{}", string);

    let mut off = 0;
    let decoded = Vec::<String>::from_bencode(&encoded, &mut off);
    //println!("{}", decoded.get(0));
    //println!("{}", off);

    for item in decoded {
        println!("{}", item);
    }
}

pub enum Poopie {
    NUMBER(i32),
    STRING(String)
}

impl ToBencode for Poopie {

    const TYPE: BencodeType = BencodeType::BYTES;

    fn to_bencode(&self) -> Vec<u8> {
        match self {
            Poopie::NUMBER(num) => num.to_bencode(),
            Poopie::STRING(s) => s.to_bencode(),
        }
    }
}

impl FromBencode for Poopie {

    const TYPE: BencodeType = BencodeType::BYTES;

    fn from_bencode(buf: &Vec<u8>, off: &mut usize) -> Self {
        match BencodeType::type_by_prefix(buf[*off] as char) {
            BencodeType::NUMBER => {
                Poopie::NUMBER(i32::from_bencode(buf, off))
            }
            BencodeType::BYTES => {
                Poopie::STRING(String::from_bencode(buf, off))
            }
            _ => panic!("Invalid Bencode type for Poopie enum"),
        }
    }
}

use std::collections::HashMap;
use crate::variables::to_bencode::ToBencode;
use crate::variables::from_bencode::FromBencode;

mod variables;
mod utils;



fn main() {
    //STRING TEST

    let encoded = "blank test".to_bencode();
    println!("{:?}", encoded);
    let mut off = 0;
    let decoded = String::from_bencode(&encoded, &mut off);
    println!("{}", decoded);
    println!("{}", off);



    //NUMBER TEST

    let encoded = 100.56.to_bencode();
    println!("{:?}", encoded);
    let mut off = 0;
    let decoded = f32::from_bencode(&encoded, &mut off);
    println!("{}", decoded);
    println!("{}", off);



    //LIST TEST

    let mut vec = Vec::new();
    vec.push("asdasd");
    vec.push("bloop");
    let encoded = vec.to_bencode();
    println!("{:?}", encoded);
    let mut off = 0;
    let decoded = Vec::<String>::from_bencode(&encoded, &mut off);
    //println!("{}", decoded.get(0));
    println!("{}", off);

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
        println!("Key: {}, Value: {}", key, value);
    }
}

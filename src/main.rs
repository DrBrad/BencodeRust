//use std::collections::{BTreeMap, HashMap};
use crate::variables::to_bencode::ToBencode;
use crate::variables::from_bencode::FromBencode;
use crate::variables::to_bencode::Value::{NUMBER, STRING};

mod variables;

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
}

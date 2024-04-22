use std::collections::{BTreeMap, HashMap};
use crate::variables::to_bencode::ToBencode;
use crate::variables::from_bencode::FromBencode;
use crate::variables::to_bencode::Value::{NUMBER, STRING};

mod variables;


fn main() {

    let test = "blank test";
    let encoded = test.to_bencode();
    println!("{:?}", encoded);
    //println!("{}", String::from_utf8(encoded).expect("Invalid UTF-8"));

    //let bytes: &[u8] = b"hello";
    let decoded = String::from_bencode(&encoded);
    println!("{}", decoded);
    //println!("{}", encoded.len());



    let test = 100.56;
    let encoded = test.to_bencode();
    println!("{:?}", encoded);
    //println!("{}", String::from_utf8(encoded).expect("Invalid UTF-8"));

    //let decoded = f32::from_bencode(&encoded);
    //println!("{}", decoded);

    let decoded = f32::from_bencode(&encoded);
    println!("{}", decoded);
    //println!("{}", encoded.len());


    /*
    let mut test: Vec<String> = Vec::new();
    test.push("hello".to_string());
    test.push("test".to_string());
    test.push("world".to_string());
    let encoded = test.to_bencode();
    println!("{:?}", encoded);
    println!("{}", String::from_utf8(encoded).expect("Invalid UTF-8"));

    let mut test: Vec<i32> = Vec::new();
    test.push(100);
    test.push(5);
    test.push(7);
    let encoded = test.to_bencode();
    println!("{:?}", encoded);
    println!("{}", String::from_utf8(encoded).expect("Invalid UTF-8"));
    */


    /*
    let mut n = BTreeMap::new();
    n.insert("hello", STRING("world".to_string()));
    n.insert("hello2z", STRING("world5z".to_string()));
    n.insert("hello3z", STRING("world6z".to_string()));
    n.insert("yo", NUMBER(123.56));

    let encoded = n.to_bencode();
    println!("{:?}", encoded);
    println!("{}", String::from_utf8(encoded).expect("Invalid UTF-8"));

    let mut l = Vec::new();
    l.push("100");
    l.push("yo yo");
    l.push("this is a test");
    let encoded = l.to_bencode();
    println!("{:?}", encoded);
    */

    //let decoded = Vec::encoded.from_bencode();
    //println!("{}", decoded);



    /*
    let mut a = BencodeArray::new();
    a.add(BencodeNumber::from(100));
    println!("{}", a.l.len());
    println!("{:?}", a.l.get(0).unwrap().object())
    */

    /*
    let mut ex = Vec::new();
    ex.push("asdasd");
    ex.push("123123");

    println!("{:?}", ex.to_bencode().unwrap());


    let example = 21;
    let encoded = example.to_bencode().unwrap();

    println!("{:?}", encoded);

    let decoded = i64::from_bencode(&encoded).unwrap();
    println!("{}", decoded);
    */

    //let s = "asdasd".to_bencode2();

    //let encoded = ?;

    //let encoded = 21.to_bencode()?;
    //assert_eq!(b"i21e", encoded.as_slice());
}

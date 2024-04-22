pub mod variables;
pub mod utils;

/*
ADD FUNCTIONALITY FOR BYTES [u8] or Vec<u8> for byte to_bencode and from_bencode...
*/

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use crate::variables::to_bencode::ToBencode;
    use crate::variables::from_bencode::FromBencode;

    #[test]
    fn main() {
        let original = "blank test".as_bytes();
        let encoded = original.to_bencode();
        let decoded = String::from_bencode(&encoded, &mut 0);

        //println!("{}", decoded);
    }

    #[test]
    fn number() {
        let original = 100.67;
        let encoded = original.to_bencode();
        let decoded = f64::from_bencode(&encoded, &mut 0);

        assert_eq!(original, decoded);

        println!("Bencode Number Encoding & Decoding 100%");
    }

    #[test]
    fn bytes() {
        let original = "blank test".to_string();
        let encoded = original.to_bencode();
        let decoded = String::from_bencode(&encoded, &mut 0);

        assert_eq!(original, decoded);

        println!("Bencode Bytes / String Encoding & Decoding 100%");
    }

    #[test]
    fn array() {
        let mut vec = Vec::new();
        vec.push("number 1");
        vec.push("num 2");
        let encoded = vec.to_bencode();
        let decoded = Vec::<String>::from_bencode(&encoded, &mut 0);

        assert_eq!(vec.len(), decoded.len());

        for i in 0..=decoded.len()-1 {
            assert_eq!(vec[i], decoded[i]);
        }

        println!("Bencode Array Encoding & Decoding 100%");
    }

    #[test]
    fn object() {
        let mut dic = HashMap::new();
        dic.insert("hello".to_string(), "123123".to_string());
        dic.insert("bloop".to_string(), "another test".to_string());
        let encoded = dic.to_bencode();
        let decoded = HashMap::<String, String>::from_bencode(&encoded, &mut 0);

        assert_eq!(dic.len(), decoded.len());

        for key in decoded.keys() {
            if dic.contains_key(key) {
                assert_eq!(dic.get(key).unwrap(), decoded.get(key).unwrap());
            } else {
                panic!("Key '{}' does not exist in both maps", key);
            }
        }

        println!("Bencode Object Encoding & Decoding 100%");
    }
}

/*
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
    dic.insert("hello", Poopie::STRING("123123".to_string()));
    dic.insert("bloop", Poopie::STRING("poop".to_string()));
    dic.insert("ben", Poopie::NUMBER(765));
    let encoded = dic.to_bencode();
    println!("{:?}", encoded);
    let mut off = 0;
    let decoded = HashMap::<String, Poopie>::from_bencode(&encoded, &mut off);

    for (key, value) in decoded.iter() {
        println!("{}: {:?}", key, value);
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
    let decoded = Vec::<Poopie>::from_bencode(&encoded, &mut off);
    //println!("{}", decoded.get(0));
    //println!("{}", off);

    for item in &decoded {
        println!("{:?}", item);
    }
}

#[derive(Debug)]
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
*/
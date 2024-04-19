//use lava_torrent::bencode::BencodeElem;
use crate::variables::bencode_array::BencodeArray;

mod variables;

fn main() {
    //BencodeElem::Dictionary();
    //BencodeElem::String("asdasd".to_string());
    //let mut b = BencodeArray::new();
    //b.add(&BencodeArray::new());

    //println!("{}", b.l.len());
}
/*
#[derive(Debug)]
enum Bencode {
    Integer(i64),
    String(String),
    List(Vec<Bencode>),
    Dictionary(Vec<(String, Bencode)>),
}

fn encode_bencode(bencode: &Bencode) -> String {
    match bencode {
        Bencode::Integer(i) => format!("i{}e", i),
        Bencode::String(s) => format!("{}:{}", s.len(), s),
        Bencode::List(list) => {
            let encoded_items: Vec<String> = list.iter().map(encode_bencode).collect();
            format!("l{}e", encoded_items.join(""))
        }
        Bencode::Dictionary(dict) => {
            let mut encoded_items: Vec<String> = Vec::new();
            for (key, value) in dict {
                encoded_items.push(format!("{}{}", encode_bencode(&Bencode::String(key.clone())), encode_bencode(value)));
            }
            format!("d{}e", encoded_items.join(""))
        }
    }
}

fn main() {
    let data = Bencode::Dictionary(vec![
        ("name".to_string(), Bencode::String("Alice".to_string())),
        ("age".to_string(), Bencode::Integer(30)),
        ("hobbies".to_string(), Bencode::List(vec![
            Bencode::String("Programming".to_string()),
            Bencode::String("Gaming".to_string()),
        ])),
    ]);

    println!("{:?}", data);

    let encoded_data = encode_bencode(&data);
    println!("{}", encoded_data);
}
*/

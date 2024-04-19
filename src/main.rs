use bendy::decoding::FromBencode;
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_number::BencodeNumber;
use bendy::encoding::{ToBencode, SingleItemEncoder, Error};

mod variables;

fn main() {
    /*
    let mut a = BencodeArray::new();
    a.add(BencodeNumber::from(100));
    println!("{}", a.l.len());
    println!("{:?}", a.l.get(0).unwrap().object())
    */

    let mut ex = Vec::new();
    ex.push("asdasd");
    ex.push("123123");

    println!("{:?}", ex.to_bencode().unwrap());


    let example = IntegerWrapper(21);
    let encoded = example.to_bencode().unwrap();

    println!("{:?}", encoded);

    let decoded = i64::from_bencode(&encoded).unwrap();
    println!("{}", decoded);


    //let encoded = ?;

    //let encoded = 21.to_bencode()?;
    //assert_eq!(b"i21e", encoded.as_slice());
}

struct IntegerWrapper(i64);

impl ToBencode for IntegerWrapper {
    const MAX_DEPTH: usize = 0;

    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
        encoder.emit_int(self.0)
    }
}



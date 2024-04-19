use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_number::BencodeNumber;
use bendy::encoding::{ToBencode, Error};

mod variables;

fn main() {
    /*
    let mut a = BencodeArray::new();
    a.add(BencodeNumber::from(100));
    println!("{}", a.l.len());
    println!("{:?}", a.l.get(0).unwrap().object())
    */


    {
    let my_data = vec!["hello", "world"];
    let encoded = my_data.to_bencode()?;

    assert_eq!(b"l5:hello5:worlde", encoded.as_slice());
    }

    //Ok::<(), Error>(())
}


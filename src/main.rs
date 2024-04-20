use crate::variables::to_bencode::ToBencode;

mod variables;


fn main() {
    let test = "blank".to_string();
    let encoded = test.to_bencode();
    println!("{:?}", encoded);
    println!("{}", String::from_utf8(encoded).expect("Invalid UTF-8"));

    let test = 100;
    let encoded = test.to_bencode();
    println!("{:?}", encoded);
    println!("{}", String::from_utf8(encoded).expect("Invalid UTF-8"));

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


/*

pub trait ToBencode2 {
    /// The maximum depth that this object could encode to. Leaves do not consume a level, so an
    /// `i1e` has depth 0 and `li1ee` has depth 1.
    //const MAX_DEPTH: usize;

    /// Encode this object into the bencode stream
    fn encode(&self, encoder: SingleItemEncoder);// -> Result<(), Error>;

    /// Encode this object to a byte string
    fn to_bencode2(&self) -> Vec<u8> {
        Vec::new()
        //let mut encoder = Encoder::new().with_max_depth(Self::MAX_DEPTH);
        //encoder.emit_with(|e| self.encode(e).map_err(Error::into))?;

        //let bytes = encoder.get_output()?;
        //Ok(bytes)
    }
}

impl<'a> ToBencode2 for &'a str {

    fn encode(&self, encoder: SingleItemEncoder) {
        //encoder.emit_str(self).map_err(Error::from)
    }
}
*/





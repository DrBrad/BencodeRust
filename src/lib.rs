pub mod variables;

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use crate::variables::to_bencode::ToBencode;
    use crate::variables::from_bencode::FromBencode;
    use crate::variables::bencode_bytes::BencodeBytes;
    use crate::variables::bencode_object::BencodeObject;

    #[test]
    fn main() {
        let test = "asdasd".as_bytes();
        let test = BencodeBytes(test);
        let encoded = test.to_bencode();
        println!("{:?}", encoded);
        let decoded = BencodeBytes::from_bencode(&encoded, &mut 0);
        println!("{}", decoded.as_string());

        /*
        let mut obj = BencodeObject(HashMap::new());

        //obj.0.insert(BencodeBytes::from("hello".to_string()), BencodeBytes::from("world".to_string()));
        //obj.0.insert(BencodeBytes::from("net".to_string()), BencodeBytes::from("test".to_string()));

        let encoded = obj.to_bencode();
        println!("{:?}", encoded);

        let decoded = BencodeObject::<BencodeBytes>::from_bencode(&encoded, &mut 0);
        for key in decoded.0.keys() {
            println!("{} => {}", key.as_string(), decoded.0.get(key).unwrap().as_string());
        }
        */


        /*
        let mut s = BencodeObject(HashMap::new());
        s.0.insert(BencodeBytes("hello".as_bytes().to_vec()), BencodeBytes("world".as_bytes().to_vec()));

        println!("{:?}", s.to_bencode());


        let original = BencodeBytes("blank test".as_bytes().to_vec());
        let encoded = original.to_bencode();
        let decoded = BencodeBytes::from_bencode(&encoded, &mut 0);

        println!("{}", decoded.as_string());

        assert_eq!(original, decoded);

        println!("Bencode Bytes Encoding & Decoding 100%");
        */
    }

    /*
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

        println!("Bencode String Encoding & Decoding 100%");
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
    */
}

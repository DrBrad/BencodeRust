pub mod variables;
pub mod utils;

//TODO
//- add remove option
//- add if contains option
//- oberserver and mutable get IE get_mut
//- add i/o iption
//- error handling - Result for parsing num and bencode bytes - as_str

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;
    use crate::variables::bencode_array::{AddArray, BencodeArray};
    use crate::variables::bencode_bytes::BencodeBytes;
    use crate::variables::bencode_object::{BencodeObject, PutObject};
    use crate::variables::inter::bencode_variable::{Bencode, BencodeVariable};

    #[test]
    fn main() {
        /*
        let mut items = vec![1];
        let item = items.last();
        items.push(2);
        println!("{:?}", item);
        */

        let mut obj = BencodeObject::new();
        obj.put("object", BencodeObject::new());
        /*
        {
            let mut m = obj.get_object("object").unwrap();
            m.put("string", "strong");
        }
        */
        {
            let key = BencodeBytes::from("object");

            match obj.m.get_mut(&key).unwrap() {
                BencodeVariable::OBJECT(obj) => obj.put("string", "strong"),
                _ => unimplemented!()
            }
        }

        println!("{}", obj.to_string());

        /*
        obj.put("b", "bar");
        obj.put("c", "far");
        obj.put("n", 100);
        obj.put("y", [ 0u8, 0u8, 0u8, 0u8, 0u8, 0u8 ]);

        let mut arr = BencodeArray::new();
        arr.add("n");
        arr.add(123.56);
        obj.put("array", arr);

        let mut obj2 = BencodeObject::new();
        obj2.put("z", "another one");
        obj.put("object", obj2);
        */

        //z.deref_mut();
        //obj.put("blank", "blonk");

        /*
        let encoded = obj.encode();
        println!("{:?}", encoded);

        println!("{}", obj.to_string());

        let decoded = BencodeObject::decode(encoded);
        println!("{}", decoded.to_string());
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

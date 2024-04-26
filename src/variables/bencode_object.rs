use std::any::Any;
use std::str::FromStr;
use crate::utils::ordered_map::OrderedMap;
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::inter::bencode_variable::Bencode;
use crate::variables::inter::bencode_type::BencodeType;

//#[derive(Debug, Clone, PartialEq)]
pub struct BencodeObject {
    m: OrderedMap<BencodeBytes, Box<dyn Bencode>>
    //o: BencodeObserver
}

pub trait PutObject<V> {

    fn put(&mut self, key: &str, value: V);
}

impl BencodeObject {

    const TYPE: BencodeType = BencodeType::Object;

    pub fn new() -> Self {
        Self {
            m: OrderedMap::<BencodeBytes, Box<dyn Bencode>>::new()
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.m.contains_key(&BencodeBytes::from(key))
    }

    pub fn remove(&mut self, key: &str) {
        self.m.remove(&BencodeBytes::from(key));
    }

    pub fn get_number<V>(&self, key: &str) -> Result<V, ()> where V: FromStr {
        let key = BencodeBytes::from(key);
        self.m.get(&key).unwrap().as_any().downcast_ref::<BencodeNumber>().unwrap().parse::<V>()
    }

    pub fn get_bytes(&self, key: &str) -> Result<&[u8], ()> {
        let key = BencodeBytes::from(key);
        Ok(self.m.get(&key).unwrap().as_any().downcast_ref::<BencodeBytes>().unwrap().as_bytes())
    }

    pub fn get_string(&self, key: &str) -> Result<&str, ()> {
        let key = BencodeBytes::from(key);
        self.m.get(&key).unwrap().as_any().downcast_ref::<BencodeBytes>().unwrap().as_str()
    }

    /*
    pub fn get_array(&self, key: &str) -> Result<&BencodeArray<'a>, ()> {
        let key = BencodeBytes::from(key);

        match self.m.get(&key).unwrap() {
            BencodeVariable::Array(arr) => Ok(arr),
            _ => Err(())
        }
    }

    pub fn get_array_mut(&mut self, key: &str) -> Result<&mut BencodeArray<'a>, ()> {
        let key = BencodeBytes::from(key);

        match self.m.get_mut(&key).unwrap() {
            BencodeVariable::Array(arr) => Ok(arr),
            _ => Err(())
        }
    }

    pub fn get_object(&self, key: &str) -> Result<&BencodeObject<'a>, ()> {
        let key = BencodeBytes::from(key);

        match self.m.get(&key).unwrap() {
            BencodeVariable::Object(obj) => Ok(obj),
            _ => Err(())
        }
    }

    pub fn get_object_mut(&mut self, key: &str) -> Result<&mut BencodeObject<'a>, ()> {
        let key = BencodeBytes::from(key);

        match self.m.get_mut(&key).unwrap() {
            BencodeVariable::Object(obj) => Ok(obj),
            _ => Err(())
        }
    }
    */

    pub fn to_string(&self) -> String {
        let mut res = "{\r\n".to_string();

        for key in self.m.keys() {
            /*
            let value = match self.m.get(&key).unwrap() {
                BencodeVariable::Number(num) => format!("\t\x1b[31m{:?}\x1b[0m: \x1b[33m{}\x1b[0m\r\n", &key.to_string(), num.to_string()),
                BencodeVariable::Array(arr) => format!("\t\x1b[32m{:?}\x1b[0m: {}\r\n", &key.to_string(), arr.to_string().replace("\r\n", "\r\n\t")),
                BencodeVariable::Object(obj) => format!("\t\x1b[32m{:?}\x1b[0m: {}\r\n", &key.to_string(), obj.to_string().replace("\r\n", "\r\n\t")),
                BencodeVariable::Bytes(byt) => format!("\t\x1b[31m{:?}\x1b[0m: \x1b[34m{:?}\x1b[0m\r\n", &key.to_string(), byt.to_string())
            };
            res.push_str(value.as_str());
            */
        }

        res.push_str("}");
        res
    }
}

//REF FROM PARENT FOR OBSERVING UPDATES TO SIZE...
/*
impl<'a> From<BencodeObject<'a>> for BencodeObject<'a> {

    fn from(value: BencodeObject<'a>) -> Self {
        Self {
            m: OrderedMap::<BencodeBytes, BencodeVariable>::new(),
            s: 2
        }
    }
}

impl<'a> From<OrderedMap<BencodeBytes, BencodeVariable<'a>>> for BencodeObject<'a> {

    fn from(value: OrderedMap<BencodeBytes, BencodeVariable<'a>>) -> Self {
        //WE NEED TO COUNT THE SIZE...

        Self {
            m: value
        }
    }
}
*/

impl<const N: usize> PutObject<[u8; N]> for BencodeObject {

    fn put(&mut self, key: &str, value: [u8; N]) {
        self.m.insert(BencodeBytes::from(key), Box::new(BencodeBytes::from(value)));
    }
}

impl PutObject<Vec<u8>> for BencodeObject {

    fn put(&mut self, key: &str, value: Vec<u8>) {
        self.m.insert(BencodeBytes::from(key), Box::new(BencodeBytes::from(value)));
    }
}

impl PutObject<&str> for BencodeObject {

    fn put(&mut self, key: &str, value: &str) {
        self.m.insert(BencodeBytes::from(key), Box::new(BencodeBytes::from(value)));
    }
}

impl PutObject<String> for BencodeObject {

    fn put(&mut self, key: &str, value: String) {
        self.m.insert(BencodeBytes::from(key), Box::new(BencodeBytes::from(value)));
    }
}
/*
impl PutObject<BencodeArray> for BencodeObject {

    fn put(&mut self, key: &str, value: BencodeArray) {
        self.m.insert(BencodeBytes::from(key), Box::new(value));
    }
}
*/
impl PutObject<BencodeObject> for BencodeObject {

    fn put(&mut self, key: &str, value: BencodeObject) {
        self.m.insert(BencodeBytes::from(key), Box::new(value));
    }
}

macro_rules! impl_object_number {
    ($($type:ty)*) => {
        $(
            impl PutObject<$type> for BencodeObject {

                fn put(&mut self, key: &str, value: $type) {
                    self.m.insert(BencodeBytes::from(key), Box::new(BencodeNumber::from(value)));
                }
            }
        )*
    }
}

impl_object_number!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 isize f32 f64);

impl Bencode for BencodeObject {

    fn encode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.byte_size());
        buf.push(Self::TYPE.prefix());

        for (key, value) in self.m.iter() {
            buf.extend_from_slice(&key.encode());
            buf.extend_from_slice(&value.encode());
        }

        buf.push(Self::TYPE.suffix());
        buf
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Self where Self: Sized {
        if BencodeType::type_by_prefix(buf[off]) != Self::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        let mut off = off+1;

        let mut res = OrderedMap::<BencodeBytes, Box<dyn Bencode>>::new();//::with_hasher(Default::default());

        while buf[off] != Self::TYPE.suffix() as u8 {
            let key = BencodeBytes::decode_with_offset(buf, off);
            off += key.byte_size();
            let type_ = BencodeType::type_by_prefix(buf[off]);

            let value = match type_ {
                BencodeType::Number => {
                    let value = BencodeNumber::decode_with_offset(buf, off);
                    off += value.byte_size();
                    Box::new(value) as Box<dyn Bencode>
                },
                BencodeType::Array => {
                    //let value = BencodeArray::decode_with_offset(buf, off);
                    //off += value.byte_size();
                    //BencodeVariable::Array(value)
                    unimplemented!()
                },
                BencodeType::Object => {
                    let value = BencodeObject::decode_with_offset(buf, off);
                    off += value.byte_size();
                    Box::new(value) as Box<dyn Bencode>
                },
                BencodeType::Bytes => {
                    let value = BencodeBytes::decode_with_offset(buf, off);
                    off += value.byte_size();
                    Box::new(value) as Box<dyn Bencode>
                },
                _ => unimplemented!()
            };

            res.insert(key, value);
        }

        Self {
            m: res
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn byte_size(&self) -> usize {
        let mut s = 2;

        for (key, value) in self.m.iter() {
            s += key.byte_size()+value.byte_size();
        }

        s
    }
}

/*
impl<'a> Bencode<'a> for BencodeObject<'a> {

    fn decode_with_offset(buf: &'a [u8], off: usize) -> Self {
        if BencodeType::type_by_prefix(buf[off]) != Self::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        let mut off = off+1;

        let mut res = OrderedMap::<BencodeBytes, BencodeVariable>::new();//::with_hasher(Default::default());

        while buf[off] != Self::TYPE.suffix() as u8 {
            let key = BencodeBytes::decode_with_offset(buf, off);
            off += key.byte_size();
            let type_ = BencodeType::type_by_prefix(buf[off]);

            let value = match type_ {
                BencodeType::Number => {
                    let value = BencodeNumber::decode_with_offset(buf, off);
                    off += value.byte_size();
                    BencodeVariable::Number(value)
                },
                BencodeType::Array => {
                    let value = BencodeArray::decode_with_offset(buf, off);
                    off += value.byte_size();
                    BencodeVariable::Array(value)
                },
                BencodeType::Object => {
                    let value = BencodeObject::decode_with_offset(buf, off);
                    off += value.byte_size();
                    BencodeVariable::Object(value)
                },
                BencodeType::Bytes => {
                    let value = BencodeBytes::decode_with_offset(buf, off);
                    off += value.byte_size();
                    BencodeVariable::Bytes(value)
                },
                _ => unimplemented!()
            };

            res.insert(key, value);
        }

        Self {
            m: res
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.byte_size());
        buf.push(Self::TYPE.prefix());

        for (key, value) in self.m.iter() {
            buf.extend_from_slice(&key.encode());
            let value = match value {
                BencodeVariable::Number(num) => num.encode(),
                BencodeVariable::Array(arr) => arr.encode(),
                BencodeVariable::Object(obj) => obj.encode(),
                BencodeVariable::Bytes(byt) => byt.encode()
            };
            buf.extend_from_slice(&value);
        }

        buf.push(Self::TYPE.suffix());
        buf
    }
    /.*
    fn encode(&self) -> &[u8] {
        let mut data = vec![0u8; self.s];
        let mut index = 0;

        data[index] = Self::TYPE.prefix();
        index += 1;

        for (key, value) in self.m.iter() {
            let key_bencode = key.encode();
            let key_len = key_bencode.len();
            data[index..index + key_len].copy_from_slice(&key_bencode);
            index += key_len;

            let value = match value {
                BencodeVariable::Number(num) => num.encode(),
                BencodeVariable::Array(arr) => arr.encode(),
                BencodeVariable::Object(obj) => obj.encode(),
                BencodeVariable::Bytes(byt) => byt.encode(),
            };
            let value_len = value.len();
            data[index..index + value_len].copy_from_slice(&value);
            index += value_len;
        }

        data[index] = Self::TYPE.suffix();

        let ptr = data.as_ptr();
        let len = data.len();

        forget(data);

        unsafe {
            from_raw_parts(ptr, len)
        }
    }
    *./
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn byte_size(&self) -> usize {
        let mut s = 2;

        for (key, value) in self.m.iter() {
            s += key.byte_size();
            s += match value {
                BencodeVariable::Number(num) => num.byte_size(),
                BencodeVariable::Array(arr) => arr.byte_size(),
                BencodeVariable::Object(obj) => obj.byte_size(),
                BencodeVariable::Bytes(byt) => byt.byte_size(),
            };
        }

        s
    }
}
*/
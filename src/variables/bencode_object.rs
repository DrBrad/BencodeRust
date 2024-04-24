use std::mem::forget;
use std::slice::from_raw_parts;
use std::str::FromStr;
use crate::utils::ordered_map::OrderedMap;
use crate::variables::inter::bencode_variable::BencodeVariable;
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::inter::bencode_variable::Bencode;
use crate::variables::inter::bencode_type::BencodeType;

#[derive(Debug)]
pub struct BencodeObject<'a> {
    m: OrderedMap<BencodeBytes<'a>, BencodeVariable<'a>>,
    s: usize
}

pub trait PutObject<'a, V> {

    fn put(&mut self, key: &'a str, value: V);
}

impl<'a> BencodeObject<'a> {

    const TYPE: BencodeType = BencodeType::OBJECT;

    pub fn new() -> Self {
        Self {
            m: OrderedMap::<BencodeBytes, BencodeVariable>::new(),
            s: 2
        }
    }

    pub fn get_number<V>(&'a self, key: &'a str) -> Result<V, ()> where V: FromStr {
        let key = BencodeBytes::from(key);

        match self.m.get(&key).unwrap() {
            BencodeVariable::NUMBER(num) => Ok(num.parse::<V>()),
            _ => Err(())
        }
    }

    pub fn get_array(&'a self, key: &'a str) -> Result<&BencodeArray, ()> {
        let key = BencodeBytes::from(key);

        match self.m.get(&key).unwrap() {
            BencodeVariable::ARRAY(arr) => Ok(arr),
            _ => Err(())
        }
    }

    pub fn get_object(&'a self, key: &'a str) -> Result<&BencodeObject, ()> {
        let key = BencodeBytes::from(key);

        match self.m.get(&key).unwrap() {
            BencodeVariable::OBJECT(obj) => Ok(obj),
            _ => Err(())
        }
    }

    pub fn get_bytes(&'a self, key: &'a str) -> Result<&[u8], ()> {
        let key = BencodeBytes::from(key);

        match self.m.get(&key).unwrap() {
            BencodeVariable::BYTES(bytes) => Ok(bytes.as_bytes()),
            _ => Err(())
        }
    }

    pub fn get_string(&'a self, key: &'a str) -> Result<&str, ()> {
        let key = BencodeBytes::from(key);

        match self.m.get(&key).unwrap() {
            BencodeVariable::BYTES(bytes) => Ok(bytes.as_str()),
            _ => Err(())
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = "{\r\n".to_string();

        for key in self.m.keys() {
            let value = match self.m.get(&key).unwrap() {
                BencodeVariable::NUMBER(num) => format!("\t\x1b[31m{:?}\x1b[0m: \x1b[33m{}\x1b[0m\r\n", &key.to_string(), num.to_string()),
                BencodeVariable::ARRAY(arr) => format!("\t\x1b[32m{:?}\x1b[0m: {}\r\n", &key.to_string(), arr.to_string().replace("\r\n", "\r\n\t")),
                BencodeVariable::OBJECT(obj) => format!("\t\x1b[32m{:?}\x1b[0m: {}\r\n", &key.to_string(), obj.to_string().replace("\r\n", "\r\n\t")),
                BencodeVariable::BYTES(byt) => format!("\t\x1b[31m{:?}\x1b[0m: \x1b[34m{:?}\x1b[0m\r\n", &key.to_string(), byt.to_string())
            };
            res.push_str(value.as_str());
        }

        res.push_str("}");
        res
    }
}

impl<'a> From<OrderedMap<BencodeBytes<'a>, BencodeVariable<'a>>> for BencodeObject<'a> {

    fn from(value: OrderedMap<BencodeBytes<'a>, BencodeVariable<'a>>) -> Self {
        //WE NEED TO COUNT THE SIZE...

        Self {
            m: value,
            s: 2
        }
    }
}

impl<'a, const N: usize> PutObject<'a, &'a [u8; N]> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: &'a [u8; N]) {
        let key = BencodeBytes::from(key);
        let value = BencodeBytes::from(value);
        self.s += key.byte_size()+value.byte_size();
        self.m.insert(key, BencodeVariable::BYTES(value));
    }
}

impl<'a> PutObject<'a, &'a str> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: &'a str) {
        let key = BencodeBytes::from(key);
        let value = BencodeBytes::from(value);
        self.s += key.byte_size()+value.byte_size();
        self.m.insert(key, BencodeVariable::BYTES(value));
    }
}

impl<'a> PutObject<'a, String> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: String) {
        let key = BencodeBytes::from(key);
        let value = BencodeBytes::from(value);
        self.s += key.byte_size()+value.byte_size();
        self.m.insert(key, BencodeVariable::BYTES(value));
    }
}

impl<'a> PutObject<'a, BencodeArray<'a>> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: BencodeArray<'a>) {
        let key = BencodeBytes::from(key);
        self.s += key.byte_size()+value.byte_size();
        self.m.insert(key, BencodeVariable::ARRAY(value));
    }
}

impl<'a> PutObject<'a, BencodeObject<'a>> for BencodeObject<'a> {

    fn put(&mut self, key: &'a str, value: BencodeObject<'a>) {
        let key = BencodeBytes::from(key);
        self.s += key.byte_size()+value.byte_size();
        self.m.insert(key, BencodeVariable::OBJECT(value));
    }
}

macro_rules! impl_object_number {
    ($($type:ty)*) => {
        $(
            impl<'a> PutObject<'a, $type> for BencodeObject<'a> {

                fn put(&mut self, key: &'a str, value: $type) {
                    let key = BencodeBytes::from(key);
                    let value = BencodeNumber::from(value);
                    self.s += key.byte_size()+value.byte_size();
                    self.m.insert(key, BencodeVariable::NUMBER(value));
                }
            }
        )*
    }
}

impl_object_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl<'a> Bencode<'a> for BencodeObject<'a> {

    fn decode_with_offset(buf: &'a [u8], off: usize) -> Self {
        if BencodeType::type_by_prefix(buf[off]) != Self::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        let mut s = off;
        let mut off = off+1;

        let mut res = OrderedMap::<BencodeBytes, BencodeVariable>::new();//::with_hasher(Default::default());

        while buf[off] != Self::TYPE.suffix() as u8 {
            let key = BencodeBytes::decode_with_offset(buf, off);
            off += key.byte_size();
            let type_ = BencodeType::type_by_prefix(buf[off]);

            let value = match type_ {
                BencodeType::NUMBER => {
                    let value = BencodeNumber::decode_with_offset(buf, off);
                    off += value.byte_size();
                    BencodeVariable::NUMBER(value)
                },
                BencodeType::ARRAY => {
                    let value = BencodeArray::decode_with_offset(buf, off);
                    off += value.byte_size();
                    BencodeVariable::ARRAY(value)
                },
                BencodeType::OBJECT => {
                    let value = BencodeObject::decode_with_offset(buf, off);
                    off += value.byte_size();
                    BencodeVariable::OBJECT(value)
                },
                BencodeType::BYTES => {
                    let value = BencodeBytes::decode_with_offset(buf, off);
                    off += value.byte_size();
                    BencodeVariable::BYTES(value)
                },
                _ => unimplemented!()
            };

            res.insert(key, value);
        }

        off += 1;
        s = off-s;

        Self {
            m: res,
            s
        }
    }

    /*
    fn to_bencode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(Self::TYPE.prefix());

        for (key, value) in self.m.iter() {
            buf.extend_from_slice(&key.to_bencode());
            let value = match value {
                BencodeVariables::NUMBER(num) => num.to_bencode(),
                BencodeVariables::ARRAY(arr) => arr.to_bencode(),
                BencodeVariables::OBJECT(obj) => obj.to_bencode(),
                BencodeVariables::BYTES(byt) => byt.to_bencode()
            };
            buf.extend_from_slice(&value);
        }

        buf.push(Self::TYPE.suffix());
        buf
    }
    */
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
                BencodeVariable::NUMBER(num) => num.encode(),
                BencodeVariable::ARRAY(arr) => arr.encode(),
                BencodeVariable::OBJECT(obj) => obj.encode(),
                BencodeVariable::BYTES(byt) => byt.encode(),
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

    fn byte_size(&self) -> usize {
        self.s
    }
}

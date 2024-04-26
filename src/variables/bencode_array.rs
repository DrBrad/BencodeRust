use std::any::Any;
use std::str::FromStr;
//use crate::variables::inter::bencode_variable::BencodeVariable;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::bencode_object::BencodeObject;
use crate::variables::inter::bencode_variable::{Bencode, Bencode2};
use crate::variables::inter::bencode_type::BencodeType;

//#[derive(Debug, Clone, PartialEq)]
pub struct BencodeArray {
    l: Vec<Box<dyn Bencode>>
}

pub trait AddArray<V> {

    fn add(&mut self, value: V);
}

impl BencodeArray {

    const TYPE: BencodeType = BencodeType::Array;

    pub fn new() -> Self {
        Self {
            l: Vec::new()
        }
    }
/*
    pub fn contains(&self, var: &Box<dyn Bencode>) -> bool {
        self.l.contains(var)
    }
*/
    pub fn remove(&mut self, index: usize) {
        self.l.remove(index);
    }

    /*
    pub fn get_number<V>(&self, index: usize) -> Result<V, ()> where V: FromStr {
        match self.l.get(index).unwrap() {
            BencodeVariable::Number(num) => num.parse::<V>(),
            _ => Err(())
        }
    }

    pub fn get_array(&self, index: usize) -> Result<&BencodeArray, ()> {
        match self.l.get(index).unwrap() {
            BencodeVariable::Array(arr) => Ok(arr),
            _ => Err(())
        }
    }

    pub fn get_array_mut(&mut self, index: usize) -> Result<&mut BencodeArray, ()> {
        match self.l.get_mut(index).unwrap() {
            BencodeVariable::Array(arr) => Ok(arr),
            _ => Err(())
        }
    }

    pub fn get_object(&self, index: usize) -> Result<&BencodeObject, ()> {
        match self.l.get(index).unwrap() {
            BencodeVariable::Object(obj) => Ok(obj),
            _ => Err(())
        }
    }

    pub fn get_object_mut(&mut self, index: usize) -> Result<&mut BencodeObject, ()> {
        match self.l.get_mut(index).unwrap() {
            BencodeVariable::Object(obj) => Ok(obj),
            _ => Err(())
        }
    }

    pub fn get_bytes(&self, index: usize) -> Result<&[u8], ()> {
        match self.l.get(index).unwrap() {
            BencodeVariable::Bytes(bytes) => Ok(bytes.as_bytes()),
            _ => Err(())
        }
    }

    pub fn get_string(&self, index: usize) -> Result<&str, ()> {
        match self.l.get(index).unwrap() {
            BencodeVariable::Bytes(bytes) => bytes.as_str(),
            _ => Err(())
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = "[\r\n".to_string();

        for item in self.l.iter() {
            let item = match item {
                BencodeVariable::Number(num) => format!("\t\x1b[33m{}\x1b[0m\r\n", num.to_string()),
                BencodeVariable::Array(arr) => format!("\t{}\r\n", arr.to_string().replace("\r\n", "\r\n\t")),
                BencodeVariable::Object(obj) => format!("\t{}\r\n", obj.to_string().replace("\r\n", "\r\n\t")),
                BencodeVariable::Bytes(byt) => format!("\t\x1b[34m{:?}\x1b[0m\r\n", byt.to_string())
            };
            res.push_str(item.as_str());
        }

        res.push_str("]");
        res
    }
    */
}
/*
impl From<Vec<Box<dyn Bencode>>> for BencodeArray {

    fn from(value: Vec<BencodeVariable>) -> Self {
        //WE NEED TO COUNT THE SIZE...

        Self {
            l: value
        }
    }
}
*/

impl<const N: usize> AddArray<[u8; N]> for BencodeArray {

    fn add(&mut self, value: [u8; N]) {
        self.l.push(Box::new(BencodeBytes::from(value)));
    }
}

impl AddArray<Vec<u8>> for BencodeArray {

    fn add(&mut self, value: Vec<u8>) {
        self.l.push(Box::new(BencodeBytes::from(value)));
    }
}

impl AddArray<&str> for BencodeArray {

    fn add(&mut self, value: &str) {
        self.l.push(Box::new(BencodeBytes::from(value)));
    }
}

impl AddArray<String> for BencodeArray {

    fn add(&mut self, value: String) {
        self.l.push(Box::new(BencodeBytes::from(value)));
    }
}

impl AddArray<BencodeArray> for BencodeArray {

    fn add(&mut self, value: BencodeArray) {
        self.l.push(Box::new(value));
    }
}

impl AddArray<BencodeObject> for BencodeArray {

    fn add(&mut self, value: BencodeObject) {
        self.l.push(Box::new(value));
    }
}

macro_rules! impl_array_number {
    ($($type:ty)*) => {
        $(
            impl AddArray<$type> for BencodeArray {

                fn add(&mut self, value: $type) {
                    self.l.push(Box::new(BencodeNumber::from(value)));
                }
            }
        )*
    }
}

impl_array_number!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 isize f32 f64);

impl Bencode for BencodeArray {

    fn encode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.byte_size());
        buf.push(Self::TYPE.prefix());

        for item in &self.l {
            buf.extend_from_slice(&item.encode());
        }

        buf.push(Self::TYPE.suffix());
        buf
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Self {
        if BencodeType::type_by_prefix(buf[off]) != Self::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        let mut off = off+1;

        let mut res = Vec::new();

        while buf[off] != Self::TYPE.suffix() {
            let type_ = BencodeType::type_by_prefix(buf[off]);

            let item = match type_ {
                BencodeType::Number => {
                    let value = BencodeNumber::decode_with_offset(buf, off);
                    off += value.byte_size();
                    Box::new(value) as Box<dyn Bencode>
                },
                BencodeType::Array => {
                    let value = BencodeArray::decode_with_offset(buf, off);
                    off += value.byte_size();
                    Box::new(value) as Box<dyn Bencode>
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

            res.push(item);
        }

        Self {
            l: res
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn byte_size(&self) -> usize {
        let mut s = 2;

        for item in &self.l {
            s += item.byte_size();
        }

        s
    }
}

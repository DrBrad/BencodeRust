use std::mem::forget;
use std::slice::from_raw_parts;
use std::str::FromStr;
use crate::BencodeVariables;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::bencode_object::BencodeObject;
use crate::variables::inter::bencode::Bencode;
use crate::variables::inter::bencode_type::BencodeType;

#[derive(Debug)]
pub struct BencodeArray<'a> {
    l: Vec<BencodeVariables<'a>>,
    s: usize
}

pub trait AddArray<'a, V> {

    fn add(&mut self, value: V);
}

impl<'a> BencodeArray<'a> {

    const TYPE: BencodeType = BencodeType::ARRAY;

    pub fn new() -> Self {
        Self {
            l: Vec::new(),
            s: 2
        }
    }

    pub fn get_number<V>(&'a self, index: usize) -> Result<V, ()> where V: FromStr {
        match self.l.get(index).unwrap() {
            BencodeVariables::NUMBER(num) => Ok(num.parse::<V>()),
            _ => Err(())
        }
    }

    pub fn get_array(&'a self, index: usize) -> Result<&BencodeArray, ()> {
        match self.l.get(index).unwrap() {
            BencodeVariables::ARRAY(arr) => Ok(arr),
            _ => Err(())
        }
    }

    pub fn get_object(&'a self, index: usize) -> Result<&BencodeObject, ()> {
        match self.l.get(index).unwrap() {
            BencodeVariables::OBJECT(obj) => Ok(obj),
            _ => Err(())
        }
    }

    pub fn get_bytes(&'a self, index: usize) -> Result<&[u8], ()> {
        match self.l.get(index).unwrap() {
            BencodeVariables::BYTES(bytes) => Ok(bytes.as_bytes()),
            _ => Err(())
        }
    }

    pub fn get_string(&'a self, index: usize) -> Result<&str, ()> {
        match self.l.get(index).unwrap() {
            BencodeVariables::BYTES(bytes) => Ok(bytes.as_str()),
            _ => Err(())
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = "[\r\n".to_string();

        for item in self.l.iter() {
            let item = match item {
                BencodeVariables::NUMBER(num) => format!("\t\x1b[33m{}\x1b[0m\r\n", num.to_string()),
                BencodeVariables::ARRAY(arr) => format!("\t{}\r\n", arr.to_string().replace("\r\n", "\r\n\t")),
                BencodeVariables::OBJECT(obj) => format!("\t{}\r\n", obj.to_string().replace("\r\n", "\r\n\t")),
                BencodeVariables::BYTES(byt) => format!("\t\x1b[34m{:?}\x1b[0m\r\n", byt.to_string())
            };
            res.push_str(item.as_str());
        }

        res.push_str("]");
        res
    }
}

impl<'a> From<Vec<BencodeVariables<'a>>> for BencodeArray<'a> {

    fn from(value: Vec<BencodeVariables<'a>>) -> Self {
        //WE NEED TO COUNT THE SIZE...

        Self {
            l: value,
            s: 2
        }
    }
}

impl<'a, const N: usize> AddArray<'a, &'a [u8; N]> for BencodeArray<'a> {

    fn add(&mut self, value: &'a [u8; N]) {
        let value = BencodeBytes::from(value);
        self.s += value.byte_size();
        self.l.push(BencodeVariables::BYTES(value));
    }
}

impl<'a> AddArray<'a, &'a str> for BencodeArray<'a> {

    fn add(&mut self, value: &'a str) {
        let value = BencodeBytes::from(value);
        self.s += value.byte_size();
        self.l.push(BencodeVariables::BYTES(value));
    }
}

impl<'a> AddArray<'a, String> for BencodeArray<'a> {

    fn add(&mut self, value: String) {
        let value = BencodeBytes::from(value);
        self.s += value.byte_size();
        self.l.push(BencodeVariables::BYTES(value));
    }
}

impl<'a> AddArray<'a, BencodeArray<'a>> for BencodeArray<'a> {

    fn add(&mut self, value: BencodeArray<'a>) {
        self.s += value.byte_size();
        self.l.push(BencodeVariables::ARRAY(value));
    }
}

impl<'a> AddArray<'a, BencodeObject<'a>> for BencodeArray<'a> {

    fn add(&mut self, value: BencodeObject<'a>) {
        self.s += value.byte_size();
        self.l.push(BencodeVariables::OBJECT(value));
    }
}

macro_rules! impl_array_number {
    ($($type:ty)*) => {
        $(
            impl<'a> AddArray<'a, $type> for BencodeArray<'a> {

                fn add(&mut self, value: $type) {
                    let value = BencodeNumber::from(value);
                    self.s += value.byte_size();
                    self.l.push(BencodeVariables::NUMBER(value));
                }
            }
        )*
    }
}

impl_array_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl<'a> Bencode<'a> for BencodeArray<'a> {

    fn decode(buf: &'a [u8], off: &mut usize) -> Self {
        if BencodeType::type_by_prefix(buf[*off]) != Self::TYPE {
            panic!("Buffer is not a bencode array.");
        }

        let mut s = *off;
        *off += 1;

        let mut res = Vec::new();

        while buf[*off] != Self::TYPE.suffix() as u8 {
            let type_ = BencodeType::type_by_prefix(buf[*off]);

            let item = match type_ {
                BencodeType::NUMBER => BencodeVariables::NUMBER(BencodeNumber::decode(buf, off)),
                BencodeType::ARRAY => BencodeVariables::ARRAY(BencodeArray::decode(buf, off)),
                BencodeType::OBJECT => BencodeVariables::OBJECT(BencodeObject::decode(buf, off)),
                BencodeType::BYTES => BencodeVariables::BYTES(BencodeBytes::decode(buf, off)),
                _ => unimplemented!()
            };

            res.push(item);
        }

        *off += 1;
        s = *off-s;

        Self {
            l: res,
            s
        }
    }

    /*
    fn to_bencode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(Self::TYPE.prefix());

        for item in &self.l {
            let item = match item {
                BencodeVariables::NUMBER(num) => num.to_bencode(),
                BencodeVariables::ARRAY(arr) => arr.to_bencode(),
                BencodeVariables::OBJECT(obj) => obj.to_bencode(),
                BencodeVariables::BYTES(byt) => byt.to_bencode()
            };
            buf.extend_from_slice(&item);
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

        for item in &self.l {
            let item = match item {
                BencodeVariables::NUMBER(num) => num.encode(),
                BencodeVariables::ARRAY(arr) => arr.encode(),
                BencodeVariables::OBJECT(obj) => obj.encode(),
                BencodeVariables::BYTES(byt) => byt.encode(),
            };
            let item_len = item.len();
            data[index..index + item_len].copy_from_slice(&item);
            index += item_len;
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

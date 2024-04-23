use crate::variables::from_bencode::FromBencode;
use crate::variables::inter::bencode_type::BencodeType;
use crate::variables::to_bencode::ToBencode;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct BencodeBytes<'a>(pub &'a [u8]);//Vec<u8>

impl<'a> BencodeBytes<'a> {

    const TYPE: BencodeType = BencodeType::BYTES;

    pub fn as_string(&self) -> &str {
        std::str::from_utf8(self.0.clone()).unwrap_or_else(|_| panic!("Failed to parse UTF-8 string"))
    }

    /*
    fn from_bencode(buf: &'a Vec<u8>, off: &mut usize) -> Self {
        let bytes = &buf[*off..];
        *off = buf.len();
        BencodeBytes(bytes)
    }
    */
}

impl<'a> From<&'a str> for BencodeBytes<'a> {

    fn from(value: &'a str) -> Self {
        BencodeBytes(value.as_bytes())
    }
}

impl<'a> From<String> for BencodeBytes<'a> {

    fn from(value: String) -> Self {
        let bytes = value.as_ptr(); // Get raw pointer to string's buffer
        let len = value.len(); // Get length of string
        std::mem::forget(value); // Forget about the string to prevent double-free

        // Safety: We assume that the pointer remains valid for the lifetime 'a
        unsafe {
            BencodeBytes(std::slice::from_raw_parts(bytes, len))
        }
        //BencodeBytes(value.as_bytes())
    }
}

impl<'a> FromBencode<'a> for BencodeBytes<'a> {

    fn from_bencode(buf: &'a Vec<u8>, off: &mut usize) -> Self {
        //let bytes = &buf[*off..];
        //*off = buf.len();
        //BencodeBytes(bytes)

        if BencodeType::type_by_prefix(buf[*off] as char) != Self::TYPE {
            panic!("Buffer is not a bencode bytes / string.");
        }

        let mut len_bytes = [0; 8];
        let start = *off;

        while buf[*off] != Self::TYPE.delimiter() as u8 {
            len_bytes[*off - start] = buf[*off];
            *off += 1;
        }

        let length = len_bytes.iter().take(*off - start).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        let bytes = &buf[*off + 1..*off + 1 + length];//.to_vec();

        *off += 1+length;

        Self(bytes)
    }
}

impl<'a> ToBencode for BencodeBytes<'_> {

    fn to_bencode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();

        r.extend_from_slice(self.0.len().to_string().as_bytes());
        r.push(Self::TYPE.delimiter() as u8);
        r.extend_from_slice(self.0.clone());
        r
    }
}

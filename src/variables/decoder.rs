
pub fn decode_string(buf: &Vec<u8>, mut off: usize) -> String {
    //let pos = v.iter().position(|&b| b == b':');
    //let len = v.as_slice(0, pos);

    /*
    if let Some(bencode_type) = BencodeType::get_type_by_prefix(buf[off] as char) {
        if bencode_type != self.type {
            panic!("Byte array is not a Bencode bytes / string.");
        }
    } else {
        panic!("Unknown Bencode type.");
    }
    */

    let mut c = [0; 8];
    let s = off;

    while buf[off] != ':' as u8 {
        c[off - s] = buf[off];
        off += 1;
    }

    let mut length = 0;
    for i in 0..(off - s) {
        length = length * 10 + (c[i] - b'0') as usize;
    }

    let b = &buf[off + 1..off + 1 + length];
    //str::from_utf8(Vec::from(b)).expect("asd")
    //self.s = (off - s) + length + 1;
    let b = Vec::from(b);
    String::from_utf8(b).expect("Invalid UTF-8 string")

    //"asdasd"
}
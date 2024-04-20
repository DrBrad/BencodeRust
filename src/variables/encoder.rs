
pub fn encode_string(v: &str) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    let z = v.as_bytes();

    r.extend_from_slice(z.len().to_string().as_bytes());
    /*
    for c in z.len().to_string().chars() {
        r.push(c as u8);
    }
    */

    r.push(b':');

    r.extend_from_slice(z);
    r
}

pub fn encode_number(v: &u32) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    r.push(b'i');
    r.extend_from_slice(v.to_string().as_bytes());
    r.push(b'e');
    r
}


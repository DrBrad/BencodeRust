pub trait Encoder {

}


pub fn encode_string(v: &str) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    let z = v.as_bytes();

    for c in z.len().to_string().chars() {
        r.push(c as u8);
    }

    r.push(':' as u8);

    r.extend_from_slice(z);
    r
}


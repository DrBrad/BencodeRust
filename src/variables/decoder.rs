
pub fn decode_string(buf: &Vec<u8>, mut off: usize) -> String {
    let mut len_bytes = [0; 8];
    let start = off;

    while buf[off] != b':' {
        len_bytes[off - start] = buf[off];
        off += 1;
    }

    let length = len_bytes.iter().take(off - start).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
    let string_bytes = &buf[off + 1..off + 1 + length];

    String::from_utf8_lossy(string_bytes).into_owned()
}
pub trait FromBencode<'a> {

    fn from_bencode(buf: &'a Vec<u8>, off: &mut usize) -> Self;
}

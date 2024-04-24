pub trait Bencode<'a> {

    fn from_bencode(buf: &'a Vec<u8>, off: &mut usize) -> Self;

    fn to_bencode(&self) -> &[u8];//Vec<u8>;

    fn byte_size(&self) -> usize;
}

pub trait Bencode<'a> {

    fn from_bencode(buf: &'a [u8], off: &mut usize) -> Self;

    fn to_bencode(&self) -> &[u8];

    fn byte_size(&self) -> usize;
}

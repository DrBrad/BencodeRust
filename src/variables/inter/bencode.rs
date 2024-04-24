pub trait Bencode<'a> {

    fn decode(buf: &'a [u8], off: &mut usize) -> Self;

    fn encode(&self) -> &[u8];

    fn byte_size(&self) -> usize;
}

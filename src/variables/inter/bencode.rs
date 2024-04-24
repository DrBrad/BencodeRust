pub trait Bencode<'a> {

    fn decode(buf: &'a [u8]) -> Self where Self: Sized {
        Self::decode_with_offset(buf, 0)
    }

    fn decode_with_offset(buf: &'a [u8], off: usize) -> Self;

    fn encode(&self) -> &[u8];

    fn byte_size(&self) -> usize;
}

/*
public interface BencodeVariable {

Object getObject();

int byteSize();

byte[] encode();
}
*/
pub trait BencodeVariable {

    //Object
    fn byte_size(&self) -> u32;

    fn encode(&self) -> Vec<u8>;
}

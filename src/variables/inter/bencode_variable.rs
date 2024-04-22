
pub enum BencodeVariable<T> {
    NUMBER(T),
    ARRAY(Vec<T>),
    OBJECT(),
    BYTEs(T)
}
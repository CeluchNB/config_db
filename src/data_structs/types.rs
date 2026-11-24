use std::fmt::Display;

pub trait HasLen {
    fn len(&self) -> usize;
    fn into_bytes(self) -> Vec<u8>;
    fn from_utf8(bytes: Vec<u8>) -> Self;
}

impl HasLen for String {
    fn len(&self) -> usize {
        String::len(self)
    }

    fn into_bytes(self) -> Vec<u8> {
        String::into_bytes(self)
    }

    fn from_utf8(bytes: Vec<u8>) -> Self {
        String::from_utf8(bytes.clone()).unwrap()
    }
}

pub trait Key: Clone + PartialOrd + Display + HasLen {}
impl<T: Clone + PartialOrd + Display + HasLen> Key for T {}

pub trait Value: Clone + Display + HasLen {}
impl<T: Clone + Display + HasLen> Value for T {}

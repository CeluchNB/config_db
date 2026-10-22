use std::fmt::Display;

pub trait HasLen {
    fn len(&self) -> usize;
    fn into_bytes(self) -> Vec<u8>;
}

impl HasLen for String {
    fn len(&self) -> usize {
        String::len(self)
    }

    fn into_bytes(self) -> Vec<u8> {
        String::into_bytes(self)
    }
}

pub trait Key: Clone + PartialOrd + Display + HasLen {}
impl<T: Clone + PartialOrd + Display + HasLen> Key for T {}

pub trait Value: Clone + Display + HasLen {}
impl<T: Clone + Display + HasLen> Value for T {}

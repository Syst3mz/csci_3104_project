use std::ops::Deref;
use crate::corpus::KnownLength;

#[derive(Debug)]
pub struct StringWrapped<T> {
    pub(crate) payload: String,
    pub(crate) internal: T
}

impl<T> ToString for StringWrapped<T> {
    fn to_string(&self) -> String {
        self.payload.clone()
    }
}

impl<T> Deref for StringWrapped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T: KnownLength> KnownLength for StringWrapped<T> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}
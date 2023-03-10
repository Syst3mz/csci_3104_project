#[doc(inline)]
use std::ops::Deref;
use crate::corpus::KnownLength;

#[derive(Debug)]

/// A simple helper struct which stores a String alongside the interal value.
pub struct StringWrapped<T> {
    /// The "payload" is the string held
    pub(crate) payload: String,
    /// What this struct is wrapping around.
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
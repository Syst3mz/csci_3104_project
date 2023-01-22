#[doc(inline)]
use std::collections::HashSet;
use std::fmt::Debug;
use std::iter::{Flatten};
use std::slice::Iter;

#[derive(Debug)]
/// A Corpus<T> is a data structure which stores its elements in buckets.
/// Each bucket is of a different length, and the buckets are stored in order from smallest to largest
pub struct Corpus<T: Debug+KnownLength> {
    pub data: Vec<Vec<T>>
}

impl<T: Debug+KnownLength> Corpus<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }

    /// When adding, place the item into its correct bucket based on that items length
    pub fn add(&mut self, item:T) {
        // Ensure that there are at least as many buckets as the length of the current input
        if item.len() > self.data.len() {
            for _c in self.data.len()..item.len() {
                self.data.push(vec![])
            }
        }

        self.data[item.len() - 1].push(item)
    }

    /// Get all buckets that are above a certain size.
    pub fn get_above(&self, n:usize) -> Flatten<Iter<'_, Vec<T>>> {
        self.data[n..].iter().flatten()
    }
}

/// Simple trait so that Corpus knows how long an item is.
pub trait KnownLength {
    fn len(&self) -> usize;
}

impl<T> KnownLength for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> KnownLength for HashSet<T> {
    fn len(&self) -> usize {
        self.len()
    }
}
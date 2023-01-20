use std::collections::HashSet;
use std::fmt::Debug;
use std::slice::Iter;

#[derive(Debug)]
pub struct Corpus<T: Debug+KnownLength> {
    pub data: Vec<Vec<T>>
}

impl<T: Debug+KnownLength> Corpus<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }

    pub fn add(&mut self, item:T) {
        if item.len() > self.data.len() {
            for _c in self.data.len()..item.len() {
                self.data.push(vec![])
            }
        }

        self.data[item.len() - 1].push(item)
    }

    pub fn get_above(&self, n:usize) -> Iter<'_, Vec<T>> {
        self.data[n..].iter()
    }
}

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
use std::collections::HashMap;
use std::hash::Hash;
use std::simd::u64x4;
use crate::corpus::KnownLength;

pub struct BitsetBuilder<T: Hash+Eq+PartialEq> {
    dict: HashMap<T, usize>
}

impl<T: Hash+Eq+PartialEq> BitsetBuilder<T> {
    pub fn add(&mut self, to_add: Vec<T>) -> Bitset {
        let mut set: Bitset = Bitset::new();
        for x in to_add {
            let l = self.dict.len();
            set.mark(*self.dict.entry(x).or_insert(l))
        }

        set
    }


    pub fn new() -> Self {
        Self { dict: HashMap::new() }
    }
}


#[derive(Debug, PartialEq)]
pub struct Bitset{
    internal: Vec<u64x4>,
    size: usize
}


impl Bitset {
    pub fn new() -> Self {
        Self { internal: Default::default(), size: 0 }
    }

    pub fn mark(&mut self, num: usize) {
        let partition = num / 256;
        let idx = num % 256;

        while self.internal.len() <= partition {
            self.internal.push(u64x4::default())
        }

        self.internal[partition].as_mut_array()[idx / 64] |= 1 << (idx % 64);
        self.size += 1;
    }

    pub fn is_subset(&self, other: &Bitset) -> bool {
        if other.len() < self.len() {
            return false
        }
        (0..self.len()).all(|i| self.internal[i] & other.internal[i] == self.internal[i])
    }
}

impl KnownLength for Bitset {
    fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
pub mod tests {
    use std::simd::u64x4;
    use crate::bitset::{Bitset, BitsetBuilder};

    #[test]
    fn test_builder() {
        let mut builder: BitsetBuilder<u32> = BitsetBuilder::new();
        assert_eq!(builder.add(vec![1, 2, 3]), Bitset {
            internal: vec![u64x4::from([7,0,0,0])],
            size: 3,
        })
    }
}

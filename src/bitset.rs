use std::collections::HashMap;
use std::hash::Hash;
use std::simd::u64x4;
use crate::corpus::KnownLength;

/// While the implementation is my own, the bitset and SIMD operations
/// owe their existence to my friend Magilan Sendhil \
/// (He attends the University of Illinois Urbana-Champlain).
/// He was of great assistance while brainstorming and debugging.

/// The bitset builder is used to build bitsets. It holds a "lookup table" to allow fast reads and writs of bitsets
pub struct BitsetBuilder<T: Hash+Eq+PartialEq> {
    dict: HashMap<T, usize>
}

impl<T: Hash+Eq+PartialEq+Ord> BitsetBuilder<T> {
    /// adds a varriable length
    pub fn add(&mut self, mut to_add: Vec<T>) -> Bitset {
        let mut set: Bitset = Bitset::new();

        // sort elements prevent multiple elements from being made for transpositions
        to_add.sort();
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


/// A bitset represents a set as a series of bitflags.
/// Every number the builder has seen is set as a bit flag
/// therefore any set can be described as a sequence of 1s and 0s.
/// This allows the subset operation to be described simply as a bitwise AND,
/// Which is very fast on modern CPUs
#[derive(Debug, PartialEq)]
pub struct Bitset{
    /// the u64x4 is using rust's Single Instruction Multiple Data (SIMD) api.
    /// Where SIMD is not available rust will supply a fallback
    /// using SIMD allows for much faster subset operations.
    /// internal is a vec because it allows theoretically infinite bitset lengths
    /// though eventually any system will run out of memory.
    /// A possible optimization would be to use a fixed length array here.
    internal: Vec<u64x4>,

    /// This refers to the number of distinct elements in the set.
    size: usize
}


impl Bitset {
    pub fn new() -> Self {
        Self { internal: Default::default(), size: 0 }
    }

    /// Ensures that there are enough SIMD vectors to slot into, and sets their flags appropriately.
    pub fn mark(&mut self, num: usize) {
        // How many SIMD vectors are needed to fit the given number
        let partition = num / 256;

        // Where in the internal vector should the data be sent to.
        let idx = num % 256;

        // Ensure there are enough SIMD vectors
        while self.internal.len() <= partition {
            self.internal.push(u64x4::default())
        }

        // Use the information from above to set the appropriate bit in the correct SIMD lane
        self.internal[partition].as_mut_array()[idx / 64] |= 1 << (idx % 64);
        self.size += 1;
    }

    /// Method to tell if this bitset is a subset of another
    pub fn is_subset(&self, other: &Bitset) -> bool {
        // Check if either the number of elements in the bitset is wrong, or in the wrong place
        if other.len() < self.len() || other.internal.len() < self.internal.len() {
            return false
        }
        // ensure that all of the bits are the same
        (0..self.internal.len()).all(|i| self.internal[i] & other.internal[i] == self.internal[i])
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

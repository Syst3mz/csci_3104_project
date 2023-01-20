use crate::corpus::KnownLength;

#[derive(Debug)]
pub struct AlmostSet<T> {
    internal_vec: Vec<T>
}

impl<T: PartialEq+Ord> AlmostSet<T> {

    pub fn is_subset(&self, other: &AlmostSet<T>) -> bool {
        if other.len() >= self.internal_vec.len() {
            for i in 0..self.internal_vec.len() {
                if self.internal_vec[i] != other.internal_vec[i] {
                    return false;
                }
            }
            return true;
        }
        return false;
    }


    pub fn new(mut internal_vec: Vec<T>) -> Self {
        internal_vec.sort();
        Self { internal_vec }
    }
}

impl<T> KnownLength for AlmostSet<T> {
    fn len(&self) -> usize {
        self.internal_vec.len()
    }
}
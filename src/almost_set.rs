use crate::corpus::KnownLength;

#[derive(Debug, PartialEq)]
pub struct AlmostSet<T> {
    internal_vec: Vec<T>
}

impl<T: PartialEq+Ord+PartialEq> AlmostSet<T> {
    pub fn is_subset(&self, other: &AlmostSet<T>) -> bool {
        if other.len() >= self.internal_vec.len() {
            let first_idx = get_first_idx(self, other);
            if let None = first_idx {
                return false;
            }

            let mut other_idx = first_idx.unwrap()+1;
            let mut idx = 1_usize;
            while other_idx < other.len() && idx < self.len() {
                if self.internal_vec[idx] != other.internal_vec[other_idx] {
                    return false
                }
                other_idx += 1;
                idx += 1;
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

fn get_first_idx<T: PartialEq + Ord + PartialEq>(a: &AlmostSet<T>, b: &AlmostSet<T>) -> Option<usize> {
    for i in 0..b.len() {
        if a.internal_vec[0] == b.internal_vec[i] {
            return Some(i);
        }
    }
    return None;
}

impl<T: ToString> ToString for AlmostSet<T> {
    fn to_string(&self) -> String {
        let mut s: String = String::new();
        for item in &self.internal_vec {
            s.push_str(&format!("{}, ", item.to_string()))
        }
        s[..s.len() - 2].to_string()
    }
}

impl<T> KnownLength for AlmostSet<T> {
    fn len(&self) -> usize {
        self.internal_vec.len()
    }
}
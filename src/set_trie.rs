use std::borrow::BorrowMut;
use std::cell::{Cell, Ref, RefCell};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;
use std::vec::IntoIter;
use clap::builder::Str;
use crate::bitset::Bitset;

#[derive(Debug)]
struct SetTrie<T: Hash+Eq+PartialEq> {
    alphabet: HashMap<T, usize>,
    root: Node,
}

impl<T: Hash+Eq+PartialEq> SetTrie<T> {
    pub(crate) fn new() -> Self {
        Self { alphabet: HashMap::new(), root: Node {
            label: 0,
            children: Default::default(),
            is_terminal: false,
        } }
    }

    fn get_alpha_iterator(alphabet: &mut HashMap<T, usize>, items: impl IntoIterator<Item=T>) -> AlphaIterator {
        let mut x:Vec<usize> = items.into_iter().map(|x| {
            let next_alpha_idx = alphabet.len() + 1;
            *alphabet
                .entry(x)
                .or_insert(next_alpha_idx)
        }).collect();
        x.sort();
        AlphaIterator::new(x, alphabet.len()+1)
    }


    pub(crate) fn exists_superset(&self, items: impl IntoIterator<Item=T>) -> bool {
        let mut tmp:Vec<usize> = items.into_iter().map(|x| self.alphabet[&x]).collect();
        tmp.sort();
        Self::exists_superset_helper(
            &self.root,AlphaIterator::new(tmp, self.alphabet.len()+1))
    }

    fn exists_superset_helper(v: &Node, mut items: AlphaIterator) -> bool {
        if let Some(current) = items.current() {
            let mut found = false;
            let mut element = current + 1;
            let next_element = if let Some(next_element) = items.peek() {
                next_element
            }
            else {
                items.sentinel
            };

            while element <= next_element && !found {
                if v.children.contains_key(&element) {
                    let u = &v.children[&element];

                    if element == next_element {
                        items.next();
                        found = Self::exists_superset_helper(u, items.clone())
                    }
                    else {
                        found = Self::exists_superset_helper(u, items.clone())
                    }
                }

                element += 1;
            }

            found
        }
        else { true }
    }

    pub(crate) fn insert(&mut self, items: impl IntoIterator<Item=T>) {
        let iter = &mut Self::get_alpha_iterator(&mut self.alphabet, items);
        Self::insert_helper(&mut self.alphabet, &mut self.root, iter)
    }

    fn insert_helper(alphabet: &mut HashMap<T, usize>, cur_node: &mut Node, items: &mut AlphaIterator) {
        if let Some(alpha_idx) = items.current(){
            let mut cur_node = cur_node;
            let next_node = cur_node.children.entry(alpha_idx.clone()).or_insert(Node {
                label: alpha_idx.clone(),
                children: Default::default(),
                is_terminal: false,
            });

            items.next();
            Self::insert_helper(alphabet, next_node, items);
        }
        else {
            cur_node.is_terminal = true;
        }
    }
}

#[derive(Clone)]
struct AlphaIterator {
    internal_vec: Vec<usize>,
    idx: usize,
    pub sentinel: usize
}

impl AlphaIterator {
    pub fn new(internal_vec: Vec<usize>, sentinel: usize) -> Self {
        Self { internal_vec, idx: 0, sentinel }
    }

    pub fn current(&self) -> Option<usize> {
        if self.idx >= self.internal_vec.len() {
            return None
        }

        return Some(self.internal_vec[self.idx])
    }

    fn peek(&self) -> Option<usize> {
        if self.idx + 1 >= self.internal_vec.len() {
            None
        }
        else {
            Some(self.internal_vec[self.idx + 1])
        }
    }

    fn first(&mut self) -> Option<usize> {
        self.idx = 0;
        if self.idx >= self.internal_vec.len() {
            None
        }
        else {
            Some(self.internal_vec[self.idx])
        }
    }
}

impl Iterator for AlphaIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if self.idx >= self.internal_vec.len() {
            None
        }
        else {
            Some(self.internal_vec[self.idx])
        }
    }
}


#[derive(Debug)]
struct Node {
    label: usize,
    children: HashMap<usize, Node>,
    is_terminal: bool
}

#[cfg(test)]
pub mod tests {
    use crate::set_trie::SetTrie;

    #[test]
    fn test_insert_not_crash() {
        let mut t = SetTrie::<usize>::new();
        t.insert(vec![1, 2]);
        t.insert(vec![1, 2, 3]);
        t.insert(vec![1, 2, 4]);
        println!("{:#?}", t);
        assert!(true)
    }

    #[test]
    fn test_exists_superset_simple() {
        let mut t = SetTrie::<usize>::new();
        t.insert(vec![1, 2]);
        t.insert(vec![1, 2, 3]);
        t.insert(vec![1, 2, 4]);
        println!("{:#?}", t);
        assert!(t.exists_superset(vec![1, 2]))
    }

    #[test]
    fn test_insert_longer() {
        let mut t = SetTrie::<usize>::new();
        t.insert(vec![1, 2]);
        t.insert(vec![1, 2, 3]);
        t.insert(vec![1, 2, 3, 4]);
        t.insert(vec![1, 2, 3, 4, 5]);
        t.insert(vec![2]);
        t.insert(vec![2, 3]);
        t.insert(vec![2, 6, 7]);
        t.insert(vec![2, 4, 5]);
        println!("{:#?}", t);
        assert!(true)
    }
}
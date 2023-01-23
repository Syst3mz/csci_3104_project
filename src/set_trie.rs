use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use crate::bitset::Bitset;

#[derive(Debug, PartialEq)]
pub enum TreeNode<'a, T:PartialEq+Debug> {
    Root(Vec<TreeNode<'a, T>>),
    Branch {
        symbol:&'a T,
        children: Vec<TreeNode<'a, T>>,
        end_of_set: bool,
    }
}

impl<'a, T: PartialEq + Debug> TreeNode<'a, T> {
    pub fn new() -> Self {
        Self::Root(vec![])
    }

    /*pub fn add(&'a mut self, mut set: Vec<T>) {
        let item = set.pop();

        if let Some(item) = item {
            match self {
                TreeNode::Root(children) => {
                    if let Some(child) = Self::find_in_children(children, &item) {
                        child.add(set)
                    }
                    else {
                        children.push(TreeNode::Branch {
                            symbol: &item,
                            children: vec![],
                            end_of_set: set.is_empty(),
                        })
                    }
                }
                TreeNode::Branch { .. } => {}
            }
        }
    }*/

    fn find_in_children(children: &'a mut Vec<TreeNode<'a, T>>, element: &T) -> Option<&'a mut TreeNode<'a, T>> {
        for child in children {
            match child {
                TreeNode::Root(_) => {return None}
                TreeNode::Branch { symbol, .. } => {
                    if *symbol == element {
                        return Some(child)
                    }
                }
            }
        }

        return None
    }
}

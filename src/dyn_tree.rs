use std::fmt::{Display};

fn run() {
    let mut tree = Node {
        payload: -1,
        children: vec![],
    };

    tree.add(&mut vec![1, 2].into_iter());
    tree.add(&mut vec![1, 2, 3].into_iter());
    tree.add(&mut vec![1, 2, 3, 4].into_iter());
    tree.add(&mut vec![1, 2, 3, 4, 5].into_iter());
    tree.add(&mut vec![2].into_iter());
    tree.add(&mut vec![2, 3].into_iter());
    println!("{}", tree.as_string())
}


#[derive(Debug)]
struct Node<T: PartialEq + Display> {
    payload: T,
    children: Vec<Node<T>>
}

impl<T: PartialEq + Display> Node<T> {
    fn add(&mut self, p: &mut dyn Iterator<Item=T>) {
        if let Some(next) = p.next() {
            match self.get_matching(&next) {
                None => {
                    let mut child = Node {
                        payload: next,
                        children: vec![],
                    };
                    child.add(p);
                    self.children.push(child);
                }
                Some(child) => {child.add(p)}
            }
        }
    }

    fn get_matching(&mut self, item: &T) -> Option<&mut Node<T>> {
        for child in &mut self.children {
            if &child.payload == item {
                return Some(child);
            }
        }

        None
    }

    fn as_string(&self) -> String {
        let mut s = format!("{}", self.payload);
        if self.children.len() == 0 {
            return s;
        }

        for child in &self.children {
            s.push_str(&format!("{}", indent(child.as_string(), String::from("----"))))
        }
        s
    }
}

fn indent(what: String, with: String) -> String {
    let mut q = String::from("\n");
    q.push_str(&with);
    q.push_str(&what.replace("\n", &q));
    q
}
/*
impl<T> From<&mut dyn Iterator<Item = dyn Iterator<Item = T>>> for TreeNode<T> {
    fn from(value: &mut dyn Iterator<Item=T>) -> Self {
        let root: Vec<TreeNode<T>> = vec![];
        while let Some(row) = value.next() {
            while let Some(col) {

            }
        }

        if root.len() > 0 {
            Node(root)
        }
        return Empty;
    }
}
 */
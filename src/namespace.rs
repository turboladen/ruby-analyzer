use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Namespace {
    inner: Vec<Node>,
}

impl Namespace {
    pub fn new(name_nodes: Vec<Node>) -> Self {
        Self { inner: name_nodes }
    }

    pub fn join(&self, scope_name_node: Node) -> Self {
        let mut new = self.inner.clone();
        new.push(scope_name_node);

        Self { inner: new }
    }

    // pub const fn each_branch(&self) -> EachBranch<'_> {
    //     EachBranch::new(self)
    // }

    pub(crate) fn push_owned(&mut self, scope_node_name: Node) {
        self.inner.push(scope_node_name)
    }

    pub fn inner(&self) -> &[Node] {
        &self.inner
    }

    pub fn leaf(&self) -> &Node {
        self.inner.last().unwrap()
    }
}

impl Deref for Namespace {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Namespace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// pub struct EachBranch<'a> {
//     original: &'a Namespace,
//     current_index: usize,
// }

// impl<'a> EachBranch<'a> {
//     const fn new(original: &'a Namespace) -> Self {
//         Self {
//             original,
//             current_index: 0,
//         }
//     }
// }

// impl<'a> Iterator for EachBranch<'a> {
//     type Item = Namespace;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current_index <= self.original.len() {
//             let branch = Namespace::new(self.original[0..self.current_index].to_vec());
//             self.current_index += 1;

//             if branch.is_empty() {
//                 self.next()
//             } else {
//                 Some(branch)
//             }
//         } else {
//             None
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// TODO: These could hold Cow<str>s, but reverting to Sting for borrowing-easy for now.
pub enum Node {
    Class { name: String },
    Module { name: String },
}

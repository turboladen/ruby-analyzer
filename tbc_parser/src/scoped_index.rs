use std::collections::BTreeMap;

use crate::ScopeGate;

// pub type ScopedIndex = BTreeMap<ScopeGateNode, ScopeItems>;

// #[derive(Debug, Default)]
// pub struct ScopeItems {
//     child_scopes: ScopedIndex,
//     nodes: Vec<Node>,
// }
#[derive(Debug, Default)]
pub struct ScopedIndex {
    inner: BTreeMap<ScopeGate, Vec<Node>>,
}

impl ScopedIndex {
    pub fn inner_mut(&mut self) -> &mut BTreeMap<ScopeGate, Vec<Node>> {
        &mut self.inner
    }

    pub fn inner(&self) -> &BTreeMap<ScopeGate, Vec<Node>> {
        &self.inner
    }
}

#[derive(Debug)]
pub enum Node {
    Class {
        id: usize,
        name: String,
        superclass_id: Option<usize>,
        body_id: Option<usize>,
    },
}

impl Node {
    pub fn id(&self) -> usize {
        match self {
            Node::Class { id, .. } => *id,
        }
    }
}

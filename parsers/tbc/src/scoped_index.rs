mod node_properties;
pub mod nodes;

use std::{collections::BTreeMap, ops::Deref};

use crate::ScopeGate;

pub use self::node_properties::NodeProperties;

// pub type ScopedIndex = BTreeMap<ScopeGateNode, ScopeItems>;

// #[derive(Debug, Default)]
// pub struct ScopeItems {
//     child_scopes: ScopedIndex,
//     nodes: Vec<Node>,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedIndex {
    inner: BTreeMap<ScopeGate, Vec<Node>>,
}

impl ScopedIndex {
    pub(crate) fn inner_mut(&mut self) -> &mut BTreeMap<ScopeGate, Vec<Node>> {
        &mut self.inner
    }
}

impl Default for ScopedIndex {
    fn default() -> Self {
        Self {
            inner: {
                let mut map = BTreeMap::default();
                map.insert(ScopeGate::default(), vec![]);
                map
            },
        }
    }
}

impl Deref for ScopedIndex {
    type Target = BTreeMap<ScopeGate, Vec<Node>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub(crate) id: usize,
    pub(crate) properties: NodeProperties,
}

impl Node {
    pub fn id(&self) -> usize {
        self.id
    }
}

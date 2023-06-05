use std::ops::{Deref, DerefMut};

/// Represents a scope gate: A class def, a module def, a method def or a class method def.
/// Ex. in "class Foo; module Bar; end; end" we have 2 scope gates: `Foo` and `Foo::Bar`, which
/// would be represented here a `vec![Node::Class("Foo".to_string())]` and
/// `vec![Node::Class("Foo".to_string()), Node::Module("Bar".to_string())]`, respectively.
///
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopeGate {
    inner: Vec<Node>,
}

impl ScopeGate {
    pub fn new(name_nodes: Vec<Node>) -> Self {
        Self { inner: name_nodes }
    }

    /// Creates a new `ScopeGate` by combining `self` with `scope_name_node`.
    ///
    /// ```
    /// use ruby_analyzer_indextree_parser::scope_gate::{ScopeGate, Node};
    ///
    /// let scope_gate = ScopeGate::new(vec![Node::Class("Foo".to_string())]);
    /// let sg2 = scope_gate.join(Node::Def("bar".to_string()));
    ///
    /// assert_eq!(
    ///     ScopeGate::new(vec![Node::Class("Foo".to_string()), Node::Def("bar".to_string())]),
    ///     sg2
    /// );
    /// ```
    ///
    pub fn join(&self, scope_name_node: Node) -> Self {
        let mut new = self.inner.clone();
        new.push(scope_name_node);

        Self { inner: new }
    }

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

impl Deref for ScopeGate {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ScopeGate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// TODO: These could hold Cow<str>s, but reverting to Sting for borrowing-easy for now.
pub enum Node {
    Class(String),
    Module(String),
    Def(String),
    Defs(String),
}

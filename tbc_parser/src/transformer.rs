use lib_ruby_parser::{nodes as lrp_nodes, traverse::visitor::Visitor};

use crate::{
    location::{LocNode, Node},
    lrp_extensions::NameFromNode,
    scoped_index::{self, ScopedIndex},
    ScopeGate, ScopeGateNode,
};

#[derive(Default)]
pub(crate) struct Transformer {
    current_id: usize,
    current_scope_gate: ScopeGate,

    locs: Vec<LocNode>,
    scoped_index: ScopedIndex,
}

impl Transformer {
    fn new_id(&mut self) -> usize {
        self.current_id += 1;
        self.current_id
    }

    fn visit_optional_single_node_child(
        &mut self,
        node: &Option<Box<lib_ruby_parser::Node>>,
    ) -> Option<usize> {
        node.as_ref().map(|n| self.visit_single_node_child(n))
    }

    fn visit_single_node_child(&mut self, node: &lib_ruby_parser::Node) -> usize {
        self.visit_node_child(|transformer| {
            transformer.visit(node);
        })
    }

    // Uses `func` to drive the visitor (that function should call a visit fucntion), then returns
    // the ID of the node that was visited.
    //
    fn visit_node_child<F>(&mut self, func: F) -> usize
    where
        F: FnOnce(&mut Transformer),
    {
        func(self);

        self.scoped_index
            .inner()
            .get(&self.current_scope_gate)
            .and_then(|nodes| nodes.last().map(|node| node.id()))
            .unwrap()
    }
}

impl Visitor for Transformer {
    fn on_class(&mut self, node: &lrp_nodes::Class) {
        self.locs.push(LocNode {
            expression_l: node.expression_l.into(),
            node: Node::Class,
            scope_gate: self.current_scope_gate.clone(),
        });

        // Not sure it matters in practice, but let's just keep the class's ID a lower number than
        // its children that were about to visit.
        let id = self.new_id();

        // Do these two before updating the current_scope_gate, since they shouldn't be treated as
        // scope-children of this class.
        let name_id = self.visit_single_node_child(&node.name);
        let superclass_id = self.visit_optional_single_node_child(&node.superclass);

        self.current_scope_gate
            .push_owned(ScopeGateNode::Class(node.name_from_node()));

        let body_id = self.visit_optional_single_node_child(&node.body);

        self.current_scope_gate.pop();

        // Do this after visiting the class's children, so we can add the child IDs to the class
        // info.
        self.scoped_index
            .inner_mut()
            .entry(self.current_scope_gate.clone())
            .and_modify(|nodes| {
                nodes.push(scoped_index::Node::Class {
                    id,
                    name: node.name_from_node(),
                    superclass_id,
                    body_id,
                });
            })
            .or_insert(vec![scoped_index::Node::Class {
                id,
                name: node.name_from_node(),
                superclass_id,
                body_id,
            }]);
    }

    fn on_def(&mut self, node: &lrp_nodes::Def) {
        self.locs.push(LocNode {
            expression_l: node.expression_l.into(),
            node: Node::Def,
            scope_gate: self.current_scope_gate.clone(),
        });

        self.current_scope_gate
            .push_owned(ScopeGateNode::Def(node.name.clone()));

        if let Some(args) = &node.args {
            self.visit(args);
        }

        if let Some(body) = &node.body {
            self.visit(body);
        }

        self.current_scope_gate.pop();
    }

    fn on_defs(&mut self, node: &lrp_nodes::Defs) {
        self.locs.push(LocNode {
            expression_l: node.expression_l.into(),
            node: Node::Defs,
            scope_gate: self.current_scope_gate.clone(),
        });

        self.current_scope_gate
            .push_owned(ScopeGateNode::Defs(node.name.clone()));

        self.visit(&node.definee);

        if let Some(args) = &node.args {
            self.visit(args);
        }

        if let Some(body) = &node.body {
            self.visit(body);
        }

        self.current_scope_gate.pop();
    }

    fn on_module(&mut self, node: &lrp_nodes::Module) {
        self.locs.push(LocNode {
            expression_l: node.expression_l.into(),
            node: Node::Module,
            scope_gate: self.current_scope_gate.clone(),
        });

        self.current_scope_gate
            .push_owned(ScopeGateNode::Module(node.name_from_node()));

        self.visit(&node.name);

        if let Some(body) = &node.body {
            self.visit(body);
        }

        self.current_scope_gate.pop();
    }
}

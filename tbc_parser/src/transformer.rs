use std::collections::btree_map::Entry;

use lib_ruby_parser::{nodes as lrp_nodes, traverse::visitor::Visitor};
use tracing::trace;

use crate::{
    location::{Loc, LocNode, NodeType},
    lrp_extensions::{NameFromNode, OptionNameFromNode},
    scoped_index::{nodes::*, Node, NodeProperties, ScopedIndex},
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
    pub(crate) fn finish(self) -> (Vec<LocNode>, ScopedIndex) {
        (self.locs, self.scoped_index)
    }

    fn new_id(&mut self) -> usize {
        self.current_id += 1;
        self.current_id
    }

    fn do_in_scope<F, T>(&mut self, scope_gate_node: ScopeGateNode, func: F) -> T
    where
        F: Fn(&mut Self) -> T,
    {
        self.current_scope_gate.push_owned(scope_gate_node);

        let result = func(self);

        self.current_scope_gate.pop();

        result
    }

    fn insert_scope_node(&mut self, node: Node) {
        match self
            .scoped_index
            .inner_mut()
            .entry(self.current_scope_gate.clone())
        {
            Entry::Vacant(e) => {
                e.insert(vec![node]);
            }
            Entry::Occupied(mut e) => e.get_mut().push(node),
        };
    }

    fn visit_optional_child(&mut self, node: &Option<Box<lib_ruby_parser::Node>>) -> Option<usize> {
        node.as_ref().map(|n| self.visit_child(n))
    }

    fn visit_child(&mut self, node: &lib_ruby_parser::Node) -> usize {
        self.visit_node_child(|transformer| {
            transformer.visit(node);
        })
    }

    fn visit_children(&mut self, nodes: &[lib_ruby_parser::Node]) -> Vec<usize> {
        nodes
            .iter()
            .map(|node| {
                self.visit_node_child(|transformer| {
                    transformer.visit(node);
                })
            })
            .collect()
    }

    // Uses `func` to drive the visitor (that function should call a visit function), then returns
    // the ID of the node that was visited.
    //
    fn visit_node_child<F>(&mut self, func: F) -> usize
    where
        F: FnOnce(&mut Transformer),
    {
        func(self);

        self.scoped_index
            .get(&self.current_scope_gate)
            .and_then(|nodes| nodes.last().map(|node| node.id()))
            .unwrap()
    }

    fn make_empty_body(&mut self, begin: usize, end: usize) {
        self.locs.push(LocNode {
            node: NodeType::EmptyBody,
            name: String::new(),
            expression_l: Loc { begin, end },
            scope_gate: self.current_scope_gate.clone(),
        });

        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::EmptyBody,
        });
    }
}

impl Visitor for Transformer {
    fn on_alias(&mut self, node: &lrp_nodes::Alias) {
        let id = self.new_id();
        let to_id = self.visit_child(&node.to);
        let from_id = self.visit_child(&node.from);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Alias(Alias { to_id, from_id }),
        });
    }

    fn on_and(&mut self, node: &lrp_nodes::And) {
        let id = self.new_id();
        let lhs_id = self.visit_child(&node.lhs);
        let rhs_id = self.visit_child(&node.rhs);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::And(And { lhs_id, rhs_id }),
        });
    }

    fn on_and_asgn(&mut self, node: &lrp_nodes::AndAsgn) {
        let id = self.new_id();
        let recv_id = self.visit_child(&node.recv);
        let value_id = self.visit_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::AndAsgn(AndAsgn { recv_id, value_id }),
        });
    }

    fn on_arg(&mut self, node: &lrp_nodes::Arg) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Arg(Arg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_args(&mut self, node: &lrp_nodes::Args) {
        let id = self.new_id();
        let arg_ids = self.visit_children(&node.args);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Args(Args { arg_ids }),
        });
    }

    fn on_array(&mut self, node: &lrp_nodes::Array) {
        let id = self.new_id();
        let element_ids = self.visit_children(&node.elements);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Array(Array { element_ids }),
        });
    }

    fn on_array_pattern(&mut self, node: &lrp_nodes::ArrayPattern) {
        let id = self.new_id();
        let element_ids = self.visit_children(&node.elements);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::ArrayPattern(ArrayPattern { element_ids }),
        });
    }

    fn on_array_pattern_with_tail(&mut self, node: &lrp_nodes::ArrayPatternWithTail) {
        let id = self.new_id();
        let element_ids = self.visit_children(&node.elements);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::ArrayPatternWithTail(ArrayPatternWithTail { element_ids }),
        });
    }

    fn on_back_ref(&mut self, node: &lrp_nodes::BackRef) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::BackRef(BackRef {
                name: node.name.clone(),
            }),
        });
    }

    fn on_begin(&mut self, node: &lrp_nodes::Begin) {
        let id = self.new_id();
        let statement_ids = self.visit_children(&node.statements);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Begin(Begin { statement_ids }),
        });
    }

    fn on_block(&mut self, node: &lrp_nodes::Block) {
        let id = self.new_id();
        let call_id = self.visit_child(&node.call);
        let args_id = self.visit_optional_child(&node.args);
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Block(Block {
                call_id,
                args_id,
                body_id,
            }),
        });
    }

    fn on_blockarg(&mut self, node: &lrp_nodes::Blockarg) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Blockarg(Blockarg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_block_pass(&mut self, node: &lrp_nodes::BlockPass) {
        let id = self.new_id();
        let value_id = self.visit_optional_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::BlockPass(BlockPass { value_id }),
        });
    }

    fn on_break(&mut self, node: &lrp_nodes::Break) {
        let id = self.new_id();
        let arg_ids = self.visit_children(&node.args);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Break(Break { arg_ids }),
        });
    }

    fn on_case(&mut self, node: &lrp_nodes::Case) {
        let id = self.new_id();
        let expr_id = self.visit_optional_child(&node.expr);
        let when_body_ids = self.visit_children(&node.when_bodies);
        let else_body_id = self.visit_optional_child(&node.else_body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Case(Case {
                expr_id,
                when_body_ids,
                else_body_id,
            }),
        });
    }

    fn on_case_match(&mut self, node: &lrp_nodes::CaseMatch) {
        let id = self.new_id();
        let expr_id = self.visit_child(&node.expr);
        let in_body_ids = self.visit_children(&node.in_bodies);
        let else_body_id = self.visit_optional_child(&node.else_body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::CaseMatch(CaseMatch {
                expr_id,
                in_body_ids,
                else_body_id,
            }),
        });
    }

    fn on_casgn(&mut self, node: &lrp_nodes::Casgn) {
        let id = self.new_id();
        let scope_id = self.visit_optional_child(&node.scope);
        let value_id = self.visit_optional_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Casgn(Casgn {
                scope_id,
                name: node.name.clone(),
                value_id,
            }),
        });
    }

    fn on_cbase(&mut self, _node: &lrp_nodes::Cbase) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Cbase,
        });
    }

    fn on_class(&mut self, node: &lrp_nodes::Class) {
        let name = node.name_from_node();

        self.locs.push(LocNode {
            node: NodeType::Class,
            name: name.clone(),
            expression_l: node.expression_l.into(),
            scope_gate: self.current_scope_gate.clone(),
        });

        // Not sure it matters in practice, but let's just keep the class's ID a lower number than
        // its children that were about to visit.
        let id = self.new_id();

        // Do these two before updating the current_scope_gate, since they shouldn't be treated as
        // scope-children of this class.
        let name_id = self.visit_child(&node.name);
        let superclass_id = self.visit_optional_child(&node.superclass);

        let body_id = self.do_in_scope(ScopeGateNode::Class(name.clone()), |me| {
            let result = me.visit_optional_child(&node.body);

            if result.is_none() {
                me.make_empty_body(node.keyword_l.end + 1, node.end_l.begin - 1);
            }

            result
        });

        // Do this after visiting the class's children, so we can add the child IDs to the class
        // info.
        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Class(Class {
                name: name.clone(),
                name_id,
                superclass_id,
                body_id,
            }),
        });
    }

    fn on_complex(&mut self, node: &lrp_nodes::Complex) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Complex(Complex {
                value: node.value.clone(),
            }),
        });
    }

    fn on_const(&mut self, node: &lrp_nodes::Const) {
        let id = self.new_id();
        let scope_id = self.visit_optional_child(&node.scope);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Const(Const {
                name: node.name.clone(),
                scope_id,
            }),
        });
    }

    fn on_const_pattern(&mut self, node: &lrp_nodes::ConstPattern) {
        let id = self.new_id();
        let const_id = self.visit_child(&node.const_);
        let pattern_id = self.visit_child(&node.pattern);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::ConstPattern(ConstPattern {
                const_id,
                pattern_id,
            }),
        });
    }

    fn on_c_send(&mut self, node: &lrp_nodes::CSend) {
        let id = self.new_id();
        let arg_ids = self.visit_children(&node.args);
        let recv_id = self.visit_child(&node.recv);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::CSend(CSend {
                method_name: node.method_name.clone(),
                arg_ids,
                recv_id,
            }),
        });
    }

    fn on_cvar(&mut self, node: &lrp_nodes::Cvar) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Cvar(Cvar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_cvasgn(&mut self, node: &lrp_nodes::Cvasgn) {
        let id = self.new_id();
        let value_id = self.visit_optional_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Cvasgn(Cvasgn {
                name: node.name.clone(),
                value_id,
            }),
        });
    }

    fn on_def(&mut self, node: &lrp_nodes::Def) {
        self.locs.push(LocNode {
            node: NodeType::Def,
            name: node.name.clone(),
            expression_l: node.expression_l.into(),
            scope_gate: self.current_scope_gate.clone(),
        });

        // Not sure it matters in practice, but let's just keep the class's ID a lower number than
        // its children that were about to visit.
        let id = self.new_id();

        let args_id = self.visit_optional_child(&node.args);

        let body_id = self.do_in_scope(ScopeGateNode::Def(node.name.clone()), |me| {
            let result = me.visit_optional_child(&node.body);

            if result.is_none() {
                let begin = node.name_l.end + 1;
                let end = match node.end_l {
                    Some(end_l) => end_l.begin - 1,
                    None => node.expression_l.end - 1,
                };

                me.make_empty_body(begin, end);
            }

            result
        });

        // Do this after visiting the def's children, so we can add the child IDs to the class
        // info.
        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Def(Def {
                name: node.name.clone(),
                args_id,
                body_id,
            }),
        });
    }

    fn on_defined(&mut self, node: &lrp_nodes::Defined) {
        let id = self.new_id();
        let value_id = self.visit_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Defined(Defined { value_id }),
        });
    }

    fn on_defs(&mut self, node: &lrp_nodes::Defs) {
        self.locs.push(LocNode {
            node: NodeType::Defs,
            name: node.name.clone(),
            expression_l: node.expression_l.into(),
            scope_gate: self.current_scope_gate.clone(),
        });

        // Not sure it matters in practice, but let's just keep the class's ID a lower number than
        // its children that were about to visit.
        let id = self.new_id();

        let definee_id = self.visit_child(&node.definee);
        let args_id = self.visit_optional_child(&node.args);

        let body_id = self.do_in_scope(ScopeGateNode::Defs(node.name.clone()), |me| {
            let result = me.visit_optional_child(&node.body);

            if result.is_none() {
                let begin = node.name_l.end + 1;
                let end = match node.end_l {
                    Some(end_l) => end_l.begin - 1,
                    None => node.expression_l.end - 1,
                };

                me.make_empty_body(begin, end);
            }

            result
        });

        // Do this after visiting the def's children, so we can add the child IDs to the class
        // info.
        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Defs(Defs {
                definee_id,
                name: node.name.clone(),
                args_id,
                body_id,
            }),
        });
    }

    fn on_dstr(&mut self, node: &lrp_nodes::Dstr) {
        let id = self.new_id();
        let part_ids = self.visit_children(&node.parts);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Dstr(Dstr { part_ids }),
        });
    }

    fn on_dsym(&mut self, node: &lrp_nodes::Dsym) {
        let id = self.new_id();
        let part_ids = self.visit_children(&node.parts);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Dsym(Dsym { part_ids }),
        });
    }

    fn on_e_flip_flop(&mut self, node: &lrp_nodes::EFlipFlop) {
        let id = self.new_id();
        let left_id = self.visit_optional_child(&node.left);
        let right_id = self.visit_optional_child(&node.right);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::EFlipFlop(EFlipFlop { left_id, right_id }),
        });
    }

    fn on_empty_else(&mut self, _node: &lrp_nodes::EmptyElse) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::EmptyElse,
        });
    }

    fn on_encoding(&mut self, _node: &lrp_nodes::Encoding) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Encoding,
        });
    }

    fn on_ensure(&mut self, node: &lrp_nodes::Ensure) {
        let id = self.new_id();
        let body_id = self.visit_optional_child(&node.body);
        let ensure_id = self.visit_optional_child(&node.ensure);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Ensure(Ensure { body_id, ensure_id }),
        });
    }

    fn on_erange(&mut self, node: &lrp_nodes::Erange) {
        let id = self.new_id();
        let left_id = self.visit_optional_child(&node.left);
        let right_id = self.visit_optional_child(&node.right);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Erange(Erange { left_id, right_id }),
        });
    }

    fn on_false(&mut self, _node: &lrp_nodes::False) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::False,
        });
    }

    fn on_file(&mut self, _node: &lrp_nodes::File) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::File,
        });
    }

    fn on_find_pattern(&mut self, node: &lrp_nodes::FindPattern) {
        let id = self.new_id();
        let element_ids = self.visit_children(&node.elements);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::FindPattern(FindPattern { element_ids }),
        });
    }

    fn on_float(&mut self, node: &lrp_nodes::Float) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Float(Float {
                value: node.value.clone(),
            }),
        });
    }

    fn on_for(&mut self, node: &lrp_nodes::For) {
        let id = self.new_id();
        let iterator_id = self.visit_child(&node.iterator);
        let iteratee_id = self.visit_child(&node.iteratee);
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::For(For {
                iterator_id,
                iteratee_id,
                body_id,
            }),
        });
    }

    fn on_forward_arg(&mut self, _node: &lrp_nodes::ForwardArg) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::ForwardArg,
        });
    }

    fn on_forwarded_args(&mut self, _node: &lrp_nodes::ForwardedArgs) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::ForwardedArgs,
        });
    }

    fn on_gvar(&mut self, node: &lrp_nodes::Gvar) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Gvar(Gvar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_gvasgn(&mut self, node: &lrp_nodes::Gvasgn) {
        let id = self.new_id();
        let value_id = self.visit_optional_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Gvasgn(Gvasgn {
                name: node.name.clone(),
                value_id,
            }),
        });
    }

    fn on_hash(&mut self, node: &lrp_nodes::Hash) {
        let id = self.new_id();
        let pair_ids = self.visit_children(&node.pairs);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Hash(Hash { pair_ids }),
        });
    }

    fn on_hash_pattern(&mut self, node: &lrp_nodes::HashPattern) {
        let id = self.new_id();
        let element_ids = self.visit_children(&node.elements);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::HashPattern(HashPattern { element_ids }),
        });
    }

    fn on_heredoc(&mut self, node: &lrp_nodes::Heredoc) {
        let id = self.new_id();
        let part_ids = self.visit_children(&node.parts);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Heredoc(Heredoc { part_ids }),
        });
    }

    fn on_if(&mut self, node: &lrp_nodes::If) {
        let id = self.new_id();
        let cond_id = self.visit_child(&node.cond);
        let if_true_id = self.visit_optional_child(&node.if_true);
        let if_false_id = self.visit_optional_child(&node.if_false);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::If(If {
                cond_id,
                if_true_id,
                if_false_id,
            }),
        });
    }

    fn on_if_guard(&mut self, node: &lrp_nodes::IfGuard) {
        let id = self.new_id();
        let cond_id = self.visit_child(&node.cond);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::IfGuard(IfGuard { cond_id }),
        });
    }

    fn on_i_flip_flop(&mut self, node: &lrp_nodes::IFlipFlop) {
        let id = self.new_id();
        let left_id = self.visit_optional_child(&node.left);
        let right_id = self.visit_optional_child(&node.right);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::IFlipFlop(IFlipFlop { left_id, right_id }),
        });
    }

    fn on_if_mod(&mut self, node: &lrp_nodes::IfMod) {
        let id = self.new_id();
        let cond_id = self.visit_child(&node.cond);
        let if_true_id = self.visit_optional_child(&node.if_true);
        let if_false_id = self.visit_optional_child(&node.if_false);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::IfMod(IfMod {
                cond_id,
                if_true_id,
                if_false_id,
            }),
        });
    }

    fn on_if_ternary(&mut self, node: &lrp_nodes::IfTernary) {
        let id = self.new_id();
        let cond_id = self.visit_child(&node.cond);
        let if_true_id = self.visit_child(&node.if_true);
        let if_false_id = self.visit_child(&node.if_false);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::IfTernary(IfTernary {
                cond_id,
                if_true_id,
                if_false_id,
            }),
        });
    }

    fn on_index(&mut self, node: &lrp_nodes::Index) {
        let id = self.new_id();
        let recv_id = self.visit_child(&node.recv);
        let index_ids = self.visit_children(&node.indexes);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Index(Index { recv_id, index_ids }),
        });
    }

    fn on_index_asgn(&mut self, node: &lrp_nodes::IndexAsgn) {
        let id = self.new_id();
        let recv_id = self.visit_child(&node.recv);
        let index_ids = self.visit_children(&node.indexes);
        let value_id = self.visit_optional_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::IndexAsgn(IndexAsgn {
                recv_id,
                index_ids,
                value_id,
            }),
        });
    }

    fn on_in_pattern(&mut self, node: &lrp_nodes::InPattern) {
        let id = self.new_id();
        let pattern_id = self.visit_child(&node.pattern);
        let guard_id = self.visit_optional_child(&node.guard);
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::InPattern(InPattern {
                pattern_id,
                guard_id,
                body_id,
            }),
        });
    }

    fn on_int(&mut self, node: &lrp_nodes::Int) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Int(Int {
                value: node.value.clone(),
            }),
        });
    }

    fn on_irange(&mut self, node: &lrp_nodes::Irange) {
        let id = self.new_id();
        let left_id = self.visit_optional_child(&node.left);
        let right_id = self.visit_optional_child(&node.right);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Irange(Irange { left_id, right_id }),
        });
    }

    fn on_ivar(&mut self, node: &lrp_nodes::Ivar) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Ivar(Ivar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_ivasgn(&mut self, node: &lrp_nodes::Ivasgn) {
        let id = self.new_id();
        let value_id = self.visit_optional_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Ivasgn(Ivasgn {
                name: node.name.clone(),
                value_id,
            }),
        });
    }

    fn on_kwarg(&mut self, node: &lrp_nodes::Kwarg) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Kwarg(Kwarg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_kwargs(&mut self, node: &lrp_nodes::Kwargs) {
        let id = self.new_id();
        let pair_ids = self.visit_children(&node.pairs);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Kwargs(Kwargs { pair_ids }),
        });
    }

    fn on_kw_begin(&mut self, node: &lrp_nodes::KwBegin) {
        let id = self.new_id();
        let statement_ids = self.visit_children(&node.statements);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::KwBegin(KwBegin { statement_ids }),
        });
    }

    fn on_kwnilarg(&mut self, _node: &lrp_nodes::Kwnilarg) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Kwnilarg,
        });
    }

    fn on_kwoptarg(&mut self, node: &lrp_nodes::Kwoptarg) {
        let id = self.new_id();
        let default_id = self.visit_child(&node.default);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Kwoptarg(Kwoptarg {
                name: node.name.clone(),
                default_id,
            }),
        });
    }

    fn on_kwrestarg(&mut self, node: &lrp_nodes::Kwrestarg) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Kwrestarg(Kwrestarg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_kwsplat(&mut self, node: &lrp_nodes::Kwsplat) {
        let id = self.new_id();
        let value_id = self.visit_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Kwsplat(Kwsplat { value_id }),
        });
    }

    fn on_lambda(&mut self, _node: &lrp_nodes::Lambda) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Lambda,
        });
    }

    fn on_line(&mut self, _node: &lrp_nodes::Line) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Line,
        });
    }

    fn on_lvar(&mut self, node: &lrp_nodes::Lvar) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Lvar(Lvar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_lvasgn(&mut self, node: &lrp_nodes::Lvasgn) {
        let id = self.new_id();
        let value_id = self.visit_optional_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Lvasgn(Lvasgn {
                name: node.name.clone(),
                value_id,
            }),
        });
    }

    fn on_masgn(&mut self, node: &lrp_nodes::Masgn) {
        let id = self.new_id();
        let lhs_id = self.visit_child(&node.lhs);
        let rhs_id = self.visit_child(&node.rhs);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Masgn(Masgn { lhs_id, rhs_id }),
        });
    }

    fn on_match_alt(&mut self, node: &lrp_nodes::MatchAlt) {
        let id = self.new_id();
        let lhs_id = self.visit_child(&node.lhs);
        let rhs_id = self.visit_child(&node.rhs);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::MatchAlt(MatchAlt { lhs_id, rhs_id }),
        });
    }

    fn on_match_as(&mut self, node: &lrp_nodes::MatchAs) {
        let id = self.new_id();
        let value_id = self.visit_child(&node.value);
        let as_id = self.visit_child(&node.as_);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::MatchAs(MatchAs { value_id, as_id }),
        });
    }

    fn on_match_current_line(&mut self, node: &lrp_nodes::MatchCurrentLine) {
        let id = self.new_id();
        let re_id = self.visit_child(&node.re);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::MatchCurrentLine(MatchCurrentLine { re_id }),
        });
    }

    fn on_match_nil_pattern(&mut self, _node: &lrp_nodes::MatchNilPattern) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::MatchNilPattern,
        });
    }

    fn on_match_pattern(&mut self, node: &lrp_nodes::MatchPattern) {
        let id = self.new_id();
        let value_id = self.visit_child(&node.value);
        let pattern_id = self.visit_child(&node.pattern);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::MatchPattern(MatchPattern {
                value_id,
                pattern_id,
            }),
        });
    }

    fn on_match_pattern_p(&mut self, node: &lrp_nodes::MatchPatternP) {
        let id = self.new_id();
        let value_id = self.visit_child(&node.value);
        let pattern_id = self.visit_child(&node.pattern);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::MatchPatternP(MatchPatternP {
                value_id,
                pattern_id,
            }),
        });
    }

    fn on_match_rest(&mut self, node: &lrp_nodes::MatchRest) {
        let id = self.new_id();
        let name_id = self.visit_optional_child(&node.name);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::MatchRest(MatchRest {
                name: node.option_name_from_node(),
                name_id,
            }),
        });
    }

    fn on_match_var(&mut self, node: &lrp_nodes::MatchVar) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::MatchVar(MatchVar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_match_with_lvasgn(&mut self, node: &lrp_nodes::MatchWithLvasgn) {
        let id = self.new_id();
        let re_id = self.visit_child(&node.re);
        let value_id = self.visit_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::MatchWithLvasgn(MatchWithLvasgn { re_id, value_id }),
        });
    }

    fn on_mlhs(&mut self, node: &lrp_nodes::Mlhs) {
        let id = self.new_id();
        let item_ids = self.visit_children(&node.items);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Mlhs(Mlhs { item_ids }),
        });
    }

    fn on_module(&mut self, node: &lrp_nodes::Module) {
        trace!("Adding LocNode for module {}", node.name_from_node());
        let name = node.name_from_node();

        self.locs.push(LocNode {
            node: NodeType::Module,
            name: name.clone(),
            expression_l: node.expression_l.into(),
            scope_gate: self.current_scope_gate.clone(),
        });
        trace!("self.locs is now {:#?}", &self.locs);

        // Not sure it matters in practice, but let's just keep the class's ID a lower number than
        // its children that were about to visit.
        let id = self.new_id();

        let name_id = self.visit_child(&node.name);

        let body_id = self.do_in_scope(ScopeGateNode::Module(name.clone()), |me| {
            let result = me.visit_optional_child(&node.body);

            if result.is_none() {
                me.make_empty_body(node.keyword_l.end + 1, node.end_l.begin - 1);
            }

            result
        });

        // Do this after visiting the def's children, so we can add the child IDs to the class
        // info.
        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Module(Module {
                name,
                name_id,
                body_id,
            }),
        });
    }

    fn on_next(&mut self, node: &lrp_nodes::Next) {
        let id = self.new_id();
        let arg_ids = self.visit_children(&node.args);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Next(Next { arg_ids }),
        });
    }

    fn on_nil(&mut self, _node: &lrp_nodes::Nil) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Nil,
        });
    }

    fn on_nth_ref(&mut self, node: &lrp_nodes::NthRef) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::NthRef(NthRef {
                name: node.name.clone(),
            }),
        });
    }

    fn on_numblock(&mut self, node: &lrp_nodes::Numblock) {
        let id = self.new_id();
        let call_id = self.visit_child(&node.call);
        let body_id = self.visit_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Numblock(Numblock {
                call_id,
                numargs: node.numargs,
                body_id,
            }),
        });
    }

    fn on_op_asgn(&mut self, node: &lrp_nodes::OpAsgn) {
        let id = self.new_id();
        let recv_id = self.visit_child(&node.recv);
        let value_id = self.visit_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::OpAsgn(OpAsgn {
                recv_id,
                operator: node.operator.clone(),
                value_id,
            }),
        });
    }

    fn on_optarg(&mut self, node: &lrp_nodes::Optarg) {
        let id = self.new_id();
        let default_id = self.visit_child(&node.default);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Optarg(Optarg {
                default_id,
                name: node.name.clone(),
            }),
        });
    }

    fn on_or(&mut self, node: &lrp_nodes::Or) {
        let id = self.new_id();
        let lhs_id = self.visit_child(&node.lhs);
        let rhs_id = self.visit_child(&node.rhs);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Or(Or { lhs_id, rhs_id }),
        });
    }

    fn on_or_asgn(&mut self, node: &lrp_nodes::OrAsgn) {
        let id = self.new_id();
        let recv_id = self.visit_child(&node.recv);
        let value_id = self.visit_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::OrAsgn(OrAsgn { recv_id, value_id }),
        });
    }

    fn on_pair(&mut self, node: &lrp_nodes::Pair) {
        let id = self.new_id();
        let key_id = self.visit_child(&node.key);
        let value_id = self.visit_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Pair(Pair { key_id, value_id }),
        });
    }

    fn on_pin(&mut self, node: &lrp_nodes::Pin) {
        let id = self.new_id();
        let var_id = self.visit_child(&node.var);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Pin(Pin { var_id }),
        });
    }

    fn on_postexe(&mut self, node: &lrp_nodes::Postexe) {
        let id = self.new_id();
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Postexe(Postexe { body_id }),
        });
    }

    fn on_preexe(&mut self, node: &lrp_nodes::Preexe) {
        let id = self.new_id();
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Preexe(Preexe { body_id }),
        });
    }

    fn on_procarg0(&mut self, node: &lrp_nodes::Procarg0) {
        let id = self.new_id();
        let arg_ids = self.visit_children(&node.args);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Procarg0(Procarg0 { arg_ids }),
        });
    }

    fn on_rational(&mut self, node: &lrp_nodes::Rational) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Rational(Rational {
                value: node.value.clone(),
            }),
        });
    }

    fn on_redo(&mut self, _node: &lrp_nodes::Redo) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Redo,
        });
    }

    fn on_regexp(&mut self, node: &lrp_nodes::Regexp) {
        let id = self.new_id();
        let part_ids = self.visit_children(&node.parts);
        let options_id = self.visit_optional_child(&node.options);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Regexp(Regexp {
                part_ids,
                options_id,
            }),
        });
    }

    fn on_reg_opt(&mut self, node: &lrp_nodes::RegOpt) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::RegOpt(RegOpt {
                options: node.options.clone(),
            }),
        });
    }

    fn on_rescue(&mut self, node: &lrp_nodes::Rescue) {
        let id = self.new_id();
        let body_id = self.visit_optional_child(&node.body);
        let rescue_body_ids = self.visit_children(&node.rescue_bodies);
        let else_id = self.visit_optional_child(&node.else_);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Rescue(Rescue {
                body_id,
                rescue_body_ids,
                else_id,
            }),
        });
    }

    fn on_rescue_body(&mut self, node: &lrp_nodes::RescueBody) {
        let id = self.new_id();
        let exc_list_id = self.visit_optional_child(&node.exc_list);
        let exc_var_id = self.visit_optional_child(&node.exc_var);
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::RescueBody(RescueBody {
                exc_list_id,
                exc_var_id,
                body_id,
            }),
        });
    }

    fn on_restarg(&mut self, node: &lrp_nodes::Restarg) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Restarg(Restarg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_retry(&mut self, _node: &lrp_nodes::Retry) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Retry,
        });
    }

    fn on_return(&mut self, node: &lrp_nodes::Return) {
        let id = self.new_id();
        let arg_ids = self.visit_children(&node.args);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Return(Return { arg_ids }),
        });
    }

    fn on_s_class(&mut self, node: &lrp_nodes::SClass) {
        let id = self.new_id();
        let expr_id = self.visit_child(&node.expr);
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::SClass(SClass { expr_id, body_id }),
        });
    }

    fn on_self_(&mut self, _node: &lrp_nodes::Self_) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Self_,
        });
    }

    fn on_send(&mut self, node: &lrp_nodes::Send) {
        let id = self.new_id();
        let recv_id = self.visit_optional_child(&node.recv);
        let arg_ids = self.visit_children(&node.args);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Send(Send {
                method_name: node.method_name.clone(),
                recv_id,
                arg_ids,
            }),
        });
    }

    fn on_shadowarg(&mut self, node: &lrp_nodes::Shadowarg) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Shadowarg(Shadowarg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_splat(&mut self, node: &lrp_nodes::Splat) {
        let id = self.new_id();
        let value_id = self.visit_optional_child(&node.value);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Splat(Splat { value_id }),
        });
    }

    fn on_str(&mut self, node: &lrp_nodes::Str) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Str(Str {
                value: node.value.clone().into_raw(),
            }),
        });
    }

    fn on_super(&mut self, node: &lrp_nodes::Super) {
        let id = self.new_id();
        let arg_ids = self.visit_children(&node.args);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Super(Super { arg_ids }),
        });
    }

    fn on_sym(&mut self, node: &lrp_nodes::Sym) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Sym(Sym {
                name: node.name.to_string_lossy(),
            }),
        });
    }

    fn on_true(&mut self, _node: &lrp_nodes::True) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::True,
        });
    }

    fn on_undef(&mut self, node: &lrp_nodes::Undef) {
        let id = self.new_id();
        let name_ids = self.visit_children(&node.names);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Undef(Undef { name_ids }),
        });
    }

    fn on_unless_guard(&mut self, node: &lrp_nodes::UnlessGuard) {
        let id = self.new_id();
        let cond_id = self.visit_child(&node.cond);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::UnlessGuard(UnlessGuard { cond_id }),
        });
    }

    fn on_until(&mut self, node: &lrp_nodes::Until) {
        let id = self.new_id();
        let cond_id = self.visit_child(&node.cond);
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Until(Until { cond_id, body_id }),
        });
    }

    fn on_until_post(&mut self, node: &lrp_nodes::UntilPost) {
        let id = self.new_id();
        let cond_id = self.visit_child(&node.cond);
        let body_id = self.visit_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::UntilPost(UntilPost { cond_id, body_id }),
        });
    }

    fn on_when(&mut self, node: &lrp_nodes::When) {
        let id = self.new_id();
        let pattern_ids = self.visit_children(&node.patterns);
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::When(When {
                pattern_ids,
                body_id,
            }),
        });
    }

    fn on_while(&mut self, node: &lrp_nodes::While) {
        let id = self.new_id();
        let cond_id = self.visit_child(&node.cond);
        let body_id = self.visit_optional_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::While(While { cond_id, body_id }),
        });
    }

    fn on_while_post(&mut self, node: &lrp_nodes::WhilePost) {
        let id = self.new_id();
        let cond_id = self.visit_child(&node.cond);
        let body_id = self.visit_child(&node.body);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::WhilePost(WhilePost { cond_id, body_id }),
        });
    }

    fn on_x_heredoc(&mut self, node: &lrp_nodes::XHeredoc) {
        let id = self.new_id();
        let part_ids = self.visit_children(&node.parts);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::XHeredoc(XHeredoc { part_ids }),
        });
    }

    fn on_xstr(&mut self, node: &lrp_nodes::Xstr) {
        let id = self.new_id();
        let part_ids = self.visit_children(&node.parts);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Xstr(Xstr { part_ids }),
        });
    }

    fn on_yield(&mut self, node: &lrp_nodes::Yield) {
        let id = self.new_id();
        let arg_ids = self.visit_children(&node.args);

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::Yield(Yield { arg_ids }),
        });
    }

    fn on_z_super(&mut self, _node: &lrp_nodes::ZSuper) {
        let id = self.new_id();

        self.insert_scope_node(Node {
            id,
            properties: NodeProperties::ZSuper,
        });
    }
}

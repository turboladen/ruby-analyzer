use indextree::{Arena, NodeId};
use lib_ruby_parser::{nodes as lrp_nodes, traverse::visitor::Visitor};
use tracing::{debug, trace};

#[allow(clippy::wildcard_imports)]
use crate::{
    lrp_extensions::NameFromNode,
    node::Node,
    nodes::*,
    properties::Properties,
    scope_gate::{Node as ScopeGateNode, ScopeGate},
};
use crate::{lrp_extensions::OptionNameFromNode, node::Loc};

#[derive(Default)]
pub(crate) struct Transformer {
    arena: Arena<Node>,
    id_stack: Vec<NodeId>,
    scope_gate: ScopeGate,
}

struct Then(NodeId);

impl Then {
    #[inline]
    fn then<F>(self, mut f: F)
    where
        F: FnMut(NodeId),
    {
        f(self.0)
    }

    #[inline]
    fn then_if<T, F>(self, maybe_value: &Option<T>, mut f: F)
    where
        F: FnMut(&T, NodeId),
    {
        if let Some(value) = maybe_value.as_ref() {
            f(value, self.0)
        }
    }
}

impl Transformer {
    /// Method to call when we're all done and ready to extract the newly transformed nodes.
    ///
    pub(crate) fn finish(self) -> Arena<Node> {
        self.arena
    }

    fn new_node(&mut self, node: Node) -> Then {
        let this_id = self.arena.new_node(node);
        self.id_stack.push(this_id);

        Then(this_id)
    }

    fn visit_child(&mut self, node: &lib_ruby_parser::Node, this_id: NodeId) {
        debug!("[visit_child] this_id: {this_id}");
        trace!("[visit_child] id_stack {:?}", &self.id_stack);
        self.visit(node);

        let new_id = self.id_stack.pop().unwrap();
        debug!("[visit_child] new_id: {new_id}");
        this_id.append(new_id, &mut self.arena);
    }

    fn visit_optional_child(&mut self, node: &Option<Box<lib_ruby_parser::Node>>, this_id: NodeId) {
        trace!("[visit_optional_child] this_id: {this_id}");

        if let Some(n) = node {
            self.visit_child(n, this_id);
        }
    }

    fn visit_children(&mut self, nodes: &[lib_ruby_parser::Node], this_id: NodeId) {
        for node in nodes {
            self.visit_child(node, this_id);
        }
    }
}

impl Visitor for Transformer {
    fn on_alias(&mut self, node: &lrp_nodes::Alias) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Alias(Alias {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.to, id);
            self.visit_child(&node.from, id);
        })
    }

    fn on_and(&mut self, node: &lrp_nodes::And) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::And(And {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.lhs, id);
            self.visit_child(&node.rhs, id);
        })
    }

    fn on_and_asgn(&mut self, node: &lrp_nodes::AndAsgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::AndAsgn(AndAsgn {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.recv, id);
            self.visit_child(&node.value, id);
        })
    }

    fn on_arg(&mut self, node: &lrp_nodes::Arg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Arg(Arg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_args(&mut self, node: &lrp_nodes::Args) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Args(Args {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.args, id))
    }

    fn on_array(&mut self, node: &lrp_nodes::Array) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Array(Array {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.elements, id))
    }

    fn on_array_pattern(&mut self, node: &lrp_nodes::ArrayPattern) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ArrayPattern(ArrayPattern {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.elements, id))
    }

    fn on_array_pattern_with_tail(&mut self, node: &lrp_nodes::ArrayPatternWithTail) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ArrayPatternWithTail(ArrayPatternWithTail {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.elements, id))
    }

    fn on_back_ref(&mut self, node: &lrp_nodes::BackRef) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::BackRef(BackRef {
                name: node.name.clone(),
            }),
        });
    }

    fn on_begin(&mut self, node: &lrp_nodes::Begin) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Begin(Begin {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.statements, id))
    }

    fn on_block(&mut self, node: &lrp_nodes::Block) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Block(Block {
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.call, id);
            self.visit_optional_child(&node.args, id);
            self.visit_optional_child(&node.body, id);
        })
    }

    fn on_block_pass(&mut self, node: &lrp_nodes::BlockPass) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::BlockPass(BlockPass {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then_if(&node.value, |value, id| self.visit_child(value, id))
    }

    fn on_blockarg(&mut self, node: &lrp_nodes::Blockarg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Blockarg(Blockarg {
                name: node.name.clone(),
                operator_l: Loc::from(node.operator_l),
                name_l: node.name_l.map(Loc::from),
            }),
        });
    }

    fn on_break(&mut self, node: &lrp_nodes::Break) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Break(Break {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| self.visit_children(&node.args, id))
    }

    fn on_c_send(&mut self, node: &lrp_nodes::CSend) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::CSend(CSend {
                method_name: node.method_name.clone(),
                dot_l: Loc::from(node.dot_l),
                selector_l: node.selector_l.map(Loc::from),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
                operator_l: node.operator_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_child(&node.recv, id);
            self.visit_children(&node.args, id);
        })
    }

    fn on_case(&mut self, node: &lrp_nodes::Case) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Case(Case {
                keyword_l: Loc::from(node.keyword_l),
                else_l: node.else_l.map(Loc::from),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.expr, id);
            self.visit_children(&node.when_bodies, id);
            self.visit_optional_child(&node.else_body, id);
        })
    }

    fn on_case_match(&mut self, node: &lrp_nodes::CaseMatch) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::CaseMatch(CaseMatch {
                keyword_l: Loc::from(node.keyword_l),
                else_l: node.else_l.map(Loc::from),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.expr, id);
            self.visit_children(&node.in_bodies, id);
            self.visit_optional_child(&node.else_body, id);
        })
    }

    fn on_casgn(&mut self, node: &lrp_nodes::Casgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Casgn(Casgn {
                name: node.name.clone(),
                double_colon_l: node.double_colon_l.map(Loc::from),
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.scope, id);
            self.visit_optional_child(&node.value, id);
        })
    }

    fn on_cbase(&mut self, node: &lrp_nodes::Cbase) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Cbase,
        });
    }

    fn on_class(&mut self, node: &lrp_nodes::Class) {
        let name = node.name_from_node();

        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Class(Class {
                name: name.clone(),
                keyword_l: Loc::from(node.keyword_l),
                operator_l: node.operator_l.map(Loc::from),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            trace!("[on_class] this_id: {id}");

            self.scope_gate
                .push_owned(ScopeGateNode::Class(name.clone()));

            debug!(
                "Transforming class '{name}'; scope branch for body: {:?}",
                &self.scope_gate
            );

            self.visit_optional_child(&node.superclass, id);
            self.visit_child(&node.name, id);
            self.visit_optional_child(&node.body, id);

            self.scope_gate.pop();
            debug!(
                "Transforming class '{name}'; scope branch for self: {:?}",
                &self.scope_gate
            );
        })
    }

    fn on_complex(&mut self, node: &lrp_nodes::Complex) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Complex(Complex {
                value: node.value.clone(),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_const(&mut self, node: &lrp_nodes::Const) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Const(Const {
                name: node.name.clone(),
                double_colon_l: node.double_colon_l.map(Loc::from),
                name_l: Loc::from(node.name_l),
            }),
        })
        .then_if(&node.scope, |scope, id| {
            // self.id_stack.push(id);
            self.visit_child(scope, id);
        })
    }

    fn on_const_pattern(&mut self, node: &lrp_nodes::ConstPattern) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ConstPattern(ConstPattern {
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.const_, id);
            self.visit_child(&node.pattern, id);
        })
    }

    fn on_cvar(&mut self, node: &lrp_nodes::Cvar) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Cvar(Cvar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_cvasgn(&mut self, node: &lrp_nodes::Cvasgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Cvasgn(Cvasgn {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        })
        .then_if(&node.value, |value, id| self.visit_child(value, id))
    }

    fn on_def(&mut self, node: &lrp_nodes::Def) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Def(Def {
                name: node.name.clone(),
                keyword_l: Loc::from(node.keyword_l),
                name_l: Loc::from(node.name_l),
                end_l: node.end_l.map(Loc::from),
                assignment_l: node.assignment_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.args, id);

            self.scope_gate
                .push_owned(ScopeGateNode::Def(node.name.clone()));

            self.visit_optional_child(&node.body, id);

            self.scope_gate.pop();
        })
    }

    fn on_defined(&mut self, node: &lrp_nodes::Defined) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Defined(Defined {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_child(&node.value, id))
    }

    fn on_defs(&mut self, node: &lrp_nodes::Defs) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Defs(Defs {
                name: node.name.clone(),
                keyword_l: Loc::from(node.keyword_l),
                operator_l: Loc::from(node.operator_l),
                name_l: Loc::from(node.name_l),
                assignment_l: node.assignment_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_child(&node.definee, id);
            self.visit_optional_child(&node.args, id);

            self.scope_gate
                .push_owned(ScopeGateNode::Defs(node.name.clone()));

            self.visit_optional_child(&node.body, id);

            self.scope_gate.pop();
        })
    }

    fn on_dstr(&mut self, node: &lrp_nodes::Dstr) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Dstr(Dstr {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.parts, id))
    }

    fn on_dsym(&mut self, node: &lrp_nodes::Dsym) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Dsym(Dsym {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.parts, id))
    }

    fn on_e_flip_flop(&mut self, node: &lrp_nodes::EFlipFlop) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::EFlipFlop(EFlipFlop {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.left, id);
            self.visit_optional_child(&node.right, id);
        })
    }

    fn on_empty_else(&mut self, node: &lrp_nodes::EmptyElse) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::EmptyElse,
        });
    }

    fn on_encoding(&mut self, node: &lrp_nodes::Encoding) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Encoding,
        });
    }

    fn on_ensure(&mut self, node: &lrp_nodes::Ensure) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Ensure(Ensure {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.body, id);
            self.visit_optional_child(&node.ensure, id);
        })
    }

    fn on_erange(&mut self, node: &lrp_nodes::Erange) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Erange(Erange {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.left, id);
            self.visit_optional_child(&node.right, id);
        })
    }

    fn on_false(&mut self, node: &lrp_nodes::False) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::False,
        });
    }

    fn on_file(&mut self, node: &lrp_nodes::File) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::File,
        });
    }

    fn on_find_pattern(&mut self, node: &lrp_nodes::FindPattern) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::FindPattern(FindPattern {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.elements, id))
    }

    fn on_float(&mut self, node: &lrp_nodes::Float) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Float(Float {
                value: node.value.clone(),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_for(&mut self, node: &lrp_nodes::For) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::For(For {
                keyword_l: Loc::from(node.keyword_l),
                operator_l: Loc::from(node.operator_l),
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.iterator, id);
            self.visit_child(&node.iteratee, id);
            self.visit_optional_child(&node.body, id);
        })
    }

    fn on_forward_arg(&mut self, node: &lrp_nodes::ForwardArg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ForwardArg,
        });
    }

    fn on_forwarded_args(&mut self, node: &lrp_nodes::ForwardedArgs) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ForwardedArgs,
        });
    }

    fn on_gvar(&mut self, node: &lrp_nodes::Gvar) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Gvar(Gvar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_gvasgn(&mut self, node: &lrp_nodes::Gvasgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Gvasgn(Gvasgn {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        })
        .then_if(&node.value, |value, id| self.visit_child(value, id))
    }

    fn on_hash(&mut self, node: &lrp_nodes::Hash) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Hash(Hash {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.pairs, id))
    }

    fn on_hash_pattern(&mut self, node: &lrp_nodes::HashPattern) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::HashPattern(HashPattern {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.elements, id))
    }

    fn on_heredoc(&mut self, node: &lrp_nodes::Heredoc) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Heredoc(Heredoc {
                heredoc_body_l: Loc::from(node.heredoc_body_l),
                heredoc_end_l: Loc::from(node.heredoc_end_l),
            }),
        })
        .then(|id| self.visit_children(&node.parts, id))
    }

    fn on_if(&mut self, node: &lrp_nodes::If) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::If(If {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
                else_l: node.else_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_child(&node.cond, id);
            self.visit_optional_child(&node.if_true, id);
            self.visit_optional_child(&node.if_false, id);
        })
    }

    fn on_if_guard(&mut self, node: &lrp_nodes::IfGuard) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IfGuard(IfGuard {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| self.visit_child(&node.cond, id))
    }

    fn on_i_flip_flop(&mut self, node: &lrp_nodes::IFlipFlop) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IFlipFlop(IFlipFlop {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.left, id);
            self.visit_optional_child(&node.right, id);
        })
    }

    fn on_if_mod(&mut self, node: &lrp_nodes::IfMod) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IfMod(IfMod {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.cond, id);
            self.visit_optional_child(&node.if_true, id);
            self.visit_optional_child(&node.if_false, id);
        })
    }

    fn on_if_ternary(&mut self, node: &lrp_nodes::IfTernary) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IfTernary(IfTernary {
                question_l: Loc::from(node.question_l),
                colon_l: Loc::from(node.colon_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.cond, id);
            self.visit_child(&node.if_true, id);
            self.visit_child(&node.if_false, id);
        })
    }

    fn on_index(&mut self, node: &lrp_nodes::Index) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Index(Index {
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.recv, id);
            self.visit_children(&node.indexes, id);
        })
    }

    fn on_index_asgn(&mut self, node: &lrp_nodes::IndexAsgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IndexAsgn(IndexAsgn {
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_child(&node.recv, id);
            self.visit_children(&node.indexes, id);
            self.visit_optional_child(&node.value, id);
        })
    }

    fn on_in_pattern(&mut self, node: &lrp_nodes::InPattern) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::InPattern(InPattern {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.pattern, id);
            self.visit_optional_child(&node.guard, id);
            self.visit_optional_child(&node.body, id);
        })
    }

    fn on_int(&mut self, node: &lrp_nodes::Int) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Int(Int {
                value: node.value.clone(),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_irange(&mut self, node: &lrp_nodes::Irange) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Irange(Irange {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.left, id);
            self.visit_optional_child(&node.right, id);
        })
    }

    fn on_ivar(&mut self, node: &lrp_nodes::Ivar) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Ivar(Ivar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_ivasgn(&mut self, node: &lrp_nodes::Ivasgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Ivasgn(Ivasgn {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        })
        .then_if(&node.value, |value, id| self.visit_child(value, id))
    }

    fn on_kwarg(&mut self, node: &lrp_nodes::Kwarg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwarg(Kwarg {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_kwargs(&mut self, node: &lrp_nodes::Kwargs) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwargs,
        })
        .then(|id| self.visit_children(&node.pairs, id))
    }

    fn on_kw_begin(&mut self, node: &lrp_nodes::KwBegin) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::KwBegin(KwBegin {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.statements, id))
    }

    fn on_kwnilarg(&mut self, node: &lrp_nodes::Kwnilarg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwnilarg(Kwnilarg {
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_kwoptarg(&mut self, node: &lrp_nodes::Kwoptarg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwoptarg(Kwoptarg {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
            }),
        })
        .then(|id| self.visit_child(&node.default, id))
    }

    fn on_kwrestarg(&mut self, node: &lrp_nodes::Kwrestarg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwrestarg(Kwrestarg {
                name: node.name.clone(),
                operator_l: Loc::from(node.operator_l),
                name_l: node.name_l.map(Loc::from),
            }),
        });
    }

    fn on_kwsplat(&mut self, node: &lrp_nodes::Kwsplat) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwsplat(Kwsplat {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| self.visit_child(&node.value, id))
    }

    fn on_lambda(&mut self, node: &lrp_nodes::Lambda) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Lambda,
        });
    }

    fn on_line(&mut self, node: &lrp_nodes::Line) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Line,
        });
    }

    fn on_lvar(&mut self, node: &lrp_nodes::Lvar) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Lvar(Lvar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_lvasgn(&mut self, node: &lrp_nodes::Lvasgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Lvasgn(Lvasgn {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        })
        .then_if(&node.value, |value, id| self.visit_child(value, id))
    }

    fn on_masgn(&mut self, node: &lrp_nodes::Masgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Masgn(Masgn {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.lhs, id);
            self.visit_child(&node.rhs, id);
        })
    }

    fn on_match_alt(&mut self, node: &lrp_nodes::MatchAlt) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchAlt(MatchAlt {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.lhs, id);
            self.visit_child(&node.rhs, id);
        })
    }

    fn on_match_as(&mut self, node: &lrp_nodes::MatchAs) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchAs(MatchAs {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.value, id);
            self.visit_child(&node.as_, id);
        })
    }

    fn on_match_current_line(&mut self, node: &lrp_nodes::MatchCurrentLine) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchCurrentLine,
        })
        .then(|id| self.visit_child(&node.re, id))
    }

    fn on_match_nil_pattern(&mut self, node: &lrp_nodes::MatchNilPattern) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchNilPattern(MatchNilPattern {
                operator_l: Loc::from(node.operator_l),
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_match_pattern(&mut self, node: &lrp_nodes::MatchPattern) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchPattern(MatchPattern {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.value, id);
            self.visit_child(&node.pattern, id);
        })
    }

    fn on_match_pattern_p(&mut self, node: &lrp_nodes::MatchPatternP) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchPatternP(MatchPatternP {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.value, id);
            self.visit_child(&node.pattern, id);
        })
    }

    fn on_match_rest(&mut self, node: &lrp_nodes::MatchRest) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchRest(MatchRest {
                name: node.option_name_from_node(),
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then_if(&node.name, |name, id| self.visit_child(name, id))
    }

    fn on_match_var(&mut self, node: &lrp_nodes::MatchVar) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchVar(MatchVar {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_match_with_lvasgn(&mut self, node: &lrp_nodes::MatchWithLvasgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchWithLvasgn(MatchWithLvasgn {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.re, id);
            self.visit_child(&node.value, id);
        })
    }

    fn on_mlhs(&mut self, node: &lrp_nodes::Mlhs) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Mlhs(Mlhs {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.items, id))
    }

    fn on_module(&mut self, node: &lrp_nodes::Module) {
        let name_string = node.name_from_node();

        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Module(Module {
                name: name_string.clone(),
                keyword_l: Loc::from(node.keyword_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(move |id| {
            self.scope_gate
                .push_owned(ScopeGateNode::Module(name_string.clone()));

            self.visit_child(&node.name, id);
            self.visit_optional_child(&node.body, id);

            self.scope_gate.pop();
        })
    }

    fn on_next(&mut self, node: &lrp_nodes::Next) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Next(Next {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| self.visit_children(&node.args, id))
    }

    fn on_nil(&mut self, node: &lrp_nodes::Nil) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Nil,
        });
    }

    fn on_nth_ref(&mut self, node: &lrp_nodes::NthRef) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::NthRef(NthRef {
                name: node.name.clone(),
            }),
        });
    }

    fn on_numblock(&mut self, node: &lrp_nodes::Numblock) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Numblock(Numblock {
                numargs: node.numargs,
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.call, id);
            self.visit_child(&node.body, id);
        })
    }

    fn on_op_asgn(&mut self, node: &lrp_nodes::OpAsgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::OpAsgn(OpAsgn {
                operator: node.operator.clone(),
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.recv, id);
            self.visit_child(&node.value, id);
        })
    }

    fn on_optarg(&mut self, node: &lrp_nodes::Optarg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Optarg(Optarg {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| self.visit_child(&node.default, id))
    }

    fn on_or(&mut self, node: &lrp_nodes::Or) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Or(Or {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.lhs, id);
            self.visit_child(&node.rhs, id);
        })
    }

    fn on_or_asgn(&mut self, node: &lrp_nodes::OrAsgn) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::OrAsgn(OrAsgn {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.recv, id);
            self.visit_child(&node.value, id);
        })
    }

    fn on_pair(&mut self, node: &lrp_nodes::Pair) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Pair(Pair {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.key, id);
            self.visit_child(&node.value, id);
        })
    }

    fn on_pin(&mut self, node: &lrp_nodes::Pin) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Pin(Pin {
                selector_l: Loc::from(node.selector_l),
            }),
        })
        .then(|id| self.visit_child(&node.var, id))
    }

    fn on_postexe(&mut self, node: &lrp_nodes::Postexe) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Postexe(Postexe {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then_if(&node.body, |body, id| self.visit_child(body, id))
    }

    fn on_preexe(&mut self, node: &lrp_nodes::Preexe) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Preexe(Preexe {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then_if(&node.body, |body, id| self.visit_child(body, id))
    }

    fn on_procarg0(&mut self, node: &lrp_nodes::Procarg0) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Procarg0(Procarg0 {
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.args, id))
    }

    fn on_rational(&mut self, node: &lrp_nodes::Rational) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Rational(Rational {
                value: node.value.clone(),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_redo(&mut self, node: &lrp_nodes::Redo) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Redo,
        });
    }

    fn on_regexp(&mut self, node: &lrp_nodes::Regexp) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Regexp(Regexp {
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            self.visit_children(&node.parts, id);
            self.visit_optional_child(&node.options, id);
        })
    }

    fn on_reg_opt(&mut self, node: &lrp_nodes::RegOpt) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::RegOpt(RegOpt {
                options: node.options.clone(),
            }),
        });
    }

    fn on_rescue(&mut self, node: &lrp_nodes::Rescue) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Rescue(Rescue {
                else_l: node.else_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.body, id);
            self.visit_children(&node.rescue_bodies, id);
            self.visit_optional_child(&node.else_, id);
        })
    }

    fn on_rescue_body(&mut self, node: &lrp_nodes::RescueBody) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::RescueBody(RescueBody {
                keyword_l: Loc::from(node.keyword_l),
                assoc_l: node.assoc_l.map(Loc::from),
                begin_l: node.begin_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.exc_list, id);
            self.visit_optional_child(&node.exc_var, id);
            self.visit_optional_child(&node.body, id);
        })
    }

    fn on_restarg(&mut self, node: &lrp_nodes::Restarg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Restarg(Restarg {
                name: node.name.clone(),
                operator_l: Loc::from(node.operator_l),
                name_l: node.name_l.map(Loc::from),
            }),
        });
    }

    fn on_retry(&mut self, node: &lrp_nodes::Retry) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Retry,
        });
    }

    fn on_return(&mut self, node: &lrp_nodes::Return) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Return(Return {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| self.visit_children(&node.args, id))
    }

    fn on_s_class(&mut self, node: &lrp_nodes::SClass) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::SClass(SClass {
                keyword_l: Loc::from(node.keyword_l),
                operator_l: Loc::from(node.operator_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.expr, id);
            self.visit_optional_child(&node.body, id);
        })
    }

    fn on_self_(&mut self, node: &lrp_nodes::Self_) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Self_,
        });
    }

    fn on_send(&mut self, node: &lrp_nodes::Send) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Send(Send {
                method_name: node.method_name.clone(),
                dot_l: node.dot_l.map(Loc::from),
                selector_l: node.selector_l.map(Loc::from),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
                operator_l: node.operator_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_optional_child(&node.recv, id);
            self.visit_children(&node.args, id);
        })
    }

    fn on_shadowarg(&mut self, node: &lrp_nodes::Shadowarg) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Shadowarg(Shadowarg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_splat(&mut self, node: &lrp_nodes::Splat) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Splat(Splat {
                operator_l: Loc::from(node.operator_l),
            }),
        })
        .then_if(&node.value, |value, id| self.visit_child(value, id))
    }

    fn on_str(&mut self, node: &lrp_nodes::Str) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Str(Str {
                value: node.value.clone().into_raw(),

                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_super(&mut self, node: &lrp_nodes::Super) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Super(Super {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.args, id))
    }

    fn on_sym(&mut self, node: &lrp_nodes::Sym) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Sym(Sym {
                // NOTE: Potential loss of data here.
                name: node.name.to_string_lossy(),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_true(&mut self, node: &lrp_nodes::True) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::True,
        });
    }

    fn on_undef(&mut self, node: &lrp_nodes::Undef) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Undef(Undef {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| self.visit_children(&node.names, id))
    }

    fn on_unless_guard(&mut self, node: &lrp_nodes::UnlessGuard) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::UnlessGuard(UnlessGuard {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| self.visit_child(&node.cond, id))
    }

    fn on_until(&mut self, node: &lrp_nodes::Until) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Until(Until {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_child(&node.cond, id);
            self.visit_optional_child(&node.body, id);
        })
    }

    fn on_until_post(&mut self, node: &lrp_nodes::UntilPost) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::UntilPost(UntilPost {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.cond, id);
            self.visit_child(&node.body, id);
        })
    }

    fn on_when(&mut self, node: &lrp_nodes::When) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::When(When {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
            }),
        })
        .then(|id| {
            self.visit_children(&node.patterns, id);
            self.visit_optional_child(&node.body, id);
        })
    }

    fn on_while(&mut self, node: &lrp_nodes::While) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::While(While {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| {
            self.visit_child(&node.cond, id);
            self.visit_optional_child(&node.body, id);
        })
    }

    fn on_while_post(&mut self, node: &lrp_nodes::WhilePost) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::WhilePost(WhilePost {
                keyword_l: Loc::from(node.keyword_l),
            }),
        })
        .then(|id| {
            self.visit_child(&node.cond, id);
            self.visit_child(&node.body, id);
        })
    }

    fn on_x_heredoc(&mut self, node: &lrp_nodes::XHeredoc) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::XHeredoc(XHeredoc {
                heredoc_body_l: Loc::from(node.heredoc_body_l),
                heredoc_end_l: Loc::from(node.heredoc_end_l),
            }),
        })
        .then(|id| self.visit_children(&node.parts, id))
    }

    fn on_xstr(&mut self, node: &lrp_nodes::Xstr) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Xstr(Xstr {
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        })
        .then(|id| self.visit_children(&node.parts, id))
    }

    fn on_yield(&mut self, node: &lrp_nodes::Yield) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Yield(Yield {
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        })
        .then(|id| self.visit_children(&node.args, id))
    }

    fn on_z_super(&mut self, node: &lrp_nodes::ZSuper) {
        self.new_node(Node {
            scope_gate: self.scope_gate.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ZSuper,
        });
    }
}

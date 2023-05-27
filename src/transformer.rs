use lib_ruby_parser::{nodes as lrp_nodes, traverse::visitor::Visitor};
use tracing::debug;

use crate::node::Loc;
#[allow(clippy::wildcard_imports)]
use crate::{
    lrp_extensions::{NameFromNode, OptionNameFromNode},
    namespace::{Namespace, Node as NamespaceNode},
    node::Node,
    nodes::*,
    properties::Properties,
};

pub(crate) struct Transformer {
    current_id: usize,
    namespace: Namespace,
    nodes: Vec<Node>,
}

impl Transformer {
    pub(crate) fn new() -> Self {
        Self {
            current_id: 0,
            namespace: Namespace::default(),
            nodes: Vec::new(),
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn into_nodes(self) -> Vec<Node> {
        self.nodes
    }

    fn new_id(&mut self) -> usize {
        self.current_id += 1;
        self.current_id
    }

    fn visit_vec_node_child(&mut self, nodes: &[lib_ruby_parser::Node]) -> Vec<usize> {
        nodes
            .iter()
            .map(|node| {
                self.visit_node_child(|transformer| {
                    transformer.visit(node);
                })
            })
            .collect()
    }

    fn visit_optional_single_node_child(
        &mut self,
        node: Option<&lib_ruby_parser::Node>,
    ) -> Option<usize> {
        node.map(|n| self.visit_single_node_child(n))
    }

    fn visit_single_node_child(&mut self, node: &lib_ruby_parser::Node) -> usize {
        self.visit_node_child(|transformer| {
            transformer.visit(node);
        })
    }

    fn visit_node_child<F>(&mut self, func: F) -> usize
    where
        F: FnOnce(&mut Transformer),
    {
        func(self);

        self.nodes.last().map(|n| n.id()).unwrap()
    }
}

impl Visitor for Transformer {
    fn on_alias(&mut self, node: &lrp_nodes::Alias) {
        let id = self.new_id();
        let to_id = self.visit_single_node_child(&node.to);
        let from_id = self.visit_single_node_child(&node.from);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Alias(Alias {
                to_id,
                from_id,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_and(&mut self, node: &lrp_nodes::And) {
        let id = self.new_id();
        let lhs_id = self.visit_single_node_child(&node.lhs);
        let rhs_id = self.visit_single_node_child(&node.rhs);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::And(And {
                lhs_id,
                rhs_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_and_asgn(&mut self, node: &lrp_nodes::AndAsgn) {
        let id = self.new_id();
        let recv_id = self.visit_single_node_child(&node.recv);
        let value_id = self.visit_single_node_child(&node.value);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::AndAsgn(AndAsgn {
                recv_id,
                value_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_arg(&mut self, node: &lrp_nodes::Arg) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Arg(Arg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_args(&mut self, node: &lrp_nodes::Args) {
        let id = self.new_id();
        let arg_ids = self.visit_vec_node_child(&node.args);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Args(Args {
                arg_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_array(&mut self, node: &lrp_nodes::Array) {
        let id = self.new_id();
        let element_ids = self.visit_vec_node_child(&node.elements);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Array(Array {
                element_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_array_pattern(&mut self, node: &lrp_nodes::ArrayPattern) {
        let id = self.new_id();
        let element_ids = self.visit_vec_node_child(&node.elements);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ArrayPattern(ArrayPattern {
                element_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_array_pattern_with_tail(&mut self, node: &lrp_nodes::ArrayPatternWithTail) {
        let id = self.new_id();
        let element_ids = self.visit_vec_node_child(&node.elements);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ArrayPatternWithTail(ArrayPatternWithTail {
                element_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_back_ref(&mut self, node: &lrp_nodes::BackRef) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::BackRef(BackRef {
                name: node.name.clone(),
            }),
        });
    }

    fn on_begin(&mut self, node: &lrp_nodes::Begin) {
        let id = self.new_id();
        let statement_ids = self.visit_vec_node_child(&node.statements);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Begin(Begin {
                statement_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_block(&mut self, node: &lrp_nodes::Block) {
        let id = self.new_id();
        let call_id = self.visit_single_node_child(&node.call);
        let args_id = self.visit_optional_single_node_child(node.args.as_deref());
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Block(Block {
                call_id,
                args_id,
                body_id,
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_block_pass(&mut self, node: &lrp_nodes::BlockPass) {
        let id = self.new_id();
        let value_id = self.visit_optional_single_node_child(node.value.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::BlockPass(BlockPass {
                value_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_blockarg(&mut self, node: &lrp_nodes::Blockarg) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Blockarg(Blockarg {
                name: node.name.clone(),
                operator_l: Loc::from(node.operator_l),
                name_l: node.name_l.map(Loc::from),
            }),
        });
    }

    fn on_break(&mut self, node: &lrp_nodes::Break) {
        let id = self.new_id();
        let arg_ids = self.visit_vec_node_child(&node.args);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Break(Break {
                arg_ids,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_c_send(&mut self, node: &lrp_nodes::CSend) {
        let id = self.new_id();
        let arg_ids = self.visit_vec_node_child(&node.args);
        let recv_id = self.visit_single_node_child(&node.recv);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::CSend(CSend {
                recv_id,
                method_name: node.method_name.clone(),
                arg_ids,
                dot_l: Loc::from(node.dot_l),
                selector_l: node.selector_l.map(Loc::from),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_case(&mut self, node: &lrp_nodes::Case) {
        let id = self.new_id();
        let expr_id = self.visit_optional_single_node_child(node.expr.as_deref());
        let when_body_ids = self.visit_vec_node_child(&node.when_bodies);
        let else_body_id = self.visit_optional_single_node_child(node.else_body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Case(Case {
                expr_id,
                when_body_ids,
                else_body_id,
                keyword_l: Loc::from(node.keyword_l),
                else_l: node.else_l.map(Loc::from),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_case_match(&mut self, node: &lrp_nodes::CaseMatch) {
        let id = self.new_id();
        let expr_id = self.visit_single_node_child(&node.expr);
        let in_body_ids = self.visit_vec_node_child(&node.in_bodies);
        let else_body_id = self.visit_optional_single_node_child(node.else_body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::CaseMatch(CaseMatch {
                expr_id,
                in_body_ids,
                else_body_id,
                keyword_l: Loc::from(node.keyword_l),
                else_l: node.else_l.map(Loc::from),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_casgn(&mut self, node: &lrp_nodes::Casgn) {
        let id = self.new_id();
        let scope_id = self.visit_optional_single_node_child(node.scope.as_deref());
        let value_id = self.visit_optional_single_node_child(node.value.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Casgn(Casgn {
                name: node.name.clone(),
                scope_id,
                value_id,
                double_colon_l: node.double_colon_l.map(Loc::from),
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_cbase(&mut self, node: &lrp_nodes::Cbase) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Cbase(Cbase),
        });
    }

    fn on_class(&mut self, node: &lrp_nodes::Class) {
        let id = self.new_id();
        let superclass_id = self.visit_optional_single_node_child(node.superclass.as_deref());
        let name_id = self.visit_single_node_child(&node.name);
        assert_ne!(id, name_id, "{:#?}", &self.nodes);

        let name = node.name_from_node();
        self.namespace
            .push_owned(NamespaceNode::Class { name: name.clone() });
        debug!(
            "Transforming class '{name}'; scope branch for body: {:?}",
            &self.namespace
        );

        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.namespace.pop();
        debug!(
            "Transforming class '{name}'; scope branch for self: {:?}",
            &self.namespace
        );

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Class(Class {
                name,
                keyword_l: Loc::from(node.keyword_l),
                operator_l: node.operator_l.map(Loc::from),
                end_l: Loc::from(node.end_l),
                name_id,
                superclass_id,
                body_id,
            }),
        });
    }

    fn on_complex(&mut self, node: &lrp_nodes::Complex) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Complex(Complex {
                value: node.value.clone(),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_const(&mut self, node: &lrp_nodes::Const) {
        let id = self.new_id();
        let scope_id = self.visit_optional_single_node_child(node.scope.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Const(Const {
                name: node.name.clone(),
                scope_id,
                double_colon_l: node.double_colon_l.map(Loc::from),
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_const_pattern(&mut self, node: &lrp_nodes::ConstPattern) {
        let id = self.new_id();
        let const_id = self.visit_single_node_child(&node.const_);
        let pattern_id = self.visit_single_node_child(&node.pattern);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ConstPattern(ConstPattern {
                const_id,
                pattern_id,
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_cvar(&mut self, node: &lrp_nodes::Cvar) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Cvar(Cvar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_cvasgn(&mut self, node: &lrp_nodes::Cvasgn) {
        let id = self.new_id();
        let value_id = self.visit_optional_single_node_child(node.value.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Cvasgn(Cvasgn {
                name: node.name.clone(),
                value_id,
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_def(&mut self, node: &lrp_nodes::Def) {
        let id = self.new_id();

        let args_id = self.visit_optional_single_node_child(node.args.as_deref());

        // self.namespace
        //     .push_owned(NamespaceNode::Def(node.name.clone()));

        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.namespace.pop();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Def(Def {
                name: node.name.clone(),
                args_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                name_l: Loc::from(node.name_l),
                end_l: node.end_l.map(Loc::from),
                assignment_l: node.assignment_l.map(Loc::from),
            }),
        });
    }

    fn on_defined(&mut self, node: &lrp_nodes::Defined) {
        let id = self.new_id();
        let value_id = self.visit_single_node_child(&node.value);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Defined(Defined {
                value_id,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_defs(&mut self, node: &lrp_nodes::Defs) {
        let id = self.new_id();
        let definee_id = self.visit_single_node_child(&node.definee);
        let args_id = self.visit_optional_single_node_child(node.args.as_deref());

        // self.namespace
        //     .push_owned(NamespaceNode::Defs(node.name.clone()));

        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.namespace.pop();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Defs(Defs {
                definee_id,
                name: node.name.clone(),
                args_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                operator_l: Loc::from(node.operator_l),
                name_l: Loc::from(node.name_l),
                assignment_l: node.assignment_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_dstr(&mut self, node: &lrp_nodes::Dstr) {
        let id = self.new_id();
        let part_ids = self.visit_vec_node_child(&node.parts);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Dstr(Dstr {
                part_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_dsym(&mut self, node: &lrp_nodes::Dsym) {
        let id = self.new_id();
        let part_ids = self.visit_vec_node_child(&node.parts);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Dsym(Dsym {
                part_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_e_flip_flop(&mut self, node: &lrp_nodes::EFlipFlop) {
        let id = self.new_id();
        let left_id = self.visit_optional_single_node_child(node.left.as_deref());
        let right_id = self.visit_optional_single_node_child(node.right.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::EFlipFlop(EFlipFlop {
                left_id,
                right_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_empty_else(&mut self, node: &lrp_nodes::EmptyElse) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::EmptyElse(EmptyElse),
        });
    }

    fn on_encoding(&mut self, node: &lrp_nodes::Encoding) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Encoding(Encoding),
        });
    }

    fn on_ensure(&mut self, node: &lrp_nodes::Ensure) {
        let id = self.new_id();
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());
        let ensure_id = self.visit_optional_single_node_child(node.ensure.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Ensure(Ensure {
                body_id,
                ensure_id,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_erange(&mut self, node: &lrp_nodes::Erange) {
        let id = self.new_id();
        let left_id = self.visit_optional_single_node_child(node.left.as_deref());
        let right_id = self.visit_optional_single_node_child(node.right.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Erange(Erange {
                left_id,
                right_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_false(&mut self, node: &lrp_nodes::False) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::False(False),
        });
    }

    fn on_file(&mut self, node: &lrp_nodes::File) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::File(File),
        });
    }

    fn on_find_pattern(&mut self, node: &lrp_nodes::FindPattern) {
        let id = self.new_id();
        let element_ids = self.visit_vec_node_child(&node.elements);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::FindPattern(FindPattern {
                element_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_float(&mut self, node: &lrp_nodes::Float) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Float(Float {
                value: node.value.clone(),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_for(&mut self, node: &lrp_nodes::For) {
        let id = self.new_id();
        let iterator_id = self.visit_single_node_child(&node.iterator);
        let iteratee_id = self.visit_single_node_child(&node.iteratee);
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::For(For {
                iterator_id,
                iteratee_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                operator_l: Loc::from(node.operator_l),
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_forward_arg(&mut self, node: &lrp_nodes::ForwardArg) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ForwardArg(ForwardArg),
        });
    }

    fn on_forwarded_args(&mut self, node: &lrp_nodes::ForwardedArgs) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ForwardedArgs(ForwardedArgs),
        });
    }

    fn on_gvar(&mut self, node: &lrp_nodes::Gvar) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Gvar(Gvar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_gvasgn(&mut self, node: &lrp_nodes::Gvasgn) {
        let id = self.new_id();
        let value_id = self.visit_optional_single_node_child(node.value.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Gvasgn(Gvasgn {
                name: node.name.clone(),
                value_id,
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_hash(&mut self, node: &lrp_nodes::Hash) {
        let id = self.new_id();
        let pair_ids = self.visit_vec_node_child(&node.pairs);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Hash(Hash {
                pair_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_hash_pattern(&mut self, node: &lrp_nodes::HashPattern) {
        let id = self.new_id();
        let element_ids = self.visit_vec_node_child(&node.elements);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::HashPattern(HashPattern {
                element_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_heredoc(&mut self, node: &lrp_nodes::Heredoc) {
        let id = self.new_id();
        let part_ids = self.visit_vec_node_child(&node.parts);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Heredoc(Heredoc {
                part_ids,
                heredoc_body_l: Loc::from(node.heredoc_body_l),
                heredoc_end_l: Loc::from(node.heredoc_end_l),
            }),
        });
    }

    fn on_if(&mut self, node: &lrp_nodes::If) {
        let id = self.new_id();
        let cond_id = self.visit_single_node_child(&node.cond);
        let if_true_id = self.visit_optional_single_node_child(node.if_true.as_deref());
        let if_false_id = self.visit_optional_single_node_child(node.if_false.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::If(If {
                cond_id,
                if_true_id,
                if_false_id,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
                else_l: node.else_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_if_guard(&mut self, node: &lrp_nodes::IfGuard) {
        let id = self.new_id();
        let cond_id = self.visit_single_node_child(&node.cond);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IfGuard(IfGuard {
                cond_id,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_i_flip_flop(&mut self, node: &lrp_nodes::IFlipFlop) {
        let id = self.new_id();
        let left_id = self.visit_optional_single_node_child(node.left.as_deref());
        let right_id = self.visit_optional_single_node_child(node.right.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IFlipFlop(IFlipFlop {
                left_id,
                right_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_if_mod(&mut self, node: &lrp_nodes::IfMod) {
        let id = self.new_id();
        let cond_id = self.visit_single_node_child(&node.cond);
        let if_true_id = self.visit_optional_single_node_child(node.if_true.as_deref());
        let if_false_id = self.visit_optional_single_node_child(node.if_false.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IfMod(IfMod {
                cond_id,
                if_true_id,
                if_false_id,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_if_ternary(&mut self, node: &lrp_nodes::IfTernary) {
        let id = self.new_id();
        let cond_id = self.visit_single_node_child(&node.cond);
        let if_true_id = self.visit_single_node_child(&node.if_true);
        let if_false_id = self.visit_single_node_child(&node.if_false);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IfTernary(IfTernary {
                cond_id,
                if_true_id,
                if_false_id,
                question_l: Loc::from(node.question_l),
                colon_l: Loc::from(node.colon_l),
            }),
        });
    }

    fn on_index(&mut self, node: &lrp_nodes::Index) {
        let id = self.new_id();
        let recv_id = self.visit_single_node_child(&node.recv);
        let index_ids = self.visit_vec_node_child(&node.indexes);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Index(Index {
                recv_id,
                index_ids,
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_index_asgn(&mut self, node: &lrp_nodes::IndexAsgn) {
        let id = self.new_id();
        let recv_id = self.visit_single_node_child(&node.recv);
        let index_ids = self.visit_vec_node_child(&node.indexes);
        let value_id = self.visit_optional_single_node_child(node.value.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::IndexAsgn(IndexAsgn {
                recv_id,
                index_ids,
                value_id,
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_in_pattern(&mut self, node: &lrp_nodes::InPattern) {
        let id = self.new_id();
        let pattern_id = self.visit_single_node_child(&node.pattern);
        let guard_id = self.visit_optional_single_node_child(node.guard.as_deref());
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::InPattern(InPattern {
                pattern_id,
                guard_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
            }),
        });
    }

    fn on_int(&mut self, node: &lrp_nodes::Int) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Int(Int {
                value: node.value.clone(),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_irange(&mut self, node: &lrp_nodes::Irange) {
        let id = self.new_id();
        let left_id = self.visit_optional_single_node_child(node.left.as_deref());
        let right_id = self.visit_optional_single_node_child(node.right.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Irange(Irange {
                left_id,
                right_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_ivar(&mut self, node: &lrp_nodes::Ivar) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Ivar(Ivar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_ivasgn(&mut self, node: &lrp_nodes::Ivasgn) {
        let id = self.new_id();
        let value_id = self.visit_optional_single_node_child(node.value.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Ivasgn(Ivasgn {
                name: node.name.clone(),
                value_id,
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_kwarg(&mut self, node: &lrp_nodes::Kwarg) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwarg(Kwarg {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_kwargs(&mut self, node: &lrp_nodes::Kwargs) {
        let id = self.new_id();
        let pair_ids = self.visit_vec_node_child(&node.pairs);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwargs(Kwargs { pair_ids }),
        });
    }

    fn on_kw_begin(&mut self, node: &lrp_nodes::KwBegin) {
        let id = self.new_id();
        let statement_ids = self.visit_vec_node_child(&node.statements);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::KwBegin(KwBegin {
                statement_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_kwnilarg(&mut self, node: &lrp_nodes::Kwnilarg) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwnilarg(Kwnilarg {
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_kwoptarg(&mut self, node: &lrp_nodes::Kwoptarg) {
        let id = self.new_id();
        let default_id = self.visit_single_node_child(&node.default);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwoptarg(Kwoptarg {
                name: node.name.clone(),
                default_id,
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_kwrestarg(&mut self, node: &lrp_nodes::Kwrestarg) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwrestarg(Kwrestarg {
                name: node.name.clone(),
                operator_l: Loc::from(node.operator_l),
                name_l: node.name_l.map(Loc::from),
            }),
        });
    }

    fn on_kwsplat(&mut self, node: &lrp_nodes::Kwsplat) {
        let id = self.new_id();
        let value_id = self.visit_single_node_child(&node.value);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Kwsplat(Kwsplat {
                value_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_lambda(&mut self, node: &lrp_nodes::Lambda) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Lambda(Lambda),
        });
    }

    fn on_line(&mut self, node: &lrp_nodes::Line) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Line(Line),
        });
    }

    fn on_lvar(&mut self, node: &lrp_nodes::Lvar) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Lvar(Lvar {
                name: node.name.clone(),
            }),
        });
    }

    fn on_lvasgn(&mut self, node: &lrp_nodes::Lvasgn) {
        let id = self.new_id();
        let value_id = self.visit_optional_single_node_child(node.value.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Lvasgn(Lvasgn {
                name: node.name.clone(),
                value_id,
                name_l: Loc::from(node.name_l),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_masgn(&mut self, node: &lrp_nodes::Masgn) {
        let id = self.new_id();
        let lhs_id = self.visit_single_node_child(&node.lhs);
        let rhs_id = self.visit_single_node_child(&node.rhs);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Masgn(Masgn {
                lhs_id,
                rhs_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_match_alt(&mut self, node: &lrp_nodes::MatchAlt) {
        let id = self.new_id();
        let lhs_id = self.visit_single_node_child(&node.lhs);
        let rhs_id = self.visit_single_node_child(&node.rhs);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchAlt(MatchAlt {
                lhs_id,
                rhs_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_match_as(&mut self, node: &lrp_nodes::MatchAs) {
        let id = self.new_id();
        let value_id = self.visit_single_node_child(&node.value);
        let as_id = self.visit_single_node_child(&node.as_);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchAs(MatchAs {
                value_id,
                as_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_match_current_line(&mut self, node: &lrp_nodes::MatchCurrentLine) {
        let id = self.new_id();
        let re_id = self.visit_single_node_child(&node.re);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchCurrentLine(MatchCurrentLine { re_id }),
        });
    }

    fn on_match_nil_pattern(&mut self, node: &lrp_nodes::MatchNilPattern) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchNilPattern(MatchNilPattern {
                operator_l: Loc::from(node.operator_l),
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_match_pattern(&mut self, node: &lrp_nodes::MatchPattern) {
        let id = self.new_id();
        let value_id = self.visit_single_node_child(&node.value);
        let pattern_id = self.visit_single_node_child(&node.pattern);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchPattern(MatchPattern {
                value_id,
                pattern_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_match_pattern_p(&mut self, node: &lrp_nodes::MatchPatternP) {
        let id = self.new_id();
        let value_id = self.visit_single_node_child(&node.value);
        let pattern_id = self.visit_single_node_child(&node.pattern);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchPatternP(MatchPatternP {
                value_id,
                pattern_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_match_rest(&mut self, node: &lrp_nodes::MatchRest) {
        let id = self.new_id();
        let name_id = self.visit_optional_single_node_child(node.name.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchRest(MatchRest {
                name: node.option_name_from_node(),
                name_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_match_var(&mut self, node: &lrp_nodes::MatchVar) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchVar(MatchVar {
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
            }),
        });
    }

    fn on_match_with_lvasgn(&mut self, node: &lrp_nodes::MatchWithLvasgn) {
        let id = self.new_id();
        let re_id = self.visit_single_node_child(&node.re);
        let value_id = self.visit_single_node_child(&node.value);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::MatchWithLvasgn(MatchWithLvasgn {
                re_id,
                value_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_mlhs(&mut self, node: &lrp_nodes::Mlhs) {
        let id = self.new_id();
        let item_ids = self.visit_vec_node_child(&node.items);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Mlhs(Mlhs {
                item_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_module(&mut self, node: &lrp_nodes::Module) {
        let id = self.new_id();
        let name_id = self.visit_single_node_child(&node.name);

        let name = node.name_from_node();
        self.namespace
            .push_owned(NamespaceNode::Module { name: name.clone() });

        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.namespace.pop();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Module(Module {
                name,
                name_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_next(&mut self, node: &lrp_nodes::Next) {
        let id = self.new_id();
        let arg_ids = self.visit_vec_node_child(&node.args);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Next(Next {
                arg_ids,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_nil(&mut self, node: &lrp_nodes::Nil) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Nil(Nil),
        });
    }

    fn on_nth_ref(&mut self, node: &lrp_nodes::NthRef) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::NthRef(NthRef {
                name: node.name.clone(),
            }),
        });
    }

    fn on_numblock(&mut self, node: &lrp_nodes::Numblock) {
        let id = self.new_id();
        let call_id = self.visit_single_node_child(&node.call);
        let body_id = self.visit_single_node_child(&node.body);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Numblock(Numblock {
                call_id,
                numargs: node.numargs,
                body_id,
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_op_asgn(&mut self, node: &lrp_nodes::OpAsgn) {
        let id = self.new_id();
        let recv_id = self.visit_single_node_child(&node.recv);
        let value_id = self.visit_single_node_child(&node.value);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::OpAsgn(OpAsgn {
                recv_id,
                operator: node.operator.clone(),
                value_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_optarg(&mut self, node: &lrp_nodes::Optarg) {
        let id = self.new_id();
        let default_id = self.visit_single_node_child(&node.default);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Optarg(Optarg {
                default_id,
                name: node.name.clone(),
                name_l: Loc::from(node.name_l),
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_or(&mut self, node: &lrp_nodes::Or) {
        let id = self.new_id();
        let lhs_id = self.visit_single_node_child(&node.lhs);
        let rhs_id = self.visit_single_node_child(&node.rhs);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Or(Or {
                lhs_id,
                rhs_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_or_asgn(&mut self, node: &lrp_nodes::OrAsgn) {
        let id = self.new_id();
        let recv_id = self.visit_single_node_child(&node.recv);
        let value_id = self.visit_single_node_child(&node.value);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::OrAsgn(OrAsgn {
                recv_id,
                value_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_pair(&mut self, node: &lrp_nodes::Pair) {
        let id = self.new_id();
        let key_id = self.visit_single_node_child(&node.key);
        let value_id = self.visit_single_node_child(&node.value);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Pair(Pair {
                key_id,
                value_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_pin(&mut self, node: &lrp_nodes::Pin) {
        let id = self.new_id();
        let var_id = self.visit_single_node_child(&node.var);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Pin(Pin {
                var_id,
                selector_l: Loc::from(node.selector_l),
            }),
        });
    }

    fn on_postexe(&mut self, node: &lrp_nodes::Postexe) {
        let id = self.new_id();
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Postexe(Postexe {
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_preexe(&mut self, node: &lrp_nodes::Preexe) {
        let id = self.new_id();
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Preexe(Preexe {
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_procarg0(&mut self, node: &lrp_nodes::Procarg0) {
        let id = self.new_id();
        let arg_ids = self.visit_vec_node_child(&node.args);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Procarg0(Procarg0 {
                arg_ids,
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_rational(&mut self, node: &lrp_nodes::Rational) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Rational(Rational {
                value: node.value.clone(),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_redo(&mut self, node: &lrp_nodes::Redo) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Redo(Redo),
        });
    }

    fn on_regexp(&mut self, node: &lrp_nodes::Regexp) {
        let id = self.new_id();
        let part_ids = self.visit_vec_node_child(&node.parts);
        let options_id = self.visit_optional_single_node_child(node.options.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Regexp(Regexp {
                part_ids,
                options_id,
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_reg_opt(&mut self, node: &lrp_nodes::RegOpt) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::RegOpt(RegOpt {
                options: node.options.clone(),
            }),
        });
    }

    fn on_rescue(&mut self, node: &lrp_nodes::Rescue) {
        let id = self.new_id();
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());
        let rescue_body_ids = self.visit_vec_node_child(&node.rescue_bodies);
        let else_id = self.visit_optional_single_node_child(node.else_.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Rescue(Rescue {
                body_id,
                rescue_body_ids,
                else_id,
                else_l: node.else_l.map(Loc::from),
            }),
        });
    }

    fn on_rescue_body(&mut self, node: &lrp_nodes::RescueBody) {
        let id = self.new_id();
        let exc_list_id = self.visit_optional_single_node_child(node.exc_list.as_deref());
        let exc_var_id = self.visit_optional_single_node_child(node.exc_var.as_deref());
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::RescueBody(RescueBody {
                exc_list_id,
                exc_var_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                assoc_l: node.assoc_l.map(Loc::from),
                begin_l: node.begin_l.map(Loc::from),
            }),
        });
    }

    fn on_restarg(&mut self, node: &lrp_nodes::Restarg) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Restarg(Restarg {
                name: node.name.clone(),
                operator_l: Loc::from(node.operator_l),
                name_l: node.name_l.map(Loc::from),
            }),
        });
    }

    fn on_retry(&mut self, node: &lrp_nodes::Retry) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Retry(Retry),
        });
    }

    fn on_return(&mut self, node: &lrp_nodes::Return) {
        let id = self.new_id();
        let arg_ids = self.visit_vec_node_child(&node.args);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Return(Return {
                arg_ids,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_s_class(&mut self, node: &lrp_nodes::SClass) {
        let id = self.new_id();
        let expr_id = self.visit_single_node_child(&node.expr);
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::SClass(SClass {
                expr_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                operator_l: Loc::from(node.operator_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_self_(&mut self, node: &lrp_nodes::Self_) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Self_(Self_),
        });
    }

    fn on_send(&mut self, node: &lrp_nodes::Send) {
        let id = self.new_id();
        let recv_id = self.visit_optional_single_node_child(node.recv.as_deref());
        let arg_ids = self.visit_vec_node_child(&node.args);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Send(Send {
                method_name: node.method_name.clone(),
                recv_id,
                arg_ids,
                dot_l: node.dot_l.map(Loc::from),
                selector_l: node.selector_l.map(Loc::from),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
                operator_l: node.operator_l.map(Loc::from),
            }),
        });
    }

    fn on_shadowarg(&mut self, node: &lrp_nodes::Shadowarg) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Shadowarg(Shadowarg {
                name: node.name.clone(),
            }),
        });
    }

    fn on_splat(&mut self, node: &lrp_nodes::Splat) {
        let id = self.new_id();
        let value_id = self.visit_optional_single_node_child(node.value.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Splat(Splat {
                value_id,
                operator_l: Loc::from(node.operator_l),
            }),
        });
    }

    fn on_str(&mut self, node: &lrp_nodes::Str) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Str(Str {
                value: node.value.clone().into_raw(),

                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_super(&mut self, node: &lrp_nodes::Super) {
        let id = self.new_id();
        let arg_ids = self.visit_vec_node_child(&node.args);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Super(Super {
                arg_ids,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_sym(&mut self, node: &lrp_nodes::Sym) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
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
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::True(True),
        });
    }

    fn on_undef(&mut self, node: &lrp_nodes::Undef) {
        let id = self.new_id();
        let name_ids = self.visit_vec_node_child(&node.names);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Undef(Undef {
                name_ids,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_unless_guard(&mut self, node: &lrp_nodes::UnlessGuard) {
        let id = self.new_id();
        let cond_id = self.visit_single_node_child(&node.cond);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::UnlessGuard(UnlessGuard {
                cond_id,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_until(&mut self, node: &lrp_nodes::Until) {
        let id = self.new_id();
        let cond_id = self.visit_single_node_child(&node.cond);
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Until(Until {
                cond_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_until_post(&mut self, node: &lrp_nodes::UntilPost) {
        let id = self.new_id();
        let cond_id = self.visit_single_node_child(&node.cond);
        let body_id = self.visit_single_node_child(&node.body);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::UntilPost(UntilPost {
                cond_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_when(&mut self, node: &lrp_nodes::When) {
        let id = self.new_id();
        let pattern_ids = self.visit_vec_node_child(&node.patterns);
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::When(When {
                pattern_ids,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: Loc::from(node.begin_l),
            }),
        });
    }

    fn on_while(&mut self, node: &lrp_nodes::While) {
        let id = self.new_id();
        let cond_id = self.visit_single_node_child(&node.cond);
        let body_id = self.visit_optional_single_node_child(node.body.as_deref());

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::While(While {
                cond_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_while_post(&mut self, node: &lrp_nodes::WhilePost) {
        let id = self.new_id();
        let cond_id = self.visit_single_node_child(&node.cond);
        let body_id = self.visit_single_node_child(&node.body);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::WhilePost(WhilePost {
                cond_id,
                body_id,
                keyword_l: Loc::from(node.keyword_l),
            }),
        });
    }

    fn on_x_heredoc(&mut self, node: &lrp_nodes::XHeredoc) {
        let id = self.new_id();
        let part_ids = self.visit_vec_node_child(&node.parts);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::XHeredoc(XHeredoc {
                part_ids,
                heredoc_body_l: Loc::from(node.heredoc_body_l),
                heredoc_end_l: Loc::from(node.heredoc_end_l),
            }),
        });
    }

    fn on_xstr(&mut self, node: &lrp_nodes::Xstr) {
        let id = self.new_id();
        let part_ids = self.visit_vec_node_child(&node.parts);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Xstr(Xstr {
                part_ids,
                begin_l: Loc::from(node.begin_l),
                end_l: Loc::from(node.end_l),
            }),
        });
    }

    fn on_yield(&mut self, node: &lrp_nodes::Yield) {
        let id = self.new_id();
        let arg_ids = self.visit_vec_node_child(&node.args);

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::Yield(Yield {
                arg_ids,
                keyword_l: Loc::from(node.keyword_l),
                begin_l: node.begin_l.map(Loc::from),
                end_l: node.end_l.map(Loc::from),
            }),
        });
    }

    fn on_z_super(&mut self, node: &lrp_nodes::ZSuper) {
        let id = self.new_id();

        self.nodes.push(Node {
            id,
            namespace: self.namespace.clone(),
            expression_l: Loc::from(node.expression_l),
            properties: Properties::ZSuper(ZSuper),
        });
    }
}

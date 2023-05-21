use std::path::Path;

use lib_ruby_parser::traverse::visitor::Visitor;
use ropey::Rope;

use crate::{node::Node, transformer};

/// Just a wrapper for calling `lib_ruby_parser`'s parse function.
///
pub(crate) fn lrp_parse(buffer_name: &Path, code: &Rope) -> lib_ruby_parser::ParserResult {
    let options = lib_ruby_parser::ParserOptions {
        buffer_name: buffer_name.to_string_lossy().to_string(),
        decoder: None,
        token_rewriter: None,
        record_tokens: false,
    };

    let parser = lib_ruby_parser::Parser::new(code.to_string(), options);

    parser.do_parse()
}

#[salsa::input]
pub struct NodeSource {
    #[return_ref]
    pub root_node: lib_ruby_parser::Node,

    #[return_ref]
    pub code: Rope,
}

/// Uses a `Transformer` to take the AST result of a `lib_ruby_parser::ParserResult` and converts
/// those `Node`s to our `Node`s.
///
#[salsa::tracked]
pub(crate) fn inner_transform(db: &dyn crate::Db, node_source: NodeSource) -> Vec<Node> {
    let root_node = node_source.root_node(db);
    let code = node_source.code(db);

    let mut transformer = transformer::Transformer::new(code);
    transformer.visit(root_node);

    transformer.into_nodes()
}

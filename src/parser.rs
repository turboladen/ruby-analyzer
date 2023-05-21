use std::path::{Path, PathBuf};

use lib_ruby_parser::traverse::visitor::Visitor;
use ropey::Rope;

use crate::{node::Node, transformer};

#[salsa::input]
pub struct FileSource {
    #[return_ref]
    file_uri: PathBuf,

    #[return_ref]
    code: Rope,
}

#[salsa::tracked]
pub struct FileAst {
    #[return_ref]
    file_uri: PathBuf,

    #[return_ref]
    nodes: Vec<Node>,
}

#[salsa::accumulator]
pub struct Diagnostics(lib_ruby_parser::Diagnostic);

#[salsa::tracked]
pub fn parse(db: &dyn crate::Db, file_source: FileSource) -> FileAst {
    let file_uri = file_source.file_uri(db);
    let code = file_source.code(db);

    let result = lrp_parse(file_uri, code);

    let nodes = if let Some(ref root_node) = result.ast {
        inner_transform(root_node, code)
    } else {
        vec![]
    };

    for diagnostic in result.diagnostics {
        Diagnostics::push(db, diagnostic);
    }

    FileAst::new(db, file_uri.clone(), nodes)
}

fn lrp_parse(buffer_name: &Path, code: &Rope) -> lib_ruby_parser::ParserResult {
    let options = lib_ruby_parser::ParserOptions {
        buffer_name: buffer_name.to_string_lossy().to_string(),
        decoder: None,
        token_rewriter: None,
        record_tokens: false,
    };

    let parser = lib_ruby_parser::Parser::new(code.to_string(), options);

    parser.do_parse()
}

fn inner_transform(root_node: &lib_ruby_parser::Node, code: &Rope) -> Vec<Node> {
    let mut transformer = transformer::Transformer::new(code);
    transformer.visit(root_node);
    transformer.into_nodes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_ruby_test() {
        let db = crate::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; end");

        let file_source = FileSource::new(&db, file_uri, code);

        let ast = parse(&db, file_source);
        let nodes = ast.nodes(&db);

        assert_eq!(2, nodes.len());

        let ast = parse(&db, file_source);
        let nodes = ast.nodes(&db);

        assert_eq!(2, nodes.len());
    }

    #[test]
    fn parse_invalid_ruby_test() {
        let db = crate::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; ");

        let file_source = FileSource::new(&db, file_uri, code);

        let ast = parse(&db, file_source);
        let nodes = ast.nodes(&db);
        assert!(nodes.is_empty());

        let diags = parse::accumulated::<Diagnostics>(&db, file_source);
        assert_eq!(diags.len(), 1);
    }
}

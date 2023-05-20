use lib_ruby_parser::{traverse::visitor::Visitor, Diagnostic};
use ropey::Rope;

use crate::{node::Node, transformer};

const DB_TABLE_NAME: &str = "parser_nodes";

pub async fn parse(file_uri: String, code: &Rope) -> Result<Vec<Diagnostic>, ()> {
    let result = lrp_parse(file_uri.clone(), code).await;

    if let Some(ref root_node) = result.ast {
        let nodes = inner_transform(root_node, code);

        // let mut statement = DB.query("BEGIN TRANSACTION");

        // for node in nodes {
        //     statement = statement
        //         .query("CREATE type::table($table) SET data = $data")
        //         .bind(("table", DB_TABLE_NAME))
        //         .bind(("node", node));
        // }

        // let response = statement.query("COMMIT TRANSACTION").await?;
    }

    Ok(result.diagnostics)
}

async fn lrp_parse(buffer_name: String, code: &Rope) -> lib_ruby_parser::ParserResult {
    let options = lib_ruby_parser::ParserOptions {
        buffer_name,
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

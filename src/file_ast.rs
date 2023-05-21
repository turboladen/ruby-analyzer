use std::path::PathBuf;

use crate::Node;

/// The output of `parse()`, it represents all of the parsed nodes in a file.
///
#[salsa::tracked]
pub struct FileAst {
    /// The file for which `nodes` represents.
    ///
    #[id]
    #[return_ref]
    pub file_uri: PathBuf,

    /// The parsed nodes in file `file_uri`.
    ///
    #[return_ref]
    pub nodes: Vec<Node>,
}

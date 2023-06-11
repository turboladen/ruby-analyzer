use ropey::Rope;
use tree_sitter::Node as TsNode;

use super::Diagnostic;

// TODO: This type probably isn't needed. In fact, I could probably just go ahead and turn
// `missing` things into `Diagnostic`s while building them, but I think it makes more sense to wait
// until I actually want to do something with those diagnostics--maybe that'll change altogether?
//
#[derive(Default)]
pub(super) struct IckyNodes<'a> {
    errors: Vec<Diagnostic<'a>>,
    missing: Vec<TsNode<'a>>,
}

impl<'a> IckyNodes<'a> {
    pub(super) fn new(errors: Vec<Diagnostic<'a>>, missing: Vec<TsNode<'a>>) -> Self {
        Self { errors, missing }
    }

    pub(super) fn into_diags(self, source: &'a Rope) -> Vec<Diagnostic<'a>> {
        let mut output = Vec::with_capacity(self.errors.len() + self.missing.len());

        for error in self.errors {
            output.push(error);
        }
        for missing in self.missing {
            output.push(Diagnostic::from_missing(missing, source));
        }

        output
    }
}

use ropey::Rope;
use tower_lsp::lsp_types;

use crate::ext_traits::GetCharRange;

/// Mirrors the code/text as it is represented in the client.
///
#[derive(Default, Debug, Clone)]
pub(crate) struct Document {
    pub(super) version: i32,
    pub(super) code: Rope,
}

impl Document {
    /// Accessor to the inner `Rope`.
    ///
    pub(crate) fn code(&self) -> &Rope {
        &self.code
    }

    /// Updates the `version` and `code` without checking to see if the given `version` is newer
    /// than `self`'s `version`.
    ///
    pub(super) fn replace_for_change_unchecked(&mut self, version: i32, code: &str) {
        self.version = version;
        self.code = Rope::from_str(code);
    }

    /// Updates the `version` without checking, then inserts the `new_text` into the inner `Rope`.
    ///
    pub(super) fn merge_for_change_unchecked(
        &mut self,
        version: i32,
        range: &lsp_types::Range,
        new_text: &str,
    ) {
        self.version = version;

        let char_range = self.code.get_char_range(range);
        let start = char_range.start;
        self.code.remove(char_range);
        self.code.insert(start, new_text);
    }
}

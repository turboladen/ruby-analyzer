//! This module provides extension traits for making it easier to work with external types that we
//! need to convert to other external types.
//!
use ropey::Rope;
use tower_lsp::lsp_types;
use tree_sitter::Point;

pub(crate) trait Endings {
    /// Intended solely to extend `Rope` functionality, this provides a way to get both the offset
    /// of the last byte of the `Rope`, and the correlating row/column (as a `tree_sitter::Point`).
    ///
    fn end_byte_and_point(&self) -> (usize, Point);
}

impl Endings for Rope {
    fn end_byte_and_point(&self) -> (usize, Point) {
        // The end/last byte is also the total number of bytes.
        let end_byte = self.len_bytes();
        // However, since an empty Rope will return 1 line here, we have to - 1.
        let end_row = self.len_lines() - 1;

        let byte_of_start_of_last_line = self.line_to_byte(end_row);
        let end_column = end_byte - byte_of_start_of_last_line;

        (end_byte, Point::new(end_row, end_column))
    }
}

pub(crate) trait GetCharRange {
    /// Intended to use with `Rope` and `lsp_types::Range`, with the goal of getting the range of
    /// `char`s represented by the `lsp_types::Range`. When editing/updating a `Rope`, you call
    /// `remove()` with the range of `char`s represented by the range of text that was changed.
    ///
    fn get_char_range(
        &self,
        lsp_range: &lsp_types::Range,
    ) -> Result<std::ops::Range<usize>, ropey::Error>;
}

impl GetCharRange for Rope {
    fn get_char_range(
        &self,
        lsp_range: &lsp_types::Range,
    ) -> Result<std::ops::Range<usize>, ropey::Error> {
        Ok(self.get_char_offset(&lsp_range.start)?..self.get_char_offset(&lsp_range.start)?)
    }
}

pub(crate) trait GetCharOffset {
    /// This is really just a compliment to `GetCharRange`, providing an ergonomic way to get both
    /// ends of the `char` range for that call.
    ///
    fn get_char_offset(&self, lsp_position: &lsp_types::Position) -> Result<usize, ropey::Error>;
}

impl GetCharOffset for Rope {
    fn get_char_offset(&self, lsp_position: &lsp_types::Position) -> Result<usize, ropey::Error> {
        Ok(
            self.try_line_to_char(usize::try_from(lsp_position.line).unwrap())?
                + usize::try_from(lsp_position.character).unwrap(),
        )
    }
}

/// Trait for converting `tree-sitter` types to types that we want.
///
pub(crate) trait FromTs<T> {
    fn from_ts(ts_type: T) -> Self;
}

impl FromTs<tree_sitter::Range> for lsp_types::Range {
    fn from_ts(ts_type: tree_sitter::Range) -> Self {
        lsp_types::Range {
            start: lsp_types::Position {
                line: u32::try_from(ts_type.start_point.row).unwrap(),
                character: u32::try_from(ts_type.start_point.column).unwrap(),
            },
            end: lsp_types::Position {
                line: u32::try_from(ts_type.end_point.row).unwrap(),
                character: u32::try_from(ts_type.end_point.column).unwrap(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod endings {
        use super::*;

        #[test]
        fn empty_test() {
            let rope = Rope::from_str("");
            assert_eq!(rope.len_lines(), 1);
            assert_eq!(rope.end_byte_and_point(), (0, Point::new(0, 0)));
        }

        #[test]
        fn single_line_test() {
            let rope = Rope::from_str("class Foo; end");
            assert_eq!(rope.len_lines(), 1);
            assert_eq!(rope.end_byte_and_point(), (14, Point::new(0, 14)));
        }

        #[test]
        fn multi_empty_line_test() {
            let rope = Rope::from_str("\n\n\n");
            assert_eq!(rope.len_lines(), 4);
            assert_eq!(rope.end_byte_and_point(), (3, Point::new(3, 0)));
        }
    } /* endings */

    mod get_char_offset {
        use super::*;

        #[test]
        fn function_name_test() {}
    } /* get_char_offset */
}

use ropey::Rope;
use tower_lsp::lsp_types;
use tree_sitter::Point;

pub(crate) trait Endings {
    fn end_byte_and_point(&self) -> (usize, Point);
}

impl Endings for Rope {
    fn end_byte_and_point(&self) -> (usize, Point) {
        let end_byte = self.len_bytes();

        // (this is also the number of the last row, - 1)
        let end_row = self.len_lines();

        let byte_of_start_of_last_line = self.line_to_byte(end_row);
        let end_column = end_byte - byte_of_start_of_last_line;

        (end_byte, Point::new(end_row, end_column))
    }
}

pub(crate) trait GetCharRange {
    fn get_char_range(&self, lsp_range: &lsp_types::Range) -> std::ops::Range<usize>;
}

impl GetCharRange for Rope {
    fn get_char_range(&self, lsp_range: &lsp_types::Range) -> std::ops::Range<usize> {
        self.get_char_offset(&lsp_range.start)..self.get_char_offset(&lsp_range.start)
    }
}

pub(crate) trait GetCharOffset {
    fn get_char_offset(&self, lsp_position: &lsp_types::Position) -> usize;
}

impl GetCharOffset for Rope {
    fn get_char_offset(&self, lsp_position: &lsp_types::Position) -> usize {
        self.line_to_char(usize::try_from(lsp_position.line).unwrap())
            + usize::try_from(lsp_position.character).unwrap()
    }
}

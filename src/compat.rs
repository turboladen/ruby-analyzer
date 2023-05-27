use lsp_types::Position;
use ropey::Rope;

pub trait OffsetToPosition {
    fn offset_to_position(&self, offset: usize) -> Position;
}

impl OffsetToPosition for Rope {
    fn offset_to_position(&self, offset: usize) -> Position {
        let line = self.char_to_line(offset);
        let first_char = self.line_to_char(line);
        let column = offset - first_char;

        Position::new(u32::try_from(line).unwrap(), u32::try_from(column).unwrap())
    }
}

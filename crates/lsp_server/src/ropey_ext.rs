use ropey::Rope;
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

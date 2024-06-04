extern crate alloc;

use alloc::string::String;

use crate::{print_force, stdout};

impl super::Editor {
    /// Clears and displayed the edit content buffer within the constrains of the top line, left column, and screen size.
    pub fn display_frame(&mut self) {
        let (memo_column, memo_line) = stdout!().cursor_position();

        // Print lines within frame
        for line_index in self.top_line..self.top_line+self.max_output_lines {
            // Print line if it exists or a blank
            if self.edit_buffer.get(line_index-1) != None {
                self.display_line(line_index);
            }
            else {
                stdout!().set_cursor_position(0, line_index - self.top_line + 1).unwrap();
                print_force!("{}", " ".repeat(self.max_output_columns));
            }
        }

        // Restore cursor position
        stdout!().set_cursor_position(memo_column, memo_line).unwrap();
    }

    pub fn display_line(&mut self, edit_line: usize) {
        let (memo_column, memo_line) = stdout!().cursor_position();
        
        // Clear the line
        stdout!().set_cursor_position(0, edit_line - self.top_line + 1).unwrap();
        print_force!("{}", " ".repeat(self.max_output_columns));

        // Print the line content
        stdout!().set_cursor_position(0, edit_line - self.top_line + 1).unwrap();
        print_force!(
            "{}",
            String::from_iter(
                self.edit_buffer.get(edit_line-1).unwrap()
                .iter()
                .skip(self.left_column-1)
                .take(self.max_output_columns)
                .map(|t| *t as char)
            )
        );

        // Restore cursor position
        stdout!().set_cursor_position(memo_column, memo_line).unwrap();
    }
}
use crate::stdout;

impl super::Editor {
    /// Moves the cursor up one line, returns `true` on success.
    pub fn move_cursor_up(&mut self) -> bool {
        // Cannot move up if on top line
        if self.line == 1 {
            return false;
        }

        // Move to the character directly above or the end of the line if not long enough
        let above_line_length = self.edit_buffer.get(self.line-2).unwrap().len();
        self.move_cursor(self.line-1, core::cmp::min(above_line_length, self.column));
        
        true
    }

    /// Moves the cursor down one line, returns `true` on success.
    pub fn move_cursor_down(&mut self) -> bool {
        // Cannot move down if on bottom line
        if self.line == self.edit_buffer.len() {
            return false;
        }

        // Move to the character directly below or the end of the line if not long enough
        let below_line_length = self.edit_buffer.get(self.line).unwrap().len();
        self.move_cursor(self.line+1, core::cmp::min(below_line_length, self.column));
        
        true
    }

    /// Moves the cursor left one column, wrapping to the previous line if needed, returns `true` on success.
    pub fn move_cursor_left(&mut self) -> bool {
        // Cannot move left from first character
        if self.line == 1 && self.column == 1 {
            return false;
        }

        if self.column == 1 {
            // Scroll to end of previous line
            self.move_cursor(self.line-1, self.edit_buffer.get(self.line-2).unwrap().len());
        }
        else {
            // Scroll to previous character in line
            self.move_cursor(self.line, self.column-1);
        }

        true
    }

    /// Moves the cursor right one column, wrapping to the next line if needed, returns `true` on success.
    pub fn move_cursor_right(&mut self) -> bool {
        // Cannot move right from last character
        if self.line == self.edit_buffer.len() && self.column == self.edit_buffer.last().unwrap().len() {
            return false;
        }

        if self.column == self.edit_buffer.get(self.line-1).unwrap().len() {
            // Scroll to start of next line
            self.move_cursor(self.line+1, 1);
        }
        else {
            // Scroll to next character in line
            self.move_cursor(self.line, self.column+1);
        }

        true
    }

    /// Moves the cursor to the beginning of the current line.
    pub fn move_cursor_to_home(&mut self) {
        self.column = 1;
        self.left_column = 1;

        stdout!().set_cursor_position(0, self.line-self.top_line+1).unwrap();
        self.display_frame();
    }

    /// Moves the cursor to the end of the current line.
    pub fn move_cursor_to_end(&mut self) {
        self.column = self.edit_buffer.get(self.line-1).unwrap().len();
        if self.column > self.max_output_columns {
            // Scroll if needed
            self.left_column = self.column - self.max_output_columns + 1;
        }

        stdout!().set_cursor_position(self.column-self.left_column, self.line-self.top_line+1).unwrap();
        self.display_frame();
    }

    /// Moves the cursor to the given coordinates of the editor, regardless of if text exists there.
    pub fn move_cursor(&mut self, dest_line: usize, dest_column: usize) {
        if !self.is_line_within_frame(dest_line) {
            // Anchor the top line to the destination if possible or destination-max_output_lines (i.e. bottom line to destination)
            self.top_line = dest_line - if self.top_line > dest_line { 0 } else { self.max_output_lines };
            self.display_frame();
        }
        if !self.is_column_within_frame(dest_column) {
            // Anchor the left column to the destination if possible or destination-max_output_columns (i.e. right column to destination)
            self.left_column = dest_column - if self.left_column > dest_column { 0 } else { self.max_output_columns - 1 };
            self.display_frame();
        }

        // Update line, column, and cursor position
        self.line = dest_line;
        self.column = dest_column;
        stdout!().set_cursor_position(
            self.column - self.left_column,
            self.line - self.top_line + 1
        ).unwrap();
    }

    /// Checks if the given line is currently displayed on the frame.
    fn is_line_within_frame(&self, line: usize) -> bool {
        self.top_line <= line && line < self.top_line + self.max_output_lines
    }

    /// Checks if the given column is currently displayed on the frame.
    fn is_column_within_frame(&self, column: usize) -> bool {
        self.left_column <= column && column < self.left_column + self.max_output_columns
    }
}
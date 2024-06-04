mod banner;
mod cursor;
mod display;
mod edit;

extern crate alloc;

use alloc::vec::Vec;

use crate::current_output_mode;

/// An instance of a text editor.
pub struct Editor {
    /// The name of the file being edited.
    filename: &'static str,
    /// A 2D array containing the file contents by coordinates, this is directly edited.
    edit_buffer: Vec<Vec<u8>>,
    /// An array containing the contents to be written to the file, created from the edit buffer.
    file_buffer: Vec<u8>,
    /// The current line of the file the cursor is on.
    line: usize,
    /// The current column of the file the cursor is on.
    column: usize,
    /// The number of lines that can be displayed at once.
    max_output_lines: usize,
    /// The number of columns that can be displayed at once.
    max_output_columns: usize,
    /// The number of the line at the top of the editor.
    top_line: usize,
    /// The number of the column at the left of the editor.
    left_column: usize,
    /// Flag for unsaved changes to the edit buffer.
    unsaved_changes: bool
}

impl Editor {
    pub fn new(filename: &'static str, buf: &[u8]) -> Self {
        let mut ret = Editor {
            filename: filename,
            edit_buffer: Vec::new(),
            file_buffer: Vec::new(),
            line: 1,
            column: 1,
            max_output_lines: current_output_mode!().rows() - 3,
            max_output_columns: current_output_mode!().columns(),
            top_line: 1,
            left_column: 1,
            unsaved_changes: false
        };

        let lines = buf.split_inclusive(|chr| *chr == b'\n');
        for line in lines {
            ret.edit_buffer.push(Vec::from(line));
        }

        ret.save_edit_buffer();

        ret
    }

    /// Saves the current edit buffer to another buffer that can be exported.
    pub fn save_edit_buffer(&mut self) {
        self.file_buffer = Vec::new();
        for line in self.edit_buffer.iter() {
            self.file_buffer.append(&mut line.clone());
        }
    }
}
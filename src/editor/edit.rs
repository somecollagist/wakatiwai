extern crate alloc;

use alloc::vec::Vec;

use uefi::proto::console::text::{Key, ScanCode};
use uefi::ResultExt;

use crate::{boot_services, stdin, stdout};

impl super::Editor {
    pub fn edit(&mut self) -> &Vec<u8> {
        // Prepare visual
        stdout!().clear().unwrap();
        self.draw_top_banner();
        self.draw_bottom_banner();
        self.display_frame();
        self.move_cursor(1, 1);
        stdout!().enable_cursor(true).unwrap();

        loop {
            // Loop for a key press
            stdin!().reset(false).unwrap();
            boot_services!()
                .wait_for_event(
                    [
                        stdin!().wait_for_key_event().unwrap()
                    ].as_mut()
                )
                .discard_errdata()
                .unwrap();

            match stdin!().read_key().unwrap() {
                // Move Cursor up
                Some(Key::Special(ScanCode::UP)) => {
                    self.move_cursor_up();
                    // For some reason, this can't be in the move_cursor method?
                    self.draw_bottom_banner();
                }
                // Move Cursor down
                Some(Key::Special(ScanCode::DOWN)) => {
                   self.move_cursor_down();
                   // For some reason, this can't be in the move_cursor method?
                   self.draw_bottom_banner();
                }
                // Move Cursor left
                Some(Key::Special(ScanCode::LEFT)) => {
                    self.move_cursor_left();
                    // For some reason, this can't be in the move_cursor method?
                    self.draw_bottom_banner();
                }
                // Move Cursor right
                Some(Key::Special(ScanCode::RIGHT)) => {
                    self.move_cursor_right();
                    // For some reason, this can't be in the move_cursor method?
                    self.draw_bottom_banner();
                }
                // Home key
                Some(Key::Special(ScanCode::HOME)) => {
                    self.move_cursor_to_home();
                    // For some reason, this can't be in the move_cursor method?
                    self.draw_bottom_banner();
                }
                // End key
                Some(Key::Special(ScanCode::END)) => {
                    self.move_cursor_to_end();
                    // For some reason, this can't be in the move_cursor method?
                    self.draw_bottom_banner();
                }

                // Save changes
                Some(Key::Special(ScanCode::FUNCTION_1)) => {
                    self.file_buffer = self.edit_buffer.concat();
                    self.unsaved_changes = false;
                    self.draw_top_banner();
                }
                // Exit editor
                Some(Key::Special(ScanCode::FUNCTION_2)) => {
                    return &self.file_buffer;
                }

                // Delete key
                Some(Key::Special(ScanCode::DELETE)) => {
                    // Cannot delete anything from an empty buffer
                    if self.edit_buffer.len() == 0 {
                        continue;
                    }

                    let mut requires_frame_redraw = false;
                    let mut line = self.edit_buffer.get_mut(self.line-1).unwrap().clone();

                    // Is the user attempting to remove last character of the line?
                    if self.column == line.len() {
                        // If not a newline or on the last line (should be identical anyway), then ignore input
                        if *line.last().unwrap() != b'\n' || self.line == self.edit_buffer.len() {
                            continue;
                        }

                        // Remove the next line and append it to the current one (frame needs redrawing)
                        let mut next_line = self.edit_buffer.remove(self.line);
                        line.append(&mut next_line);
                        requires_frame_redraw = true;
                    }

                    // Remove character at cursor position
                    line.remove(self.column-1);
                    self.edit_buffer.get_mut(self.line-1).unwrap().clear();
                    self.edit_buffer.get_mut(self.line-1).unwrap().append(&mut line);
                    
                    // Mark for unsaved changes
                    if !self.unsaved_changes {
                        self.unsaved_changes = true;
                        self.draw_top_banner();
                    }

                    if requires_frame_redraw {
                        self.display_frame();
                    }
                    else {
                        self.display_line(self.line);
                    }
                }
                // Backspace
                Some(Key::Printable(backspace)) if u16::from(backspace) == 0x0008 => {
                    let mut requires_frame_redraw = false;

                    if self.column == 1 {
                        if self.line == 1 {
                            // Cannot backspace from an empty edit buffer
                            continue;
                        }

                        // Remove newline from previous line and append current
                        let mut line = self.edit_buffer.remove(self.line-1);
                        let previous_line = self.edit_buffer.get_mut(self.line-2).unwrap();

                        previous_line.pop(); // Remove the newline
                        previous_line.append(&mut line);
                        requires_frame_redraw = true;
                    }
                    else {
                        // Remove the character behind the cursor
                        let line = self.edit_buffer.get_mut(self.line-1).unwrap();
                        line.remove(self.column-2);
                    }

                    // Mark for unsaved changes
                    if !self.unsaved_changes {
                        self.unsaved_changes = true;
                        self.draw_top_banner();
                    }

                    if requires_frame_redraw {
                        self.display_frame();
                    }
                    else {
                        self.display_line(self.line);
                    }
                    // Move the cursor backwards
                    self.move_cursor_left();
                }

                // Writable key
                Some(Key::Printable(key)) => {
                    // Mark for unsaved changes
                    if !self.unsaved_changes {
                        self.unsaved_changes = true;
                        self.draw_top_banner();
                    }

                    // If the edit buffer is empty, give it something
                    if self.edit_buffer.len() == 0 {
                        self.edit_buffer.push(Vec::new());
                    }

                    match u16::from(key) as u8 as char {
                        '\r' => {
                            // Newline
                            
                            // Get the current line and divide into two parts, remove the current line from the grid
                            let origin_line = self.edit_buffer.get(self.line-1).unwrap().clone();
                            let split_line = origin_line.split_at(self.column-1);
                            self.edit_buffer.remove(self.line-1);

                            // Cast the parts of the line into vectors and inject a newline
                            let (mut first_line, second_line) = (
                                Vec::from(split_line.0),
                                Vec::from(split_line.1)
                            );
                            first_line.push(b'\n');
                            
                            // Insert the above vectors
                            self.edit_buffer.insert(self.line-1, first_line);
                            self.edit_buffer.insert(self.line, second_line);

                            // Move cursor and render changes
                            self.move_cursor_right();
                            self.display_frame();
                        }
                        _ => {
                            // Insert character, move cursor, render changes
                            self.edit_buffer.get_mut(self.line-1).unwrap().insert(self.column-1, u16::from(key) as u8);
                            self.move_cursor_right();
                            self.display_line(self.line);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
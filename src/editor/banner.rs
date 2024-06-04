extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

use uefi::{proto::console::text::Color, CStr16};

use crate::*;

impl super::Editor {
    /// Draws a horizontal banner on a given row containing the given message.
    fn draw_banner(&mut self, row: usize, message: String) -> Result<(), ()> {
        // If the row is beyond the scope of the mode, fail
        /*
            We include an extra -1 here to avoid printing banners on the last line of the
            screen. If this were done, the cursor would wrap to the next line and force
            a scroll down, which messes everything up. Therefore, you must consider the
            last line of the screen to be reserved for the editor.
        */
        if row >= current_output_mode!().rows()-1 {
            return Err(());
        }

        // Go to the beginning of the banner's line and prepare to draw the banner
        stdout!().set_cursor_position(0, row).unwrap();
        stdout!().set_color(Color::White, Color::LightGray).unwrap();

        // Print out the banner message and spaces until the banner is filled
        let mut msg = message;
        msg.push_str(&" ".repeat(self.max_output_columns-msg.len()));
        msg.push_str("\0");
        stdout!().output_string(
            CStr16::from_u16_with_nul(
                &msg.chars()
                    .into_iter()
                    .map(|t| t as u16)
                    .collect::<Vec<u16>>()
            ).unwrap()
        ).unwrap();

        // Restore cursor position and colour
        stdout!().set_color(Color::LightGray, Color::Black).unwrap();
        self.move_cursor(self.line, self.column);

        Ok(())
    }

    /// Draws the top banner of the editor, containing the file name and the current edit status.
    pub fn draw_top_banner(&mut self) {
        let mut top_banner_content = String::from(" ");

        // Filename
        top_banner_content.push_str(self.filename);

        // Unsaved changes marker
        top_banner_content.push_str( if self.unsaved_changes { " * " } else { "" } );

        self.draw_banner(0, top_banner_content).unwrap();
    }

    /// Draws the bottom banner of the editor, containing the manual and the current cursor position.
    pub fn draw_bottom_banner(&mut self) {
        let mut bottom_banner_content = String::from(" ");
        
        // Manual
        bottom_banner_content.push_str("F1: Save - F2: Exit ");

        // Current cursor position
        bottom_banner_content.push_str("(");
        bottom_banner_content.push_str(&self.line.to_string());
        bottom_banner_content.push_str(",");
        bottom_banner_content.push_str(&self.column.to_string());
        bottom_banner_content.push_str(")");

        self.draw_banner(current_output_mode!().rows()-2,bottom_banner_content).unwrap();
    }
}
extern crate alloc;

use alloc::vec::Vec;

use dev::DISK_GUID_HANDLE_MAPPING;
use uefi::proto::console::text::{Color, Key, ScanCode};
use uefi::CStr16;
use wtcore::get_unix_time;

use crate::*;
use crate::wtcore::config::BootEntry;

/// Options that can be selected by the boot menu.
#[derive(Clone)]
pub enum MenuOption {
    /// A bootable option, backed by a boot entry.
    BootOption(BootEntry),
    /// Option to exit the bootloader.
    Exit,
    /// Option to edit the bootloader configuration file.
    EditConfig,
    /// Option to reboot the computer
    Reboot,
    /// Option to power off the computer
    Poweroff
}

impl MenuOption {
    #[doc(hidden)]
    const EXIT_LABEL: &'static str = "Exit";
    #[doc(hidden)]
    const EDIT_CONFIG_LABEL: &'static str = "Edit Bootloader Config";
}

/// A structure describing the boot menu displayed to the user.
#[derive(Default)]
pub struct BootMenu {
    menu_options: Vec<MenuOption>,
    current_menu_option_index: usize,
    anchor_start: (usize, usize),
    anchor_end: (usize, usize)
}

impl BootMenu {
    /// Returns a selected option from the boot menu.
    pub fn select_option() -> MenuOption {
        let mut menu = BootMenu::default();
        let config = CONFIG.read();

        menu.init();
        let mut focused_option: &MenuOption;
        if config.timeout == 0 {
            // May only instant boot to a boot option
            if let MenuOption::BootOption(entry) = menu.focus_option(0).unwrap() {
                return MenuOption::BootOption(entry.clone());
            }
            // Theoretically, this should never happen:
            eprintln!("Instant boot did not point to a boot entry");
        }

        // Timeout markers
        let mut input_given = false;
        let target_time = get_unix_time() + config.timeout as i64;
        if config.timeout < 0 {
            // Negative timeout implies wait for user input
            input_given = true;
        }

        // Use a locally-scoped variable to avoid confusing focus_option
        let mut idx = 0;
        focused_option = menu.focus_option(0).unwrap();
        loop {
            if !input_given {
                if target_time <= get_unix_time() {
                    return focused_option.clone()
                }
            }

            match stdin!().read_key().unwrap() {
                // Select the previous entry if possible
                Some(Key::Special(ScanCode::UP)) => {
                    input_given = true;
                    if idx > 0 {
                        idx -= 1;
                    }
                    focused_option = menu.focus_option(idx).unwrap();
                }
                // Select the next entry if possible
                Some(Key::Special(ScanCode::DOWN)) => {
                    input_given = true;
                    if idx < menu.menu_options.len()-1 {
                        idx += 1;
                    }
                    focused_option = menu.focus_option(idx).unwrap();
                }
                // Reboot if the F5 Key is pressed
                Some(Key::Special(ScanCode::FUNCTION_5)) => {
                    return MenuOption::Reboot;
                }
                // Power off if the F12 Key is pressed
                Some(Key::Special(ScanCode::FUNCTION_12)) => {
                    return MenuOption::Poweroff;
                }
                // Boot the given entry
                Some(Key::Printable(key)) => {
                    match u16::from(key) as u8 {
                        b' ' | b'\r' => {
                            return focused_option.clone();
                        },
                        _ => {
                            input_given = true;
                        }
                    }
                }
                None => {}
                _ => {
                    input_given = true;
                }
            }
        }
    }

    /// Draws and initialises the boot menu.
    fn init(&mut self) {
        let config = CONFIG.read();

        // Clear menu if told to do so
        if config.menu_clear {
            stdout!().clear().unwrap();
        }

        println_force!(" Wakatiwai Bootloader |");
        println_force!("=#====================|");
        for entry in &config.boot_entries {
            if entry.removable {
                if !DISK_GUID_HANDLE_MAPPING.contains_key(&entry.disk_guid) {
                    continue;
                }
            }
            println_force!(" #-> {}", entry.name);
            self.menu_options.push(MenuOption::BootOption(entry.clone()));
        }
        if config.exit {
            println_force!(" #-! {}", MenuOption::EXIT_LABEL);
            self.menu_options.push(MenuOption::Exit);
        }
        if config.edit_config {
            println_force!(" #-@ {}", MenuOption::EDIT_CONFIG_LABEL);
            self.menu_options.push(MenuOption::EditConfig);
        }
        
        // Set anchor_start after anchor_end since the menu might cause the screen to scroll - this ensures validity
        self.anchor_end = stdout!().cursor_position();
        self.anchor_start = (self.anchor_end.0, self.anchor_end.1 - self.menu_options.len() - 2);
        self.current_menu_option_index = 0;
    }

    /// Focuses a given menu option.
    fn focus_option(&mut self, index: usize) -> Option<&MenuOption> {
        // Check if index is legal
        if index >= self.menu_options.len() {
            return None;
        }

        // Remove highlight on currently focused option
        self.colour_menu_option(self.current_menu_option_index, Color::LightGray);
        // Add highlight to specified option
        self.colour_menu_option(index, Color::White);
        // Update focused option index
        self.current_menu_option_index = index;

        // Return the newly focused option
        self.menu_options.get(self.current_menu_option_index)
    }

    /// Gets the coordinates of the label of a given menu option.
    fn get_menu_option_coordinates(&self, index: usize) -> Option<(usize, usize)> {
        // Check if index is legal
        if index >= self.menu_options.len() {
            return None;
        }

        /* 
            Text starts five characters in: #-? ... 
            |-----------------------------------^ here!
            The line is offset by 2 to account for the bootloader name
            and the box around it, then offset by the index to get the
            correct line
        */ 
        Some((self.anchor_start.0 + 5, self.anchor_start.1 + 2 + index))
    }

    /// Sets the foreground of a menu option
    /// 
    /// Returns `true` if operation failed, otherwise `false`.
    fn colour_menu_option(&self, index: usize, foreground: Color) -> bool {
        // Check if index is legal
        if index >= self.menu_options.len() {
            return true;
        }

        // Get coordinates of the menu option
        let target = match self.get_menu_option_coordinates(index) {
            Some(some) => some,
            None => return true
        };
        // Get the text of the menu option
        let option_text = match self.menu_options.get(index).unwrap() {
            MenuOption::BootOption(entry) => &entry.name,
            MenuOption::Exit => MenuOption::EXIT_LABEL,
            MenuOption::EditConfig => MenuOption::EDIT_CONFIG_LABEL,
            _ => unreachable!()
        };

        // Overwrite the menu option's label in a new colour
        stdout!().set_cursor_position(target.0, target.1).unwrap();
        stdout!().set_color(foreground, Color::Black).unwrap();
        stdout!().output_string(
            // Needs to write a CStr16, so do a quick allocation
            CStr16::from_str_with_buf(
                option_text,
                &mut [0 as u16; BootEntry::MAX_NAME_LENGTH + 1]
            ).unwrap()
        ).unwrap();
        // Move cursor back to the end otherwise your output looks weird
        stdout!().set_cursor_position(self.anchor_end.0, self.anchor_end.1).unwrap();

        false
    }
}
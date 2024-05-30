extern crate alloc;

use alloc::vec::Vec;

use uefi::proto::console::text::{Color, Key, ScanCode};
use uefi::CStr16;

use crate::*;
use crate::wtcore::config::BootEntry;

/// Options that can be selected by the boot menu.
#[derive(Clone)]
pub enum MenuOption {
    /// A bootable option, backed by a boot entry.
    BootOption(BootEntry),
    /// Option to enter the UEFI shell.
    UEFIShell,
    /// Option to edit the bootloader configuration file.
    EditConfig,
}

impl MenuOption {
    #[doc(hidden)]
    const UEFI_SHELL_LABEL: &'static str = "UEFI Shell";
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
        if config.instant_boot {
            // May only instant boot to a boot option
            if let MenuOption::BootOption(entry) = menu.focus_option(0).unwrap() {
                return MenuOption::BootOption(entry.clone());
            }
            // Theoretically, this should never happen:
            eprintln!("Instant boot did not point to a boot entry");
        }

        // Use a locally-scoped variable to avoid confusing focus_option
        let mut idx = 0;
        loop {
            // Use focus_option here to highlight the option
            focused_option = menu.focus_option(idx).unwrap();

            // Loop for a key press
            system_table!().stdin().reset(false).unwrap();
            boot_services!()
                .wait_for_event(
                    [
                        system_table!().stdin().wait_for_key_event().unwrap()
                    ].as_mut()
                )
                .discard_errdata()
                .unwrap();

            match system_table!().stdin().read_key().unwrap() {
                // Select the previous entry if possible
                Some(Key::Special(ScanCode::UP)) => {
                    if idx > 0 {
                        idx -= 1;
                    }
                }
                // Select the next entry if possible
                Some(Key::Special(ScanCode::DOWN)) => {
                    if idx < menu.menu_options.len()-1 {
                        idx += 1;
                    }
                }
                // Boot the given entry
                Some(Key::Printable(key)) => {
                    match u16::from(key) {
                        0x0020 |    // Space
                        0x000D      // Enter (technically CR)
                        => {
                            return focused_option.clone();
                        },
                        _ => {}
                    }
                    
                }
                _ => {}
            }
        }
    }

    /// Draws and initialises the boot menu.
    fn init(&mut self) {
        let config = CONFIG.read();

        // Clear menu if told to do so
        if config.menu_clear {
            system_table!().stdout().clear().unwrap();
        }

        println_force!(" Wakatiwai Bootloader ");
        println_force!("=#====================");
        for entry in &config.boot_entries {
            println_force!(" #-> {}", entry.name);
            self.menu_options.push(MenuOption::BootOption(entry.clone()));
        }
        if config.offer_shell {
            println_force!(" #-$ {}", MenuOption::UEFI_SHELL_LABEL);
            self.menu_options.push(MenuOption::UEFIShell);
        }
        if config.edit_config {
            println_force!(" #-@ {}", MenuOption::EDIT_CONFIG_LABEL);
            self.menu_options.push(MenuOption::EditConfig);
        }
        
        // Set anchor_start after anchor_end since the menu might cause the screen to scroll - this ensures validity
        self.anchor_end = system_table!().stdout().cursor_position();
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
            MenuOption::UEFIShell => MenuOption::UEFI_SHELL_LABEL,
            MenuOption::EditConfig => MenuOption::EDIT_CONFIG_LABEL
        };

        // Overwrite the menu option's label in a new colour
        system_table!().stdout().set_cursor_position(target.0, target.1).unwrap();
        system_table!().stdout().set_color(foreground, Color::Black).unwrap();
        system_table!().stdout().output_string(
            // Needs to write a CStr16, so do a quick allocation
            CStr16::from_str_with_buf(
                option_text,
                &mut [0 as u16; BootEntry::MAX_NAME_LENGTH + 1]
            ).unwrap()
        ).unwrap();
        // Move cursor back to the end otherwise your output looks weird
        system_table!().stdout().set_cursor_position(self.anchor_end.0, self.anchor_end.1).unwrap();

        false
    }
}

// /// Coordinates of the top-left of the menu.
// static ANCHOR_START: Once<(usize, usize)> = Once::new();
// /// Coordinates of the bottom-left of the menu.
// static ANCHOR_END: Once<(usize, usize)> = Once::new();

// // The 
// static MENU_OPTIONS: RwLock<Vec<String>> = RwLock::new(Vec::new());
// /// The option currently focused on the boot menu.
// static mut MENU_OPTION_INDEX: usize = 0;

// pub fn display_menu() -> Result<(), Status> {
//     draw_menu();

//     let config = CONFIG.read();
//     if config.instant_boot {
//         let entry = match select_menu_option(0).unwrap() {
//             MenuOption::BootOption(entry) => entry,
//             _ => {
//                 eprintln!("Instant boot was not a boot entry");
//                 return Err(Status::ABORTED);
//             }
//         };
//         if let Some(boot_error) = crate::wtcore::boot::attempt_boot(&entry) {
//             eprintln!("Unable to boot \"{}\": {:?}", entry.name, boot_error);
//             return Err(Status::ABORTED);
//         }
//     }

//     let menu_options = MENU_OPTIONS.read();
//     let mut menu_option_index = 0;
//     #[allow(unused_assignments)] // They are used??
//     let mut menu_option = select_menu_option(0).unwrap();
//     loop {
//         menu_option = select_menu_option(menu_option_index).unwrap_or_else(|| {
//             select_menu_option(unsafe { MENU_OPTION_INDEX }).unwrap()
//         });

//         system_table!().stdin().reset(false).unwrap();
//         boot_services!()
//             .wait_for_event(
//                 [uefi::helpers::system_table()
//                     .stdin()
//                     .wait_for_key_event()
//                     .unwrap()]
//                 .as_mut(),
//             )
//             .discard_errdata()
//             .unwrap();

//         match system_table!().stdin().read_key().unwrap() {
//             Some(Key::Special(ScanCode::UP)) => {
//                 if menu_option_index > 0 {
//                     menu_option_index -= 1;
//                 }
//             }
//             Some(Key::Special(ScanCode::DOWN)) => {
//                 if menu_option_index < menu_options.len() - 1 {
//                     menu_option_index += 1;
//                 }
//             }
//             Some(Key::Printable(key)) if key == Char16::try_from('\r').unwrap() => {
//                 match menu_option {
//                     MenuOption::BootOption(entry) => {
//                         if let Some(boot_error) = crate::wtcore::boot::attempt_boot(&entry) {
//                             eprintln!("Unable to boot \"{}\": {:?}", entry.name, boot_error);
//                             return Err(Status::ABORTED);
//                         }
//                     }
//                     MenuOption::UEFIShell => {
//                         return Ok(());
//                     }
//                     MenuOption::EditConfig => {
//                         todo!();
//                     }
//                 }
//             }
//             _ => {
//                 continue;
//             }
//         }
//     }
// }

// fn select_menu_option(index: usize) -> Option<MenuOption> {
//     if get_index_cursor(index).is_err() {
//         return None;
//     }

//     let config = CONFIG.read();
//     let menu_options = MENU_OPTIONS.read();

//     colour_menu_option(
//         unsafe { MENU_OPTION_INDEX },
//         Color::LightGray,
//         Color::Black
//     );
//     colour_menu_option(index, Color::White, Color::Black);
//     system_table!()
//         .stdout()
//         .set_color(Color::LightGray, Color::Black)
//         .unwrap();

//     unsafe {
//         MENU_OPTION_INDEX = index;
//     }

//     // Last two options might indicate either Shell or Config
//     if index == menu_options.len() - 1 {
//         if config.edit_config {
//             return Some(MenuOption::EditConfig);
//         } else if config.offer_shell {
//             return Some(MenuOption::UEFIShell);
//         }
//     } else if index == menu_options.len() - 2 && config.edit_config && config.offer_shell {
//         return Some(MenuOption::UEFIShell);
//     }

//     return Some(MenuOption::BootOption(
//         (*config.boot_entries.get(index).unwrap()).clone(),
//     ));
// }

// fn colour_menu_option(index: usize, foreground: Color, background: Color) -> Option<()> {
//     let menu_options = MENU_OPTIONS.read();
//     let cursor_target = match get_index_cursor(index) {
//         Ok(ok) => ok,
//         Err(_) => {
//             return None;
//         }
//     };

//     system_table!()
//         .stdout()
//         .set_cursor_position(cursor_target.0, cursor_target.1)
//         .unwrap();
//     system_table!()
//         .stdout()
//         .set_color(foreground, background)
//         .unwrap();
//     system_table!()
//         .stdout()
//         .output_string(
//             CStr16::from_str_with_buf(
//                 menu_options.get(index).unwrap(),
//                 &mut [0 as u16; BootEntry::MAX_NAME_LENGTH + 1],
//             )
//             .unwrap(),
//         )
//         .unwrap();

//     let anchor_end = ANCHOR_END.get().unwrap();
//     system_table!()
//         .stdout()
//         .set_cursor_position(anchor_end.0, anchor_end.1)
//         .unwrap();

//     Some(())
// }

// fn get_index_cursor(index: usize) -> Result<(usize, usize), ()> {
//     let menu_options = MENU_OPTIONS.read();

//     if index >= menu_options.len() {
//         return Err(());
//     }

//     let mut pos = ANCHOR_START.get().unwrap().clone();
//     pos.0 += 5;
//     pos.1 += 2; // Options start 2 lines below anchor
//     pos.1 += index; // Offset line down by index

//     Ok(pos)
// }

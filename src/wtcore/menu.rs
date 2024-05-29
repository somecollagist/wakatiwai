extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use spin::Once;
use spin::RwLock;
use uefi::proto::console::text::{Color, Key, ScanCode};
use uefi::CStr16;
use uefi::Char16;

use crate::*;

enum MenuOption {
    BootOption(BootEntry),
    UEFIShell,
    EditConfig,
}

static ANCHOR_START: Once<(usize, usize)> = Once::new();
static ANCHOR_END: Once<(usize, usize)> = Once::new();

static mut CURRENT_OPTION_INDEX: usize = 0;
static MENU_OPTIONS: RwLock<Vec<String>> = RwLock::new(Vec::new());

pub fn display_menu() -> Result<(), Status> {
    draw_menu();

    let config = CONFIG.read();
    if config.is_instant_boot() {
        let entry = match select_menu_option(0).unwrap() {
            MenuOption::BootOption(entry) => entry,
            _ => {
                eprintln!("Instant boot was not a boot entry");
                return Err(Status::ABORTED);
            }
        };
        if let Some(boot_error) = crate::wtcore::boot::attempt_boot(&entry) {
            eprintln!("Unable to boot \"{}\": {:?}", entry.name, boot_error);
            return Err(Status::ABORTED);
        }
    }

    let menu_options = MENU_OPTIONS.read();
    let mut menu_option_index = 0;
    #[allow(unused_assignments)] // They are used??
    let mut menu_option = select_menu_option(0).unwrap();
    loop {
        menu_option = select_menu_option(menu_option_index).unwrap_or_else(|| {
            select_menu_option(unsafe { CURRENT_OPTION_INDEX }).unwrap()
        });

        system_table!().stdin().reset(false).unwrap();
        boot_services!()
            .wait_for_event(
                [uefi::helpers::system_table()
                    .stdin()
                    .wait_for_key_event()
                    .unwrap()]
                .as_mut(),
            )
            .discard_errdata()
            .unwrap();

        match system_table!().stdin().read_key().unwrap() {
            Some(Key::Special(ScanCode::UP)) => {
                if menu_option_index > 0 {
                    menu_option_index -= 1;
                }
            }
            Some(Key::Special(ScanCode::DOWN)) => {
                if menu_option_index < menu_options.len() - 1 {
                    menu_option_index += 1;
                }
            }
            Some(Key::Printable(key)) if key == Char16::try_from('\r').unwrap() => {
                match menu_option {
                    MenuOption::BootOption(entry) => {
                        if let Some(boot_error) = crate::wtcore::boot::attempt_boot(&entry) {
                            eprintln!("Unable to boot \"{}\": {:?}", entry.name, boot_error);
                            return Err(Status::ABORTED);
                        }
                    }
                    MenuOption::UEFIShell => {
                        return Ok(());
                    }
                    MenuOption::EditConfig => {
                        todo!();
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }
}

fn draw_menu() {
    let config = CONFIG.read();

    if config.menu_clear {
        system_table!().stdout().clear().unwrap();
    }
    ANCHOR_START.call_once(|| system_table!().stdout().cursor_position());

    let mut menu_options = MENU_OPTIONS.write();
    println_force!(" Wakatiwai Bootloader ");
    println_force!("=#====================");
    for entry in &config.boot_entries {
        println_force!(" #-> {}", entry.name);
        menu_options.push(entry.name.clone());
    }
    if config.offer_shell {
        const UEFI_SHELL_LABEL: &str = "UEFI Shell";
        println_force!(" #-$ {}", UEFI_SHELL_LABEL);
        menu_options.push(String::from(UEFI_SHELL_LABEL));
    }
    if config.edit_config {
        const EDIT_CONFIG_LABEL: &str = "Edit Bootloader Config";
        println_force!(" #-@ {}", EDIT_CONFIG_LABEL);
        menu_options.push(String::from(EDIT_CONFIG_LABEL));
    }
    ANCHOR_END.call_once(|| system_table!().stdout().cursor_position());

    drop(menu_options);
}

fn select_menu_option(index: usize) -> Option<MenuOption> {
    if get_index_cursor(index).is_err() {
        return None;
    }

    let config = CONFIG.read();
    let menu_options = MENU_OPTIONS.read();

    colour_menu_option(
        unsafe { CURRENT_OPTION_INDEX },
        Color::LightGray,
        Color::Black,
    );
    colour_menu_option(index, Color::White, Color::Black);

    unsafe {
        CURRENT_OPTION_INDEX = index;
    }

    // Last two options might indicate either Shell or Config
    if index == menu_options.len() - 1 {
        if config.edit_config {
            return Some(MenuOption::EditConfig);
        } else if config.offer_shell {
            return Some(MenuOption::UEFIShell);
        }
    } else if index == menu_options.len() - 2 && config.edit_config && config.offer_shell {
        return Some(MenuOption::UEFIShell);
    }

    return Some(MenuOption::BootOption(
        (*config.boot_entries.get(index).unwrap()).clone(),
    ));
}

fn colour_menu_option(index: usize, foreground: Color, background: Color) -> Option<()> {
    let menu_options = MENU_OPTIONS.read();
    let cursor_target = match get_index_cursor(index) {
        Ok(ok) => ok,
        Err(_) => {
            return None;
        }
    };

    system_table!()
        .stdout()
        .set_cursor_position(cursor_target.0, cursor_target.1)
        .unwrap();
    system_table!()
        .stdout()
        .set_color(foreground, background)
        .unwrap();
    system_table!()
        .stdout()
        .output_string(
            CStr16::from_str_with_buf(
                menu_options.get(index).unwrap(),
                &mut [0 as u16; crate::BootEntry::MAX_NAME_LENGTH + 1],
            )
            .unwrap(),
        )
        .unwrap();

    let anchor_end = ANCHOR_END.get().unwrap();
    system_table!()
        .stdout()
        .set_cursor_position(anchor_end.0, anchor_end.1)
        .unwrap();

    Some(())
}

fn get_index_cursor(index: usize) -> Result<(usize, usize), ()> {
    let menu_options = MENU_OPTIONS.read();

    if index >= menu_options.len() {
        return Err(());
    }

    let mut pos = ANCHOR_START.get().unwrap().clone();
    pos.0 += 5;
    pos.1 += 2; // Options start 2 lines below anchor
    pos.1 += index; // Offset line down by index

    Ok(pos)
}

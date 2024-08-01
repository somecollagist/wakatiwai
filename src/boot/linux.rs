use crate::BootEntry;

use super::BootFailure;

pub fn boot(entry: &BootEntry) -> Option<BootFailure> {
    if entry.initrd.is_empty() {
        return Some(BootFailure::NoInitrd);
    }

    None
}
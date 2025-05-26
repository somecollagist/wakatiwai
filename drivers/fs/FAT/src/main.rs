#![no_main]
#![no_std]

wakatiwai_udive::fs_prelude!();

mod data;
mod disk;

#[derive(Clone, Copy, Debug, PartialEq)]
enum FATType {
    FAT12,
    FAT16,
    FAT32
}

fn main(args: &FSDriverArgs) -> Result<Vec<u8>, Status> {
    let fs = disk::FAT::new(&args.diskreader);
    
    let mut path_directories: Vec<&str> = args.path.split("/").skip(1).collect();
    let file_name = path_directories.pop().unwrap();

    // Navigate to the correct directory
    let mut directory_entries = fs.get_root_directory(&args.diskreader);
    for directory in path_directories {
        match directory_entries.iter().find(
            |t| t.name() == directory.to_uppercase()
        ) {
            Some(some) => {
                if !some.is_directory() {
                    return Err(Status::NOT_FOUND);
                }

                directory_entries = fs.read_raw_directory(
                    &fs.read_cluster_chain(
                        &args.diskreader,
                        some.metadata.first_cluster()
                    )
                );
            }
            None => {
                return Err(Status::NOT_FOUND);
            }
        }
    }

    // Find the correct file
    match directory_entries.iter().find(
        |t| t.name() == file_name.to_uppercase()
    ) {
        Some(some) => {
            if !some.is_file() {
                return Err(Status::NOT_FOUND);
            }

            let mut ret = fs.read_cluster_chain(
                &args.diskreader,
                some.metadata.first_cluster()
            );
            ret.truncate(some.metadata.file_size as usize);
            return Ok(ret.to_vec());
        }
        None => {
            return Err(Status::NOT_FOUND);
        }
    }
}
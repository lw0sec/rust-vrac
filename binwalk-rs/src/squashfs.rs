use crate::context;

use std::fs::File;
use std::io::prelude::*;

#[repr(C)]
pub struct SquashfsHeader {
    pub magic: u32,
    pub inode_count: u32,
    pub modification_time: u32,
    pub block_size: u32,
    pub fragment_entry_count: u32,
    pub compression_id: u16,
    pub block_log: u16,
    pub flags: u16,
    pub id_count: u16,
    pub version_major: u16,
    pub version_minor: u16,
    pub root_inode_ref: u64,
    pub bytes_used: u64,
}

impl SquashfsHeader {
    pub fn validate(&self) -> bool {
        if self.version_major == 4 && self.version_minor == 0 {
            true
        } else {
            false
        }
    }
}

pub fn check_squashfs(c: &mut context::Context, offset: usize, skip: &mut usize) {
    unsafe {
        // Check the magic value
        if *(c.current_ptr as *const u32) == 0x73717368 {
            // Define squashfs header
            let squashfs_header = c.current_ptr as *const SquashfsHeader;

            // Header validation
            if (&(*squashfs_header)).validate() {
                *skip = (*squashfs_header).bytes_used as usize;

                println!(
                        "{:<20} 0x{:<18x} Squashfs filesystem, version {}, compression: {}, {} inodes, blocksize : {}, size {}",
                        offset as usize, offset as usize,
                        (*squashfs_header).version_major,
                        (*squashfs_header).compression_id,
                        (*squashfs_header).inode_count,
                        (*squashfs_header).block_size,
                        (*squashfs_header).bytes_used,
                    );

                // Check if extraction is required
                if c.args.extract {
                    let file_name = format!("{:X}.squashfs", offset as usize);
                    let path = format!("./{}/{}", c.extract_dir, file_name);

                    if !std::path::Path::new(&format!("./{}", c.extract_dir)).exists() {
                        std::fs::create_dir(&format!("./{}", c.extract_dir)).unwrap();
                    }

                    if !std::path::Path::new(&path).exists() {
                        println!("Extracting to {path}");

                        let mut file =
                            File::create(path).expect("Error encountered while creating file!");

                        // Create data
                        let mut dst = Vec::with_capacity((*squashfs_header).bytes_used as usize);
                        std::ptr::copy(
                            c.current_ptr,
                            dst.as_mut_ptr(),
                            (*squashfs_header).bytes_used as usize,
                        );
                        dst.set_len((*squashfs_header).bytes_used as usize);

                        file.write_all(&dst);
                    }
                }
            }
        }
    }
}

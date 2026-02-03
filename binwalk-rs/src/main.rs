use clap::Parser;

use std::env;
use std::fs;

pub mod context;
pub mod elf;
pub mod squashfs;

pub fn check_str(ptr: *const u8, str: &str) -> bool {
    let str_slice = unsafe { std::slice::from_raw_parts(ptr, str.len()) };

    match std::str::from_utf8(str_slice) {
        Ok(s) => {
            if s == str {
                //println!("\"{}\" detected", str);
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

fn run(mut c: context::Context) {
    println!("{:<20} {:<20} {}", "DECIMAL", "HEXADECIMAL", "DESCRIPTION");
    println!("--------------------------------------------------------------------------------");

    unsafe {
        while c.current_ptr < c.data_ptr.add(c.data_size) {
            // Get offset of current ptr
            let offset = c.current_ptr.sub(c.data_ptr as usize) as usize;

            // Used to skip data if something is detected
            let mut skip: usize = 0;

            /*
                Squashfs filesystem
                magic : 0x73717368
            */
            squashfs::check_squashfs(&mut c, offset, &mut skip);

            if skip != 0 {
                c.current_ptr = c.current_ptr.add(skip);
            } else {
                c.current_ptr = c.current_ptr.add(1);
            }
        }
    }
}

fn main() {
    // Parse command line args
    let args = context::Args::parse();

    // Load file into Vec<u8>
    let data = fs::read(args.file.clone()).unwrap();

    // Get raw pointer to the vector buffer
    let data_ptr = data.as_ptr() as *const u8;

    let path = args.file.clone();
    let file_name = path.split("/").last().unwrap();

    let extract_dir = "_".to_string() + file_name;
    //let file_name = args.file.split("/").last().unwrap();

    // Init context
    let context = context::Context {
        args: args,
        file_name: file_name.to_string(),
        extract_dir: extract_dir,
        data_size: data.len(),
        data_ptr: data_ptr,
        current_ptr: data_ptr,
    };

    // Start analysis
    run(context);
}

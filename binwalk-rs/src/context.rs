use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // File to analyse
    #[arg(short, long)]
    pub file: String,

    // If extraction of files is required
    #[arg(short, long, default_value_t = false)]
    pub extract: bool,
}

pub struct Context {
    pub args: Args,

    pub file_name: String,
    pub extract_dir: String,

    pub data_size: usize,
    pub data_ptr: *const u8,
    pub current_ptr: *const u8,
}

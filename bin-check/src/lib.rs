mod elf;

#[derive(Debug)]
pub enum FileType {
    Raw,
    Elf,
    Pe,
}

#[derive(Debug)]
pub enum Class {
    Unknown,
    _32,
    _64,
}

#[derive(Debug)]
pub enum Machine {
    Unknown,
    X64,
    Arm,
    RiscV,
}

pub fn check_mem(ptr: &[u8], size: usize) -> (FileType, Class, Machine) {
    let mut file_type = FileType::Raw;
    let mut class = Class::Unknown;
    let mut machine = Machine::Unknown;

    // ELF Check
    elf::check_elf(
        ptr as *const _ as *const elf::ElfHeader,
        &mut file_type,
        &mut class,
        &mut machine,
    );

    (file_type, class, machine)
}

pub fn check_path(path: &str) -> (FileType, Class, Machine) {
    let path = std::path::PathBuf::from(path);
    let file_data = std::fs::read(path).expect("Could not read file.");
    let slice = file_data.as_slice();

    check_mem(slice, slice.len())
}

use crate::{Class, FileType, Machine};

#[repr(C)]
#[derive(Debug)]
pub struct ElfHeader {
    // Magic value : ELF + 0x7f
    pub magic: [u8; 4],

    // 0 = None
    // 1 = 32 bits
    // 2 = 64 bits
    pub class: u8,

    // 0 = None
    // 1 = LSB (little)
    // 2 = MSB (big)
    pub endianess: u8,

    // Should be equal to 1
    pub header_version: u8,

    // ABI
    // 3 = Linux
    // 64 = ARM EABI
    // 97 = ARM
    pub abi: u8,

    pub abi_version: u8,

    // Reserved
    pub reserved: [u8; 7],

    // 0 = Unknown
    // 1 = Relocatable file
    // 2 = Executable file
    // 3 = Shared object
    // 4 = Core file
    pub file_type: u16,

    // Target architecture
    // 7 = Intel i860
    // 40 = ARM
    // 50 = Intel IA64
    // 62 = X64
    // 243 = RISC-V
    pub machine: u16,

    // Should be equal to 1
    pub version: u32,

    // Entry point
    // 0 if no entrypoint
    pub entry_point: usize,

    // Offset of the program header table
    pub ph_offset: usize,

    // Offset of the section header table
    pub sh_offset: usize,

    // Flags
    pub flags: u32,

    // Size of this header
    pub header_size: u16,

    // Size of a program header table entry
    pub ph_entry_size: u16,

    // Number of entries in the program header table
    pub ph_num: u16,

    // Size of a section heade table entry
    pub sh_entry_size: u16,

    // Number of entries in the section header table
    pub sh_num: u16,

    // Index of the entry in the section table header that contains the section names
    pub sh_str_index: u16,
}

pub fn check_elf(
    header: *const ElfHeader,
    file_type: &mut FileType,
    class: &mut Class,
    machine: &mut Machine,
) {
    if unsafe { (*header).magic == [127, 69, 76, 70] } {
        *file_type = FileType::Elf;

        match unsafe { (*header).class } {
            1 => *class = Class::_32,
            2 => *class = Class::_64,
            _ => {}
        };

        match unsafe { (*header).machine } {
            40 => *machine = Machine::Arm,
            62 => *machine = Machine::X64,
            243 => *machine = Machine::RiscV,
            _ => {}
        };
    }
}

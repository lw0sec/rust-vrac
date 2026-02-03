#[repr(C)]
#[derive(Debug)]
pub enum ElfArchBits {
    None = 0,
    X32 = 1,
    X64 = 2,
}

impl ElfArchBits {
    fn from_u8(value: u8) -> ElfArchBits {
        match value {
            0 => ElfArchBits::None,
            1 => ElfArchBits::X32,
            2 => ElfArchBits::X64,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum ElfFileType {
    None = 0,
    Repo = 1,
    Exec = 2,
    Shared = 3,
    Core = 4,
}

impl ElfFileType {
    fn from_u16(value: u16) -> ElfFileType {
        match value {
            0 => ElfFileType::None,
            1 => ElfFileType::Repo,
            2 => ElfFileType::Exec,
            3 => ElfFileType::Shared,
            4 => ElfFileType::Core,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum ElfArch {
    ARM = 40,
    X64 = 62,
    RISCV = 243,
}

impl ElfArch {
    fn from_u16(value: u16) -> ElfArch {
        match value {
            40 => ElfArch::ARM,
            62 => ElfArch::X64,
            243 => ElfArch::RISCV,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[repr(C)]
pub struct ElfHeader {
    // Magic value 0x7fELF
    pub magic: [u8; 4],
    pub arch_bits: u8,
    pub endianess: u8,
    pub header_version: u8,
    pub abi: u8,
    pub version_abi: u8,
    pub _a: [u8; 6],
    pub ident_size: u8,

    pub file_type: u16,
    pub arch: u16,
}

pub fn check_elf(ptr: *const u8, offset: usize, skip: &mut usize) {
    unsafe {
        if *(ptr as *const u32) == 0x464c457f {
            let elf_header = ptr as *const ElfHeader;

            // Valid the elf file
            if (*elf_header).arch_bits <= 2
                && (*elf_header).arch_bits >= 0
                && (*elf_header).endianess <= 2
                && (*elf_header).endianess >= 0
            {
                println!(
                    "{:<20} 0x{:<18x} ELF, type : {:?}, arch_bits : {:?}, arch : {:?}",
                    offset,
                    offset,
                    ElfFileType::from_u16((*elf_header).file_type),
                    ElfArchBits::from_u8((*elf_header).arch_bits),
                    ElfArch::from_u16((*elf_header).arch),
                );
            }
        }
    }
}

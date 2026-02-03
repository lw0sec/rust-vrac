use disassembler::core::Disassembler;

use object::{Object, ObjectSection};

const ELF_BASE: usize = 0x100000;

pub fn elf_file_handler(binary: &str, class: bin_check::Class, machine: bin_check::Machine) {
    let bin_data = std::fs::read(binary).unwrap();
    let bin_data_slice = bin_data.as_slice();

    let mut disas = Disassembler::new_sleigh(class, machine).unwrap();

    let obj_file = object::File::parse(bin_data_slice).unwrap();
    for section in obj_file.sections() {
        println!("{:?}", section);

        if section.name().unwrap() == ".text" {
            disas.disasm_count(
                section.data().unwrap(),
                ELF_BASE + section.address() as usize,
                section.size() as usize,
            );
        }
    }
}

fn main() {
    let binary = "/bin/ls";

    let (file_type, class, machine) = bin_check::check_path(binary);
    println!(
        "file_type: {:?} class: {:?} machine: {:?}",
        file_type, class, machine
    );

    match file_type {
        bin_check::FileType::Elf => {
            elf_file_handler(binary, class, machine);
        }
        _ => {}
    }

    // let mut disas = Disassembler::new_sleigh(class, machine).unwrap();
    // disas.load_data(bin_data_slice);

    // let obj_file = object::File::parse(bin_data_slice).unwrap();

    // for section in obj_file.sections() {
    //     println!("{:?}", section);
    // }

    // if let Some(section) = obj_file.section_by_name(".text") {
    //     let section_size = section.size();
    //     let section_data = section.data().unwrap();

    //     let (offset, size) = section.file_range().unwrap();
    //     //println!("{:?}", section.file_range());

    //     // disas.disasm_count(
    //     //     &bin_data_slice[offset as usize..(offset + size) as usize],
    //     //     0x1000,
    //     //     section_size as usize,
    //     // );

    //     disas.disasm_count(
    //         section_data,
    //         0x100000 + section.address(),
    //         section_size as usize,
    //     );

    //     println!("{} instructions", disas.instructions.len());

    //     //disas.display_instructions();
    //     //disas.analysis_functions();
    // } else {
    //     eprintln!("section not available");
    // }
}

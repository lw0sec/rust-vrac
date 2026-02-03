use bin_check::{Class, Machine};

use crate::core::Disassembler;

use capstone::prelude::*;

use std::collections::HashMap;

/// Print register names
fn reg_names(cs: &Capstone, regs: &[RegId]) -> String {
    let names: Vec<String> = regs.iter().map(|&x| cs.reg_name(x).unwrap()).collect();
    names.join(", ")
}

/// Print instruction group names
fn group_names(cs: &Capstone, regs: &[InsnGroupId]) -> Vec<String> {
    let names: Vec<String> = regs.iter().map(|&x| cs.group_name(x).unwrap()).collect();
    names
    //names.join(", ")
}

impl<'a> Disassembler<'a, Capstone> {
    pub fn new(class: Class, machine: Machine) -> Result<Disassembler<'a, Capstone>, ()> {
        let cs_builder = Capstone::new();

        let arch_cs_builder = match (class, machine) {
            (Class::_32, Machine::X64) => cs_builder
                .x86()
                .mode(arch::x86::ArchMode::Mode32)
                .syntax(arch::x86::ArchSyntax::Att),
            (Class::_64, Machine::X64) => cs_builder
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .syntax(arch::x86::ArchSyntax::Att),
            _ => return Err(()),
        };

        let cs = arch_cs_builder
            .detail(true)
            .build()
            .expect("Failed to create Capstone object");

        Ok(Self {
            engine: cs,
            data: &[0],
            instructions_map: HashMap::new(),
            instructions: Vec::new(),
            basic_blocks: Vec::new(),
        })
    }

    pub fn disasm_count(&self, code: &[u8], addr: u64, size: usize) {
        let insns = self
            .engine
            .disasm_count(code, addr, size)
            .expect("Failed to disassemble");

        println!("Found {} instructions", insns.len());

        for i in insns.as_ref() {
            let detail: InsnDetail = self
                .engine
                .insn_detail(&i)
                .expect("Failed to get insn detail");
            let arch_detail: ArchDetail = detail.arch_detail();

            //let groups = group_names(&self.engine, detail.groups());

            // if groups.contains(&"call".to_string())
            //     || groups.contains(&"jump".to_string())
            //     || groups.contains(&"branch_relative".to_string())
            // {
            //     let ops = arch_detail.operands();

            //     if ops.len() == 1 {
            //         //let op = &ops[0];
            //         match &ops[0] {
            //             capstone::arch::ArchOperand::X86Operand(op) => match op.op_type {
            //                 capstone::arch::x86::X86OperandType::Imm(v) => {
            //                     println!("{}", i);
            //                     println!("{:x}", v);
            //                 }
            //                 _ => {}
            //             },
            //             _ => {}
            //         };
            //     }
            //     // println!("{:4}operands: {}", "", ops.len());
            //     // for op in ops {
            //     //     println!("{:8}{:?}", "", op);
            //     // }
            // }
        }
    }
}

use bin_check::{Class, Machine};

use crate::core::instruction::{Instruction, InstructionAnalysis};
use crate::core::Disassembler;

use sleigh::Decompiler as Sleigh;
use sleigh::X86Mode;

use std::collections::HashMap;
use std::marker::PhantomPinned;
use std::pin::Pin;

impl<'a> Disassembler<'a, Sleigh> {
    pub fn new(class: Class, machine: Machine) -> Result<Disassembler<'a, Sleigh>, ()> {
        let builder = Sleigh::builder();

        let builder = match (class, machine) {
            (Class::_32, Machine::X64) => builder.x86(X86Mode::Mode32),
            (Class::_64, Machine::X64) => builder.x86(X86Mode::Mode64),
            _ => return Err(()),
        };

        let decompiler = builder.build();

        let disas = Self {
            engine: decompiler,
            data: &[0],
            instructions_map: HashMap::new(),
            instructions: Vec::new(),
            basic_blocks: Vec::new(),
        };
        //let boxed = Box::pin(disas);
        Ok(disas)
    }

    //pub fn disasm_count(&mut self, code: &'a [u8], addr: u64, size: usize) {
    pub fn disasm_count(&mut self, code: &'a [u8], addr: usize, size: usize) -> Vec<Instruction> {
        let mut ret: Vec<Instruction> = Vec::new();
        let mut offset = 0;

        while offset < size {
            /*  Disassemble at current offset
             */
            let (insts_len, mut insts) = self
                .engine
                .disassemble(&code[offset..], addr as u64 + offset as u64);

            /*  If len = 0 no instruction found
             */
            if insts_len == 0 {
                offset += 1;
            /*  If len > 0 handle the instructions
             */
            } else {
                /*  Map Sleigh instructions to disassembler instructions
                 */
                let mut instructions: Vec<Instruction> = insts
                    .iter()
                    .map(|x| {
                        let mut instruction = Instruction::new(
                            x.address as usize,
                            &code[offset..offset + insts_len],
                            x.mnemonic.clone(),
                            x.body.clone(),
                        );

                        let (len, pcodes) = self
                            .engine
                            .translate(&code[offset..], addr as u64 + offset as u64);

                        instruction
                            .analysis
                            .push(InstructionAnalysis::PCode(pcodes));

                        instruction
                    })
                    .collect();

                /*  Add the instructions to instructions_map using address as key
                 */
                // for i in instructions {
                //     self.instructions_map.insert(i.addr as usize, i);
                // }

                //self.instructions.append(&mut instructions);
                ret.append(&mut instructions);

                offset += insts_len;
            }
        }
        ret
    }

    pub fn translate_count(&mut self, code: &[u8], addr: u64, size: usize) {
        let mut offset = 0;

        while offset < size {
            let (len, pcodes) = self
                .engine
                .translate(&code[offset..], 0x1000 + offset as u64);
            if len == 0 {
                offset += 1;
            } else {
                println!("{} {:?}", len, pcodes);
                offset += len;
            }
        }
    }
}

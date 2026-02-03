//pub mod capstone;
pub mod instruction;
pub mod sleigh;

use bin_check::{Class, Machine};

use ::capstone::prelude::*;
use ::sleigh::Decompiler as Sleigh;

use crate::core::instruction::Instruction;

use std::collections::HashMap;

use std::marker::PhantomPinned;
use std::pin::Pin;

pub enum BasicBlockPoint {
    Start,
    End,
}
pub struct BasicBlock<'a> {
    pub instructions: &'a [Instruction<'a>],
    // pub left: Option<Box<BasicBlock<'a>>>,
    // pub right: Option<Box<BasicBlock<'a>>>,
}

pub struct Disassembler<'a, T> {
    engine: T,
    pub data: &'a [u8],
    pub instructions_map: HashMap<usize, Instruction<'a>>,
    pub instructions: Vec<Instruction<'a>>,
    pub basic_blocks: Vec<BasicBlock<'a>>,
}

impl<'a> Disassembler<'a, ()> {
    // pub fn new_capstone(class: Class, machine: Machine) -> Result<Disassembler<'a, Capstone>, ()> {
    //     Ok(Disassembler::<Capstone>::new(class, machine).unwrap())
    // }

    pub fn new_sleigh(class: Class, machine: Machine) -> Result<Disassembler<'a, Sleigh>, ()> {
        Ok(Disassembler::<Sleigh>::new(class, machine).unwrap())
    }
}

impl<'a, T> Disassembler<'a, T> {
    pub fn load_data(&mut self, data: &'a [u8]) {
        self.data = data;
    }

    pub fn display_instructions(&self) {
        for insn in &self.instructions {
            println!("0x{:x} {} {}", insn.addr, insn.mnemonic, insn.body);
        }
    }

    // pub fn analysis_functions(&mut self) {
    //     self.analysis_basic_blocks2();

    //     println!("{:?}", self.basic_blocks.len());
    // }

    // pub fn analysis_basic_blocks(&mut self) {
    //     //let bb_points: HashMap<usize, BasicBlockPoint> = HashMap::new();

    //     let mut current_bb_entry: Option<&Instruction> = None;

    //     for (addr, insn) in &self.instructions_map {
    //         if insn.is_direct_call() {
    //             match current_bb_entry {
    //                 None => current_bb_entry = Some(insn),
    //                 _ => {}
    //             }

    //             //println!("0x{:x} {} {}", insn.addr, insn.mnemonic, insn.body);

    //             let call_addr = insn.get_direct_call_addr().unwrap();

    //             // println!(
    //             //     "{:x} {:x}",
    //             //     call_addr,
    //             //     call_addr + insn.addr + insn.bytes.len()
    //             // );
    //             // let mut user_input = String::new();
    //             // std::io::stdin().read_line(&mut user_input).unwrap();

    //             let i = self.instructions_map.get(&(call_addr));
    //             //println!("{:?}", i);
    //             //if Some(i) == i {}
    //             //println!("==== 0x{:x} {} {}", i.addr, i.mnemonic, i.body);
    //         } else if insn.is_jump() {
    //             match current_bb_entry {
    //                 None => current_bb_entry = Some(insn),
    //                 _ => {}
    //             }

    //             // let bb = BasicBlock {
    //             //     right: None,
    //             //     left: None,
    //             // };

    //             // self.basic_blocks.push(bb);
    //         }
    //     }
    // }

    // pub fn analysis_basic_blocks2(&mut self) {
    // for (i, insn) in self.instructions.iter().enumerate() {
    //     let slice: &[Instruction] = &self.instructions[i..i + 1];
    //     let bb = BasicBlock {
    //         instructions: slice,
    //     };
    //     // println!("{:?}", slice);

    //     //self.basic_blocks.push(bb);
    // }
    // let mut current_bb_entry: Option<&Instruction> = None;
    // let mut current_bb_index: usize = 0;

    // let mut bb_left: Option<&Instruction> = None;
    // let mut bb_right: Option<&Instruction> = None;

    // let mut bb_entries_store: Vec<usize> = Vec::new();

    // for (i, insn) in self.instructions.iter().enumerate() {
    //     match current_bb_entry {
    //         None => {
    //             current_bb_entry = Some(insn);
    //             current_bb_index = i;
    //         }
    //         _ => {}
    //     }

    //     if insn.is_direct_call() {
    //         let call_addr = insn.get_direct_call_addr().unwrap();
    //         bb_entries_store.push(call_addr);
    //     } else if insn.is_jump() {
    //         let slice = &self.instructions[current_bb_index..i];

    //         let bb = BasicBlock {
    //             instructions: slice,
    //             right: None,
    //             left: None,
    //         };

    //         //self.basic_blocks.push(bb);
    //     }
    // }
    //}

    //pub fn analyse_basic_block(&mut self) {}
}

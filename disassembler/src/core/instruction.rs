use ::capstone::prelude::*;
use ::sleigh::PCode;
use ::sleigh_sys::Opcode;

#[derive(Debug)]
pub enum InstructionAnalysis<'a> {
    Capstone(InsnDetail<'a>),
    PCode(Vec<PCode>),
}

#[derive(Debug)]
pub struct Instruction<'a> {
    pub addr: usize,
    pub bytes: &'a [u8],
    pub mnemonic: String,
    pub body: String,

    pub analysis: Vec<InstructionAnalysis<'a>>,
}

impl<'a> Instruction<'a> {
    pub fn new(addr: usize, bytes: &'a [u8], mnemonic: String, body: String) -> Instruction {
        Self {
            addr: addr,
            bytes: bytes,
            mnemonic: mnemonic,
            body: body,
            analysis: Vec::new(),
        }
    }

    pub fn is_jump(&self) -> bool {
        for a in &self.analysis {
            match a {
                InstructionAnalysis::PCode(pcodes) => {
                    for pcode in pcodes {
                        match pcode.opcode {
                            Opcode::Branch | Opcode::CBranch => return true,
                            _ => {}
                        };
                    }
                }
                _ => {}
            }
        }
        false
    }

    pub fn is_direct_call(&self) -> bool {
        for a in &self.analysis {
            match a {
                InstructionAnalysis::PCode(pcodes) => {
                    for pcode in pcodes {
                        match pcode.opcode {
                            Opcode::Call => return true,
                            _ => {}
                        };
                    }
                }
                _ => {}
            }
        }
        false
    }

    pub fn get_direct_call_addr(&self) -> Option<usize> {
        for a in &self.analysis {
            match a {
                InstructionAnalysis::PCode(pcodes) => {
                    for pcode in pcodes {
                        match pcode.opcode {
                            Opcode::Call => return Some(pcode.vars[0].offset as usize),
                            _ => {}
                        };
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn is_flow_redirect(&self) -> bool {
        for a in &self.analysis {
            match a {
                InstructionAnalysis::PCode(pcodes) => {
                    for pcode in pcodes {
                        //println!("{:?}", pcode.opcode);

                        match pcode.opcode {
                            Opcode::Branch
                            | Opcode::CBranch
                            | Opcode::BranchInd
                            | Opcode::Call
                            | Opcode::CallInd
                            | Opcode::CallOther => return true,
                            _ => {}
                        };
                        //if pcode.opcode as u32 == Opcode::Branch as u32 {}
                        // if [
                        //     Opcode::Branch,
                        //     Opcode::CBranch,
                        //     Opcode::BranchInd,
                        //     Opcode::Call,
                        //     Opcode::CallInd,
                        //     Opcode::CallOther,
                        // ]
                        // .contains(&pcode.opcode)
                        // {}
                    }
                }
                _ => {}
            }
        }
        false
    }
}

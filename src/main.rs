
extern crate num;
#[macro_use]
extern crate num_derive;

use std::fs::File;
use std::io::Read;

mod bitops;
mod memif;
mod rv64alu;
mod rv64inst;

use memif::*;
use rv64inst::*;

fn read_bin(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

struct RiscvCpu {
    pc : u64,
    regs : [u64; 32]
}

impl RiscvCpu {
    fn fetch_inst(&mut self, mem : &mut dyn MemIf) -> RiscvInst {
        RiscvInst { raw: read32(mem, self.pc) as u32 }
    }

    #[inline(always)]
    pub fn get_reg(&self, reg : usize) -> u64 { 
        match reg {
            0 ..= 31 => self.regs[reg],
            _ => panic!("register index out of range!")
        }
    }

    #[inline(always)]
    pub fn set_reg(&mut self, reg : usize, val : u64) {
        match reg {
            0 => (),
            1 ..= 31 => { self.regs[reg] = val; },
            _ => panic!("Write to invalid register!")
        }
    }

    pub fn exec(&mut self, mem : &mut dyn MemIf) -> bool {
        let inst = self.fetch_inst(mem);

        match inst.spec() {
            (RiscvOpcode::LUI, _, _) => {
                self.set_reg(inst.rd(), inst.imm());
                self.pc = rv64alu::add(self.pc, 4);
                true
            },
            (RiscvOpcode::AUIPC, _, _) => {
                self.set_reg(inst.rd(), rv64alu::add(self.pc, inst.imm()));
                self.pc = rv64alu::add(self.pc, 4);
                true
            },
            (RiscvOpcode::JAL, _, _) => {
                self.set_reg(inst.rd(), rv64alu::add(self.pc, 4));
                self.pc = rv64alu::add(self.pc, inst.imm());
                true
            },
            (RiscvOpcode::JALR, 0, _) => {
                self.set_reg(inst.rd(), rv64alu::add(self.pc, 4));
                self.pc = rv64alu::add(self.get_reg(inst.rs1()), inst.imm());
                true
            },
            (RiscvOpcode::BRANCH, funct3, _) => {
                let pred = match funct3 {
                    0 => self.get_reg(inst.rs1()) == self.get_reg(inst.rs2()),
                    1 => self.get_reg(inst.rs1()) != self.get_reg(inst.rs2()),
                    4 => (self.get_reg(inst.rs1()) as i64) < (self.get_reg(inst.rs2()) as i64),
                    5 => (self.get_reg(inst.rs1()) as i64) > (self.get_reg(inst.rs2()) as i64),
                    6 => self.get_reg(inst.rs1()) < self.get_reg(inst.rs2()),
                    7 => self.get_reg(inst.rs1()) > self.get_reg(inst.rs2()),
                    _ => panic!("Unsupported funct3 for branch!")
                };

                if pred {
                    self.pc += inst.imm();
                }
                else {
                    self.pc = rv64alu::add(self.pc, 4);
                }

                true
            },
            (RiscvOpcode::LOAD, funct3, _) => {
                let addr = rv64alu::add(self.get_reg(inst.rs1()), inst.imm());

                let ld_val = match funct3 {
                    0 => bitops::sign_ext8to64(read8(mem, addr)),
                    1 => bitops::sign_ext16to64(read16(mem, addr)),
                    2 => bitops::sign_ext32to64(read32(mem, addr)),
                    3 => read64(mem, addr),
                    4 => read8(mem, addr),
                    5 => read16(mem, addr),
                    6 => read32(mem, addr),
                    _ => panic!("Unsupported funct3 for load!")
                };

                self.set_reg(inst.rd(), ld_val);
                self.pc = rv64alu::add(self.pc, 4);
                true
            },
            (RiscvOpcode::STORE, funct3, _) => {
                let addr = rv64alu::add(self.get_reg(inst.rs1()), inst.imm());
                
                match funct3 {
                    0 => write8(mem, addr, self.get_reg(inst.rs2())),
                    1 => write16(mem, addr, self.get_reg(inst.rs2())),
                    2 => write32(mem, addr, self.get_reg(inst.rs2())),
                    3 => write64(mem, addr, self.get_reg(inst.rs2())),
                    _ => panic!("Unsupported funct3 for store!")
                };

                self.pc = rv64alu::add(self.pc, 4);
                true
            },
            (opcode, funct3, funct7) if 
                (opcode == RiscvOpcode::OP) ||
                (opcode == RiscvOpcode::OPIMM) ||
                (opcode == RiscvOpcode::OP32) ||
                (opcode == RiscvOpcode::OPIMM32) => {
                    
                let op1 = self.get_reg(inst.rs1());
                let op2 = match opcode {
                    RiscvOpcode::OP => self.get_reg(inst.rs2()),
                    RiscvOpcode::OPIMM => inst.imm(),
                    RiscvOpcode::OP32 => self.get_reg(inst.rs2()),
                    RiscvOpcode::OPIMM32 => inst.imm(),
                    x => panic!("Unsupported alu spec: ({:?})", x)
                };

                let func = match (opcode, funct3, funct7) {
                    (RiscvOpcode::OP, 0, 0b0000000) => rv64alu::add,
                    (RiscvOpcode::OP, 0, 0b0100000) => rv64alu::sub,
                    (RiscvOpcode::OPIMM, 0, _) => rv64alu::add,
                    (RiscvOpcode::OP32, 0, 0b0000000) => rv64alu::addw,
                    (RiscvOpcode::OP32, 0, 0b0100000) => rv64alu::subw,
                    (RiscvOpcode::OPIMM32, 0, _) => rv64alu::addw,
                    x => panic!("Unsupported alu spec: ({:?})", x)
                };

                self.set_reg(inst.rd(), func(op1, op2));
                self.pc = rv64alu::add(self.pc, 4);
                true
            },
            (RiscvOpcode::SYSTEM, 0, 8) => {
                false
            }
            x => panic!("Unsupported instruction: {:?}", x)
        }
    }
}

impl MemIf for Vec<u8> {
    fn read(&self, addr : u64) -> u8 {
        self[addr as usize]
    }

    fn write(&mut self, addr : u64, value : u8) {
        self[addr as usize] = value;
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("No filename provided!");
    let mut arr : Vec<u8> = read_bin(&filename);
    
    let mut cpu = RiscvCpu {
        pc: 0,
        regs: [0; 32]
    };

    while cpu.exec(&mut arr) {
        println!("{}", cpu.pc);
    }
}




extern crate num;
#[macro_use]
extern crate num_derive;

use std::fs::File;
use std::io::Read;

mod bitops;
mod memif;
mod rv64alu;

use memif::*;

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

#[derive(Debug)]
struct RiscvInst {
    raw : u32
}

#[derive(Debug)]
#[derive(PartialEq)]
enum RiscvInstFormat { 
    R, I, S, B, U, J, Unsupported
}

#[derive(Debug)]
#[derive(FromPrimitive)]
#[derive(PartialEq)]
enum RiscvOpcode {
    LOAD = 0b0000011,
    STORE = 0b0100011,
    MADD = 0b1000011,
    BRANCH = 0b1100011,
    LOADFP = 0b0000111,
    STOREFP = 0b0100111,
    MSUB = 0b1000111,
    JALR = 0b1100111,
    CUSTOM0 = 0b0001011,
    CUSTOM1 = 0b0101011,
    NMSUB = 0b1001011,
    MISCMEM = 0b0001111,
    AMO = 0b0101111,
    NMADD = 0b1001111,
    JAL = 0b1101111,
    OPIMM = 0b0010011,
    OP = 0b0110011,
    OPFP = 0b1010011,
    SYSTEM = 0b1110011,
    AUIPC = 0b0010111,
    LUI = 0b0110111,
    OPIMM32 = 0b0011011,
    OP32 = 0b0111011,
    CUSTOM2 = 0b1011011,
    CUSTOM3 = 0b1111011
}

impl RiscvInst {

    #[inline(always)]
    pub fn opcode(&self) -> Option<RiscvOpcode> { 
        num::FromPrimitive::from_u32(self.raw & 0b1111111) 
    }

    #[inline(always)]
    pub fn format(&self) -> RiscvInstFormat {
        match self.opcode() {
            Some(RiscvOpcode::LOAD) => RiscvInstFormat::I,
            Some(RiscvOpcode::STORE) => RiscvInstFormat::S,
            Some(RiscvOpcode::MADD) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::BRANCH) => RiscvInstFormat::B,
            Some(RiscvOpcode::LOADFP) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::STOREFP) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::MSUB) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::JALR) => RiscvInstFormat::I,
            Some(RiscvOpcode::CUSTOM0) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::CUSTOM1) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::NMSUB) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::MISCMEM) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::AMO) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::NMADD) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::JAL) => RiscvInstFormat::J,
            Some(RiscvOpcode::OPIMM) => RiscvInstFormat::I,
            Some(RiscvOpcode::OP) => RiscvInstFormat::R,
            Some(RiscvOpcode::OPFP) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::SYSTEM) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::AUIPC) => RiscvInstFormat::U,
            Some(RiscvOpcode::LUI) => RiscvInstFormat::U,
            Some(RiscvOpcode::OPIMM32) => RiscvInstFormat::I,
            Some(RiscvOpcode::OP32) => RiscvInstFormat::R,
            Some(RiscvOpcode::CUSTOM2) => RiscvInstFormat::Unsupported,
            Some(RiscvOpcode::CUSTOM3) => RiscvInstFormat::Unsupported,
            _ => RiscvInstFormat::Unsupported
        }
    }

    #[inline(always)]
    pub fn rs1(&self) -> usize { ((self.raw >> 15) & 0b11111) as usize }

    #[inline(always)]
    pub fn rs2(&self) -> usize { ((self.raw >> 20) & 0b11111) as usize }

    #[inline(always)]
    pub fn rd(&self) -> usize { ((self.raw >> 7) & 0b11111) as usize }

    #[inline(always)]
    pub fn funct3(&self) -> usize { ((self.raw >> 12) & 0b111) as usize }

    #[inline(always)]
    pub fn funct7(&self) -> usize { ((self.raw >> 25) & 0b1111111) as usize }

    #[inline(always)]
    pub fn imm(&self) -> Option<u64> {
        let format = self.format();
        let sign : u64 = if self.raw & 0x80000000 == 0 { 0 } else { 1 };

        match format {
            RiscvInstFormat::I => Some(bitops::sign_ext32to64(
                bitops::bit_range_set(bitops::bit_repeat(sign, 21), bitops::BitRange(11, 31)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(25, 30), bitops::BitRange(5, 10)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(21, 24), bitops::BitRange(1, 4)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(20, 20), bitops::BitRange(0, 0))
            )),
            RiscvInstFormat::S => Some(bitops::sign_ext32to64(
                bitops::bit_range_set(bitops::bit_repeat(sign, 21), bitops::BitRange(11, 31)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(25, 30), bitops::BitRange(5, 10)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(8, 11), bitops::BitRange(1, 4)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(7, 7), bitops::BitRange(0, 0))
            )),
            RiscvInstFormat::B => Some(bitops::sign_ext32to64(
                bitops::bit_range_set(bitops::bit_repeat(sign, 21), bitops::BitRange(12, 31)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(7, 7), bitops::BitRange(11, 11)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(25, 30), bitops::BitRange(5, 10)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(8, 11), bitops::BitRange(1, 4))
            )),
            RiscvInstFormat::U => Some(bitops::sign_ext32to64(
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(12, 31), bitops::BitRange(12, 31))
            )),
            RiscvInstFormat::J => Some(bitops::sign_ext32to64(
                bitops::bit_range_set(bitops::bit_repeat(sign, 12), bitops::BitRange(20, 31)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(12, 19), bitops::BitRange(12, 19)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(20, 20), bitops::BitRange(11, 11)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(25, 30), bitops::BitRange(5, 10)) |
                bitops::bit_range_map(self.raw as u64, bitops::BitRange(21, 24), bitops::BitRange(1, 4))
            )),
            _ => None
        }
    }

    #[inline(always)]
    pub fn spec(&self) -> (RiscvOpcode, usize, usize) { 
        (self.opcode().unwrap(), self.funct3(), self.funct7()) 
    }

}

#[test]
fn test_format_1() {
    let inst = RiscvInst { raw: 0x00400793 };
    assert_eq!(inst.format(), RiscvInstFormat::I);
}

#[test]
fn test_imm_1() {
    let inst = RiscvInst { raw: 0x0080006f };
    assert_eq!(inst.imm().unwrap(), 0x8);
}

#[test]
fn test_imm_2() {
    let inst = RiscvInst { raw: 0xfe010113 };
    assert_eq!(inst.format(), RiscvInstFormat::I);
    assert_eq!(inst.imm().unwrap(), 0xffffffffffffffe0);
}

impl RiscvCpu {
    fn fetch_inst(&mut self, mem : &mut dyn MemIf) -> RiscvInst {
        RiscvInst { raw: read32(mem, self.pc) as u32 }
    }

    #[inline(always)]
    fn rs1(&self, inst : &RiscvInst) -> Option<u64> { 
        match inst.rs1() {
            0 ..= 31 => Some(self.regs[inst.rs1()]),
            _ => None
        }
    }
    
    #[inline(always)]
    fn rs2(&self, inst : &RiscvInst) -> Option<u64> { 
        match inst.rs2() {
            0 ..= 31 => Some(self.regs[inst.rs2()]),
            _ => None
        }
    }
    
    #[inline(always)]
    fn writeback(&mut self, inst : &RiscvInst, val : u64) { 
        match inst.rd() {
            0 => (),
            1 ..= 31 => { self.regs[inst.rd()] = val; },
            _ => panic!("Write to invalid register!")
        }
    }

    pub fn exec(&mut self, mem : &mut dyn MemIf) -> bool {
        let inst = self.fetch_inst(mem);
        print!("{:#04x}: {:#010x} ({:?}-type): ", self.pc, inst.raw, inst.format());
        
        let imm = inst.imm();
        let rs1_val = self.rs1(&inst);
        let rs2_val = self.rs2(&inst);

        match inst.spec() {
            (RiscvOpcode::LUI, _, _) => {
                println!("lui");
                self.writeback(&inst, imm.unwrap());
                self.pc += 4;
                true
            },
            (RiscvOpcode::AUIPC, _, _) => {
                println!("auipc");
                self.writeback(&inst, self.pc + imm.unwrap());
                self.pc += 4;
                true
            },
            (RiscvOpcode::JAL, _, _) => {
                println!("jal (dest = {:x})", rv64alu::add(self.pc, imm.unwrap()));
                self.writeback(&inst, self.pc + 4);
                self.pc = rv64alu::add(self.pc, imm.unwrap());
                true
            },
            (RiscvOpcode::JALR, 0, _) => {
                println!("jalr (dest = {:x})", rv64alu::add(rs1_val.unwrap(), imm.unwrap()));
                self.writeback(&inst, self.pc + 4);
                self.pc = rv64alu::add(rs1_val.unwrap(), imm.unwrap());
                true
            },
            (RiscvOpcode::BRANCH, funct3, _) => {
                println!("Branch (funct3 = {})", funct3);
                let pred = match funct3 {
                    0 => rs1_val.unwrap() == rs2_val.unwrap(),
                    1 => rs1_val.unwrap() != rs2_val.unwrap(),
                    4 => (rs1_val.unwrap() as i64) < (rs2_val.unwrap() as i64),
                    5 => (rs1_val.unwrap() as i64) > (rs2_val.unwrap() as i64),
                    6 => rs1_val.unwrap() < rs2_val.unwrap(),
                    7 => rs1_val.unwrap() > rs2_val.unwrap(),
                    _ => panic!("Unsupported funct3 for branch!")
                };

                if pred {
                    self.pc += imm.unwrap();
                }
                else {
                    self.pc += 4;
                }

                true
            },
            (RiscvOpcode::LOAD, funct3, _) => {
                println!("Load (funct3 = {})", funct3);
                let addr = rs1_val.unwrap().overflowing_add(imm.unwrap()).0;
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

                self.writeback(&inst, ld_val);
                self.pc += 4;
                true
            },
            (RiscvOpcode::STORE, funct3, _) => {
                println!("Store (funct3 = {})", funct3);
                let addr = rs1_val.unwrap().overflowing_add(imm.unwrap()).0;
                match funct3 {
                    0 => write8(mem, addr, rs2_val.unwrap()),
                    1 => write16(mem, addr, rs2_val.unwrap()),
                    2 => write32(mem, addr, rs2_val.unwrap()),
                    3 => write64(mem, addr, rs2_val.unwrap()),
                    _ => panic!("Unsupported funct3 for store!")
                };

                self.pc += 4;
                true
            },
            (opcode, funct3, funct7) if 
                (opcode == RiscvOpcode::OP) ||
                (opcode == RiscvOpcode::OPIMM) ||
                (opcode == RiscvOpcode::OP32) ||
                (opcode == RiscvOpcode::OPIMM32) => {
                println!("OP");
                    
                let op1 = rs1_val.unwrap();
                let op2 = match opcode {
                    RiscvOpcode::OP => rs2_val.unwrap(),
                    RiscvOpcode::OPIMM => imm.unwrap(),
                    RiscvOpcode::OP32 => rs2_val.unwrap(),
                    RiscvOpcode::OPIMM32 => imm.unwrap(),
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

                self.writeback(&inst, func(op1, op2));
                self.pc += 4;
                true
            },
            (RiscvOpcode::SYSTEM, 0, 8) => {
                println!("wfi");
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
        
    }
}



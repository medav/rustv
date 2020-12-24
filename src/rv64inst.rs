
use crate::bitops::*;

#[derive(Debug)]
pub struct RiscvInst {
    pub raw : u32
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RiscvInstFormat { 
    R, I, S, B, U, J, Unsupported
}

#[derive(Debug)]
#[derive(FromPrimitive)]
#[derive(PartialEq)]
pub enum RiscvOpcode {
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
        match self.opcode().expect("No opcode?!") {
            RiscvOpcode::LOAD => RiscvInstFormat::I,
            RiscvOpcode::STORE => RiscvInstFormat::S,
            RiscvOpcode::MADD => RiscvInstFormat::Unsupported,
            RiscvOpcode::BRANCH => RiscvInstFormat::B,
            RiscvOpcode::LOADFP => RiscvInstFormat::Unsupported,
            RiscvOpcode::STOREFP => RiscvInstFormat::Unsupported,
            RiscvOpcode::MSUB => RiscvInstFormat::Unsupported,
            RiscvOpcode::JALR => RiscvInstFormat::I,
            RiscvOpcode::CUSTOM0 => RiscvInstFormat::Unsupported,
            RiscvOpcode::CUSTOM1 => RiscvInstFormat::Unsupported,
            RiscvOpcode::NMSUB => RiscvInstFormat::Unsupported,
            RiscvOpcode::MISCMEM => RiscvInstFormat::Unsupported,
            RiscvOpcode::AMO => RiscvInstFormat::Unsupported,
            RiscvOpcode::NMADD => RiscvInstFormat::Unsupported,
            RiscvOpcode::JAL => RiscvInstFormat::J,
            RiscvOpcode::OPIMM => RiscvInstFormat::I,
            RiscvOpcode::OP => RiscvInstFormat::R,
            RiscvOpcode::OPFP => RiscvInstFormat::Unsupported,
            RiscvOpcode::SYSTEM => RiscvInstFormat::Unsupported,
            RiscvOpcode::AUIPC => RiscvInstFormat::U,
            RiscvOpcode::LUI => RiscvInstFormat::U,
            RiscvOpcode::OPIMM32 => RiscvInstFormat::I,
            RiscvOpcode::OP32 => RiscvInstFormat::R,
            RiscvOpcode::CUSTOM2 => RiscvInstFormat::Unsupported,
            RiscvOpcode::CUSTOM3 => RiscvInstFormat::Unsupported,
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
    pub fn imm(&self) -> u64 {
        let format = self.format();
        let sign : u64 = if self.raw & 0x80000000 == 0 { 0 } else { 1 };

        match format {
            RiscvInstFormat::I => sign_ext32to64(
                bit_range_set(bit_repeat(sign, 21), BitRange(11, 31)) |
                bit_range_map(self.raw as u64, BitRange(25, 30), BitRange(5, 10)) |
                bit_range_map(self.raw as u64, BitRange(21, 24), BitRange(1, 4)) |
                bit_range_map(self.raw as u64, BitRange(20, 20), BitRange(0, 0))
            ),
            RiscvInstFormat::S => sign_ext32to64(
                bit_range_set(bit_repeat(sign, 21), BitRange(11, 31)) |
                bit_range_map(self.raw as u64, BitRange(25, 30), BitRange(5, 10)) |
                bit_range_map(self.raw as u64, BitRange(8, 11), BitRange(1, 4)) |
                bit_range_map(self.raw as u64, BitRange(7, 7), BitRange(0, 0))
            ),
            RiscvInstFormat::B => sign_ext32to64(
                bit_range_set(bit_repeat(sign, 21), BitRange(12, 31)) |
                bit_range_map(self.raw as u64, BitRange(7, 7), BitRange(11, 11)) |
                bit_range_map(self.raw as u64, BitRange(25, 30), BitRange(5, 10)) |
                bit_range_map(self.raw as u64, BitRange(8, 11), BitRange(1, 4))
            ),
            RiscvInstFormat::U => sign_ext32to64(
                bit_range_map(self.raw as u64, BitRange(12, 31), BitRange(12, 31))
            ),
            RiscvInstFormat::J => sign_ext32to64(
                bit_range_set(bit_repeat(sign, 12), BitRange(20, 31)) |
                bit_range_map(self.raw as u64, BitRange(12, 19), BitRange(12, 19)) |
                bit_range_map(self.raw as u64, BitRange(20, 20), BitRange(11, 11)) |
                bit_range_map(self.raw as u64, BitRange(25, 30), BitRange(5, 10)) |
                bit_range_map(self.raw as u64, BitRange(21, 24), BitRange(1, 4))
            ),
            _ => panic!("Unsupported immediate!")
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

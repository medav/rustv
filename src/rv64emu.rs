use std::fmt;

use crate::memif::*;
use crate::bitops::*;
use crate::rv64defs::*;
use crate::rv64inst::*;
use crate::rv64alu;


#[derive(Debug)]
pub struct ArchState {
    pub pc : u64,
    pub regs : [u64; 32]
}

#[derive(Debug)]
pub enum ExecResult {
    Continue,
    Trap,
    Halt
}

pub fn fetch_inst(arch : &ArchState, mem : &mut dyn MemIf) -> RawInst {
    let low = read16(mem, arch.pc);

    if low & 0b11 == 0b11 {
        let high = read16(mem, arch.pc + 2);
        RawInst { pc : arch.pc, raw : ((high << 16) as u32) | (low as u32) }
    }
    else {
        RawInst { pc : arch.pc, raw : low as u32 }
    }
}

#[inline(always)]
pub fn regr(arch : &ArchState, rnum : usize) -> u64 {
    match rnum {
        0 => 0,
        1..=31 => arch.regs[rnum],
        _ => panic!("Invalid register!")
    }
}

#[inline(always)]
pub fn regw(arch : &mut ArchState, rnum : usize, val : u64) {
    match rnum {
        0 => (),
        1..=31 => arch.regs[rnum] = val,
        _ => panic!("Invalid register!")
    }
}

pub fn exec_inst(
    arch : &mut ArchState, mem : &mut dyn MemIf, inst : &DecodedInst) -> ExecResult {

    use DecodedInst::*;
    use ExecResult::*;

    match inst {

        //
        // Op
        //

        Add  {rs1, rs2, rd} => {
            regw(arch, *rd, rv64alu::add(regr(arch, *rs1), regr(arch, *rs2)));
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },
        Sub  {rs1, rs2, rd} => {
            regw(arch, *rd, rv64alu::sub(regr(arch, *rs1), regr(arch, *rs2)));
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },
        Sll  {rs1, rs2, rd} => {
            regw(arch, *rd, rv64alu::sll(regr(arch, *rs1), regr(arch, *rs2)));
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },
        // Slt  {rs1, rs2, rd} => {

        // },
        // Sltu {rs1, rs2, rd} => {

        // },
        // Xor  {rs1, rs2, rd} => {

        // },
        // Srl  {rs1, rs2, rd} => {

        // },
        // Sra  {rs1, rs2, rd} => {

        // },
        // Or   {rs1, rs2, rd} => {

        // },
        // And  {rs1, rs2, rd} => {

        // },

        //
        // OpImm
        //

        Addi  {rs1, rd, imm} => {
            regw(arch, *rd, rv64alu::add(regr(arch, *rs1), *imm));
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },
        Subi  {rs1, rd, imm} => {
            regw(arch, *rd, rv64alu::sub(regr(arch, *rs1), *imm));
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },
        // Slti  {rs1, rd, imm} => {

        // },
        // Sltiu {rs1, rd, imm} => {

        // },
        // Xori  {rs1, rd, imm} => {

        // },
        // Ori   {rs1, rd, imm} => {

        // },
        // Andi  {rs1, rd, imm} => {

        // },
        // Slli  {rs1, rd, imm} => {

        // },
        // Srli  {rs1, rd, imm} => {

        // },
        // Srai  {rs1, rd, imm} => {

        // },

        Lui {rd, imm} => {
            regw(arch, *rd, *imm);
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },

        Jal {rd, imm} => {
            regw(arch, *rd, rv64alu::add(arch.pc, 4));
            arch.pc = rv64alu::add(arch.pc, *imm);
            Continue
        },

        Branch {func, rs1, rs2, imm} => {
            use BranchType::*;
            let pred = match func {
                Eq => regr(arch, *rs1) == regr(arch, *rs2),
                Neq => regr(arch, *rs1) != regr(arch, *rs2),
                Lt => (regr(arch, *rs1) as i64) < (regr(arch, *rs2) as i64),
                Ge => (regr(arch, *rs1) as i64) >= (regr(arch, *rs2) as i64),
                Ltu => regr(arch, *rs1) < regr(arch, *rs2),
                Geu => regr(arch, *rs1) >= regr(arch, *rs2),
            };

            if pred {
                arch.pc = rv64alu::add(arch.pc, *imm);
            }
            else {
                arch.pc = rv64alu::add(arch.pc, 4);
            }

            Continue
        },

        Load {width, rs1, rd, imm} => {
            let addr = rv64alu::add(regr(arch, *rs1), *imm);

            let val = match width {
                Byte => sign_ext64!(8, read8(mem, addr)),
                Half => sign_ext64!(16, read16(mem, addr)),
                Word => sign_ext64!(32, read32(mem, addr)),
                Double => read64(mem, addr),
                ByteU => read8(mem, addr),
                HalfU => read16(mem, addr),
                WordU => read32(mem, addr),
                _ => panic!("Unimplemented")
            };
            
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },

        Store {width, rs1, rs2, imm} => {
            let addr = rv64alu::add(regr(arch, *rs1), *imm);
            let val = regr(arch, *rs2);

            match width {
                Byte => write8(mem, addr, val.into()),
                Half => write16(mem, addr, val.into()),
                Word => write32(mem, addr, val.into()),
                Double => write64(mem, addr, val),
                _ => panic!("Unimplemented")
            };
            
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },

        //
        // Compressed Quandrant 0 Instructions
        //

        CAddi4spn {rd, imm} => {
            regw(arch, *rd, rv64alu::add(regr(arch, 2), *imm));
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        CStore {width, rs1, rs2, imm} => {
            let addr = rv64alu::add(regr(arch, *rs1), *imm);
            let val = regr(arch, *rs2);

            match width {
                Cfd => panic!("Unimplemented"),
                Cw => write32(mem, addr, val.into()),
                Cd => write64(mem, addr, val.into())
            };
            
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        //
        // Compressed Quandrant 1 Instructions
        //

        CAddi {rsrd, imm} => {
            regw(arch, *rsrd, rv64alu::add(regr(arch, *rsrd), *imm));
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        CAddiw {rsrd, imm} => {
            regw(arch, *rsrd, rv64alu::addw(regr(arch, *rsrd), *imm));
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        CLi {rd, imm} => {
            regw(arch, *rd, *imm);
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        CAddi16sp {imm} => {
            regw(arch, 2, rv64alu::add(regr(arch, 2), *imm));
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        CAddw {rsrd, rs2} => {
            regw(arch, *rsrd, rv64alu::add(regr(arch, *rsrd), regr(arch, *rs2)));
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },
        
        //
        // Compressed Quandrant 2 Instructions
        //

        CMv {rs1, rs2} => {
            regw(arch, *rs1, regr(arch, *rs2));
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        CSdsp {rs2, imm} => {
            let addr = regr(arch, 2) + *imm;
            write64(mem, addr, regr(arch, *rs2));
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        x => panic!("Unimplemented instruction: {:?}", x)
    }

}

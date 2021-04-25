use std::fmt;

use crate::syscalls::*;
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

#[derive(Debug, PartialEq)]
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

    macro_rules! op_inst {
        ($arch:ident, $rs1:expr, $rs2:expr, $rd:expr, $func:ident) => {
            {
                regw($arch, $rd, rv64alu::$func(regr($arch, $rs1), regr($arch, $rs2)));
                $arch.pc = rv64alu::add($arch.pc, 4);
                Continue
            }
        }
    }

    macro_rules! opimm_inst {
        ($arch:ident, $rs1:expr, $imm:expr, $rd:expr, $func:ident) => {
            {
                regw($arch, $rd, rv64alu::$func(regr($arch, $rs1), $imm));
                $arch.pc = rv64alu::add($arch.pc, 4);
                Continue
            }
        }
    }

    match inst {

        //
        // Op
        //

        Add {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, add),
        Sub {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, sub),
        Sll {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, sll),
        Slt {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, slt),
        Sltu {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, sltu),
        Xor {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, xor),
        Srl {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, srl),
        Sra {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, sra),
        Or {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, or),
        And {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, and),

        //
        // OpImm
        //

        Addi {rs1, imm, rd} => opimm_inst!(arch, *rs1, *imm, *rd, add),
        Subi {rs1, imm, rd} => opimm_inst!(arch, *rs1, *imm, *rd, sub),
        Slli {rs1, shamt, rd} => opimm_inst!(arch, *rs1, *shamt, *rd, sll),
        Slti {rs1, imm, rd} => opimm_inst!(arch, *rs1, *imm, *rd, slt),
        Sltiu {rs1, imm, rd} => opimm_inst!(arch, *rs1, *imm, *rd, sltu),
        Xori {rs1, imm, rd} => opimm_inst!(arch, *rs1, *imm, *rd, xor),
        Srli {rs1, shamt, rd} => opimm_inst!(arch, *rs1, *shamt, *rd, srl),
        Srai {rs1, shamt, rd} => opimm_inst!(arch, *rs1, *shamt, *rd, sra),
        Ori {rs1, imm, rd} => opimm_inst!(arch, *rs1, *imm, *rd, or),
        Andi {rs1, imm, rd} => opimm_inst!(arch, *rs1, *imm, *rd, and),

        //
        // Op32
        //

        Addw {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, addw),
        Subw {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, subw),
        Sllw {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, sllw),
        Srlw {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, srlw),
        Sraw {rs1, rs2, rd} => op_inst!(arch, *rs1, *rs2, *rd, sraw),

        //
        // OpImm32
        //

        Addiw {rs1, imm, rd} => opimm_inst!(arch, *rs1, *imm, *rd, addw),
        Subiw {rs1, imm, rd} => opimm_inst!(arch, *rs1, *imm, *rd, subw),
        Slliw {rs1, shamt, rd} => opimm_inst!(arch, *rs1, *shamt, *rd, sllw),
        Srliw {rs1, shamt, rd} => opimm_inst!(arch, *rs1, *shamt, *rd, srlw),
        Sraiw {rs1, shamt, rd} => opimm_inst!(arch, *rs1, *shamt, *rd, sraw),

        Lui {rd, imm} => {
            regw(arch, *rd, *imm);
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },

        Auipc {rd, imm} => {
            regw(arch, *rd, rv64alu::add(arch.pc, *imm));
            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        }

        Jal {rd, imm} => {
            regw(arch, *rd, rv64alu::add(arch.pc, 4));
            arch.pc = rv64alu::add(arch.pc, *imm);
            Continue
        },

        Jalr {rs1, rd, imm} => {
            let target = rv64alu::add(regr(arch, *rs1), *imm);
            let ra = rv64alu::add(arch.pc, 4);
            regw(arch, *rd, ra);
            arch.pc = target;
            // println!("---");
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
                LoadStoreWidth::Byte => sign_ext64!(8, read8(mem, addr)),
                LoadStoreWidth::Half => sign_ext64!(16, read16(mem, addr)),
                LoadStoreWidth::Word => sign_ext64!(32, read32(mem, addr)),
                LoadStoreWidth::Double => read64(mem, addr),
                LoadStoreWidth::ByteU => read8(mem, addr),
                LoadStoreWidth::HalfU => read16(mem, addr),
                LoadStoreWidth::WordU => read32(mem, addr),
                _ => panic!("Unimplemented")
            };

            println!("Load ({:?}) [{:x}] => {}", width, addr, val);
            regw(arch, *rd, val);

            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },

        Store {width, rs1, rs2, imm} => {
            let addr = rv64alu::add(regr(arch, *rs1), *imm);
            let val = regr(arch, *rs2);

            println!("Store ({:?}) [{:x}] <= {}", width, addr, val);

            match width {
                LoadStoreWidth::Byte => write8(mem, addr, val.into()),
                LoadStoreWidth::Half => write16(mem, addr, val.into()),
                LoadStoreWidth::Word => write32(mem, addr, val.into()),
                LoadStoreWidth::Double => write64(mem, addr, val),
                _ => panic!("Unimplemented")
            };

            arch.pc = rv64alu::add(arch.pc, 4);
            Continue
        },

        //
        // System instructions
        //

        ECall => {
            arch.pc = rv64alu::add(arch.pc, 4);
            Trap
        },

        EBreak => {
            Halt
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
                CLoadStoreWidth::Cfd => panic!("Unimplemented"),
                CLoadStoreWidth::Cw => write32(mem, addr, val.into()),
                CLoadStoreWidth::Cd => write64(mem, addr, val.into())
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


        CLdsp {rd, imm} => {
            let addr = regr(arch, 2) + *imm;
            let val = read64(mem, addr);
            regw(arch, *rd, val);
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        CJr {rs1} => {
            arch.pc = regr(arch, *rs1);
            Continue
        }

        CMv {rs1, rs2} => {
            regw(arch, *rs1, regr(arch, *rs2));
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        CEBreak => {
            Halt
        }

        CSdsp {rs2, imm} => {
            let addr = regr(arch, 2) + *imm;
            write64(mem, addr, regr(arch, *rs2));
            arch.pc = rv64alu::add(arch.pc, 2);
            Continue
        },

        x => panic!("Unimplemented instruction: {:?}", x)
    }

}

pub fn rv64_parse_syscall(arch : &mut ArchState) -> Syscall {
    let raw_num = regr(arch, 17);

    Syscall {
        num : num::FromPrimitive::from_u64(raw_num)
            .expect(&format!("Unknown syscall: {}", raw_num)),
        args : [
            regr(arch, 10),
            regr(arch, 11),
            regr(arch, 12),
            regr(arch, 13),
            regr(arch, 14),
            regr(arch, 15),
            regr(arch, 16),
        ]
    }
}

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

impl ArchState {
    pub fn fetch_inst(&self, mem : &mut dyn MemIf) -> RawInst {
        let low = read16(mem, self.pc);

        if low & 0b11 == 0b11 {
            let high = read16(mem, self.pc + 2);
            RawInst { pc : self.pc, raw : ((high << 16) as u32) | (low as u32) }
        }
        else {
            RawInst { pc : self.pc, raw : low as u32 }
        }
    }

    #[inline(always)]
    pub fn regr(&self, rnum : usize) -> u64 {
        match rnum {
            0 => 0,
            1..=31 => self.regs[rnum],
            _ => panic!("Invalid register!")
        }
    }

    #[inline(always)]
    pub fn regw(&mut self, rnum : usize, val : u64) {
        if rnum == 2 {
            println!("        sp <= {:08x}", val)
        }
        match rnum {
            0 => (),
            1..=31 => self.regs[rnum] = val,
            _ => panic!("Invalid register!")
        }
    }

    pub fn exec_inst(
        &mut self, mem : &mut dyn MemIf, inst : &DecodedInst) -> ExecResult {

        use DecodedInst::*;
        use ExecResult::*;

        macro_rules! op_inst {
            ($rs1:expr, $rs2:expr, $rd:expr, $func:ident) => {
                {
                    self.regw($rd, rv64alu::$func(self.regr($rs1), self.regr($rs2)));
                    self.pc = rv64alu::add(self.pc, 4);
                    Continue
                }
            }
        }

        macro_rules! c_op_inst {
            ($rs1:expr, $rs2:expr, $rd:expr, $func:ident) => {
                {
                    self.regw($rd, rv64alu::$func(self.regr($rs1), self.regr($rs2)));
                    self.pc = rv64alu::add(self.pc, 2);
                    Continue
                }
            }
        }

        macro_rules! opimm_inst {
            ($rs1:expr, $imm:expr, $rd:expr, $func:ident) => {
                {
                    self.regw($rd, rv64alu::$func(self.regr($rs1), $imm));
                    self.pc = rv64alu::add(self.pc, 4);
                    Continue
                }
            }
        }

        macro_rules! c_opimm_inst {
            ($rs1:expr, $imm:expr, $rd:expr, $func:ident) => {
                {
                    self.regw($rd, rv64alu::$func(self.regr($rs1), $imm));
                    self.pc = rv64alu::add(self.pc, 2);
                    Continue
                }
            }
        }

        match inst {

            //
            // Op
            //

            Add {rs1, rs2, rd} =>  op_inst!(*rs1, *rs2, *rd, add),
            Sub {rs1, rs2, rd} =>  op_inst!(*rs1, *rs2, *rd, sub),
            Sll {rs1, rs2, rd} =>  op_inst!(*rs1, *rs2, *rd, sll),
            Slt {rs1, rs2, rd} =>  op_inst!(*rs1, *rs2, *rd, slt),
            Sltu {rs1, rs2, rd} => op_inst!(*rs1, *rs2, *rd, sltu),
            Xor {rs1, rs2, rd} =>  op_inst!(*rs1, *rs2, *rd, xor),
            Srl {rs1, rs2, rd} =>  op_inst!(*rs1, *rs2, *rd, srl),
            Sra {rs1, rs2, rd} =>  op_inst!(*rs1, *rs2, *rd, sra),
            Or {rs1, rs2, rd} =>   op_inst!(*rs1, *rs2, *rd, or),
            And {rs1, rs2, rd} =>  op_inst!(*rs1, *rs2, *rd, and),

            //
            // OpImm
            //

            Addi {rs1, imm, rd} =>   opimm_inst!(*rs1, *imm, *rd, add),
            Subi {rs1, imm, rd} =>   opimm_inst!(*rs1, *imm, *rd, sub),
            Slli {rs1, shamt, rd} => opimm_inst!(*rs1, *shamt, *rd, sll),
            Slti {rs1, imm, rd} =>   opimm_inst!(*rs1, *imm, *rd, slt),
            Sltiu {rs1, imm, rd} =>  opimm_inst!(*rs1, *imm, *rd, sltu),
            Xori {rs1, imm, rd} =>   opimm_inst!(*rs1, *imm, *rd, xor),
            Srli {rs1, shamt, rd} => opimm_inst!(*rs1, *shamt, *rd, srl),
            Srai {rs1, shamt, rd} => opimm_inst!(*rs1, *shamt, *rd, sra),
            Ori {rs1, imm, rd} =>    opimm_inst!(*rs1, *imm, *rd, or),
            Andi {rs1, imm, rd} =>   opimm_inst!(*rs1, *imm, *rd, and),

            //
            // Op32
            //

            Addw {rs1, rs2, rd} => op_inst!(*rs1, *rs2, *rd, addw),
            Subw {rs1, rs2, rd} => op_inst!(*rs1, *rs2, *rd, subw),
            Sllw {rs1, rs2, rd} => op_inst!(*rs1, *rs2, *rd, sllw),
            Srlw {rs1, rs2, rd} => op_inst!(*rs1, *rs2, *rd, srlw),
            Sraw {rs1, rs2, rd} => op_inst!(*rs1, *rs2, *rd, sraw),

            //
            // OpImm32
            //

            Addiw {rs1, imm, rd} =>   opimm_inst!(*rs1, *imm, *rd, addw),
            Subiw {rs1, imm, rd} =>   opimm_inst!(*rs1, *imm, *rd, subw),
            Slliw {rs1, shamt, rd} => opimm_inst!(*rs1, *shamt, *rd, sllw),
            Srliw {rs1, shamt, rd} => opimm_inst!(*rs1, *shamt, *rd, srlw),
            Sraiw {rs1, shamt, rd} => opimm_inst!(*rs1, *shamt, *rd, sraw),

            Lui {rd, imm} => {
                self.regw(*rd, *imm);
                self.pc = rv64alu::add(self.pc, 4);
                Continue
            },

            Auipc {rd, imm} => {
                self.regw(*rd, rv64alu::add(self.pc, *imm));
                self.pc = rv64alu::add(self.pc, 4);
                Continue
            }

            Jal {rd, imm} => {
                self.regw(*rd, rv64alu::add(self.pc, 4));
                self.pc = rv64alu::add(self.pc, *imm);
                Continue
            },

            Jalr {rs1, rd, imm} => {
                let target = rv64alu::add(self.regr(*rs1), *imm);
                let ra = rv64alu::add(self.pc, 4);
                self.regw(*rd, ra);
                self.pc = target;
                // println!("---");
                Continue
            },

            Branch {func, rs1, rs2, imm} => {
                use BranchType::*;
                let pred = match func {
                    Eq => self.regr(*rs1) == self.regr(*rs2),
                    Neq => self.regr(*rs1) != self.regr(*rs2),
                    Lt => (self.regr(*rs1) as i64) < (self.regr(*rs2) as i64),
                    Ge => (self.regr(*rs1) as i64) >= (self.regr(*rs2) as i64),
                    Ltu => self.regr(*rs1) < self.regr(*rs2),
                    Geu => self.regr(*rs1) >= self.regr(*rs2),
                };

                if pred {
                    self.pc = rv64alu::add(self.pc, *imm);
                }
                else {
                    self.pc = rv64alu::add(self.pc, 4);
                }

                Continue
            },

            Load {width, rs1, rd, imm} => {
                let addr = rv64alu::add(self.regr(*rs1), *imm);

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

                println!("        Load ({:?}) [{:x}] => {}", width, addr, val);
                self.regw(*rd, val);

                self.pc = rv64alu::add(self.pc, 4);
                Continue
            },

            Store {width, rs1, rs2, imm} => {
                let addr = rv64alu::add(self.regr(*rs1), *imm);
                let val = self.regr(*rs2);

                println!("        Store ({:?}) [{:x}] <= {}", width, addr, val);

                match width {
                    LoadStoreWidth::Byte => write8(mem, addr, val.into()),
                    LoadStoreWidth::Half => write16(mem, addr, val.into()),
                    LoadStoreWidth::Word => write32(mem, addr, val.into()),
                    LoadStoreWidth::Double => write64(mem, addr, val),
                    _ => panic!("Unimplemented")
                };

                self.pc = rv64alu::add(self.pc, 4);
                Continue
            },

            //
            // System instructions
            //

            ECall => {
                self.pc = rv64alu::add(self.pc, 4);
                Trap
            },

            EBreak => {
                Halt
            },

            //
            // Compressed Quandrant 0 Instructions
            //

            CAddi4spn {rd, imm} => {
                self.regw(*rd, rv64alu::add(self.regr(2), *imm));
                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CLoad {width, rs1, rd, imm} => {
                let addr = rv64alu::add(self.regr(*rs1), *imm);

                let val = match width {
                    CLoadStoreWidth::Cfd => panic!("Unimplemented"),
                    CLoadStoreWidth::Cw => read32(mem, addr),
                    CLoadStoreWidth::Cd => read64(mem, addr)
                };

                println!("        Load ({:?}) [{:x}] => {}", width, addr, val);
                self.regw(*rd, val);

                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CStore {width, rs1, rs2, imm} => {
                let addr = rv64alu::add(self.regr(*rs1), *imm);
                let val = self.regr(*rs2);

                match width {
                    CLoadStoreWidth::Cfd => panic!("Unimplemented"),
                    CLoadStoreWidth::Cw => write32(mem, addr, val.into()),
                    CLoadStoreWidth::Cd => write64(mem, addr, val.into())
                };

                println!("        Store ({:?}) [{:x}] <= {}", width, addr, val);

                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            //
            // Compressed Quandrant 1 Instructions
            //

            CAddi {rsrd, imm} => {
                self.regw(*rsrd, rv64alu::add(self.regr(*rsrd), *imm));
                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CAndi {rsrd, imm} => {
                self.regw(*rsrd, rv64alu::and(self.regr(*rsrd), *imm));
                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CAddiw {rsrd, imm} => {
                self.regw(*rsrd, rv64alu::addw(self.regr(*rsrd), *imm));
                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CLi {rd, imm} => {
                self.regw(*rd, *imm);
                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CLui {rd, imm} => {
                self.regw(*rd, *imm);
                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CAddi16sp {imm} => {
                self.regw(2, rv64alu::add(self.regr(2), *imm));
                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CAdd {rsrd, rs2} => c_op_inst!(*rsrd, *rs2, *rsrd, add),
            CAddw {rsrd, rs2} => c_op_inst!(*rsrd, *rs2, *rsrd, addw),
            CSub {rsrd, rs2} => c_op_inst!(*rsrd, *rs2, *rsrd, sub),
            CSubw {rsrd, rs2} => c_op_inst!(*rsrd, *rs2, *rsrd, subw),

            COr {rsrd, rs2} => c_op_inst!(*rsrd, *rs2, *rsrd, or),
            CAnd {rsrd, rs2} => c_op_inst!(*rsrd, *rs2, *rsrd, and),
            CXor {rsrd, rs2} => c_op_inst!(*rsrd, *rs2, *rsrd, xor),

            CBeqz {rs1, imm} => {
                if self.regr(*rs1) == 0 {
                    self.pc = rv64alu::add(self.pc, *imm);
                }
                else {
                    self.pc = rv64alu::add(self.pc, 2);
                }

                Continue
            },

            CBnez {rs1, imm} => {
                if self.regr(*rs1) != 0 {
                    self.pc = rv64alu::add(self.pc, *imm);
                }
                else {
                    self.pc = rv64alu::add(self.pc, 2);
                }

                Continue
            },

            CJ {imm} => {
                self.pc = rv64alu::add(self.pc, *imm);
                Continue
            },


            CJal {imm} => {
                let next_pc = rv64alu::add(self.pc, 2);
                self.pc = rv64alu::add(self.pc, *imm);
                self.regw(1, next_pc);
                Continue
            },

            //
            // Compressed Quandrant 2 Instructions
            //

            CSlli {rsrd, shamt} =>
                c_opimm_inst!(*rsrd, *shamt, *rsrd, sll),

            CSrli {rsrd, shamt} =>
                c_opimm_inst!(*rsrd, *shamt, *rsrd, srl),


            CLoadStack {width, rd, imm} => {
                let addr = self.regr(2) + *imm;

                let val = match width {
                    CLoadStoreWidth::Cfd => panic!("Unimplemented"),
                    CLoadStoreWidth::Cw => read32(mem, addr),
                    CLoadStoreWidth::Cd => read64(mem, addr)
                };

                println!("        Load ({:?}) [{:x}] => {}", width, addr, val);
                self.regw(*rd, val);

                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CStoreStack {width, rs2, imm} => {
                let addr = rv64alu::add(self.regr(2), *imm);
                let val = self.regr(*rs2);

                match width {
                    CLoadStoreWidth::Cfd => panic!("Unimplemented"),
                    CLoadStoreWidth::Cw => write32(mem, addr, val.into()),
                    CLoadStoreWidth::Cd => write64(mem, addr, val.into())
                };

                println!("        Store ({:?}) [{:x}] <= {}", width, addr, val);

                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CJr {rs1} => {
                self.pc = self.regr(*rs1);
                Continue
            },


            CJalr {rs1} => {
                let next_pc = rv64alu::add(self.pc, 2);
                self.pc = self.regr(*rs1);
                self.regw(1, next_pc);
                Continue
            },

            CMv {rsrd, rs2} => {
                self.regw(*rsrd, self.regr(*rs2));
                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            CEBreak => {
                Halt
            }

            CSdsp {rs2, imm} => {
                let addr = self.regr(2) + *imm;
                write64(mem, addr, self.regr(*rs2));
                self.pc = rv64alu::add(self.pc, 2);
                Continue
            },

            x => panic!("Unimplemented instruction: {:?}", x)
        }

    }

    pub fn rv64_parse_syscall(&self) -> Syscall {
        let raw_num = self.regr(17);

        Syscall {
            num : num::FromPrimitive::from_u64(raw_num)
                .expect(&format!("Unknown syscall: {}", raw_num)),
            args : [
                self.regr(10),
                self.regr(11),
                self.regr(12),
                self.regr(13),
                self.regr(14),
                self.regr(15),
                self.regr(16),
            ]
        }
    }

}

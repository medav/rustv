use std::fmt;

use crate::bitops::*;
use crate::rv64defs::*;

#[inline(always)]
pub fn opt_creg_to_reg(creg : Option<usize>) -> Option<usize> {
    match creg {
        Some(rn) => Some(8 + rn),
        None => None
    }
}

macro_rules! immgen {
    (I, $v:expr) => {
        sign_ext64!(12,
            bit_range_map!($v as u64, (31, 31), (11, 11)) |
            bit_range_map!($v as u64, (25, 30), (5, 10)) |
            bit_range_map!($v as u64, (21, 24), (1, 4)) |
            bit_range_map!($v as u64, (20, 20), (0, 0))
        )
    };
    (S, $v:expr) => {
        sign_ext64!(12,
            bit_range_map!($v as u64, (31, 31), (11, 11)) |
            bit_range_map!($v as u64, (25, 30), (5, 10)) |
            bit_range_map!($v as u64, (8, 11), (1, 4)) |
            bit_range_map!($v as u64, (7, 7), (0, 0))
        )
    };
    (B, $v:expr) => {
        sign_ext64!(13,
            bit_range_map!($v as u64, (31, 31), (12, 12)) |
            bit_range_map!($v as u64, (7, 7), (11, 11)) |
            bit_range_map!($v as u64, (25, 30), (5, 10)) |
            bit_range_map!($v as u64, (8, 11), (1, 4))
        )
    };
    (U, $v:expr) => {
        sign_ext64!(32,
            bit_range_map!($v as u64, (12, 31), (12, 31)))
    };
    (J, $v:expr) => {
        sign_ext64!(21,
            bit_range_map!($v as u64, (31, 31), (20, 20)) |
            bit_range_map!($v as u64, (12, 19), (12, 19)) |
            bit_range_map!($v as u64, (20, 20), (11, 11)) |
            bit_range_map!($v as u64, (25, 30), (5, 10)) |
            bit_range_map!($v as u64, (21, 24), (1, 4))
        )
    };

    //
    // C2 Compressed Instructions
    //

    (C2_LWSP, $v:expr) => {
        bit_range_map!($v as u64, (2, 3),   (6, 7)) |
        bit_range_map!($v as u64, (12, 12), (5, 5)) |
        bit_range_map!($v as u64, (4, 6),   (2, 4))
    };
    (C2_LDSP, $v:expr) => {
        bit_range_map!($v as u64, (2, 4),   (6, 8)) |
        bit_range_map!($v as u64, (12, 12), (5, 5)) |
        bit_range_map!($v as u64, (5, 6),   (3, 4))
    };
    (C2_LQSP, $v:expr) => {
        bit_range_map!($v as u64, (2, 5),   (6, 9)) |
        bit_range_map!($v as u64, (12, 12), (5, 5)) |
        bit_range_map!($v as u64, (6, 6),   (4, 4))
    };
    (C2_SWSP, $v:expr) => {
        bit_range_map!($v as u64, (7, 8),  (6, 7)) |
        bit_range_map!($v as u64, (9, 12), (2, 5))
    };
    (C2_SDSP, $v:expr) => {
        bit_range_map!($v as u64, (7, 9),   (6, 8)) |
        bit_range_map!($v as u64, (10, 12), (3, 5))
    };
    (C2_SQSP, $v:expr) => {
        bit_range_map!($v as u64, (7, 10),  (6, 9)) |
        bit_range_map!($v as u64, (11, 12), (4, 5))
    };
    (C2_SLLI, $v:expr) => {
        sign_ext64!(6,
            bit_range_map!($v as u64, (2, 6), (0, 4)) |
            bit_range_map!($v as u64, (12, 12), (5, 5))
        )
    };

    //
    // C0 Compressed Instructions
    //

    (C0_LSW, $v:expr) => {
        bit_range_map!($v as u64, (5, 5),   (6, 6)) |
        bit_range_map!($v as u64, (10, 12), (3, 5)) |
        bit_range_map!($v as u64, (6, 6),   (2, 2))
    };
    (C0_LSD, $v:expr) => {
        bit_range_map!($v as u64, (5, 6),   (6, 7)) |
        bit_range_map!($v as u64, (10, 12), (3, 5))
    };
    // (C0_LSQ, $v:expr) => {
    //     bit_range_map!($v as u64, (5, 6),  (6, 7)) |
    //     bit_range_map!($v as u64, (10, 12), (3, 5))
    // };

    (C0_ADDI4SPN, $v:expr) => {
        bit_range_map!($v as u64, (5, 5), (3, 3)) |
        bit_range_map!($v as u64, (6, 6), (2, 2)) |
        bit_range_map!($v as u64, (7, 10), (6, 9)) |
        bit_range_map!($v as u64, (11, 12), (4, 5))
    };


    //
    // C1 Compressed Instructions
    //

    (C1_J_JAL, $v:expr) => {
        sign_ext64!(12,
            bit_range_map!($v as u64, (2, 2), (5, 5)) |
            bit_range_map!($v as u64, (3, 5), (1, 3)) |
            bit_range_map!($v as u64, (6, 6), (7, 7)) |
            bit_range_map!($v as u64, (7, 7), (6, 6)) |
            bit_range_map!($v as u64, (8, 8), (10, 10)) |
            bit_range_map!($v as u64, (9, 10), (8, 9)) |
            bit_range_map!($v as u64, (11, 11), (4, 4)) |
            bit_range_map!($v as u64, (12, 12), (11, 11))
        )
    };
    (C1_BRA, $v:expr) => {
        sign_ext64!(9,
            bit_range_map!($v as u64, (12, 12), (8, 8)) |
            bit_range_map!($v as u64, (10, 11), (3, 4)) |
            bit_range_map!($v as u64, (5, 6), (6, 7)) |
            bit_range_map!($v as u64, (3, 4), (1, 2)) |
            bit_range_map!($v as u64, (2, 2), (5, 5))
        )
    };
    (C1_LI, $v:expr) => {
        sign_ext64!(6,
            bit_range_map!($v as u64, (2, 6), (0, 4)) |
            bit_range_map!($v as u64, (12, 12), (5, 5))
        )
    };
    (C1_LUI, $v:expr) => {
        sign_ext64!(18,
            bit_range_map!($v as u64, (2, 6), (12, 16)) |
            bit_range_map!($v as u64, (12, 12), (17, 17))
        )
    };
    (C1_OPIMM, $v:expr) => {
        sign_ext64!(6,
            bit_range_map!($v as u64, (2, 6), (0, 4)) |
            bit_range_map!($v as u64, (12, 12), (5, 5))
        )
    };
    (C1_ADDI16SP, $v:expr) => {
        sign_ext64!(6,
            bit_range_map!($v as u64, (2, 2), (5, 5)) |
            bit_range_map!($v as u64, (3, 4), (7, 8)) |
            bit_range_map!($v as u64, (5, 5), (6, 6)) |
            bit_range_map!($v as u64, (6, 6), (4, 4)) |
            bit_range_map!($v as u64, (12, 12), (9, 9))
        )
    };

    //
    // C2 Compressed Instructions
    //

    (C2_LD, $v:expr) => {
        bit_range_map!($v as u64, (2, 4), (6, 8)) |
        bit_range_map!($v as u64, (5, 6), (3, 4)) |
        bit_range_map!($v as u64, (12, 12), (5, 5))
    };
    (C2_LW, $v:expr) => {
        bit_range_map!($v as u64, (2, 3), (6, 7)) |
        bit_range_map!($v as u64, (4, 6), (2, 4)) |
        bit_range_map!($v as u64, (12, 12), (5, 5))
    };
    (C2_SD, $v:expr) => {
        bit_range_map!($v as u64, (7, 9), (6, 8)) |
        bit_range_map!($v as u64, (10, 12), (3, 5))
    };
    (C2_SW, $v:expr) => {
        bit_range_map!($v as u64, (7, 8), (6, 7)) |
        bit_range_map!($v as u64, (9, 12), (2, 5))
    };
}

#[inline(always)]
fn pre_decode(rinst : &RawInst) -> InstSpec {
    use InstOpcode::*;

    match rinst.raw & 0b11 {
        0 => InstSpec(C0, bit_range_get!(rinst.raw, (13, 15)) as usize),
        1 => InstSpec(C1, bit_range_get!(rinst.raw, (13, 15)) as usize),
        2 => InstSpec(C2, bit_range_get!(rinst.raw, (13, 15)) as usize),
        _ => InstSpec(
            num::FromPrimitive::from_u32(rinst.raw & 0b1111111)
                .expect("Unknown opcode!"),
            bit_range_get!(rinst.raw, (12, 14)) as usize),
    }
}

#[inline(always)]
fn rs1(rinst : &RawInst) -> usize {
    bit_range_get!(rinst.raw, (15, 19)) as usize
}

#[inline(always)]
fn rs2(rinst : &RawInst) -> usize {
    bit_range_get!(rinst.raw, (20, 24)) as usize
}

#[inline(always)]
fn rd(rinst : &RawInst) -> usize {
    bit_range_get!(rinst.raw, (7, 11)) as usize
}

#[inline(always)]
fn funct7_32(rinst : &RawInst) -> u64 {
    bit_range_get!(rinst.raw, (25, 31)) as u64
}

#[inline(always)]
fn rs1_c(rinst : &RawInst) -> usize {
    bit_range_get!(rinst.raw, (7, 9)) as usize
}

#[inline(always)]
fn rs2_c(rinst : &RawInst) -> usize {
    bit_range_get!(rinst.raw, (2, 4)) as usize
}

#[inline(always)]
pub fn decode(rinst : &RawInst) -> DecodedInst {
    let spec = pre_decode(rinst);

    match spec {
        //
        // Base Integer Instructions
        //

        InstSpec(InstOpcode::OP, 0) => {
            let funct7 = bit_range_get!(rinst.raw, (25, 31));
            match funct7 {
                0b0000000 => DecodedInst::Add {
                    rs1 : rs1(rinst),
                    rs2 : rs2(rinst),
                    rd : rd(rinst)
                },
                0b0100000 => DecodedInst::Sub {
                    rs1 : rs1(rinst),
                    rs2 : rs2(rinst),
                    rd : rd(rinst)
                },
                _ => panic!("Invalid funct7!")
            }

        },
        InstSpec(InstOpcode::OP, 1) => DecodedInst::Sll {
            rs1 : rs1(rinst),
            rs2 : rs2(rinst),
            rd : rd(rinst)
        },
        InstSpec(InstOpcode::OP, 2) => DecodedInst::Slt {
            rs1 : rs1(rinst),
            rs2 : rs2(rinst),
            rd : rd(rinst)
        },
        InstSpec(InstOpcode::OP, 3) => DecodedInst::Sltu {
            rs1 : rs1(rinst),
            rs2 : rs2(rinst),
            rd : rd(rinst)
        },
        InstSpec(InstOpcode::OP, 4) => DecodedInst::Xor {
            rs1 : rs1(rinst),
            rs2 : rs2(rinst),
            rd : rd(rinst)
        },
        InstSpec(InstOpcode::OP, 5) => {
            let funct7 = bit_range_get!(rinst.raw, (25, 31));
            match funct7 {
                0b0000000 => DecodedInst::Srl {
                    rs1 : rs1(rinst),
                    rs2 : rs2(rinst),
                    rd : rd(rinst)
                },
                0b0100000 => DecodedInst::Sra {
                    rs1 : rs1(rinst),
                    rs2 : rs2(rinst),
                    rd : rd(rinst)
                },
                _ => panic!("Invalid funct7!")
            }
        },
        InstSpec(InstOpcode::OP, 6) => DecodedInst::Or {
            rs1 : rs1(rinst),
            rs2 : rs2(rinst),
            rd : rd(rinst)
        },
        InstSpec(InstOpcode::OP, 7) => DecodedInst::And {
            rs1 : rs1(rinst),
            rs2 : rs2(rinst),
            rd : rd(rinst)
        },

        //
        // InstOpcode::Op32
        //

        InstSpec(InstOpcode::OP32, 0) => {
            let funct7 = bit_range_get!(rinst.raw, (25, 31));
            match funct7 {
                0b0000000 => DecodedInst::Addw {
                    rs1 : rs1(rinst),
                    rs2 : rs2(rinst),
                    rd : rd(rinst)
                },
                0b0100000 => DecodedInst::Subw {
                    rs1 : rs1(rinst),
                    rs2 : rs2(rinst),
                    rd : rd(rinst)
                },
                _ => panic!("Invalid funct7!")
            }

        },
        InstSpec(InstOpcode::OP32, 1) => DecodedInst::Sllw {
            rs1 : rs1(rinst),
            rs2 : rs2(rinst),
            rd : rd(rinst)
        },
        InstSpec(InstOpcode::OP32, 5) => {
            let funct7 = bit_range_get!(rinst.raw, (25, 31));
            match funct7 {
                0b0000000 => DecodedInst::Srlw {
                    rs1 : rs1(rinst),
                    rs2 : rs2(rinst),
                    rd : rd(rinst)
                },
                0b0100000 => DecodedInst::Sraw {
                    rs1 : rs1(rinst),
                    rs2 : rs2(rinst),
                    rd : rd(rinst)
                },
                _ => panic!("Invalid funct7: {}", funct7)
            }
        },

        //
        // InstOpcode::OpImm
        //

        InstSpec(InstOpcode::OPIMM, 0) => DecodedInst::Addi {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            imm : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::OPIMM, 2) => DecodedInst::Slti {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            imm : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::OPIMM, 3) => DecodedInst::Sltiu {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            imm : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::OPIMM, 4) => DecodedInst::Xori {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            imm : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::OPIMM, 6) => DecodedInst::Ori {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            imm : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::OPIMM, 7) => DecodedInst::Andi {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            imm : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::OPIMM, 1) => DecodedInst::Slli {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            shamt : immgen!(I, rinst.raw) & 0b111111
        },
        InstSpec(InstOpcode::OPIMM, 1) => DecodedInst::Slli {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            shamt : immgen!(I, rinst.raw) & 0b111111
        },
        InstSpec(InstOpcode::OPIMM, 5) => {
            let funct6 = bit_range_get!(rinst.raw, (26, 31));
            match funct6 {
                0b000000 => DecodedInst::Srli {
                    rs1 : rs1(rinst),
                    rd : rd(rinst),
                    shamt : immgen!(I, rinst.raw) & 0b111111
                },
                0b010000 => DecodedInst::Srai {
                    rs1 : rs1(rinst),
                    rd : rd(rinst),
                    shamt : immgen!(I, rinst.raw) & 0b111111
                },
                _ => panic!("Invalid funct6: {}", funct6)
            }
        },

        //
        // InstOpcode::OpImm32
        //

        InstSpec(InstOpcode::OPIMM32, 0) => DecodedInst::Addiw {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            imm : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::OP32, 1) => DecodedInst::Slliw {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            shamt : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::OP32, 5) => {
            let funct7 = bit_range_get!(rinst.raw, (25, 31));
            match funct7 {
                0b0000000 => DecodedInst::Srliw {
                    rs1 : rs1(rinst),
                    rd : rd(rinst),
                    shamt : immgen!(I, rinst.raw) & 0b11111
                },
                0b0100000 => DecodedInst::Sraiw {
                    rs1 : rs1(rinst),
                    rd : rd(rinst),
                    shamt : immgen!(I, rinst.raw) & 0b11111
                },
                _ => panic!("Invalid funct7!")
            }
        },

        //
        // Other Regular Instructions
        //

        InstSpec(InstOpcode::LUI, _) => DecodedInst::Lui {
            rd : rd(rinst),
            imm : immgen!(U, rinst.raw)
        },
        InstSpec(InstOpcode::AUIPC, _) => DecodedInst::Auipc {
            rd : rd(rinst),
            imm : immgen!(U, rinst.raw)
        },
        InstSpec(InstOpcode::JAL, _) => DecodedInst::Jal {
            rd : rd(rinst),
            imm : immgen!(J, rinst.raw)
        },
        InstSpec(InstOpcode::JALR, _) => DecodedInst::Jalr {
            rs1 : rs1(rinst),
            rd : rd(rinst),
            imm : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::BRANCH, funct3) => DecodedInst::Branch {
            func : num::FromPrimitive::from_usize(funct3).expect("Unknown funct!"),
            rs1 : rs1(rinst),
            rs2 : rs2(rinst),
            imm : immgen!(B, rinst.raw)
        },
        InstSpec(InstOpcode::LOAD, funct3) => DecodedInst::Load {
            width : num::FromPrimitive::from_usize(funct3).expect("Unknown funct!"),
            rs1 : rs1(rinst),
            rd : rd(rinst),
            imm : immgen!(I, rinst.raw)
        },
        InstSpec(InstOpcode::STORE, funct3) => DecodedInst::Store {
            width : num::FromPrimitive::from_usize(funct3).expect("Unknown funct!"),
            rs1 : rs1(rinst),
            rs2 : rs2(rinst),
            imm : immgen!(S, rinst.raw)
        },
        InstSpec(InstOpcode::SYSTEM, 0) => {
            let rs1 = rs1(rinst);
            let rs2 = rs2(rinst);
            let imm = immgen!(U, rinst.raw);

            println!("{}, {}, {}", rs1, rs2, imm);

            match (rs1, rs2, imm) {
                (0, 0, 0) => DecodedInst::ECall,
                (0, 0, 1) => DecodedInst::EBreak,
                _ => DecodedInst::EBreak //panic!("Invalid decode for InstOpcode::SYSTEM!")
            }
        }

        //
        // Compressed Quandrant 0 Instructions
        //

        InstSpec(InstOpcode::C0, 0) => DecodedInst::CAddi4spn {
            rd : rs2_c(rinst) + 8,
            imm : immgen!(C0_ADDI4SPN, rinst.raw)
        },
        InstSpec(InstOpcode::C0, 1) => DecodedInst::CLoad {
            width : CLoadStoreWidth::Cfd,
            rs1 : rs1_c(rinst) + 8,
            rd : rs2_c(rinst) + 8,
            imm : immgen!(C0_LSD, rinst.raw)
        },
        InstSpec(InstOpcode::C0, 2) => DecodedInst::CLoad {
            width : CLoadStoreWidth::Cw,
            rs1 : rs1_c(rinst) + 8,
            rd : rs2_c(rinst) + 8,
            imm : immgen!(C0_LSW, rinst.raw)
        },
        InstSpec(InstOpcode::C0, 3) => DecodedInst::CLoad {
            width : CLoadStoreWidth::Cd,
            rs1 : rs1_c(rinst) + 8,
            rd : rs2_c(rinst) + 8,
            imm : immgen!(C0_LSD, rinst.raw)
        },
        InstSpec(InstOpcode::C0, 5) => DecodedInst::CStore {
            width : CLoadStoreWidth::Cfd,
            rs1 : rs1_c(rinst) + 8,
            rs2 : rs2_c(rinst) + 8,
            imm : immgen!(C0_LSD, rinst.raw)
        },
        InstSpec(InstOpcode::C0, 6) => DecodedInst::CStore {
            width : CLoadStoreWidth::Cw,
            rs1 : rs1_c(rinst) + 8,
            rs2 : rs2_c(rinst) + 8,
            imm : immgen!(C0_LSW, rinst.raw)
        },
        InstSpec(InstOpcode::C0, 7) => DecodedInst::CStore {
            width : CLoadStoreWidth::Cd,
            rs1 : rs1_c(rinst) + 8,
            rs2 : rs2_c(rinst) + 8,
            imm : immgen!(C0_LSD, rinst.raw)
        },

        //
        // Compressed Quandrant 1 Instructions
        //

        InstSpec(InstOpcode::C1, 0) => DecodedInst::CAddi {
            rsrd : rd(rinst),
            imm : immgen!(C1_OPIMM, rinst.raw)
        },
        InstSpec(InstOpcode::C1, 1) => DecodedInst::CAddiw {
            rsrd : rd(rinst),
            imm : immgen!(C1_OPIMM, rinst.raw)
        },
        InstSpec(InstOpcode::C1, 2) => DecodedInst::CLi {
            rd : rd(rinst),
            imm : immgen!(C1_OPIMM, rinst.raw)
        },
        InstSpec(InstOpcode::C1, 3) => {
            let rd = rd(rinst);
            match rd {
                2 => DecodedInst::CAddi16sp {
                    imm : immgen!(C1_ADDI16SP, rinst.raw)
                },
                n => DecodedInst::CLui {
                    imm : immgen!(C1_LUI, rinst.raw)
                }
            }
        },
        InstSpec(InstOpcode::C1, 4) => {
            let bit12 = bit_range_get!(rinst.raw, (12, 12));
            let bit10_11 = bit_range_get!(rinst.raw, (10, 11));
            let bit5_6 = bit_range_get!(rinst.raw, (5, 6));
            let rsrd = bit_range_get!(rinst.raw, (7, 9)) as usize + 8;
            let rs2 = bit_range_get!(rinst.raw, (2, 4)) as usize + 8;

            match (bit12, bit10_11, bit5_6) {
                (_, 0, _) => DecodedInst::CSrli {
                    rsrd : rsrd,
                    imm : immgen!(C1_OPIMM, rinst.raw)
                },
                (_, 1, _) => DecodedInst::CSrai {
                    rsrd : rsrd,
                    imm : immgen!(C1_OPIMM, rinst.raw)
                },
                (_, 2, _) => DecodedInst::CAndi {
                    rsrd : rsrd,
                    imm : immgen!(C1_OPIMM, rinst.raw)
                },
                (0, 3, 0) => DecodedInst::CSub {
                    rsrd : rsrd,
                    rs2 : rs2
                },
                (0, 3, 1) => DecodedInst::CXor {
                    rsrd : rsrd,
                    rs2 : rs2
                },
                (0, 3, 2) => DecodedInst::COr {
                    rsrd : rsrd,
                    rs2 : rs2
                },
                (0, 3, 3) => DecodedInst::CAnd {
                    rsrd : rsrd,
                    rs2 : rs2
                },
                (1, 3, 0) => DecodedInst::CSubw {
                    rsrd : rsrd,
                    rs2 : rs2
                },
                (1, 3, 1) => DecodedInst::CAddw {
                    rsrd : rsrd,
                    rs2 : rs2
                },
                _ => panic!("Invalid decode for C1!")
            }

        },
        InstSpec(InstOpcode::C1, 5) => DecodedInst::CJ {
            imm : immgen!(C1_J_JAL, rinst.raw)
        },
        InstSpec(InstOpcode::C1, 6) => DecodedInst::CBeq {
            rs1 : rs1_c(rinst) + 8,
            imm : immgen!(C1_BRA, rinst.raw)
        },
        InstSpec(InstOpcode::C1, 7) => DecodedInst::CBne {
            rs1 : rs1_c(rinst) + 8,
            imm : immgen!(C1_BRA, rinst.raw)
        },

        //
        // Compressed Quandrant 2 Instructions
        //

        InstSpec(InstOpcode::C2, 0) => DecodedInst::CSlli {
            rsrd : rd(rinst),
            imm : immgen!(C1_OPIMM, rinst.raw)
        },
        InstSpec(InstOpcode::C2, 1) => DecodedInst::CFldsp {
            rd : rd(rinst),
            imm : immgen!(C2_LD, rinst.raw)
        },
        InstSpec(InstOpcode::C2, 2) => DecodedInst::CLwsp {
            rd : rd(rinst),
            imm : immgen!(C2_LW, rinst.raw)
        },
        InstSpec(InstOpcode::C2, 3) => DecodedInst::CLdsp {
            rd : rd(rinst),
            imm : immgen!(C2_LD, rinst.raw)
        },
        InstSpec(InstOpcode::C2, 4) => {
            let bit12 = bit_range_get!(rinst.raw, (12, 12));
            let rs1 = rd(rinst);
            let rs2 = bit_range_get!(rinst.raw, (2, 6)) as usize;

            match (bit12, rs1, rs2) {
                (0, rs1, 0) => DecodedInst::CJr {
                    rs1 : rs1
                },
                (0, rs1, rs2) => DecodedInst::CMv {
                    rs1 : rs1,
                    rs2 : rs2
                },
                (1, 0, 0) => DecodedInst::CEBreak,
                (1, rs1, 0) => DecodedInst::CJalr {
                    rs1 : rs1
                },
                (1, rs1, rs2) => DecodedInst::CAdd {
                    rsrd : rs1,
                    rs2 : rs2
                },
                _ => panic!("Invalid decode for C2 Opcode!")
            }
        },
        InstSpec(InstOpcode::C2, 5) => DecodedInst::CFsdsp {
            rs2 : bit_range_get!(rinst.raw, (2, 6)) as usize,
            imm : immgen!(C2_SD, rinst.raw)
        },
        InstSpec(InstOpcode::C2, 6) => DecodedInst::CSwsp {
            rs2 : bit_range_get!(rinst.raw, (2, 6)) as usize,
            imm : immgen!(C2_SW, rinst.raw)
        },
        InstSpec(InstOpcode::C2, 7) => DecodedInst::CSdsp {
            rs2 : bit_range_get!(rinst.raw, (2, 6)) as usize,
            imm : immgen!(C2_SD, rinst.raw)
        },


        x => panic!("Unknown instruction: {:?}", x)
    }
}

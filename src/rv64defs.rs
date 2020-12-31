
#[derive(Debug)]
pub struct RawInst {
    pub pc: u64,
    pub raw : u32
}

#[derive(Debug)]
pub enum ArchWidth {
    RV32,
    RV64,
    RV128
}

// #[derive(Debug, PartialEq)]
// pub enum InstFormat { 
//     R, I, S, B, U, J,
//     CR, CI, CSS, CIW, CL, CS, CB, CJ
// }

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum OpFunct { 
    AddSub = 0b000,
    Sll    = 0b001,
    Slt    = 0b010,
    Sltu   = 0b011,
    Xor    = 0b100,
    SrlSra = 0b101,
    Or     = 0b110,
    And    = 0b111
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum LoadStoreWidth { 
    Byte   = 0b000,
    Half   = 0b001,
    Word   = 0b010,
    Double = 0b011,
    ByteU  = 0b100,
    HalfU  = 0b101,
    WordU  = 0b110
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum BranchType { 
    Eq  = 0b000,
    Neq = 0b001,
    Lt  = 0b100,
    Ge  = 0b101,
    Ltu = 0b110,
    Geu = 0b111
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum CLoadStoreWidth {
    Cfd,
    Cw,
    Cd
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum InstOpcode {
    C0      = 0b00,
    C1      = 0b01,
    C2      = 0b10,
    LOAD    = 0b0000011,
    STORE   = 0b0100011,
    MADD    = 0b1000011,
    BRANCH  = 0b1100011,
    LOADFP  = 0b0000111,
    STOREFP = 0b0100111,
    MSUB    = 0b1000111,
    JALR    = 0b1100111,
    CUSTOM0 = 0b0001011,
    CUSTOM1 = 0b0101011,
    NMSUB   = 0b1001011,
    MISCMEM = 0b0001111,
    AMO     = 0b0101111,
    NMADD   = 0b1001111,
    JAL     = 0b1101111,
    OPIMM   = 0b0010011,
    OP      = 0b0110011,
    OPFP    = 0b1010011,
    SYSTEM  = 0b1110011,
    AUIPC   = 0b0010111,
    LUI     = 0b0110111,
    OPIMM32 = 0b0011011,
    OP32    = 0b0111011,
    CUSTOM2 = 0b1011011,
    CUSTOM3 = 0b1111011
}

pub struct InstSpec(pub InstOpcode, pub usize);

#[derive(Debug, PartialEq)]
pub enum DecodedInst {
    //
    // Base Integer Instructions
    //

    // Op
    Add   { rs1 : usize, rs2 : usize, rd : usize },
    Sub   { rs1 : usize, rs2 : usize, rd : usize },
    Sll   { rs1 : usize, rs2 : usize, rd : usize },
    Slt   { rs1 : usize, rs2 : usize, rd : usize },
    Sltu  { rs1 : usize, rs2 : usize, rd : usize },
    Xor   { rs1 : usize, rs2 : usize, rd : usize },
    Srl   { rs1 : usize, rs2 : usize, rd : usize },
    Sra   { rs1 : usize, rs2 : usize, rd : usize },
    Or    { rs1 : usize, rs2 : usize, rd : usize },
    And   { rs1 : usize, rs2 : usize, rd : usize },
    
    // OpImm
    Addi  { rs1 : usize, rd : usize, imm : u64 },
    Subi  { rs1 : usize, rd : usize, imm : u64 },
    Slti  { rs1 : usize, rd : usize, imm : u64 },
    Sltiu { rs1 : usize, rd : usize, imm : u64 },
    Xori  { rs1 : usize, rd : usize, imm : u64 },
    Ori   { rs1 : usize, rd : usize, imm : u64 },
    Andi  { rs1 : usize, rd : usize, imm : u64 },
    Slli  { rs1 : usize, rd : usize, shamt : u64 },
    Srli  { rs1 : usize, rd : usize, shamt : u64 },
    Srai  { rs1 : usize, rd : usize, shamt : u64 },

    // Op32
    Addw  { rs1 : usize, rs2 : usize, rd : usize },
    Subw  { rs1 : usize, rs2 : usize, rd : usize },
    Sllw  { rs1 : usize, rs2 : usize, rd : usize },
    Srlw  { rs1 : usize, rs2 : usize, rd : usize },
    Sraw  { rs1 : usize, rs2 : usize, rd : usize },

    // OpImm32
    Addiw { rs1 : usize, rd : usize, imm : u64 },
    Subiw { rs1 : usize, rd : usize, imm : u64 },
    Slliw { rs1 : usize, rd : usize, shamt : u64 },
    Srliw { rs1 : usize, rd : usize, shamt : u64 },
    Sraiw { rs1 : usize, rd : usize, shamt : u64 },

    Lui   { rd : usize, imm : u64 },
    Auipc { rd : usize, imm : u64 },
    Jal   { rd : usize, imm : u64 },
    Jalr  { rs1 : usize, rd : usize, imm : u64 },
    
    Branch { func : BranchType, rs1 : usize, rs2 : usize, imm : u64 },
    Load   { width : LoadStoreWidth, rs1 : usize, rd : usize, imm : u64 },
    Store  { width : LoadStoreWidth, rs1 : usize, rs2 : usize, imm : u64 },

    //
    // System Instructions
    //
    // System { func : SystemFunct, rs1 : usize, rs2 : usize, imm : u64 },

    //
    // Compressed Quandrant 0 Instructions
    //

    CAddi4spn { rd : usize, imm : u64 },
    CLoad     { width : CLoadStoreWidth, rs1 : usize, rd : usize, imm : u64 },
    CStore    { width : CLoadStoreWidth, rs1 : usize, rs2 : usize, imm : u64 },
    
    //
    // Compressed Quandrant 1 Instructions
    //

    CAddi     { rsrd : usize, imm : u64 },
    // CJal      { imm : u64 },
    CAddiw    { rsrd : usize, imm : u64 },
    CLi       { rd : usize, imm : u64 },
    CAddi16sp { imm : u64 },
    CLui      { imm : u64 },

    CSrli     { rsrd : usize, imm : u64 },
    CSrai     { rsrd : usize, imm : u64 },
    CAndi     { rsrd : usize, imm : u64 },

    CSub      { rsrd : usize, rs2 : usize },
    CXor      { rsrd : usize, rs2 : usize },
    COr       { rsrd : usize, rs2 : usize },
    CAnd      { rsrd : usize, rs2 : usize },
    CSubw     { rsrd : usize, rs2 : usize },
    CAddw     { rsrd : usize, rs2 : usize },

    CJ        { imm : u64 },
    CBeq      { rs1 : usize, imm : u64 },
    CBne      { rs1 : usize, imm : u64 },
    
    //
    // Compressed Quandrant 2 Instructions
    //

    CSlli   { rsrd : usize, imm : u64 },
    // CSlli64 { rsrd : usize },
    CFldsp  { rd : usize, imm : u64 },
    CLwsp   { rd : usize, imm : u64 },
    // CLqsp   { rd : usize, imm : u64 },
    CFlwsp  { rd : usize, imm : u64 },
    CLdsp   { rd : usize, imm : u64 },
    CJr     { rs1 : usize},
    CMv     { rs1 : usize, rs2 : usize},
    CEBreak,
    CJalr   { rs1 : usize },
    CAdd    { rsrd : usize, rs2 : usize },
    CFsdsp  { rs2 : usize, imm : u64 },
    CSwsp   { rs2 : usize, imm : u64 },
    CSdsp   { rs2 : usize, imm : u64 }

}

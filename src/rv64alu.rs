
use std::mem;
use crate::bitops::*;

#[inline(always)]
pub fn add(op1 : u64, op2 : u64) -> u64 {
    op1.overflowing_add(op2).0
}

#[test]
fn test_add() {
    assert_eq!(add(1, 1), 2);
    assert_eq!(add(0xFFFFFFFFFFFFFFFF, 1), 0);
}

#[inline(always)]
pub fn sub(op1 : u64, op2 : u64) -> u64 {
    op1.overflowing_sub(op2).0
}

#[test]
fn test_sub() {
    assert_eq!(sub(1, 0xFFFFFFFFFFFFFFFF), 2);
    assert_eq!(sub(0xFFFFFFFFFFFFFFFF, 1), 0xFFFFFFFFFFFFFFFE);
}

#[inline(always)]
pub fn addw(op1 : u64, op2 : u64) -> u64 {
    sign_ext64!(32, ((op1 as u32).overflowing_add(op2 as u32).0 & 0xFFFFFFFF) as u64)
}

#[test]
fn test_addw() {
    assert_eq!(addw(1, 1), 2);
    assert_eq!(addw(0x00000000FFFFFFFF, 1), 0);
}

#[inline(always)]
pub fn subw(op1 : u64, op2 : u64) -> u64 {
    sign_ext64!(32, ((op1 as u32).overflowing_sub(op2 as u32).0 & 0xFFFFFFFF) as u64)
}

#[test]
fn test_subw() {
    assert_eq!(subw(1, 0x00000000FFFFFFFF), 2);
    assert_eq!(subw(0x00000000FFFFFFFF, 1), 0x00000000FFFFFFFE);
}

#[inline(always)]
pub fn sltu(op1 : u64, op2 : u64) -> u64 {
    if op1 < op2 {
        1
    }
    else {
        0
    }
}

#[test]
fn test_sltu() {
    let f : u64 = 0xFFFFFFFFFFFFFFFF;
    println!("{}", f as i64);
    assert_eq!(sltu(0xFFFFFFFFFFFFFFFF, 0), 0);
    assert_eq!(sltu(0, 0xFFFFFFFFFFFFFFFF), 1);
}

#[inline(always)]
pub fn slt(op1 : u64, op2 : u64) -> u64 {
    if (op1 as i64) < (op2 as i64) {
        1
    }
    else {
        0
    }
}

#[test]
fn test_slt() {
    assert_eq!(slt(0xFFFFFFFFFFFFFFFF, 0), 1);
    assert_eq!(slt(0, 0xFFFFFFFFFFFFFFFF), 0);
}

#[inline(always)]
pub fn and(op1 : u64, op2 : u64) -> u64 {
    op1 & op2
}

#[inline(always)]
pub fn or(op1 : u64, op2 : u64) -> u64 {
    op1 | op2
}

#[inline(always)]
pub fn xor(op1 : u64, op2 : u64) -> u64 {
    op1 ^ op2
}

#[inline(always)]
pub fn not(op1 : u64) -> u64 {
    !op1
}

#[inline(always)]
pub fn sll(v : u64, shamt : u64) -> u64 {
    v << shamt
}

#[inline(always)]
pub fn srl(v : u64, shamt : u64) -> u64 {
    v >> shamt
}

#[inline(always)]
pub fn sra(v : u64, shamt : u64) -> u64 {
    if shamt == 0 {
        v >> shamt
    }
    else {
        let w = 64 - shamt;
        sign_ext64!(w, v >> shamt)
    }
}

#[inline(always)]
pub fn sllw(v : u64, shamt : u64) -> u64 {
    sign_ext64!(32, sll(v, shamt & 0x1F) & 0xFFFFFFFF)
}

#[inline(always)]
pub fn srlw(v : u64, shamt : u64) -> u64 {
    sign_ext64!(32, srl(v, shamt  & 0x1F) & 0xFFFFFFFF)
}

#[inline(always)]
pub fn sraw(v : u64, shamt : u64) -> u64 {
    sign_ext64!(32, sra(v, shamt) & 0xFFFFFFFF)
}

#[inline(always)]
pub fn div(n : u64, d : u64) -> u64 {
    unsafe {
        let sn : i64 = mem::transmute(n);
        let sd : i64 = mem::transmute(d);
        let res : i64 = sn / sd;
        mem::transmute(res)
    }
}

#[inline(always)]
pub fn divu(n : u64, d : u64) -> u64 {
    n / d
}

#[inline(always)]
pub fn rem(n : u64, d : u64) -> u64 {
    unsafe {
        let sn : i64 = mem::transmute(n);
        let sd : i64 = mem::transmute(d);
        let res : i64 = sn % sd;
        mem::transmute(res)
    }
}

#[inline(always)]
pub fn remu(n : u64, d : u64) -> u64 {
    n % d
}

#[inline(always)]
pub fn remw(n : u64, d : u64) -> u64 {
    unsafe {
        let sn : i32 = mem::transmute((n & 0xFFFFFFFF) as u32);
        let sd : i32 = mem::transmute((d & 0xFFFFFFFF) as u32);
        let res_i32 : i32 = sn % sd;
        let res_u32 : u32 = mem::transmute(res_i32);
        res_u32 as u64
    }
}

#[inline(always)]
pub fn remuw(n : u64, d : u64) -> u64 {
    (n & 0xFFFFFFFF) % (d & 0xFFFFFFFF)
}

#[test]
fn test_rem() {
    let x1 = (1 as u64) % (3 as u64);
    let x2 = (3 as u64) % (3 as u64);
    let x3 = (4 as u64) % (3 as u64);
    let x4 = rem(u64::MAX, 3 as u64);


    println!("{}", x1);
    println!("{}", x2);
    println!("{}", x3);
    println!("{:016x}", x4);

}

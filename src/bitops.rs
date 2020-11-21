
pub struct BitRange(pub usize, pub usize);

impl BitRange {
    #[inline(always)]
    pub fn nbits(&self) -> usize { self.1 - self.0 + 1 }

    #[inline(always)]
    #[allow(unused)]
    pub fn start(&self) -> usize { self.0 }

    #[inline(always)]
    #[allow(unused)]
    pub fn end(&self) -> usize { self.1 }

    #[inline(always)]
    pub fn mask(&self) -> u64 { 
        match self.nbits() {
            0 ..= 63 => (1 << self.nbits()) - 1,
            64 => 0xFFFFFFFFFFFFFFFF,
            _ => panic!("More than 64 bits??")
        }
    }
}

#[inline(always)]
#[allow(unused)]
pub fn sign_ext32to64(i : u64) -> u64 {
    if i & 0x80000000 != 0 {
        0xFFFFFFFF00000000 | i
    }
    else {
        i as u64
    }
}

#[inline(always)]
#[allow(unused)]
pub fn sign_ext16to64(i : u64) -> u64 {
    if i & 0x8000 != 0 {
        0xFFFFFFFFFFFF0000 | i
    }
    else {
        i as u64
    }
}

#[inline(always)]
#[allow(unused)]
pub fn sign_ext8to64(i : u64) -> u64 {
    if i & 0x80 != 0 {
        0xFFFFFFFFFFFFFF00 | i
    }
    else {
        i as u64
    }
}

#[inline(always)]
#[allow(unused)]
pub fn bit_range_get(v : u64, r : BitRange) -> u64 { (v >> r.0) & r.mask() }

#[inline(always)]
#[allow(unused)]
pub fn bit_range_set(v : u64, r : BitRange) -> u64 { (v & r.mask()) << r.0 }

#[inline(always)]
#[allow(unused)]
pub fn bit_range_map(v : u64, from : BitRange, to : BitRange) -> u64 {
    bit_range_set(bit_range_get(v, from), to)
}

#[inline(always)]
#[allow(unused)]
pub fn bit_repeat(v : u64, nbits : usize) -> u64 {
    match v {
        0 => 0,
        _ => (BitRange(0, nbits - 1)).mask()
    }
}

#[test]
fn test_sign_ext32to64() {
    assert_eq!(sign_ext32to64(0x8), 0x8);
}

#[test]
fn test_bit_range_get_1() {
    assert_eq!(
        bit_range_get(0xF000000000000000, BitRange(60, 63)),
        0xF);
}

#[test]
fn test_bit_range_get_2() {
    assert_eq!(
        bit_range_get(0x8000000000000000, BitRange(60, 63)),
        0x8);
}

#[test]
fn test_bit_range_get_3() {
    assert_eq!(
        bit_range_get(0x8000000000000000, BitRange(63, 63)),
        0x1);
}

#[test]
fn test_bit_range_set_1() {
    assert_eq!(
        bit_range_set(1, BitRange(60, 63)),
        0x1000000000000000);
}

#[test]
fn test_bit_range_set_2() {
    assert_eq!(
        bit_range_set(1, BitRange(63, 63)),
        0x8000000000000000);
}

#[test]
fn test_bit_repeat_1() {
    assert_eq!(
        bit_repeat(1, 20),
        0xFFFFF);
}


use crate::bitops;

pub trait MemIf {
    fn read(&self, addr : u64) -> u8;
    fn write(&mut self, addr : u64, value : u8);
}

#[inline(always)]
pub fn read8(mem : &dyn MemIf, addr : u64) -> u64 {
    mem.read(addr + 0) as u64
}

#[inline(always)]
pub fn read16(mem : &dyn MemIf, addr : u64) -> u64 {
    (mem.read(addr + 1) as u64) << 8 |
    (mem.read(addr + 0) as u64)
}

#[inline(always)]
pub fn read32(mem : &dyn MemIf, addr : u64) -> u64 {
    (mem.read(addr + 3) as u64) << 24 |
    (mem.read(addr + 2) as u64) << 16 |
    (mem.read(addr + 1) as u64) << 8 |
    (mem.read(addr + 0) as u64)
}

#[inline(always)]
pub fn read64(mem : &dyn MemIf, addr : u64) -> u64 {
    (mem.read(addr + 7) as u64) << 56 |
    (mem.read(addr + 6) as u64) << 48 |
    (mem.read(addr + 5) as u64) << 40 |
    (mem.read(addr + 4) as u64) << 32 |
    (mem.read(addr + 3) as u64) << 24 |
    (mem.read(addr + 2) as u64) << 16 |
    (mem.read(addr + 1) as u64) << 8 |
    (mem.read(addr + 0) as u64)
}

#[inline(always)]
pub fn write8(mem : &mut dyn MemIf, addr : u64, val : u64) {
    mem.write(addr + 0, bitops::bit_range_get(val, bitops::BitRange(0, 7)) as u8);
}

#[inline(always)]
pub fn write16(mem : &mut dyn MemIf, addr : u64, val : u64) {
    mem.write(addr + 0, bitops::bit_range_get(val, bitops::BitRange(0, 7)) as u8);
    mem.write(addr + 1, bitops::bit_range_get(val, bitops::BitRange(8, 15)) as u8);
}

#[inline(always)]
pub fn write32(mem : &mut dyn MemIf, addr : u64, val : u64) {
    mem.write(addr + 0, bitops::bit_range_get(val, bitops::BitRange(0, 7)) as u8);
    mem.write(addr + 1, bitops::bit_range_get(val, bitops::BitRange(8, 15)) as u8);
    mem.write(addr + 2, bitops::bit_range_get(val, bitops::BitRange(16, 23)) as u8);
    mem.write(addr + 3, bitops::bit_range_get(val, bitops::BitRange(24, 31)) as u8);
}

#[inline(always)]
pub fn write64(mem : &mut dyn MemIf, addr : u64, val : u64) {
    mem.write(addr + 0, bitops::bit_range_get(val, bitops::BitRange(0, 7)) as u8);
    mem.write(addr + 1, bitops::bit_range_get(val, bitops::BitRange(8, 15)) as u8);
    mem.write(addr + 2, bitops::bit_range_get(val, bitops::BitRange(16, 23)) as u8);
    mem.write(addr + 3, bitops::bit_range_get(val, bitops::BitRange(24, 31)) as u8);
    mem.write(addr + 4, bitops::bit_range_get(val, bitops::BitRange(32, 39)) as u8);
    mem.write(addr + 5, bitops::bit_range_get(val, bitops::BitRange(40, 47)) as u8);
    mem.write(addr + 6, bitops::bit_range_get(val, bitops::BitRange(48, 55)) as u8);
    mem.write(addr + 7, bitops::bit_range_get(val, bitops::BitRange(56, 63)) as u8);
}

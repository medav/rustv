
use std::fs::File;
use std::io::Read;
use memmap2::MmapMut;
use crate::memif::*;

const MAX_HEAP : u64 = 4 * (1 << 30);
const MAX_STACK : u64 = 256 * (1 << 20);

pub struct ProgramMemory {
    image : Vec<u8>,
    heap : MmapMut,
    heap_start : u64,
    heap_end : u64,
    stack : MmapMut,
    stack_start : u64
}


fn read_bin(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

impl ProgramMemory {

    pub fn new(image_file : &String) -> Self {
        let image = read_bin(&image_file);
        let image_len = image.len();
        Self {
            image,
            heap : memmap2::MmapMut::map_anon(MAX_HEAP as usize).unwrap(),
            heap_start : image_len as u64,
            heap_end : image_len as u64,
            stack : memmap2::MmapMut::map_anon(MAX_STACK as usize).unwrap(),
            stack_start : 0x7000_0000_0000
        }
    }
}


impl MemIf for ProgramMemory {
    fn read(&self, addr : u64) -> u8 {
        if addr < self.heap_start {
            self.image[addr as usize]
        }
        else if addr < self.heap_end {
            self.heap[(addr - self.heap_start) as usize]
        }
        else if addr > self.stack_start - MAX_STACK {
            self.stack[(self.stack_start - addr) as usize]
        }
        else {
            println!("    Image: [0x{:016x}-0x{:016x}]", 0, self.heap_start);
            println!("    Heap:  [0x{:016x}-0x{:016x}]", self.heap_start, self.heap_end);
            println!("    Stack Base: 0x{:016x}", self.stack_start);

            panic!("Unmapped memory address! 0x{:016x}", addr);
        }
    }

    fn write(&mut self, addr : u64, value : u8) {
        if addr < self.heap_start {
            self.image[addr as usize] = value
        }
        else if addr < self.heap_end {
            self.heap[(addr - self.heap_start) as usize] = value
        }
        else if addr > self.stack_start - MAX_STACK {
            self.stack[(self.stack_start - addr) as usize] = value
        }
        else {
            panic!("Unmapped memory address! 0x{:016x}", addr);
        }
    }

    unsafe fn mut_ptr(&mut self, addr : u64) -> *mut u8 {
        if addr < self.heap_start {
            let image_ptr = self.image.as_mut_ptr();
            image_ptr.add(addr as usize)
        }
        else if addr < self.heap_end {
            let heap_ptr = self.heap.as_mut_ptr();
            heap_ptr.add((addr - self.heap_start) as usize)
        }
        else if addr > self.stack_start - MAX_STACK {
            let stack_ptr = self.stack.as_mut_ptr();
            stack_ptr.sub((self.stack_start - addr) as usize)
        }
        else {
            panic!("Unmapped memory address! 0x{:016x}", addr);
        }
    }

    fn heap_start(&self) -> u64 {
        self.heap_start
    }

    fn brk(&mut self, new_heap_end : u64) -> Result<u64, ()> {
        if new_heap_end == 0 {
            Ok(self.heap_end)
        }
        else if new_heap_end < self.heap_start {
            panic!("Attempt to set heap < heap_start!")
        }
        else if new_heap_end - self.heap_start > MAX_HEAP {
            Err(())
        }
        else {
            self.heap_end = new_heap_end;
            Ok(self.heap_end)
        }
    }
}

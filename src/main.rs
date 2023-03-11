
extern crate num;
#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate libc;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

mod syscalls;
#[macro_use]
mod bitops;
mod memif;
mod rv64defs;
mod rv64alu;
mod rv64inst;
mod rv64emu;
mod disasm;

use memif::*;
use bitops::*;
use rv64defs::*;
use rv64inst::*;
use rv64emu::*;


fn read_bin(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

impl MemIf for Vec<u8> {
    fn read(&self, addr : u64) -> u8 {
        self[addr as usize]
    }

    fn write(&mut self, addr : u64, value : u8) {
        self[addr as usize] = value;
    }

    fn mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("No filename provided!");

    let disasm_map =
        if let Some(disasm_file ) = std::env::args().nth(2) {
            disasm::parse_disasm(&disasm_file)
        }
        else {
            HashMap::<u64, String>::new()
        };

    let mut arr : Vec<u8> = read_bin(&filename);

    let mut arch = ArchState {
        pc: 0,
        regs: [0; 32]
    };


    loop {
        let raw_inst = fetch_inst(&arch, &mut arr);
        // println!("{:04x}: {:?}", arch.pc, raw_inst);
        let decoded = decode(&raw_inst);
        // println!("{:?}", arch.regs);

        println!("    {:04x}: ({:08x}) {:?}", arch.pc, raw_inst.raw, decoded);



        let res = exec_inst(&mut arch, &mut arr, &decoded);

        if let DecodedInst::Jalr {rs1 , rd, imm } = decoded {
            if let Some(sym) = disasm_map.get(&arch.pc) {
                println!("Call {}", sym);
            }
        }

        if let DecodedInst::CJalr {rs1} = decoded {
            if let Some(sym) = disasm_map.get(&arch.pc) {
                println!("Call {}", sym);
            }
        }

        if let DecodedInst::CJr {rs1} = decoded {
            if rs1 == 1 {
                println!("Return");
            }
        }

        // if let DecodedInst::JR {rs1} = decoded {
        //     if rs1 == 1 {
        //         println!("Return");
        //     }
        // }

        if res == ExecResult::Trap {
            println!("{:?}", arch.regs);
            let syscall = rv64_parse_syscall(&mut arch);
            let res = syscalls::exec_syscall(&syscall, &mut arr);
            println!("Syscall result = {}", res);
            arch.regs[10] = res as u64;
        }
        else if res == ExecResult::Halt {
            break;
        }
    }
}

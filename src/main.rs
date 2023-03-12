
extern crate num;
#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate libc;

extern crate memmap2;

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
mod progmem;

use libc::ENOTNAM;
use memif::*;
use bitops::*;
use rv64defs::*;
use rv64inst::*;
use rv64emu::*;


fn main() {
    let filename = std::env::args().nth(1).expect("No filename provided!");

    let disasm_map =
        if let Some(disasm_file ) = std::env::args().nth(2) {
            disasm::parse_disasm(&disasm_file)
        }
        else {
            HashMap::<u64, String>::new()
        };

    let mut mem =
        progmem::ProgramMemory::new(&filename);

    let mut arch = ArchState::new();
    arch.set_stack_addr(0x7000_0000_0000);

    let mut debug = false;

    loop {
        let raw_inst = arch.fetch_inst(&mut mem);
        let decoded = decode(&raw_inst);

        if debug {
            println!("    {:04x}: ({:08x}) {:?}", arch.pc, raw_inst.raw, decoded);
        }


        let res = arch.exec_inst(&mut mem, &decoded);

        if debug {
            if let DecodedInst::Jalr {rs1 , rd, imm } = decoded {
                if let Some(sym) = disasm_map.get(&arch.pc) {
                    println!("Call {}", sym);
                }
                else if rs1 == 1 {
                    println!("Return");
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
        }


        if let DecodedInst::Addi {rs1, rd, imm} = decoded {
            if rd == 0 && imm == 1 {
                debug = true;
            }
            else if rd == 0 && imm == 2 {
                debug = false;
            }
        }

        // if let DecodedInst::JR {rs1} = decoded {
        //     if rs1 == 1 {
        //         println!("Return");
        //     }
        // }

        if res == ExecResult::Trap {
            // println!("{:?}", arch.regs);
            let syscall = arch.rv64_parse_syscall();
            let res = syscalls::exec_syscall(&syscall, &mut mem, debug);
            // println!("Syscall result = {}", res);
            arch.regs[10] = res as u64;
        }
        else if res == ExecResult::Halt {
            break;
        }
    }
}

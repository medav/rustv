
use crate::memif::*;

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum SyscallNum {
    Getcwd = 17,
    Dup = 23,
    Fcntl = 25,
    Faccessat = 48,
    Chdir = 49,
    Openat = 56,
    Close = 57,
    Getdents = 61,
    Lseek = 62,
    Read = 63,
    Write = 64,
    Writev = 66,
    Pread = 67,
    Pwrite = 68,
    Fstatat = 79,
    Fstat = 80,
    Exit = 93,
    ExitGroup = 94,
    Kill = 129,
    RtSigaction = 134,
    Times = 153,
    Uname = 160,
    Gettimeofday = 169,
    Getpid = 172,
    Getuid = 174,
    Geteuid = 175,
    Getgid = 176,
    Getegid = 177,
    Brk = 214,
    Munmap = 215,
    Mremap = 216,
    Mmap = 222,
    Open = 1024,
    Link = 1025,
    Unlink = 1026,
    Mkdir = 1030,
    Access = 1033,
    Stat = 1038,
    Lstat = 1039,
    Time = 1062,
    Getmainvars = 2011
}

#[derive(Debug)]
pub struct Syscall {
    pub num : SyscallNum,
    pub args : [u64; 7]
}


pub fn exec_syscall(syscall : &Syscall, mem : &mut dyn MemIf, debug : bool) -> u64 {
    if debug {
        println!("Syscall: {:?}", syscall);
    }

    match &syscall.num {
        SyscallNum::Fstat => {
            unsafe {
                let stat = mem.mut_ptr(syscall.args[1]) as *mut libc::stat;
                libc::fstat( syscall.args[0] as i32, stat) as u64
            }
        },
        SyscallNum::Brk => {
            match mem.brk(syscall.args[0]) {
                Ok(new_heap_end) => new_heap_end,
                _ => u64::MAX
            }
        },
        SyscallNum::Write => {
            unsafe {
                if debug {
                    println!("Write count: {}", syscall.args[0]);
                    let cstr : *mut u8 = mem.mut_ptr(syscall.args[1]);

                    for i in 0..syscall.args[0] {
                        println!("/{}/", *cstr.add(i as usize));
                    }
                }
                // println!("{}", *cstr.add(0));
                // println!("{}", *cstr.add(1));
                // println!("{}", *cstr.add(2));
                // println!("{}", *cstr.add(3));
                // println!("{}", *cstr.add(4));
                // println!("{}", *cstr.add(5));

                libc::write(
                    syscall.args[0] as i32,
                    mem.mut_ptr(syscall.args[1]) as *mut libc::c_void,
                    syscall.args[0] as usize) as u64
            }
        },
        x => panic!("Unimplemented syscall: {:?}", x)
    }
}

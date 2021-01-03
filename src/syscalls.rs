

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

pub fn exec_syscall(syscall : &Syscall) {
    match &syscall.num {
        Fstat => {
            
        },
        x => panic!("Unimplemented syscall: {:?}", x)
    }
}

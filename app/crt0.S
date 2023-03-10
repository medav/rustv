
.section .text.init, "ax", @progbits
.globl _init
_init:
    li    sp, 0x00010000
    jal main
_end:
    ebreak
    j _end


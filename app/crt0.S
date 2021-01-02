
.section .text.init, "ax", @progbits
.globl _init
_init:
    li    sp, 0x00001000
    jal _start
_end:
    ebreak

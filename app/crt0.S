
.section .text.init, "ax", @progbits
.globl _init
_init:
    jal main
_end:
    ebreak
    j _end


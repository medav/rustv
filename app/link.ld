SECTIONS
{
    ROM_BASE = 0x0;
    . = ROM_BASE;
    .text.init : { *(.text.init) }
}
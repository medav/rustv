SECTIONS
{
    ROM_BASE = 0x0;
    . = ROM_BASE;
    .text.init : { *(.text.init) }
    .text :
    {
        *(.text.unlikely .text.*_unlikely .text.unlikely.*)
        *(.text.exit .text.exit.*)
        *(.text.startup .text.startup.*)
        *(.text.hot .text.hot.*)
        *(.text .stub .text.* .gnu.linkonce.t.*)
        /* .gnu.warning sections are handled specially by elf32.em.  */
        *(.gnu.warning)
    }
    PROVIDE (__etext = .);
    PROVIDE (_etext = .);
    PROVIDE (etext = .);


    __bss_start = .;
    .bss            :
    {
    *(.dynbss)
    *(.bss .bss.* .gnu.linkonce.b.*)
    *(COMMON)
    /* Align here to ensure that the .bss section occupies space up to
        _end.  Align after .bss to ensure correct alignment even if the
        .bss section disappears because there are no input sections.
        FIXME: Why do we need it? When there is no .bss section, we don't
        pad the .data section.  */
    . = ALIGN(. != 0 ? 64 / 8 : 1);
    }
    .lbss   :
    {
        *(.dynlbss)
        *(.lbss .lbss.* .gnu.linkonce.lb.*)
        *(LARGE_COMMON)
    }
    . = ALIGN(64 / 8);
    . = SEGMENT_START("ldata-segment", .);
    .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :
    {
        *(.lrodata .lrodata.* .gnu.linkonce.lr.*)
    }

    .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :
    {
        *(.ldata .ldata.* .gnu.linkonce.l.*)
        . = ALIGN(. != 0 ? 64 / 8 : 1);
    }
    . = ALIGN(64 / 8);

    PROVIDE (_edata = .);
    PROVIDE (_end = .);
    PROVIDE( __global_pointer$ = . + (4K / 2) );
}
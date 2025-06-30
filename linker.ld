OUTPUT_ARCH(loongarch64)
ENTRY(_start)

SECTIONS
{
    . = 0x9000000000200000;
    
    __kernel_start = .;

    .text : {
        *(.text.start)
        *(.text .text.*)
        PROVIDE(__etext = .);
    }
    
    . = ALIGN(4096);

    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .eh_frame_hdr : {
        PROVIDE(__GNU_EH_FRAME_HDR = .);
        KEEP(*(.eh_frame_hdr .eh_frame_hdr.*))
    }
    
    . = ALIGN(8);
    .eh_frame : {
        PROVIDE(__eh_frame = .);
        KEEP(*(.eh_frame .eh_frame.*))
    }

    .data : {
        *(.data .data.*)
    }
    
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
        __stack_start = .;
        . += 0x1000;
        __stack_end = .;
    }
    
    /DISCARD/ : {
        *(.comment)
        *(.gnu*)
        *(.note*)
        *(.eh_frame*)
    }

    __kernel_end = .;
} 
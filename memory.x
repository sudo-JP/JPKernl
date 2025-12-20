MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    /*
     * Here we assume you have 2048 KiB of Flash. This is what the Pi Pico
     * has, but your board may have more or less Flash and you should adjust
     * this value to suit.
     */
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100

    /* Ram size */ 
    RAM : ORIGIN = 0x20000000, LENGTH = 256K

    /*
     * We want KERNEL + WIFI + PROCESSE = 256K 
     * RAM size is 256kB of memory 
     * Reserve 20kB for kernel stack 
     */

    KERNEL_DATA : ORIGIN = 0x20000000, LENGTH = 20K 

    /* Wifi needs 90kB */
    WIFI : ORIGIN = 0x20005000, LENGTH = 90K 

    /*
     * RAM consists of 4 banks, SRAM0-SRAM3, with a striped mapping.
     * This is usually good for performance, as it distributes load on
     * those banks evenly.
     */
    PROCESSES : ORIGIN = 0x2001B800, LENGTH = 146K

    /*
     * RAM banks 4 and 5 use a direct mapping. They can be used to have
     * memory areas dedicated for some specific job, improving predictability
     * of access times.
     * Example: Separate stacks for core0 and core1.
     */
    /* Use SRAM 4 and 5 for interrupt stack */
    CORE0_STACK : ORIGIN = 0x20040000, LENGTH = 4k
    CORE1_STACK : ORIGIN = 0x20041000, LENGTH = 4k
}

EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    /* ### Boot loader
     *
     * An executable block of code which sets up the QSPI interface for
     * 'Execute-In-Place' (or XIP) mode. Also sends chip-specific commands to
     * the external flash chip.
     *
     * Must go at the start of external flash, where the Boot ROM expects it.
     */
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;

SECTIONS {
    /* ### Boot ROM info
     *
     * Goes after .vector_table, to keep it in the first 512 bytes of flash,
     * where picotool can find it
     */
    .boot_info : ALIGN(4)
    {
        KEEP(*(.boot_info));
    } > FLASH

} INSERT AFTER .vector_table;

/* move .text to start /after/ the boot info */
_stext = ADDR(.boot_info) + SIZEOF(.boot_info);

SECTIONS {
    /* ### Picotool 'Binary Info' Entries
     *
     * Picotool looks through this block (as we have pointers to it in our
     * header) to find interesting information.
     */
    .bi_entries : ALIGN(4)
    {
        /* We put this in the header */
        __bi_entries_start = .;
        /* Here are the entries */
        KEEP(*(.bi_entries));
        /* Keep this block a nice round size */
        . = ALIGN(4);
        /* We put this in the header */
        __bi_entries_end = .;
    } > FLASH
} INSERT AFTER .text;

SECTIONS {
    .flash_end : {
        __flash_binary_end = .;
    } > FLASH
} INSERT AFTER .uninit;

/* Export symbols for Rust code to access memory regions */
_kernel_data_start = ORIGIN(KERNEL_DATA);
_kernel_data_size = LENGTH(KERNEL_DATA);

_wifi_start = ORIGIN(WIFI);
_wifi_size = LENGTH(WIFI);

_processes_start = ORIGIN(PROCESSES);
_processes_size = LENGTH(PROCESSES);

_core0_stack_top = ORIGIN(CORE0_STACK) + LENGTH(CORE0_STACK);
_core1_stack_top = ORIGIN(CORE1_STACK) + LENGTH(CORE1_STACK);

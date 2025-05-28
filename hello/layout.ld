MEMORY
{
        flash (rx) : ORIGIN = 0x20010000, LENGTH = 0x10000
        ram (!rx)  : ORIGIN = 0x10000000, LENGTH = 0x20000
}

SECTIONS {
    .manifest ORIGIN(flash):
    {
        _manifest = .;
        /* see: sw/device/silicon_creator/lib/manifest.h */
        . += 384; /* rsa_signature */
        . += 4;   /* usage_constraints.selector_bits */
        . += 32;  /* usage_constraints.device_id */
        . += 4;   /* usage_constraints.manuf_state_creator */
        . += 4;   /* usage_constraints.manuf_state_owner */
        . += 4;   /* usage_constraints.life_cycle_state */
        . += 384; /* rsa_modulus */
        LONG(0x1d4); /* . = . + 4; address_translation (HardenedFalse) */
        LONG(0x3042544f); /* . = . + 4; identifier (OTB0) */
        SHORT(0x6c47); /* . = . + 2; manifest_version.minor */
        SHORT(0x2);    /* . = . + 2; manifest_version.major */
        LONG(_flash_end - ORIGIN(flash)); /* . = . + 4; signed_region_end */
        LONG(_flash_end - ORIGIN(flash)); /* . = . + 4; length */
        . += 4;   /* version_major */
        . += 4;   /* version_minor */
        . += 4;   /* security_version */
        . += 8;   /* timestamp */
        . += 32;  /* binding_value */
        . += 4;   /* max_key_version */
        LONG(_stext - ORIGIN(flash)); /* . = . + 4; code_start */
        LONG(_etext - ORIGIN(flash)); /* . = . + 4; code_end */
        LONG(_start - ORIGIN(flash)); /* . = . + 4; entry_point */
        /* manifest extension table */
        /* see: sw/device/silicon_creator/lib/base/chip.h */
        /* CHIP_MANIFEST_EXT_TABLE_COUNT = 15 */
        /* sizeof(manifest_ext_table_entry) = 8 */
        . += 120; /* manifest_ext_table */
    } > flash

    .stack (NOLOAD) : {
        . = ALIGN(8);
        _sstack = .;
        // Does this work with LLD?
        . += 0x1000;
        . = ALIGN(8);
        _estack = .;
    } > ram

    .text : {
        . = ALIGN(4);
        _stext = .;
        *(.text .text.* .gnu.linkonce.t.*)
        _etext = .;

        _srodata = .;
        *(.rodata .rodata.* .gnu.linkonce.r.*)
        _erodata = .;
    } > flash
    _flash_end = .;
}

ENTRY(_start)

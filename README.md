# AArch64 Architectural Support Library

This repository contains an library to program `aarch64` hardware. This
includes hardware specific data structure defintions, register layouts
and access methods to read/write registers exposed as high-level functions.

## Authors

Reto Achermann

## License

See `LICENSE` file.

## Dependencies (Generation)

For the generation of the register definitions and system instructions,
you will need to get Python3 and wget

```
$ sudo apt-get install python3 python3-plumbum wget
```

## Dependencies (Building)

You will need to have the aarch66 target installed.

```
rustup target add  aarch64-unknown-linux-gnu
```

## Building

The library can be built using cargo and supplying it with the aarch64 target:

```
cargo build --target=aarch64-unknown-linux-gnu
```

## Overview

The carte contains several modules:

### Registers

Register definitions and access functions are generated from the XML files
`https://developer.arm.com/products/architecture/a-profile/exploration-tools`

### Instructions

Architecture specific instructions for cache invalidation etc
`https://developer.arm.com/products/architecture/a-profile/exploration-tools`

## Unsupported

Currenlty, there are a few instructions / registers which are not supported due
to variable sized fields etc. See list in `tools/config.json`.
```
    "ignored" : [
        "AArch64-s1_op1_cn_cm_op2.xml",
        "AArch64-s3_op1_cn_cm_op2.xml",
        "AArch64-sp_el3.xml",
        "AArch64-amevcntr0n_el0.xml",
        "AArch64-amevcntr1n_el0.xml",
        "AArch64-amevcntvoff0n_el2.xml",
        "AArch64-amevcntvoff1n_el2.xml",
        "AArch64-amevtyper0n_el0.xml",
        "AArch64-amevtyper1n_el0.xml",
        "AArch64-dbgbcrn_el1.xml",
        "AArch64-dbgbvrn_el1.xml",
        "AArch64-dbgwcrn_el1.xml",
        "AArch64-dbgwvrn_el1.xml",
        "AArch64-ich_lrn_el2.xml",
        "AArch64-pmevcntrn_el0.xml",
        "AArch64-pmevtypern_el0.xml",
        "AArch64-icc_ap0rn_el1.xml",
        "AArch64-icc_ap1rn_el1.xml",
        "AArch64-ich_ap0rn_el2.xml",
        "AArch64-ich_ap1rn_el2.xml",
        "AArch64-icv_ap0rn_el1.xml",
        "AArch64-icv_ap1rn_el1.xml"
    ],
```



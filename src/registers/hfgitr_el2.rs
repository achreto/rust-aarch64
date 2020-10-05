/*
 * MIT License
 *
 * Copyright (c) 2020 Reto Achermann
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */


/***************************************************************************
 * ***********************
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN
 * !!!!
 *
 * Generated on: 2020-10-05T16:30:11.691939
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN
 * !!!!
 *
 **************************************************************************
 * ********************* */

/*
 * ================================================================================================
 * Register Information
 * ================================================================================================
 *
 * Register:    Hypervisor Fine-Grained Instruction Trap Register
 * (hfgitr_el2) Group:       A group mapping that does not have a known
 * primary Type:        64-bit Register
 * Description: Provides instruction trap controls.
 * File:        AArch64-hfgitr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Hypervisor Fine-Grained Instruction Trap Register (hfgitr_el2)
/// register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, HFGITR_EL2
        llvm_asm!("mrs $0, S3_4_C1_C1_6" : "=r"(regval));
    }
    return regval;
}


/// writing the Hypervisor Fine-Grained Instruction Trap Register (hfgitr_el2)
/// register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR HFGITR_EL2, <Xt>
        llvm_asm!("msr S3_4_C1_C1_6, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn dccvac_read() -> u64 {
    // bits 54..54
    let val = reg_rawrd();
    (val >> 54) & 0x1
}

/// inserts field val into register
pub fn dccvac_write(newval: u64) {
    // bits 54..54
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 54) | ((newval & 0x1) << 54));
}

/// reads field val from register
pub fn svc_el1_read() -> u64 {
    // bits 53..53
    let val = reg_rawrd();
    (val >> 53) & 0x1
}

/// inserts field val into register
pub fn svc_el1_write(newval: u64) {
    // bits 53..53
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 53) | ((newval & 0x1) << 53));
}

/// reads field val from register
pub fn svc_el0_read() -> u64 {
    // bits 52..52
    let val = reg_rawrd();
    (val >> 52) & 0x1
}

/// inserts field val into register
pub fn svc_el0_write(newval: u64) {
    // bits 52..52
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 52) | ((newval & 0x1) << 52));
}

/// reads field val from register
pub fn eret_read() -> u64 {
    // bits 51..51
    let val = reg_rawrd();
    (val >> 51) & 0x1
}

/// inserts field val into register
pub fn eret_write(newval: u64) {
    // bits 51..51
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 51) | ((newval & 0x1) << 51));
}

/// reads field val from register
pub fn cpprctx_1_read() -> u64 {
    // bits 50..50
    let val = reg_rawrd();
    (val >> 50) & 0x1
}

/// inserts field val into register
pub fn cpprctx_1_write(newval: u64) {
    // bits 50..50
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 50) | ((newval & 0x1) << 50));
}

/// reads field val from register
pub fn dvprctx_1_read() -> u64 {
    // bits 49..49
    let val = reg_rawrd();
    (val >> 49) & 0x1
}

/// inserts field val into register
pub fn dvprctx_1_write(newval: u64) {
    // bits 49..49
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 49) | ((newval & 0x1) << 49));
}

/// reads field val from register
pub fn cfprctx_1_read() -> u64 {
    // bits 48..48
    let val = reg_rawrd();
    (val >> 48) & 0x1
}

/// inserts field val into register
pub fn cfprctx_1_write(newval: u64) {
    // bits 48..48
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 48) | ((newval & 0x1) << 48));
}

/// reads field val from register
pub fn tlbivaale1_read() -> u64 {
    // bits 47..47
    let val = reg_rawrd();
    (val >> 47) & 0x1
}

/// inserts field val into register
pub fn tlbivaale1_write(newval: u64) {
    // bits 47..47
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 47) | ((newval & 0x1) << 47));
}

/// reads field val from register
pub fn tlbivale1_read() -> u64 {
    // bits 46..46
    let val = reg_rawrd();
    (val >> 46) & 0x1
}

/// inserts field val into register
pub fn tlbivale1_write(newval: u64) {
    // bits 46..46
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 46) | ((newval & 0x1) << 46));
}

/// reads field val from register
pub fn tlbivaae1_read() -> u64 {
    // bits 45..45
    let val = reg_rawrd();
    (val >> 45) & 0x1
}

/// inserts field val into register
pub fn tlbivaae1_write(newval: u64) {
    // bits 45..45
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 45) | ((newval & 0x1) << 45));
}

/// reads field val from register
pub fn tlbiaside1_read() -> u64 {
    // bits 44..44
    let val = reg_rawrd();
    (val >> 44) & 0x1
}

/// inserts field val into register
pub fn tlbiaside1_write(newval: u64) {
    // bits 44..44
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 44) | ((newval & 0x1) << 44));
}

/// reads field val from register
pub fn tlbivae1_read() -> u64 {
    // bits 43..43
    let val = reg_rawrd();
    (val >> 43) & 0x1
}

/// inserts field val into register
pub fn tlbivae1_write(newval: u64) {
    // bits 43..43
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 43) | ((newval & 0x1) << 43));
}

/// reads field val from register
pub fn tlbivmalle1_read() -> u64 {
    // bits 42..42
    let val = reg_rawrd();
    (val >> 42) & 0x1
}

/// inserts field val into register
pub fn tlbivmalle1_write(newval: u64) {
    // bits 42..42
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 42) | ((newval & 0x1) << 42));
}

/// reads field val from register
pub fn tlbirvaale1_1_read() -> u64 {
    // bits 41..41
    let val = reg_rawrd();
    (val >> 41) & 0x1
}

/// inserts field val into register
pub fn tlbirvaale1_1_write(newval: u64) {
    // bits 41..41
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 41) | ((newval & 0x1) << 41));
}

/// reads field val from register
pub fn tlbirvale1_1_read() -> u64 {
    // bits 40..40
    let val = reg_rawrd();
    (val >> 40) & 0x1
}

/// inserts field val into register
pub fn tlbirvale1_1_write(newval: u64) {
    // bits 40..40
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 40) | ((newval & 0x1) << 40));
}

/// reads field val from register
pub fn tlbirvaae1_1_read() -> u64 {
    // bits 39..39
    let val = reg_rawrd();
    (val >> 39) & 0x1
}

/// inserts field val into register
pub fn tlbirvaae1_1_write(newval: u64) {
    // bits 39..39
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 39) | ((newval & 0x1) << 39));
}

/// reads field val from register
pub fn tlbirvae1_1_read() -> u64 {
    // bits 38..38
    let val = reg_rawrd();
    (val >> 38) & 0x1
}

/// inserts field val into register
pub fn tlbirvae1_1_write(newval: u64) {
    // bits 38..38
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 38) | ((newval & 0x1) << 38));
}

/// reads field val from register
pub fn tlbirvaale1is_1_read() -> u64 {
    // bits 37..37
    let val = reg_rawrd();
    (val >> 37) & 0x1
}

/// inserts field val into register
pub fn tlbirvaale1is_1_write(newval: u64) {
    // bits 37..37
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 37) | ((newval & 0x1) << 37));
}

/// reads field val from register
pub fn tlbirvale1is_1_read() -> u64 {
    // bits 36..36
    let val = reg_rawrd();
    (val >> 36) & 0x1
}

/// inserts field val into register
pub fn tlbirvale1is_1_write(newval: u64) {
    // bits 36..36
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 36) | ((newval & 0x1) << 36));
}

/// reads field val from register
pub fn tlbirvaae1is_1_read() -> u64 {
    // bits 35..35
    let val = reg_rawrd();
    (val >> 35) & 0x1
}

/// inserts field val into register
pub fn tlbirvaae1is_1_write(newval: u64) {
    // bits 35..35
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 35) | ((newval & 0x1) << 35));
}

/// reads field val from register
pub fn tlbirvae1is_1_read() -> u64 {
    // bits 34..34
    let val = reg_rawrd();
    (val >> 34) & 0x1
}

/// inserts field val into register
pub fn tlbirvae1is_1_write(newval: u64) {
    // bits 34..34
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 34) | ((newval & 0x1) << 34));
}

/// reads field val from register
pub fn tlbivaale1is_read() -> u64 {
    // bits 33..33
    let val = reg_rawrd();
    (val >> 33) & 0x1
}

/// inserts field val into register
pub fn tlbivaale1is_write(newval: u64) {
    // bits 33..33
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 33) | ((newval & 0x1) << 33));
}

/// reads field val from register
pub fn tlbivale1is_read() -> u64 {
    // bits 32..32
    let val = reg_rawrd();
    (val >> 32) & 0x1
}

/// inserts field val into register
pub fn tlbivale1is_write(newval: u64) {
    // bits 32..32
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 32) | ((newval & 0x1) << 32));
}

/// reads field val from register
pub fn tlbivaae1is_read() -> u64 {
    // bits 31..31
    let val = reg_rawrd();
    (val >> 31) & 0x1
}

/// inserts field val into register
pub fn tlbivaae1is_write(newval: u64) {
    // bits 31..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 31) | ((newval & 0x1) << 31));
}

/// reads field val from register
pub fn tlbiaside1is_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn tlbiaside1is_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn tlbivae1is_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

/// inserts field val into register
pub fn tlbivae1is_write(newval: u64) {
    // bits 29..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 29) | ((newval & 0x1) << 29));
}

/// reads field val from register
pub fn tlbivmalle1is_read() -> u64 {
    // bits 28..28
    let val = reg_rawrd();
    (val >> 28) & 0x1
}

/// inserts field val into register
pub fn tlbivmalle1is_write(newval: u64) {
    // bits 28..28
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 28) | ((newval & 0x1) << 28));
}

/// reads field val from register
pub fn tlbirvaale1os_1_read() -> u64 {
    // bits 27..27
    let val = reg_rawrd();
    (val >> 27) & 0x1
}

/// inserts field val into register
pub fn tlbirvaale1os_1_write(newval: u64) {
    // bits 27..27
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 27) | ((newval & 0x1) << 27));
}

/// reads field val from register
pub fn tlbirvale1os_1_read() -> u64 {
    // bits 26..26
    let val = reg_rawrd();
    (val >> 26) & 0x1
}

/// inserts field val into register
pub fn tlbirvale1os_1_write(newval: u64) {
    // bits 26..26
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 26) | ((newval & 0x1) << 26));
}

/// reads field val from register
pub fn tlbirvaae1os_1_read() -> u64 {
    // bits 25..25
    let val = reg_rawrd();
    (val >> 25) & 0x1
}

/// inserts field val into register
pub fn tlbirvaae1os_1_write(newval: u64) {
    // bits 25..25
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 25) | ((newval & 0x1) << 25));
}

/// reads field val from register
pub fn tlbirvae1os_1_read() -> u64 {
    // bits 24..24
    let val = reg_rawrd();
    (val >> 24) & 0x1
}

/// inserts field val into register
pub fn tlbirvae1os_1_write(newval: u64) {
    // bits 24..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 24) | ((newval & 0x1) << 24));
}

/// reads field val from register
pub fn tlbivaale1os_1_read() -> u64 {
    // bits 23..23
    let val = reg_rawrd();
    (val >> 23) & 0x1
}

/// inserts field val into register
pub fn tlbivaale1os_1_write(newval: u64) {
    // bits 23..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 23) | ((newval & 0x1) << 23));
}

/// reads field val from register
pub fn tlbivale1os_1_read() -> u64 {
    // bits 22..22
    let val = reg_rawrd();
    (val >> 22) & 0x1
}

/// inserts field val into register
pub fn tlbivale1os_1_write(newval: u64) {
    // bits 22..22
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 22) | ((newval & 0x1) << 22));
}

/// reads field val from register
pub fn tlbivaae1os_1_read() -> u64 {
    // bits 21..21
    let val = reg_rawrd();
    (val >> 21) & 0x1
}

/// inserts field val into register
pub fn tlbivaae1os_1_write(newval: u64) {
    // bits 21..21
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 21) | ((newval & 0x1) << 21));
}

/// reads field val from register
pub fn tlbiaside1os_1_read() -> u64 {
    // bits 20..20
    let val = reg_rawrd();
    (val >> 20) & 0x1
}

/// inserts field val into register
pub fn tlbiaside1os_1_write(newval: u64) {
    // bits 20..20
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 20) | ((newval & 0x1) << 20));
}

/// reads field val from register
pub fn tlbivae1os_1_read() -> u64 {
    // bits 19..19
    let val = reg_rawrd();
    (val >> 19) & 0x1
}

/// inserts field val into register
pub fn tlbivae1os_1_write(newval: u64) {
    // bits 19..19
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 19) | ((newval & 0x1) << 19));
}

/// reads field val from register
pub fn tlbivmalle1os_1_read() -> u64 {
    // bits 18..18
    let val = reg_rawrd();
    (val >> 18) & 0x1
}

/// inserts field val into register
pub fn tlbivmalle1os_1_write(newval: u64) {
    // bits 18..18
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 18) | ((newval & 0x1) << 18));
}

/// reads field val from register
pub fn ats1e1wp_1_read() -> u64 {
    // bits 17..17
    let val = reg_rawrd();
    (val >> 17) & 0x1
}

/// inserts field val into register
pub fn ats1e1wp_1_write(newval: u64) {
    // bits 17..17
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 17) | ((newval & 0x1) << 17));
}

/// reads field val from register
pub fn ats1e1rp_1_read() -> u64 {
    // bits 16..16
    let val = reg_rawrd();
    (val >> 16) & 0x1
}

/// inserts field val into register
pub fn ats1e1rp_1_write(newval: u64) {
    // bits 16..16
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 16) | ((newval & 0x1) << 16));
}

/// reads field val from register
pub fn ats1e0w_read() -> u64 {
    // bits 15..15
    let val = reg_rawrd();
    (val >> 15) & 0x1
}

/// inserts field val into register
pub fn ats1e0w_write(newval: u64) {
    // bits 15..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 15) | ((newval & 0x1) << 15));
}

/// reads field val from register
pub fn ats1e0r_read() -> u64 {
    // bits 14..14
    let val = reg_rawrd();
    (val >> 14) & 0x1
}

/// inserts field val into register
pub fn ats1e0r_write(newval: u64) {
    // bits 14..14
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 14) | ((newval & 0x1) << 14));
}

/// reads field val from register
pub fn ats1e1w_read() -> u64 {
    // bits 13..13
    let val = reg_rawrd();
    (val >> 13) & 0x1
}

/// inserts field val into register
pub fn ats1e1w_write(newval: u64) {
    // bits 13..13
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 13) | ((newval & 0x1) << 13));
}

/// reads field val from register
pub fn ats1e1r_read() -> u64 {
    // bits 12..12
    let val = reg_rawrd();
    (val >> 12) & 0x1
}

/// inserts field val into register
pub fn ats1e1r_write(newval: u64) {
    // bits 12..12
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 12) | ((newval & 0x1) << 12));
}

/// reads field val from register
pub fn dczva_read() -> u64 {
    // bits 11..11
    let val = reg_rawrd();
    (val >> 11) & 0x1
}

/// inserts field val into register
pub fn dczva_write(newval: u64) {
    // bits 11..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 11) | ((newval & 0x1) << 11));
}

/// reads field val from register
pub fn dccivac_read() -> u64 {
    // bits 10..10
    let val = reg_rawrd();
    (val >> 10) & 0x1
}

/// inserts field val into register
pub fn dccivac_write(newval: u64) {
    // bits 10..10
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 10) | ((newval & 0x1) << 10));
}

/// reads field val from register
pub fn dccvadp_1_read() -> u64 {
    // bits 9..9
    let val = reg_rawrd();
    (val >> 9) & 0x1
}

/// inserts field val into register
pub fn dccvadp_1_write(newval: u64) {
    // bits 9..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 9) | ((newval & 0x1) << 9));
}

/// reads field val from register
pub fn dccvap_read() -> u64 {
    // bits 8..8
    let val = reg_rawrd();
    (val >> 8) & 0x1
}

/// inserts field val into register
pub fn dccvap_write(newval: u64) {
    // bits 8..8
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 8) | ((newval & 0x1) << 8));
}

/// reads field val from register
pub fn dccvau_read() -> u64 {
    // bits 7..7
    let val = reg_rawrd();
    (val >> 7) & 0x1
}

/// inserts field val into register
pub fn dccvau_write(newval: u64) {
    // bits 7..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 7) | ((newval & 0x1) << 7));
}

/// reads field val from register
pub fn dccisw_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

/// inserts field val into register
pub fn dccisw_write(newval: u64) {
    // bits 6..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 6) | ((newval & 0x1) << 6));
}

/// reads field val from register
pub fn dccsw_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn dccsw_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn dcisw_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn dcisw_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn dcivac_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

/// inserts field val into register
pub fn dcivac_write(newval: u64) {
    // bits 3..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 3) | ((newval & 0x1) << 3));
}

/// reads field val from register
pub fn icivau_read() -> u64 {
    // bits 2..2
    let val = reg_rawrd();
    (val >> 2) & 0x1
}

/// inserts field val into register
pub fn icivau_write(newval: u64) {
    // bits 2..2
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 2) | ((newval & 0x1) << 2));
}

/// reads field val from register
pub fn iciallu_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn iciallu_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}

/// reads field val from register
pub fn icialluis_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn icialluis_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Hypervisor Fine-Grained Instruction Trap
/// Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register hfgitr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x7fffffffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x7fffffffffffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x7fffffffffffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 36028797018963967;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn dccvac_extract(&mut self) -> u64 {
        // bits 54..54
        (self.val >> 54) & 0x1
    }

    /// inserts field val into current value
    pub fn dccvac_insert(&mut self, val: u64) {
        // bits 54..54
        self.val = self.val & !(0x1 << 54) | ((val & 0x1) << 54);
    }

    /// extracts field val from current value
    pub fn svc_el1_extract(&mut self) -> u64 {
        // bits 53..53
        (self.val >> 53) & 0x1
    }

    /// inserts field val into current value
    pub fn svc_el1_insert(&mut self, val: u64) {
        // bits 53..53
        self.val = self.val & !(0x1 << 53) | ((val & 0x1) << 53);
    }

    /// extracts field val from current value
    pub fn svc_el0_extract(&mut self) -> u64 {
        // bits 52..52
        (self.val >> 52) & 0x1
    }

    /// inserts field val into current value
    pub fn svc_el0_insert(&mut self, val: u64) {
        // bits 52..52
        self.val = self.val & !(0x1 << 52) | ((val & 0x1) << 52);
    }

    /// extracts field val from current value
    pub fn eret_extract(&mut self) -> u64 {
        // bits 51..51
        (self.val >> 51) & 0x1
    }

    /// inserts field val into current value
    pub fn eret_insert(&mut self, val: u64) {
        // bits 51..51
        self.val = self.val & !(0x1 << 51) | ((val & 0x1) << 51);
    }

    /// extracts field val from current value
    pub fn cpprctx_1_extract(&mut self) -> u64 {
        // bits 50..50
        (self.val >> 50) & 0x1
    }

    /// inserts field val into current value
    pub fn cpprctx_1_insert(&mut self, val: u64) {
        // bits 50..50
        self.val = self.val & !(0x1 << 50) | ((val & 0x1) << 50);
    }

    /// extracts field val from current value
    pub fn dvprctx_1_extract(&mut self) -> u64 {
        // bits 49..49
        (self.val >> 49) & 0x1
    }

    /// inserts field val into current value
    pub fn dvprctx_1_insert(&mut self, val: u64) {
        // bits 49..49
        self.val = self.val & !(0x1 << 49) | ((val & 0x1) << 49);
    }

    /// extracts field val from current value
    pub fn cfprctx_1_extract(&mut self) -> u64 {
        // bits 48..48
        (self.val >> 48) & 0x1
    }

    /// inserts field val into current value
    pub fn cfprctx_1_insert(&mut self, val: u64) {
        // bits 48..48
        self.val = self.val & !(0x1 << 48) | ((val & 0x1) << 48);
    }

    /// extracts field val from current value
    pub fn tlbivaale1_extract(&mut self) -> u64 {
        // bits 47..47
        (self.val >> 47) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivaale1_insert(&mut self, val: u64) {
        // bits 47..47
        self.val = self.val & !(0x1 << 47) | ((val & 0x1) << 47);
    }

    /// extracts field val from current value
    pub fn tlbivale1_extract(&mut self) -> u64 {
        // bits 46..46
        (self.val >> 46) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivale1_insert(&mut self, val: u64) {
        // bits 46..46
        self.val = self.val & !(0x1 << 46) | ((val & 0x1) << 46);
    }

    /// extracts field val from current value
    pub fn tlbivaae1_extract(&mut self) -> u64 {
        // bits 45..45
        (self.val >> 45) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivaae1_insert(&mut self, val: u64) {
        // bits 45..45
        self.val = self.val & !(0x1 << 45) | ((val & 0x1) << 45);
    }

    /// extracts field val from current value
    pub fn tlbiaside1_extract(&mut self) -> u64 {
        // bits 44..44
        (self.val >> 44) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbiaside1_insert(&mut self, val: u64) {
        // bits 44..44
        self.val = self.val & !(0x1 << 44) | ((val & 0x1) << 44);
    }

    /// extracts field val from current value
    pub fn tlbivae1_extract(&mut self) -> u64 {
        // bits 43..43
        (self.val >> 43) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivae1_insert(&mut self, val: u64) {
        // bits 43..43
        self.val = self.val & !(0x1 << 43) | ((val & 0x1) << 43);
    }

    /// extracts field val from current value
    pub fn tlbivmalle1_extract(&mut self) -> u64 {
        // bits 42..42
        (self.val >> 42) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivmalle1_insert(&mut self, val: u64) {
        // bits 42..42
        self.val = self.val & !(0x1 << 42) | ((val & 0x1) << 42);
    }

    /// extracts field val from current value
    pub fn tlbirvaale1_1_extract(&mut self) -> u64 {
        // bits 41..41
        (self.val >> 41) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvaale1_1_insert(&mut self, val: u64) {
        // bits 41..41
        self.val = self.val & !(0x1 << 41) | ((val & 0x1) << 41);
    }

    /// extracts field val from current value
    pub fn tlbirvale1_1_extract(&mut self) -> u64 {
        // bits 40..40
        (self.val >> 40) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvale1_1_insert(&mut self, val: u64) {
        // bits 40..40
        self.val = self.val & !(0x1 << 40) | ((val & 0x1) << 40);
    }

    /// extracts field val from current value
    pub fn tlbirvaae1_1_extract(&mut self) -> u64 {
        // bits 39..39
        (self.val >> 39) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvaae1_1_insert(&mut self, val: u64) {
        // bits 39..39
        self.val = self.val & !(0x1 << 39) | ((val & 0x1) << 39);
    }

    /// extracts field val from current value
    pub fn tlbirvae1_1_extract(&mut self) -> u64 {
        // bits 38..38
        (self.val >> 38) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvae1_1_insert(&mut self, val: u64) {
        // bits 38..38
        self.val = self.val & !(0x1 << 38) | ((val & 0x1) << 38);
    }

    /// extracts field val from current value
    pub fn tlbirvaale1is_1_extract(&mut self) -> u64 {
        // bits 37..37
        (self.val >> 37) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvaale1is_1_insert(&mut self, val: u64) {
        // bits 37..37
        self.val = self.val & !(0x1 << 37) | ((val & 0x1) << 37);
    }

    /// extracts field val from current value
    pub fn tlbirvale1is_1_extract(&mut self) -> u64 {
        // bits 36..36
        (self.val >> 36) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvale1is_1_insert(&mut self, val: u64) {
        // bits 36..36
        self.val = self.val & !(0x1 << 36) | ((val & 0x1) << 36);
    }

    /// extracts field val from current value
    pub fn tlbirvaae1is_1_extract(&mut self) -> u64 {
        // bits 35..35
        (self.val >> 35) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvaae1is_1_insert(&mut self, val: u64) {
        // bits 35..35
        self.val = self.val & !(0x1 << 35) | ((val & 0x1) << 35);
    }

    /// extracts field val from current value
    pub fn tlbirvae1is_1_extract(&mut self) -> u64 {
        // bits 34..34
        (self.val >> 34) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvae1is_1_insert(&mut self, val: u64) {
        // bits 34..34
        self.val = self.val & !(0x1 << 34) | ((val & 0x1) << 34);
    }

    /// extracts field val from current value
    pub fn tlbivaale1is_extract(&mut self) -> u64 {
        // bits 33..33
        (self.val >> 33) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivaale1is_insert(&mut self, val: u64) {
        // bits 33..33
        self.val = self.val & !(0x1 << 33) | ((val & 0x1) << 33);
    }

    /// extracts field val from current value
    pub fn tlbivale1is_extract(&mut self) -> u64 {
        // bits 32..32
        (self.val >> 32) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivale1is_insert(&mut self, val: u64) {
        // bits 32..32
        self.val = self.val & !(0x1 << 32) | ((val & 0x1) << 32);
    }

    /// extracts field val from current value
    pub fn tlbivaae1is_extract(&mut self) -> u64 {
        // bits 31..31
        (self.val >> 31) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivaae1is_insert(&mut self, val: u64) {
        // bits 31..31
        self.val = self.val & !(0x1 << 31) | ((val & 0x1) << 31);
    }

    /// extracts field val from current value
    pub fn tlbiaside1is_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbiaside1is_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn tlbivae1is_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivae1is_insert(&mut self, val: u64) {
        // bits 29..29
        self.val = self.val & !(0x1 << 29) | ((val & 0x1) << 29);
    }

    /// extracts field val from current value
    pub fn tlbivmalle1is_extract(&mut self) -> u64 {
        // bits 28..28
        (self.val >> 28) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivmalle1is_insert(&mut self, val: u64) {
        // bits 28..28
        self.val = self.val & !(0x1 << 28) | ((val & 0x1) << 28);
    }

    /// extracts field val from current value
    pub fn tlbirvaale1os_1_extract(&mut self) -> u64 {
        // bits 27..27
        (self.val >> 27) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvaale1os_1_insert(&mut self, val: u64) {
        // bits 27..27
        self.val = self.val & !(0x1 << 27) | ((val & 0x1) << 27);
    }

    /// extracts field val from current value
    pub fn tlbirvale1os_1_extract(&mut self) -> u64 {
        // bits 26..26
        (self.val >> 26) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvale1os_1_insert(&mut self, val: u64) {
        // bits 26..26
        self.val = self.val & !(0x1 << 26) | ((val & 0x1) << 26);
    }

    /// extracts field val from current value
    pub fn tlbirvaae1os_1_extract(&mut self) -> u64 {
        // bits 25..25
        (self.val >> 25) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvaae1os_1_insert(&mut self, val: u64) {
        // bits 25..25
        self.val = self.val & !(0x1 << 25) | ((val & 0x1) << 25);
    }

    /// extracts field val from current value
    pub fn tlbirvae1os_1_extract(&mut self) -> u64 {
        // bits 24..24
        (self.val >> 24) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbirvae1os_1_insert(&mut self, val: u64) {
        // bits 24..24
        self.val = self.val & !(0x1 << 24) | ((val & 0x1) << 24);
    }

    /// extracts field val from current value
    pub fn tlbivaale1os_1_extract(&mut self) -> u64 {
        // bits 23..23
        (self.val >> 23) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivaale1os_1_insert(&mut self, val: u64) {
        // bits 23..23
        self.val = self.val & !(0x1 << 23) | ((val & 0x1) << 23);
    }

    /// extracts field val from current value
    pub fn tlbivale1os_1_extract(&mut self) -> u64 {
        // bits 22..22
        (self.val >> 22) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivale1os_1_insert(&mut self, val: u64) {
        // bits 22..22
        self.val = self.val & !(0x1 << 22) | ((val & 0x1) << 22);
    }

    /// extracts field val from current value
    pub fn tlbivaae1os_1_extract(&mut self) -> u64 {
        // bits 21..21
        (self.val >> 21) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivaae1os_1_insert(&mut self, val: u64) {
        // bits 21..21
        self.val = self.val & !(0x1 << 21) | ((val & 0x1) << 21);
    }

    /// extracts field val from current value
    pub fn tlbiaside1os_1_extract(&mut self) -> u64 {
        // bits 20..20
        (self.val >> 20) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbiaside1os_1_insert(&mut self, val: u64) {
        // bits 20..20
        self.val = self.val & !(0x1 << 20) | ((val & 0x1) << 20);
    }

    /// extracts field val from current value
    pub fn tlbivae1os_1_extract(&mut self) -> u64 {
        // bits 19..19
        (self.val >> 19) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivae1os_1_insert(&mut self, val: u64) {
        // bits 19..19
        self.val = self.val & !(0x1 << 19) | ((val & 0x1) << 19);
    }

    /// extracts field val from current value
    pub fn tlbivmalle1os_1_extract(&mut self) -> u64 {
        // bits 18..18
        (self.val >> 18) & 0x1
    }

    /// inserts field val into current value
    pub fn tlbivmalle1os_1_insert(&mut self, val: u64) {
        // bits 18..18
        self.val = self.val & !(0x1 << 18) | ((val & 0x1) << 18);
    }

    /// extracts field val from current value
    pub fn ats1e1wp_1_extract(&mut self) -> u64 {
        // bits 17..17
        (self.val >> 17) & 0x1
    }

    /// inserts field val into current value
    pub fn ats1e1wp_1_insert(&mut self, val: u64) {
        // bits 17..17
        self.val = self.val & !(0x1 << 17) | ((val & 0x1) << 17);
    }

    /// extracts field val from current value
    pub fn ats1e1rp_1_extract(&mut self) -> u64 {
        // bits 16..16
        (self.val >> 16) & 0x1
    }

    /// inserts field val into current value
    pub fn ats1e1rp_1_insert(&mut self, val: u64) {
        // bits 16..16
        self.val = self.val & !(0x1 << 16) | ((val & 0x1) << 16);
    }

    /// extracts field val from current value
    pub fn ats1e0w_extract(&mut self) -> u64 {
        // bits 15..15
        (self.val >> 15) & 0x1
    }

    /// inserts field val into current value
    pub fn ats1e0w_insert(&mut self, val: u64) {
        // bits 15..15
        self.val = self.val & !(0x1 << 15) | ((val & 0x1) << 15);
    }

    /// extracts field val from current value
    pub fn ats1e0r_extract(&mut self) -> u64 {
        // bits 14..14
        (self.val >> 14) & 0x1
    }

    /// inserts field val into current value
    pub fn ats1e0r_insert(&mut self, val: u64) {
        // bits 14..14
        self.val = self.val & !(0x1 << 14) | ((val & 0x1) << 14);
    }

    /// extracts field val from current value
    pub fn ats1e1w_extract(&mut self) -> u64 {
        // bits 13..13
        (self.val >> 13) & 0x1
    }

    /// inserts field val into current value
    pub fn ats1e1w_insert(&mut self, val: u64) {
        // bits 13..13
        self.val = self.val & !(0x1 << 13) | ((val & 0x1) << 13);
    }

    /// extracts field val from current value
    pub fn ats1e1r_extract(&mut self) -> u64 {
        // bits 12..12
        (self.val >> 12) & 0x1
    }

    /// inserts field val into current value
    pub fn ats1e1r_insert(&mut self, val: u64) {
        // bits 12..12
        self.val = self.val & !(0x1 << 12) | ((val & 0x1) << 12);
    }

    /// extracts field val from current value
    pub fn dczva_extract(&mut self) -> u64 {
        // bits 11..11
        (self.val >> 11) & 0x1
    }

    /// inserts field val into current value
    pub fn dczva_insert(&mut self, val: u64) {
        // bits 11..11
        self.val = self.val & !(0x1 << 11) | ((val & 0x1) << 11);
    }

    /// extracts field val from current value
    pub fn dccivac_extract(&mut self) -> u64 {
        // bits 10..10
        (self.val >> 10) & 0x1
    }

    /// inserts field val into current value
    pub fn dccivac_insert(&mut self, val: u64) {
        // bits 10..10
        self.val = self.val & !(0x1 << 10) | ((val & 0x1) << 10);
    }

    /// extracts field val from current value
    pub fn dccvadp_1_extract(&mut self) -> u64 {
        // bits 9..9
        (self.val >> 9) & 0x1
    }

    /// inserts field val into current value
    pub fn dccvadp_1_insert(&mut self, val: u64) {
        // bits 9..9
        self.val = self.val & !(0x1 << 9) | ((val & 0x1) << 9);
    }

    /// extracts field val from current value
    pub fn dccvap_extract(&mut self) -> u64 {
        // bits 8..8
        (self.val >> 8) & 0x1
    }

    /// inserts field val into current value
    pub fn dccvap_insert(&mut self, val: u64) {
        // bits 8..8
        self.val = self.val & !(0x1 << 8) | ((val & 0x1) << 8);
    }

    /// extracts field val from current value
    pub fn dccvau_extract(&mut self) -> u64 {
        // bits 7..7
        (self.val >> 7) & 0x1
    }

    /// inserts field val into current value
    pub fn dccvau_insert(&mut self, val: u64) {
        // bits 7..7
        self.val = self.val & !(0x1 << 7) | ((val & 0x1) << 7);
    }

    /// extracts field val from current value
    pub fn dccisw_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }

    /// inserts field val into current value
    pub fn dccisw_insert(&mut self, val: u64) {
        // bits 6..6
        self.val = self.val & !(0x1 << 6) | ((val & 0x1) << 6);
    }

    /// extracts field val from current value
    pub fn dccsw_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn dccsw_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn dcisw_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn dcisw_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn dcivac_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }

    /// inserts field val into current value
    pub fn dcivac_insert(&mut self, val: u64) {
        // bits 3..3
        self.val = self.val & !(0x1 << 3) | ((val & 0x1) << 3);
    }

    /// extracts field val from current value
    pub fn icivau_extract(&mut self) -> u64 {
        // bits 2..2
        (self.val >> 2) & 0x1
    }

    /// inserts field val into current value
    pub fn icivau_insert(&mut self, val: u64) {
        // bits 2..2
        self.val = self.val & !(0x1 << 2) | ((val & 0x1) << 2);
    }

    /// extracts field val from current value
    pub fn iciallu_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn iciallu_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }

    /// extracts field val from current value
    pub fn icialluis_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn icialluis_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

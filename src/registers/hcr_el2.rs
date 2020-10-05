/*
 * MIT License
 *
 * Copyright (c) 2020 Reto Achermann
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */


/***********************************************************************************************
 * ***
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 * Generated on: 2020-10-05T16:49:32.040579
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 **********************************************************************************************
 * * */

/*
 * ================================================================================================
 * Register Information
 * ================================================================================================
 *
 * Register:    Hypervisor Configuration Register (hcr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Provides configuration controls for virtualization, including defining whether
 * various operations are trapped to EL2. File:        AArch64-hcr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Hypervisor Configuration Register (hcr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, HCR_EL2
        llvm_asm!("mrs $0, hcr_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Hypervisor Configuration Register (hcr_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR HCR_EL2, <Xt>
        llvm_asm!("msr hcr_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn twedel_1_read() -> u64 {
    // bits 60..63
    let val = reg_rawrd();
    (val >> 60) & 0xf
}

/// inserts field val into register
pub fn twedel_1_write(newval: u64) {
    // bits 60..63
    let val = reg_rawrd();
    reg_rawwr(val & !(0xf << 60) | ((newval & 0xf) << 60));
}

/// reads field val from register
pub fn tweden_1_read() -> u64 {
    // bits 59..59
    let val = reg_rawrd();
    (val >> 59) & 0x1
}

/// inserts field val into register
pub fn tweden_1_write(newval: u64) {
    // bits 59..59
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 59) | ((newval & 0x1) << 59));
}

/// reads field val from register
pub fn tid5_1_read() -> u64 {
    // bits 58..58
    let val = reg_rawrd();
    (val >> 58) & 0x1
}

/// inserts field val into register
pub fn tid5_1_write(newval: u64) {
    // bits 58..58
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 58) | ((newval & 0x1) << 58));
}

/// reads field val from register
pub fn dct_1_read() -> u64 {
    // bits 57..57
    let val = reg_rawrd();
    (val >> 57) & 0x1
}

/// inserts field val into register
pub fn dct_1_write(newval: u64) {
    // bits 57..57
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 57) | ((newval & 0x1) << 57));
}

/// reads field val from register
pub fn ata_1_read() -> u64 {
    // bits 56..56
    let val = reg_rawrd();
    (val >> 56) & 0x1
}

/// inserts field val into register
pub fn ata_1_write(newval: u64) {
    // bits 56..56
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 56) | ((newval & 0x1) << 56));
}

/// reads field val from register
pub fn ttlbos_1_read() -> u64 {
    // bits 55..55
    let val = reg_rawrd();
    (val >> 55) & 0x1
}

/// inserts field val into register
pub fn ttlbos_1_write(newval: u64) {
    // bits 55..55
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 55) | ((newval & 0x1) << 55));
}

/// reads field val from register
pub fn ttlbis_1_read() -> u64 {
    // bits 54..54
    let val = reg_rawrd();
    (val >> 54) & 0x1
}

/// inserts field val into register
pub fn ttlbis_1_write(newval: u64) {
    // bits 54..54
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 54) | ((newval & 0x1) << 54));
}

/// reads field val from register
pub fn enscxt_1_read() -> u64 {
    // bits 53..53
    let val = reg_rawrd();
    (val >> 53) & 0x1
}

/// inserts field val into register
pub fn enscxt_1_write(newval: u64) {
    // bits 53..53
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 53) | ((newval & 0x1) << 53));
}

/// reads field val from register
pub fn tocu_1_read() -> u64 {
    // bits 52..52
    let val = reg_rawrd();
    (val >> 52) & 0x1
}

/// inserts field val into register
pub fn tocu_1_write(newval: u64) {
    // bits 52..52
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 52) | ((newval & 0x1) << 52));
}

/// reads field val from register
pub fn amvoffen_1_read() -> u64 {
    // bits 51..51
    let val = reg_rawrd();
    (val >> 51) & 0x1
}

/// inserts field val into register
pub fn amvoffen_1_write(newval: u64) {
    // bits 51..51
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 51) | ((newval & 0x1) << 51));
}

/// reads field val from register
pub fn ticab_1_read() -> u64 {
    // bits 50..50
    let val = reg_rawrd();
    (val >> 50) & 0x1
}

/// inserts field val into register
pub fn ticab_1_write(newval: u64) {
    // bits 50..50
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 50) | ((newval & 0x1) << 50));
}

/// reads field val from register
pub fn tid4_1_read() -> u64 {
    // bits 49..49
    let val = reg_rawrd();
    (val >> 49) & 0x1
}

/// inserts field val into register
pub fn tid4_1_write(newval: u64) {
    // bits 49..49
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 49) | ((newval & 0x1) << 49));
}

/// reads field val from register
pub fn fien_1_read() -> u64 {
    // bits 47..47
    let val = reg_rawrd();
    (val >> 47) & 0x1
}

/// inserts field val into register
pub fn fien_1_write(newval: u64) {
    // bits 47..47
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 47) | ((newval & 0x1) << 47));
}

/// reads field val from register
pub fn fwb_1_read() -> u64 {
    // bits 46..46
    let val = reg_rawrd();
    (val >> 46) & 0x1
}

/// inserts field val into register
pub fn fwb_1_write(newval: u64) {
    // bits 46..46
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 46) | ((newval & 0x1) << 46));
}

/// reads field val from register
pub fn nv2_1_read() -> u64 {
    // bits 45..45
    let val = reg_rawrd();
    (val >> 45) & 0x1
}

/// inserts field val into register
pub fn nv2_1_write(newval: u64) {
    // bits 45..45
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 45) | ((newval & 0x1) << 45));
}

/// reads field val from register
pub fn at_1_read() -> u64 {
    // bits 44..44
    let val = reg_rawrd();
    (val >> 44) & 0x1
}

/// inserts field val into register
pub fn at_1_write(newval: u64) {
    // bits 44..44
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 44) | ((newval & 0x1) << 44));
}

/// reads field val from register
pub fn nv1_1_read() -> u64 {
    // bits 43..43
    let val = reg_rawrd();
    (val >> 43) & 0x1
}

/// inserts field val into register
pub fn nv1_1_write(newval: u64) {
    // bits 43..43
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 43) | ((newval & 0x1) << 43));
}

/// reads field val from register
pub fn nv1_2_read() -> u64 {
    // bits 43..43
    let val = reg_rawrd();
    (val >> 43) & 0x1
}

/// inserts field val into register
pub fn nv1_2_write(newval: u64) {
    // bits 43..43
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 43) | ((newval & 0x1) << 43));
}

/// reads field val from register
pub fn nv_1_read() -> u64 {
    // bits 42..42
    let val = reg_rawrd();
    (val >> 42) & 0x1
}

/// inserts field val into register
pub fn nv_1_write(newval: u64) {
    // bits 42..42
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 42) | ((newval & 0x1) << 42));
}

/// reads field val from register
pub fn nv_2_read() -> u64 {
    // bits 42..42
    let val = reg_rawrd();
    (val >> 42) & 0x1
}

/// inserts field val into register
pub fn nv_2_write(newval: u64) {
    // bits 42..42
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 42) | ((newval & 0x1) << 42));
}

/// reads field val from register
pub fn api_1_read() -> u64 {
    // bits 41..41
    let val = reg_rawrd();
    (val >> 41) & 0x1
}

/// inserts field val into register
pub fn api_1_write(newval: u64) {
    // bits 41..41
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 41) | ((newval & 0x1) << 41));
}

/// reads field val from register
pub fn apk_1_read() -> u64 {
    // bits 40..40
    let val = reg_rawrd();
    (val >> 40) & 0x1
}

/// inserts field val into register
pub fn apk_1_write(newval: u64) {
    // bits 40..40
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 40) | ((newval & 0x1) << 40));
}

/// reads field val from register
pub fn miocnce_read() -> u64 {
    // bits 38..38
    let val = reg_rawrd();
    (val >> 38) & 0x1
}

/// inserts field val into register
pub fn miocnce_write(newval: u64) {
    // bits 38..38
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 38) | ((newval & 0x1) << 38));
}

/// reads field val from register
pub fn tea_1_read() -> u64 {
    // bits 37..37
    let val = reg_rawrd();
    (val >> 37) & 0x1
}

/// inserts field val into register
pub fn tea_1_write(newval: u64) {
    // bits 37..37
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 37) | ((newval & 0x1) << 37));
}

/// reads field val from register
pub fn terr_1_read() -> u64 {
    // bits 36..36
    let val = reg_rawrd();
    (val >> 36) & 0x1
}

/// inserts field val into register
pub fn terr_1_write(newval: u64) {
    // bits 36..36
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 36) | ((newval & 0x1) << 36));
}

/// reads field val from register
pub fn tlor_1_read() -> u64 {
    // bits 35..35
    let val = reg_rawrd();
    (val >> 35) & 0x1
}

/// inserts field val into register
pub fn tlor_1_write(newval: u64) {
    // bits 35..35
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 35) | ((newval & 0x1) << 35));
}

/// reads field val from register
pub fn e2h_1_read() -> u64 {
    // bits 34..34
    let val = reg_rawrd();
    (val >> 34) & 0x1
}

/// inserts field val into register
pub fn e2h_1_write(newval: u64) {
    // bits 34..34
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 34) | ((newval & 0x1) << 34));
}

/// reads field val from register
pub fn id_read() -> u64 {
    // bits 33..33
    let val = reg_rawrd();
    (val >> 33) & 0x1
}

/// inserts field val into register
pub fn id_write(newval: u64) {
    // bits 33..33
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 33) | ((newval & 0x1) << 33));
}

/// reads field val from register
pub fn cd_read() -> u64 {
    // bits 32..32
    let val = reg_rawrd();
    (val >> 32) & 0x1
}

/// inserts field val into register
pub fn cd_write(newval: u64) {
    // bits 32..32
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 32) | ((newval & 0x1) << 32));
}

/// reads field val from register
pub fn rw_1_read() -> u64 {
    // bits 31..31
    let val = reg_rawrd();
    (val >> 31) & 0x1
}

/// inserts field val into register
pub fn rw_1_write(newval: u64) {
    // bits 31..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 31) | ((newval & 0x1) << 31));
}

/// reads field val from register
pub fn trvm_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn trvm_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn hcd_1_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

/// inserts field val into register
pub fn hcd_1_write(newval: u64) {
    // bits 29..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 29) | ((newval & 0x1) << 29));
}

/// reads field val from register
pub fn tdz_read() -> u64 {
    // bits 28..28
    let val = reg_rawrd();
    (val >> 28) & 0x1
}

/// inserts field val into register
pub fn tdz_write(newval: u64) {
    // bits 28..28
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 28) | ((newval & 0x1) << 28));
}

/// reads field val from register
pub fn tge_read() -> u64 {
    // bits 27..27
    let val = reg_rawrd();
    (val >> 27) & 0x1
}

/// inserts field val into register
pub fn tge_write(newval: u64) {
    // bits 27..27
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 27) | ((newval & 0x1) << 27));
}

/// reads field val from register
pub fn tvm_read() -> u64 {
    // bits 26..26
    let val = reg_rawrd();
    (val >> 26) & 0x1
}

/// inserts field val into register
pub fn tvm_write(newval: u64) {
    // bits 26..26
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 26) | ((newval & 0x1) << 26));
}

/// reads field val from register
pub fn ttlb_read() -> u64 {
    // bits 25..25
    let val = reg_rawrd();
    (val >> 25) & 0x1
}

/// inserts field val into register
pub fn ttlb_write(newval: u64) {
    // bits 25..25
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 25) | ((newval & 0x1) << 25));
}

/// reads field val from register
pub fn tpu_read() -> u64 {
    // bits 24..24
    let val = reg_rawrd();
    (val >> 24) & 0x1
}

/// inserts field val into register
pub fn tpu_write(newval: u64) {
    // bits 24..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 24) | ((newval & 0x1) << 24));
}

/// reads field val from register
pub fn tpcp_1_read() -> u64 {
    // bits 23..23
    let val = reg_rawrd();
    (val >> 23) & 0x1
}

/// inserts field val into register
pub fn tpcp_1_write(newval: u64) {
    // bits 23..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 23) | ((newval & 0x1) << 23));
}

/// reads field val from register
pub fn tpc_2_read() -> u64 {
    // bits 23..23
    let val = reg_rawrd();
    (val >> 23) & 0x1
}

/// inserts field val into register
pub fn tpc_2_write(newval: u64) {
    // bits 23..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 23) | ((newval & 0x1) << 23));
}

/// reads field val from register
pub fn tsw_read() -> u64 {
    // bits 22..22
    let val = reg_rawrd();
    (val >> 22) & 0x1
}

/// inserts field val into register
pub fn tsw_write(newval: u64) {
    // bits 22..22
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 22) | ((newval & 0x1) << 22));
}

/// reads field val from register
pub fn tacr_read() -> u64 {
    // bits 21..21
    let val = reg_rawrd();
    (val >> 21) & 0x1
}

/// inserts field val into register
pub fn tacr_write(newval: u64) {
    // bits 21..21
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 21) | ((newval & 0x1) << 21));
}

/// reads field val from register
pub fn tidcp_read() -> u64 {
    // bits 20..20
    let val = reg_rawrd();
    (val >> 20) & 0x1
}

/// inserts field val into register
pub fn tidcp_write(newval: u64) {
    // bits 20..20
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 20) | ((newval & 0x1) << 20));
}

/// reads field val from register
pub fn tsc_read() -> u64 {
    // bits 19..19
    let val = reg_rawrd();
    (val >> 19) & 0x1
}

/// inserts field val into register
pub fn tsc_write(newval: u64) {
    // bits 19..19
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 19) | ((newval & 0x1) << 19));
}

/// reads field val from register
pub fn tid3_read() -> u64 {
    // bits 18..18
    let val = reg_rawrd();
    (val >> 18) & 0x1
}

/// inserts field val into register
pub fn tid3_write(newval: u64) {
    // bits 18..18
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 18) | ((newval & 0x1) << 18));
}

/// reads field val from register
pub fn tid2_read() -> u64 {
    // bits 17..17
    let val = reg_rawrd();
    (val >> 17) & 0x1
}

/// inserts field val into register
pub fn tid2_write(newval: u64) {
    // bits 17..17
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 17) | ((newval & 0x1) << 17));
}

/// reads field val from register
pub fn tid1_read() -> u64 {
    // bits 16..16
    let val = reg_rawrd();
    (val >> 16) & 0x1
}

/// inserts field val into register
pub fn tid1_write(newval: u64) {
    // bits 16..16
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 16) | ((newval & 0x1) << 16));
}

/// reads field val from register
pub fn tid0_1_read() -> u64 {
    // bits 15..15
    let val = reg_rawrd();
    (val >> 15) & 0x1
}

/// inserts field val into register
pub fn tid0_1_write(newval: u64) {
    // bits 15..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 15) | ((newval & 0x1) << 15));
}

/// reads field val from register
pub fn twe_read() -> u64 {
    // bits 14..14
    let val = reg_rawrd();
    (val >> 14) & 0x1
}

/// inserts field val into register
pub fn twe_write(newval: u64) {
    // bits 14..14
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 14) | ((newval & 0x1) << 14));
}

/// reads field val from register
pub fn twi_read() -> u64 {
    // bits 13..13
    let val = reg_rawrd();
    (val >> 13) & 0x1
}

/// inserts field val into register
pub fn twi_write(newval: u64) {
    // bits 13..13
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 13) | ((newval & 0x1) << 13));
}

/// reads field val from register
pub fn dc_read() -> u64 {
    // bits 12..12
    let val = reg_rawrd();
    (val >> 12) & 0x1
}

/// inserts field val into register
pub fn dc_write(newval: u64) {
    // bits 12..12
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 12) | ((newval & 0x1) << 12));
}

/// reads field val from register
pub fn bsu_read() -> u64 {
    // bits 10..11
    let val = reg_rawrd();
    (val >> 10) & 0x3
}

/// inserts field val into register
pub fn bsu_write(newval: u64) {
    // bits 10..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 10) | ((newval & 0x3) << 10));
}

/// reads field val from register
pub fn fb_read() -> u64 {
    // bits 9..9
    let val = reg_rawrd();
    (val >> 9) & 0x1
}

/// inserts field val into register
pub fn fb_write(newval: u64) {
    // bits 9..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 9) | ((newval & 0x1) << 9));
}

/// reads field val from register
pub fn vse_read() -> u64 {
    // bits 8..8
    let val = reg_rawrd();
    (val >> 8) & 0x1
}

/// inserts field val into register
pub fn vse_write(newval: u64) {
    // bits 8..8
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 8) | ((newval & 0x1) << 8));
}

/// reads field val from register
pub fn vi_read() -> u64 {
    // bits 7..7
    let val = reg_rawrd();
    (val >> 7) & 0x1
}

/// inserts field val into register
pub fn vi_write(newval: u64) {
    // bits 7..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 7) | ((newval & 0x1) << 7));
}

/// reads field val from register
pub fn vf_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

/// inserts field val into register
pub fn vf_write(newval: u64) {
    // bits 6..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 6) | ((newval & 0x1) << 6));
}

/// reads field val from register
pub fn amo_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn amo_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn imo_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn imo_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn fmo_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

/// inserts field val into register
pub fn fmo_write(newval: u64) {
    // bits 3..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 3) | ((newval & 0x1) << 3));
}

/// reads field val from register
pub fn ptw_read() -> u64 {
    // bits 2..2
    let val = reg_rawrd();
    (val >> 2) & 0x1
}

/// inserts field val into register
pub fn ptw_write(newval: u64) {
    // bits 2..2
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 2) | ((newval & 0x1) << 2));
}

/// reads field val from register
pub fn swio_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn swio_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}

/// reads field val from register
pub fn vm_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn vm_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Hypervisor Configuration Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register hcr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xfffeff7fffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xfffeff7fffffffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xfffeff7fffffffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 18446462048977027071;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn twedel_1_extract(&mut self) -> u64 {
        // bits 60..63
        (self.val >> 60) & 0xf
    }

    /// inserts field val into current value
    pub fn twedel_1_insert(&mut self, val: u64) {
        // bits 60..63
        self.val = self.val & !(0xf << 60) | ((val & 0xf) << 60);
    }

    /// extracts field val from current value
    pub fn tweden_1_extract(&mut self) -> u64 {
        // bits 59..59
        (self.val >> 59) & 0x1
    }

    /// inserts field val into current value
    pub fn tweden_1_insert(&mut self, val: u64) {
        // bits 59..59
        self.val = self.val & !(0x1 << 59) | ((val & 0x1) << 59);
    }

    /// extracts field val from current value
    pub fn tid5_1_extract(&mut self) -> u64 {
        // bits 58..58
        (self.val >> 58) & 0x1
    }

    /// inserts field val into current value
    pub fn tid5_1_insert(&mut self, val: u64) {
        // bits 58..58
        self.val = self.val & !(0x1 << 58) | ((val & 0x1) << 58);
    }

    /// extracts field val from current value
    pub fn dct_1_extract(&mut self) -> u64 {
        // bits 57..57
        (self.val >> 57) & 0x1
    }

    /// inserts field val into current value
    pub fn dct_1_insert(&mut self, val: u64) {
        // bits 57..57
        self.val = self.val & !(0x1 << 57) | ((val & 0x1) << 57);
    }

    /// extracts field val from current value
    pub fn ata_1_extract(&mut self) -> u64 {
        // bits 56..56
        (self.val >> 56) & 0x1
    }

    /// inserts field val into current value
    pub fn ata_1_insert(&mut self, val: u64) {
        // bits 56..56
        self.val = self.val & !(0x1 << 56) | ((val & 0x1) << 56);
    }

    /// extracts field val from current value
    pub fn ttlbos_1_extract(&mut self) -> u64 {
        // bits 55..55
        (self.val >> 55) & 0x1
    }

    /// inserts field val into current value
    pub fn ttlbos_1_insert(&mut self, val: u64) {
        // bits 55..55
        self.val = self.val & !(0x1 << 55) | ((val & 0x1) << 55);
    }

    /// extracts field val from current value
    pub fn ttlbis_1_extract(&mut self) -> u64 {
        // bits 54..54
        (self.val >> 54) & 0x1
    }

    /// inserts field val into current value
    pub fn ttlbis_1_insert(&mut self, val: u64) {
        // bits 54..54
        self.val = self.val & !(0x1 << 54) | ((val & 0x1) << 54);
    }

    /// extracts field val from current value
    pub fn enscxt_1_extract(&mut self) -> u64 {
        // bits 53..53
        (self.val >> 53) & 0x1
    }

    /// inserts field val into current value
    pub fn enscxt_1_insert(&mut self, val: u64) {
        // bits 53..53
        self.val = self.val & !(0x1 << 53) | ((val & 0x1) << 53);
    }

    /// extracts field val from current value
    pub fn tocu_1_extract(&mut self) -> u64 {
        // bits 52..52
        (self.val >> 52) & 0x1
    }

    /// inserts field val into current value
    pub fn tocu_1_insert(&mut self, val: u64) {
        // bits 52..52
        self.val = self.val & !(0x1 << 52) | ((val & 0x1) << 52);
    }

    /// extracts field val from current value
    pub fn amvoffen_1_extract(&mut self) -> u64 {
        // bits 51..51
        (self.val >> 51) & 0x1
    }

    /// inserts field val into current value
    pub fn amvoffen_1_insert(&mut self, val: u64) {
        // bits 51..51
        self.val = self.val & !(0x1 << 51) | ((val & 0x1) << 51);
    }

    /// extracts field val from current value
    pub fn ticab_1_extract(&mut self) -> u64 {
        // bits 50..50
        (self.val >> 50) & 0x1
    }

    /// inserts field val into current value
    pub fn ticab_1_insert(&mut self, val: u64) {
        // bits 50..50
        self.val = self.val & !(0x1 << 50) | ((val & 0x1) << 50);
    }

    /// extracts field val from current value
    pub fn tid4_1_extract(&mut self) -> u64 {
        // bits 49..49
        (self.val >> 49) & 0x1
    }

    /// inserts field val into current value
    pub fn tid4_1_insert(&mut self, val: u64) {
        // bits 49..49
        self.val = self.val & !(0x1 << 49) | ((val & 0x1) << 49);
    }

    /// extracts field val from current value
    pub fn fien_1_extract(&mut self) -> u64 {
        // bits 47..47
        (self.val >> 47) & 0x1
    }

    /// inserts field val into current value
    pub fn fien_1_insert(&mut self, val: u64) {
        // bits 47..47
        self.val = self.val & !(0x1 << 47) | ((val & 0x1) << 47);
    }

    /// extracts field val from current value
    pub fn fwb_1_extract(&mut self) -> u64 {
        // bits 46..46
        (self.val >> 46) & 0x1
    }

    /// inserts field val into current value
    pub fn fwb_1_insert(&mut self, val: u64) {
        // bits 46..46
        self.val = self.val & !(0x1 << 46) | ((val & 0x1) << 46);
    }

    /// extracts field val from current value
    pub fn nv2_1_extract(&mut self) -> u64 {
        // bits 45..45
        (self.val >> 45) & 0x1
    }

    /// inserts field val into current value
    pub fn nv2_1_insert(&mut self, val: u64) {
        // bits 45..45
        self.val = self.val & !(0x1 << 45) | ((val & 0x1) << 45);
    }

    /// extracts field val from current value
    pub fn at_1_extract(&mut self) -> u64 {
        // bits 44..44
        (self.val >> 44) & 0x1
    }

    /// inserts field val into current value
    pub fn at_1_insert(&mut self, val: u64) {
        // bits 44..44
        self.val = self.val & !(0x1 << 44) | ((val & 0x1) << 44);
    }

    /// extracts field val from current value
    pub fn nv1_1_extract(&mut self) -> u64 {
        // bits 43..43
        (self.val >> 43) & 0x1
    }

    /// inserts field val into current value
    pub fn nv1_1_insert(&mut self, val: u64) {
        // bits 43..43
        self.val = self.val & !(0x1 << 43) | ((val & 0x1) << 43);
    }

    /// extracts field val from current value
    pub fn nv1_2_extract(&mut self) -> u64 {
        // bits 43..43
        (self.val >> 43) & 0x1
    }

    /// inserts field val into current value
    pub fn nv1_2_insert(&mut self, val: u64) {
        // bits 43..43
        self.val = self.val & !(0x1 << 43) | ((val & 0x1) << 43);
    }

    /// extracts field val from current value
    pub fn nv_1_extract(&mut self) -> u64 {
        // bits 42..42
        (self.val >> 42) & 0x1
    }

    /// inserts field val into current value
    pub fn nv_1_insert(&mut self, val: u64) {
        // bits 42..42
        self.val = self.val & !(0x1 << 42) | ((val & 0x1) << 42);
    }

    /// extracts field val from current value
    pub fn nv_2_extract(&mut self) -> u64 {
        // bits 42..42
        (self.val >> 42) & 0x1
    }

    /// inserts field val into current value
    pub fn nv_2_insert(&mut self, val: u64) {
        // bits 42..42
        self.val = self.val & !(0x1 << 42) | ((val & 0x1) << 42);
    }

    /// extracts field val from current value
    pub fn api_1_extract(&mut self) -> u64 {
        // bits 41..41
        (self.val >> 41) & 0x1
    }

    /// inserts field val into current value
    pub fn api_1_insert(&mut self, val: u64) {
        // bits 41..41
        self.val = self.val & !(0x1 << 41) | ((val & 0x1) << 41);
    }

    /// extracts field val from current value
    pub fn apk_1_extract(&mut self) -> u64 {
        // bits 40..40
        (self.val >> 40) & 0x1
    }

    /// inserts field val into current value
    pub fn apk_1_insert(&mut self, val: u64) {
        // bits 40..40
        self.val = self.val & !(0x1 << 40) | ((val & 0x1) << 40);
    }

    /// extracts field val from current value
    pub fn miocnce_extract(&mut self) -> u64 {
        // bits 38..38
        (self.val >> 38) & 0x1
    }

    /// inserts field val into current value
    pub fn miocnce_insert(&mut self, val: u64) {
        // bits 38..38
        self.val = self.val & !(0x1 << 38) | ((val & 0x1) << 38);
    }

    /// extracts field val from current value
    pub fn tea_1_extract(&mut self) -> u64 {
        // bits 37..37
        (self.val >> 37) & 0x1
    }

    /// inserts field val into current value
    pub fn tea_1_insert(&mut self, val: u64) {
        // bits 37..37
        self.val = self.val & !(0x1 << 37) | ((val & 0x1) << 37);
    }

    /// extracts field val from current value
    pub fn terr_1_extract(&mut self) -> u64 {
        // bits 36..36
        (self.val >> 36) & 0x1
    }

    /// inserts field val into current value
    pub fn terr_1_insert(&mut self, val: u64) {
        // bits 36..36
        self.val = self.val & !(0x1 << 36) | ((val & 0x1) << 36);
    }

    /// extracts field val from current value
    pub fn tlor_1_extract(&mut self) -> u64 {
        // bits 35..35
        (self.val >> 35) & 0x1
    }

    /// inserts field val into current value
    pub fn tlor_1_insert(&mut self, val: u64) {
        // bits 35..35
        self.val = self.val & !(0x1 << 35) | ((val & 0x1) << 35);
    }

    /// extracts field val from current value
    pub fn e2h_1_extract(&mut self) -> u64 {
        // bits 34..34
        (self.val >> 34) & 0x1
    }

    /// inserts field val into current value
    pub fn e2h_1_insert(&mut self, val: u64) {
        // bits 34..34
        self.val = self.val & !(0x1 << 34) | ((val & 0x1) << 34);
    }

    /// extracts field val from current value
    pub fn id_extract(&mut self) -> u64 {
        // bits 33..33
        (self.val >> 33) & 0x1
    }

    /// inserts field val into current value
    pub fn id_insert(&mut self, val: u64) {
        // bits 33..33
        self.val = self.val & !(0x1 << 33) | ((val & 0x1) << 33);
    }

    /// extracts field val from current value
    pub fn cd_extract(&mut self) -> u64 {
        // bits 32..32
        (self.val >> 32) & 0x1
    }

    /// inserts field val into current value
    pub fn cd_insert(&mut self, val: u64) {
        // bits 32..32
        self.val = self.val & !(0x1 << 32) | ((val & 0x1) << 32);
    }

    /// extracts field val from current value
    pub fn rw_1_extract(&mut self) -> u64 {
        // bits 31..31
        (self.val >> 31) & 0x1
    }

    /// inserts field val into current value
    pub fn rw_1_insert(&mut self, val: u64) {
        // bits 31..31
        self.val = self.val & !(0x1 << 31) | ((val & 0x1) << 31);
    }

    /// extracts field val from current value
    pub fn trvm_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn trvm_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn hcd_1_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }

    /// inserts field val into current value
    pub fn hcd_1_insert(&mut self, val: u64) {
        // bits 29..29
        self.val = self.val & !(0x1 << 29) | ((val & 0x1) << 29);
    }

    /// extracts field val from current value
    pub fn tdz_extract(&mut self) -> u64 {
        // bits 28..28
        (self.val >> 28) & 0x1
    }

    /// inserts field val into current value
    pub fn tdz_insert(&mut self, val: u64) {
        // bits 28..28
        self.val = self.val & !(0x1 << 28) | ((val & 0x1) << 28);
    }

    /// extracts field val from current value
    pub fn tge_extract(&mut self) -> u64 {
        // bits 27..27
        (self.val >> 27) & 0x1
    }

    /// inserts field val into current value
    pub fn tge_insert(&mut self, val: u64) {
        // bits 27..27
        self.val = self.val & !(0x1 << 27) | ((val & 0x1) << 27);
    }

    /// extracts field val from current value
    pub fn tvm_extract(&mut self) -> u64 {
        // bits 26..26
        (self.val >> 26) & 0x1
    }

    /// inserts field val into current value
    pub fn tvm_insert(&mut self, val: u64) {
        // bits 26..26
        self.val = self.val & !(0x1 << 26) | ((val & 0x1) << 26);
    }

    /// extracts field val from current value
    pub fn ttlb_extract(&mut self) -> u64 {
        // bits 25..25
        (self.val >> 25) & 0x1
    }

    /// inserts field val into current value
    pub fn ttlb_insert(&mut self, val: u64) {
        // bits 25..25
        self.val = self.val & !(0x1 << 25) | ((val & 0x1) << 25);
    }

    /// extracts field val from current value
    pub fn tpu_extract(&mut self) -> u64 {
        // bits 24..24
        (self.val >> 24) & 0x1
    }

    /// inserts field val into current value
    pub fn tpu_insert(&mut self, val: u64) {
        // bits 24..24
        self.val = self.val & !(0x1 << 24) | ((val & 0x1) << 24);
    }

    /// extracts field val from current value
    pub fn tpcp_1_extract(&mut self) -> u64 {
        // bits 23..23
        (self.val >> 23) & 0x1
    }

    /// inserts field val into current value
    pub fn tpcp_1_insert(&mut self, val: u64) {
        // bits 23..23
        self.val = self.val & !(0x1 << 23) | ((val & 0x1) << 23);
    }

    /// extracts field val from current value
    pub fn tpc_2_extract(&mut self) -> u64 {
        // bits 23..23
        (self.val >> 23) & 0x1
    }

    /// inserts field val into current value
    pub fn tpc_2_insert(&mut self, val: u64) {
        // bits 23..23
        self.val = self.val & !(0x1 << 23) | ((val & 0x1) << 23);
    }

    /// extracts field val from current value
    pub fn tsw_extract(&mut self) -> u64 {
        // bits 22..22
        (self.val >> 22) & 0x1
    }

    /// inserts field val into current value
    pub fn tsw_insert(&mut self, val: u64) {
        // bits 22..22
        self.val = self.val & !(0x1 << 22) | ((val & 0x1) << 22);
    }

    /// extracts field val from current value
    pub fn tacr_extract(&mut self) -> u64 {
        // bits 21..21
        (self.val >> 21) & 0x1
    }

    /// inserts field val into current value
    pub fn tacr_insert(&mut self, val: u64) {
        // bits 21..21
        self.val = self.val & !(0x1 << 21) | ((val & 0x1) << 21);
    }

    /// extracts field val from current value
    pub fn tidcp_extract(&mut self) -> u64 {
        // bits 20..20
        (self.val >> 20) & 0x1
    }

    /// inserts field val into current value
    pub fn tidcp_insert(&mut self, val: u64) {
        // bits 20..20
        self.val = self.val & !(0x1 << 20) | ((val & 0x1) << 20);
    }

    /// extracts field val from current value
    pub fn tsc_extract(&mut self) -> u64 {
        // bits 19..19
        (self.val >> 19) & 0x1
    }

    /// inserts field val into current value
    pub fn tsc_insert(&mut self, val: u64) {
        // bits 19..19
        self.val = self.val & !(0x1 << 19) | ((val & 0x1) << 19);
    }

    /// extracts field val from current value
    pub fn tid3_extract(&mut self) -> u64 {
        // bits 18..18
        (self.val >> 18) & 0x1
    }

    /// inserts field val into current value
    pub fn tid3_insert(&mut self, val: u64) {
        // bits 18..18
        self.val = self.val & !(0x1 << 18) | ((val & 0x1) << 18);
    }

    /// extracts field val from current value
    pub fn tid2_extract(&mut self) -> u64 {
        // bits 17..17
        (self.val >> 17) & 0x1
    }

    /// inserts field val into current value
    pub fn tid2_insert(&mut self, val: u64) {
        // bits 17..17
        self.val = self.val & !(0x1 << 17) | ((val & 0x1) << 17);
    }

    /// extracts field val from current value
    pub fn tid1_extract(&mut self) -> u64 {
        // bits 16..16
        (self.val >> 16) & 0x1
    }

    /// inserts field val into current value
    pub fn tid1_insert(&mut self, val: u64) {
        // bits 16..16
        self.val = self.val & !(0x1 << 16) | ((val & 0x1) << 16);
    }

    /// extracts field val from current value
    pub fn tid0_1_extract(&mut self) -> u64 {
        // bits 15..15
        (self.val >> 15) & 0x1
    }

    /// inserts field val into current value
    pub fn tid0_1_insert(&mut self, val: u64) {
        // bits 15..15
        self.val = self.val & !(0x1 << 15) | ((val & 0x1) << 15);
    }

    /// extracts field val from current value
    pub fn twe_extract(&mut self) -> u64 {
        // bits 14..14
        (self.val >> 14) & 0x1
    }

    /// inserts field val into current value
    pub fn twe_insert(&mut self, val: u64) {
        // bits 14..14
        self.val = self.val & !(0x1 << 14) | ((val & 0x1) << 14);
    }

    /// extracts field val from current value
    pub fn twi_extract(&mut self) -> u64 {
        // bits 13..13
        (self.val >> 13) & 0x1
    }

    /// inserts field val into current value
    pub fn twi_insert(&mut self, val: u64) {
        // bits 13..13
        self.val = self.val & !(0x1 << 13) | ((val & 0x1) << 13);
    }

    /// extracts field val from current value
    pub fn dc_extract(&mut self) -> u64 {
        // bits 12..12
        (self.val >> 12) & 0x1
    }

    /// inserts field val into current value
    pub fn dc_insert(&mut self, val: u64) {
        // bits 12..12
        self.val = self.val & !(0x1 << 12) | ((val & 0x1) << 12);
    }

    /// extracts field val from current value
    pub fn bsu_extract(&mut self) -> u64 {
        // bits 10..11
        (self.val >> 10) & 0x3
    }

    /// inserts field val into current value
    pub fn bsu_insert(&mut self, val: u64) {
        // bits 10..11
        self.val = self.val & !(0x3 << 10) | ((val & 0x3) << 10);
    }

    /// extracts field val from current value
    pub fn fb_extract(&mut self) -> u64 {
        // bits 9..9
        (self.val >> 9) & 0x1
    }

    /// inserts field val into current value
    pub fn fb_insert(&mut self, val: u64) {
        // bits 9..9
        self.val = self.val & !(0x1 << 9) | ((val & 0x1) << 9);
    }

    /// extracts field val from current value
    pub fn vse_extract(&mut self) -> u64 {
        // bits 8..8
        (self.val >> 8) & 0x1
    }

    /// inserts field val into current value
    pub fn vse_insert(&mut self, val: u64) {
        // bits 8..8
        self.val = self.val & !(0x1 << 8) | ((val & 0x1) << 8);
    }

    /// extracts field val from current value
    pub fn vi_extract(&mut self) -> u64 {
        // bits 7..7
        (self.val >> 7) & 0x1
    }

    /// inserts field val into current value
    pub fn vi_insert(&mut self, val: u64) {
        // bits 7..7
        self.val = self.val & !(0x1 << 7) | ((val & 0x1) << 7);
    }

    /// extracts field val from current value
    pub fn vf_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }

    /// inserts field val into current value
    pub fn vf_insert(&mut self, val: u64) {
        // bits 6..6
        self.val = self.val & !(0x1 << 6) | ((val & 0x1) << 6);
    }

    /// extracts field val from current value
    pub fn amo_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn amo_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn imo_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn imo_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn fmo_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }

    /// inserts field val into current value
    pub fn fmo_insert(&mut self, val: u64) {
        // bits 3..3
        self.val = self.val & !(0x1 << 3) | ((val & 0x1) << 3);
    }

    /// extracts field val from current value
    pub fn ptw_extract(&mut self) -> u64 {
        // bits 2..2
        (self.val >> 2) & 0x1
    }

    /// inserts field val into current value
    pub fn ptw_insert(&mut self, val: u64) {
        // bits 2..2
        self.val = self.val & !(0x1 << 2) | ((val & 0x1) << 2);
    }

    /// extracts field val from current value
    pub fn swio_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn swio_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }

    /// extracts field val from current value
    pub fn vm_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn vm_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

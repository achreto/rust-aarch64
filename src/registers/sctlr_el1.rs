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
 * Generated on: 2020-10-05T16:30:11.724260
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
 * Register:    System Control Register (EL1) (sctlr_el1)
 * Group:       Other system control registers
 * Type:        64-bit Register
 * Description: Provides top level control of the system, including its
 * memory system, at EL1 and EL0. File:        AArch64-sctlr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the System Control Register (EL1) (sctlr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, SCTLR_EL1
        llvm_asm!("mrs $0, sctlr_el1" : "=r"(regval));
    }
    return regval;
}


/// writing the System Control Register (EL1) (sctlr_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR SCTLR_EL1, <Xt>
        llvm_asm!("msr sctlr_el1, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn epan_1_read() -> u64 {
    // bits 57..57
    let val = reg_rawrd();
    (val >> 57) & 0x1
}

/// inserts field val into register
pub fn epan_1_write(newval: u64) {
    // bits 57..57
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 57) | ((newval & 0x1) << 57));
}

/// reads field val from register
pub fn enals_1_read() -> u64 {
    // bits 56..56
    let val = reg_rawrd();
    (val >> 56) & 0x1
}

/// inserts field val into register
pub fn enals_1_write(newval: u64) {
    // bits 56..56
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 56) | ((newval & 0x1) << 56));
}

/// reads field val from register
pub fn enas0_1_read() -> u64 {
    // bits 55..55
    let val = reg_rawrd();
    (val >> 55) & 0x1
}

/// inserts field val into register
pub fn enas0_1_write(newval: u64) {
    // bits 55..55
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 55) | ((newval & 0x1) << 55));
}

/// reads field val from register
pub fn enasr_1_read() -> u64 {
    // bits 54..54
    let val = reg_rawrd();
    (val >> 54) & 0x1
}

/// inserts field val into register
pub fn enasr_1_write(newval: u64) {
    // bits 54..54
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 54) | ((newval & 0x1) << 54));
}

/// reads field val from register
pub fn twedel_1_read() -> u64 {
    // bits 46..49
    let val = reg_rawrd();
    (val >> 46) & 0xf
}

/// inserts field val into register
pub fn twedel_1_write(newval: u64) {
    // bits 46..49
    let val = reg_rawrd();
    reg_rawwr(val & !(0xf << 46) | ((newval & 0xf) << 46));
}

/// reads field val from register
pub fn tweden_1_read() -> u64 {
    // bits 45..45
    let val = reg_rawrd();
    (val >> 45) & 0x1
}

/// inserts field val into register
pub fn tweden_1_write(newval: u64) {
    // bits 45..45
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 45) | ((newval & 0x1) << 45));
}

/// reads field val from register
pub fn dssbs_1_read() -> u64 {
    // bits 44..44
    let val = reg_rawrd();
    (val >> 44) & 0x1
}

/// inserts field val into register
pub fn dssbs_1_write(newval: u64) {
    // bits 44..44
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 44) | ((newval & 0x1) << 44));
}

/// reads field val from register
pub fn ata_1_read() -> u64 {
    // bits 43..43
    let val = reg_rawrd();
    (val >> 43) & 0x1
}

/// inserts field val into register
pub fn ata_1_write(newval: u64) {
    // bits 43..43
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 43) | ((newval & 0x1) << 43));
}

/// reads field val from register
pub fn ata0_1_read() -> u64 {
    // bits 42..42
    let val = reg_rawrd();
    (val >> 42) & 0x1
}

/// inserts field val into register
pub fn ata0_1_write(newval: u64) {
    // bits 42..42
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 42) | ((newval & 0x1) << 42));
}

/// reads field val from register
pub fn tcf_1_read() -> u64 {
    // bits 40..41
    let val = reg_rawrd();
    (val >> 40) & 0x3
}

/// inserts field val into register
pub fn tcf_1_write(newval: u64) {
    // bits 40..41
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 40) | ((newval & 0x3) << 40));
}

/// reads field val from register
pub fn tcf0_1_read() -> u64 {
    // bits 38..39
    let val = reg_rawrd();
    (val >> 38) & 0x3
}

/// inserts field val into register
pub fn tcf0_1_write(newval: u64) {
    // bits 38..39
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 38) | ((newval & 0x3) << 38));
}

/// reads field val from register
pub fn itfsb_1_read() -> u64 {
    // bits 37..37
    let val = reg_rawrd();
    (val >> 37) & 0x1
}

/// inserts field val into register
pub fn itfsb_1_write(newval: u64) {
    // bits 37..37
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 37) | ((newval & 0x1) << 37));
}

/// reads field val from register
pub fn bt1_1_read() -> u64 {
    // bits 36..36
    let val = reg_rawrd();
    (val >> 36) & 0x1
}

/// inserts field val into register
pub fn bt1_1_write(newval: u64) {
    // bits 36..36
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 36) | ((newval & 0x1) << 36));
}

/// reads field val from register
pub fn bt0_1_read() -> u64 {
    // bits 35..35
    let val = reg_rawrd();
    (val >> 35) & 0x1
}

/// inserts field val into register
pub fn bt0_1_write(newval: u64) {
    // bits 35..35
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 35) | ((newval & 0x1) << 35));
}

/// reads field val from register
pub fn enia_1_read() -> u64 {
    // bits 31..31
    let val = reg_rawrd();
    (val >> 31) & 0x1
}

/// inserts field val into register
pub fn enia_1_write(newval: u64) {
    // bits 31..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 31) | ((newval & 0x1) << 31));
}

/// reads field val from register
pub fn enib_1_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn enib_1_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn lsmaoe_1_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

/// inserts field val into register
pub fn lsmaoe_1_write(newval: u64) {
    // bits 29..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 29) | ((newval & 0x1) << 29));
}

/// reads field val from register
pub fn ntlsmd_1_read() -> u64 {
    // bits 28..28
    let val = reg_rawrd();
    (val >> 28) & 0x1
}

/// inserts field val into register
pub fn ntlsmd_1_write(newval: u64) {
    // bits 28..28
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 28) | ((newval & 0x1) << 28));
}

/// reads field val from register
pub fn enda_1_read() -> u64 {
    // bits 27..27
    let val = reg_rawrd();
    (val >> 27) & 0x1
}

/// inserts field val into register
pub fn enda_1_write(newval: u64) {
    // bits 27..27
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 27) | ((newval & 0x1) << 27));
}

/// reads field val from register
pub fn uci_read() -> u64 {
    // bits 26..26
    let val = reg_rawrd();
    (val >> 26) & 0x1
}

/// inserts field val into register
pub fn uci_write(newval: u64) {
    // bits 26..26
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 26) | ((newval & 0x1) << 26));
}

/// reads field val from register
pub fn ee_read() -> u64 {
    // bits 25..25
    let val = reg_rawrd();
    (val >> 25) & 0x1
}

/// inserts field val into register
pub fn ee_write(newval: u64) {
    // bits 25..25
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 25) | ((newval & 0x1) << 25));
}

/// reads field val from register
pub fn e0e_read() -> u64 {
    // bits 24..24
    let val = reg_rawrd();
    (val >> 24) & 0x1
}

/// inserts field val into register
pub fn e0e_write(newval: u64) {
    // bits 24..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 24) | ((newval & 0x1) << 24));
}

/// reads field val from register
pub fn span_1_read() -> u64 {
    // bits 23..23
    let val = reg_rawrd();
    (val >> 23) & 0x1
}

/// inserts field val into register
pub fn span_1_write(newval: u64) {
    // bits 23..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 23) | ((newval & 0x1) << 23));
}

/// reads field val from register
pub fn eis_1_read() -> u64 {
    // bits 22..22
    let val = reg_rawrd();
    (val >> 22) & 0x1
}

/// inserts field val into register
pub fn eis_1_write(newval: u64) {
    // bits 22..22
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 22) | ((newval & 0x1) << 22));
}

/// reads field val from register
pub fn iesb_1_read() -> u64 {
    // bits 21..21
    let val = reg_rawrd();
    (val >> 21) & 0x1
}

/// inserts field val into register
pub fn iesb_1_write(newval: u64) {
    // bits 21..21
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 21) | ((newval & 0x1) << 21));
}

/// reads field val from register
pub fn tscxt_1_read() -> u64 {
    // bits 20..20
    let val = reg_rawrd();
    (val >> 20) & 0x1
}

/// inserts field val into register
pub fn tscxt_1_write(newval: u64) {
    // bits 20..20
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 20) | ((newval & 0x1) << 20));
}

/// reads field val from register
pub fn wxn_read() -> u64 {
    // bits 19..19
    let val = reg_rawrd();
    (val >> 19) & 0x1
}

/// inserts field val into register
pub fn wxn_write(newval: u64) {
    // bits 19..19
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 19) | ((newval & 0x1) << 19));
}

/// reads field val from register
pub fn ntwe_read() -> u64 {
    // bits 18..18
    let val = reg_rawrd();
    (val >> 18) & 0x1
}

/// inserts field val into register
pub fn ntwe_write(newval: u64) {
    // bits 18..18
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 18) | ((newval & 0x1) << 18));
}

/// reads field val from register
pub fn ntwi_read() -> u64 {
    // bits 16..16
    let val = reg_rawrd();
    (val >> 16) & 0x1
}

/// inserts field val into register
pub fn ntwi_write(newval: u64) {
    // bits 16..16
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 16) | ((newval & 0x1) << 16));
}

/// reads field val from register
pub fn uct_read() -> u64 {
    // bits 15..15
    let val = reg_rawrd();
    (val >> 15) & 0x1
}

/// inserts field val into register
pub fn uct_write(newval: u64) {
    // bits 15..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 15) | ((newval & 0x1) << 15));
}

/// reads field val from register
pub fn dze_read() -> u64 {
    // bits 14..14
    let val = reg_rawrd();
    (val >> 14) & 0x1
}

/// inserts field val into register
pub fn dze_write(newval: u64) {
    // bits 14..14
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 14) | ((newval & 0x1) << 14));
}

/// reads field val from register
pub fn endb_1_read() -> u64 {
    // bits 13..13
    let val = reg_rawrd();
    (val >> 13) & 0x1
}

/// inserts field val into register
pub fn endb_1_write(newval: u64) {
    // bits 13..13
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 13) | ((newval & 0x1) << 13));
}

/// reads field val from register
pub fn i_read() -> u64 {
    // bits 12..12
    let val = reg_rawrd();
    (val >> 12) & 0x1
}

/// inserts field val into register
pub fn i_write(newval: u64) {
    // bits 12..12
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 12) | ((newval & 0x1) << 12));
}

/// reads field val from register
pub fn eos_1_read() -> u64 {
    // bits 11..11
    let val = reg_rawrd();
    (val >> 11) & 0x1
}

/// inserts field val into register
pub fn eos_1_write(newval: u64) {
    // bits 11..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 11) | ((newval & 0x1) << 11));
}

/// reads field val from register
pub fn enrctx_1_read() -> u64 {
    // bits 10..10
    let val = reg_rawrd();
    (val >> 10) & 0x1
}

/// inserts field val into register
pub fn enrctx_1_write(newval: u64) {
    // bits 10..10
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 10) | ((newval & 0x1) << 10));
}

/// reads field val from register
pub fn uma_read() -> u64 {
    // bits 9..9
    let val = reg_rawrd();
    (val >> 9) & 0x1
}

/// inserts field val into register
pub fn uma_write(newval: u64) {
    // bits 9..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 9) | ((newval & 0x1) << 9));
}

/// reads field val from register
pub fn sed_1_read() -> u64 {
    // bits 8..8
    let val = reg_rawrd();
    (val >> 8) & 0x1
}

/// inserts field val into register
pub fn sed_1_write(newval: u64) {
    // bits 8..8
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 8) | ((newval & 0x1) << 8));
}

/// reads field val from register
pub fn itd_1_read() -> u64 {
    // bits 7..7
    let val = reg_rawrd();
    (val >> 7) & 0x1
}

/// inserts field val into register
pub fn itd_1_write(newval: u64) {
    // bits 7..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 7) | ((newval & 0x1) << 7));
}

/// reads field val from register
pub fn naa_1_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

/// inserts field val into register
pub fn naa_1_write(newval: u64) {
    // bits 6..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 6) | ((newval & 0x1) << 6));
}

/// reads field val from register
pub fn cp15ben_1_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn cp15ben_1_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn sa0_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn sa0_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn sa_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

/// inserts field val into register
pub fn sa_write(newval: u64) {
    // bits 3..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 3) | ((newval & 0x1) << 3));
}

/// reads field val from register
pub fn c_read() -> u64 {
    // bits 2..2
    let val = reg_rawrd();
    (val >> 2) & 0x1
}

/// inserts field val into register
pub fn c_write(newval: u64) {
    // bits 2..2
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 2) | ((newval & 0x1) << 2));
}

/// reads field val from register
pub fn a_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn a_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}

/// reads field val from register
pub fn m_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn m_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the System Control Register (EL1) value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register sctlr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x3c3fff8fffdffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x3c3fff8fffdffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x3c3fff8fffdffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 271341847484170239;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn epan_1_extract(&mut self) -> u64 {
        // bits 57..57
        (self.val >> 57) & 0x1
    }

    /// inserts field val into current value
    pub fn epan_1_insert(&mut self, val: u64) {
        // bits 57..57
        self.val = self.val & !(0x1 << 57) | ((val & 0x1) << 57);
    }

    /// extracts field val from current value
    pub fn enals_1_extract(&mut self) -> u64 {
        // bits 56..56
        (self.val >> 56) & 0x1
    }

    /// inserts field val into current value
    pub fn enals_1_insert(&mut self, val: u64) {
        // bits 56..56
        self.val = self.val & !(0x1 << 56) | ((val & 0x1) << 56);
    }

    /// extracts field val from current value
    pub fn enas0_1_extract(&mut self) -> u64 {
        // bits 55..55
        (self.val >> 55) & 0x1
    }

    /// inserts field val into current value
    pub fn enas0_1_insert(&mut self, val: u64) {
        // bits 55..55
        self.val = self.val & !(0x1 << 55) | ((val & 0x1) << 55);
    }

    /// extracts field val from current value
    pub fn enasr_1_extract(&mut self) -> u64 {
        // bits 54..54
        (self.val >> 54) & 0x1
    }

    /// inserts field val into current value
    pub fn enasr_1_insert(&mut self, val: u64) {
        // bits 54..54
        self.val = self.val & !(0x1 << 54) | ((val & 0x1) << 54);
    }

    /// extracts field val from current value
    pub fn twedel_1_extract(&mut self) -> u64 {
        // bits 46..49
        (self.val >> 46) & 0xf
    }

    /// inserts field val into current value
    pub fn twedel_1_insert(&mut self, val: u64) {
        // bits 46..49
        self.val = self.val & !(0xf << 46) | ((val & 0xf) << 46);
    }

    /// extracts field val from current value
    pub fn tweden_1_extract(&mut self) -> u64 {
        // bits 45..45
        (self.val >> 45) & 0x1
    }

    /// inserts field val into current value
    pub fn tweden_1_insert(&mut self, val: u64) {
        // bits 45..45
        self.val = self.val & !(0x1 << 45) | ((val & 0x1) << 45);
    }

    /// extracts field val from current value
    pub fn dssbs_1_extract(&mut self) -> u64 {
        // bits 44..44
        (self.val >> 44) & 0x1
    }

    /// inserts field val into current value
    pub fn dssbs_1_insert(&mut self, val: u64) {
        // bits 44..44
        self.val = self.val & !(0x1 << 44) | ((val & 0x1) << 44);
    }

    /// extracts field val from current value
    pub fn ata_1_extract(&mut self) -> u64 {
        // bits 43..43
        (self.val >> 43) & 0x1
    }

    /// inserts field val into current value
    pub fn ata_1_insert(&mut self, val: u64) {
        // bits 43..43
        self.val = self.val & !(0x1 << 43) | ((val & 0x1) << 43);
    }

    /// extracts field val from current value
    pub fn ata0_1_extract(&mut self) -> u64 {
        // bits 42..42
        (self.val >> 42) & 0x1
    }

    /// inserts field val into current value
    pub fn ata0_1_insert(&mut self, val: u64) {
        // bits 42..42
        self.val = self.val & !(0x1 << 42) | ((val & 0x1) << 42);
    }

    /// extracts field val from current value
    pub fn tcf_1_extract(&mut self) -> u64 {
        // bits 40..41
        (self.val >> 40) & 0x3
    }

    /// inserts field val into current value
    pub fn tcf_1_insert(&mut self, val: u64) {
        // bits 40..41
        self.val = self.val & !(0x3 << 40) | ((val & 0x3) << 40);
    }

    /// extracts field val from current value
    pub fn tcf0_1_extract(&mut self) -> u64 {
        // bits 38..39
        (self.val >> 38) & 0x3
    }

    /// inserts field val into current value
    pub fn tcf0_1_insert(&mut self, val: u64) {
        // bits 38..39
        self.val = self.val & !(0x3 << 38) | ((val & 0x3) << 38);
    }

    /// extracts field val from current value
    pub fn itfsb_1_extract(&mut self) -> u64 {
        // bits 37..37
        (self.val >> 37) & 0x1
    }

    /// inserts field val into current value
    pub fn itfsb_1_insert(&mut self, val: u64) {
        // bits 37..37
        self.val = self.val & !(0x1 << 37) | ((val & 0x1) << 37);
    }

    /// extracts field val from current value
    pub fn bt1_1_extract(&mut self) -> u64 {
        // bits 36..36
        (self.val >> 36) & 0x1
    }

    /// inserts field val into current value
    pub fn bt1_1_insert(&mut self, val: u64) {
        // bits 36..36
        self.val = self.val & !(0x1 << 36) | ((val & 0x1) << 36);
    }

    /// extracts field val from current value
    pub fn bt0_1_extract(&mut self) -> u64 {
        // bits 35..35
        (self.val >> 35) & 0x1
    }

    /// inserts field val into current value
    pub fn bt0_1_insert(&mut self, val: u64) {
        // bits 35..35
        self.val = self.val & !(0x1 << 35) | ((val & 0x1) << 35);
    }

    /// extracts field val from current value
    pub fn enia_1_extract(&mut self) -> u64 {
        // bits 31..31
        (self.val >> 31) & 0x1
    }

    /// inserts field val into current value
    pub fn enia_1_insert(&mut self, val: u64) {
        // bits 31..31
        self.val = self.val & !(0x1 << 31) | ((val & 0x1) << 31);
    }

    /// extracts field val from current value
    pub fn enib_1_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn enib_1_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn lsmaoe_1_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }

    /// inserts field val into current value
    pub fn lsmaoe_1_insert(&mut self, val: u64) {
        // bits 29..29
        self.val = self.val & !(0x1 << 29) | ((val & 0x1) << 29);
    }

    /// extracts field val from current value
    pub fn ntlsmd_1_extract(&mut self) -> u64 {
        // bits 28..28
        (self.val >> 28) & 0x1
    }

    /// inserts field val into current value
    pub fn ntlsmd_1_insert(&mut self, val: u64) {
        // bits 28..28
        self.val = self.val & !(0x1 << 28) | ((val & 0x1) << 28);
    }

    /// extracts field val from current value
    pub fn enda_1_extract(&mut self) -> u64 {
        // bits 27..27
        (self.val >> 27) & 0x1
    }

    /// inserts field val into current value
    pub fn enda_1_insert(&mut self, val: u64) {
        // bits 27..27
        self.val = self.val & !(0x1 << 27) | ((val & 0x1) << 27);
    }

    /// extracts field val from current value
    pub fn uci_extract(&mut self) -> u64 {
        // bits 26..26
        (self.val >> 26) & 0x1
    }

    /// inserts field val into current value
    pub fn uci_insert(&mut self, val: u64) {
        // bits 26..26
        self.val = self.val & !(0x1 << 26) | ((val & 0x1) << 26);
    }

    /// extracts field val from current value
    pub fn ee_extract(&mut self) -> u64 {
        // bits 25..25
        (self.val >> 25) & 0x1
    }

    /// inserts field val into current value
    pub fn ee_insert(&mut self, val: u64) {
        // bits 25..25
        self.val = self.val & !(0x1 << 25) | ((val & 0x1) << 25);
    }

    /// extracts field val from current value
    pub fn e0e_extract(&mut self) -> u64 {
        // bits 24..24
        (self.val >> 24) & 0x1
    }

    /// inserts field val into current value
    pub fn e0e_insert(&mut self, val: u64) {
        // bits 24..24
        self.val = self.val & !(0x1 << 24) | ((val & 0x1) << 24);
    }

    /// extracts field val from current value
    pub fn span_1_extract(&mut self) -> u64 {
        // bits 23..23
        (self.val >> 23) & 0x1
    }

    /// inserts field val into current value
    pub fn span_1_insert(&mut self, val: u64) {
        // bits 23..23
        self.val = self.val & !(0x1 << 23) | ((val & 0x1) << 23);
    }

    /// extracts field val from current value
    pub fn eis_1_extract(&mut self) -> u64 {
        // bits 22..22
        (self.val >> 22) & 0x1
    }

    /// inserts field val into current value
    pub fn eis_1_insert(&mut self, val: u64) {
        // bits 22..22
        self.val = self.val & !(0x1 << 22) | ((val & 0x1) << 22);
    }

    /// extracts field val from current value
    pub fn iesb_1_extract(&mut self) -> u64 {
        // bits 21..21
        (self.val >> 21) & 0x1
    }

    /// inserts field val into current value
    pub fn iesb_1_insert(&mut self, val: u64) {
        // bits 21..21
        self.val = self.val & !(0x1 << 21) | ((val & 0x1) << 21);
    }

    /// extracts field val from current value
    pub fn tscxt_1_extract(&mut self) -> u64 {
        // bits 20..20
        (self.val >> 20) & 0x1
    }

    /// inserts field val into current value
    pub fn tscxt_1_insert(&mut self, val: u64) {
        // bits 20..20
        self.val = self.val & !(0x1 << 20) | ((val & 0x1) << 20);
    }

    /// extracts field val from current value
    pub fn wxn_extract(&mut self) -> u64 {
        // bits 19..19
        (self.val >> 19) & 0x1
    }

    /// inserts field val into current value
    pub fn wxn_insert(&mut self, val: u64) {
        // bits 19..19
        self.val = self.val & !(0x1 << 19) | ((val & 0x1) << 19);
    }

    /// extracts field val from current value
    pub fn ntwe_extract(&mut self) -> u64 {
        // bits 18..18
        (self.val >> 18) & 0x1
    }

    /// inserts field val into current value
    pub fn ntwe_insert(&mut self, val: u64) {
        // bits 18..18
        self.val = self.val & !(0x1 << 18) | ((val & 0x1) << 18);
    }

    /// extracts field val from current value
    pub fn ntwi_extract(&mut self) -> u64 {
        // bits 16..16
        (self.val >> 16) & 0x1
    }

    /// inserts field val into current value
    pub fn ntwi_insert(&mut self, val: u64) {
        // bits 16..16
        self.val = self.val & !(0x1 << 16) | ((val & 0x1) << 16);
    }

    /// extracts field val from current value
    pub fn uct_extract(&mut self) -> u64 {
        // bits 15..15
        (self.val >> 15) & 0x1
    }

    /// inserts field val into current value
    pub fn uct_insert(&mut self, val: u64) {
        // bits 15..15
        self.val = self.val & !(0x1 << 15) | ((val & 0x1) << 15);
    }

    /// extracts field val from current value
    pub fn dze_extract(&mut self) -> u64 {
        // bits 14..14
        (self.val >> 14) & 0x1
    }

    /// inserts field val into current value
    pub fn dze_insert(&mut self, val: u64) {
        // bits 14..14
        self.val = self.val & !(0x1 << 14) | ((val & 0x1) << 14);
    }

    /// extracts field val from current value
    pub fn endb_1_extract(&mut self) -> u64 {
        // bits 13..13
        (self.val >> 13) & 0x1
    }

    /// inserts field val into current value
    pub fn endb_1_insert(&mut self, val: u64) {
        // bits 13..13
        self.val = self.val & !(0x1 << 13) | ((val & 0x1) << 13);
    }

    /// extracts field val from current value
    pub fn i_extract(&mut self) -> u64 {
        // bits 12..12
        (self.val >> 12) & 0x1
    }

    /// inserts field val into current value
    pub fn i_insert(&mut self, val: u64) {
        // bits 12..12
        self.val = self.val & !(0x1 << 12) | ((val & 0x1) << 12);
    }

    /// extracts field val from current value
    pub fn eos_1_extract(&mut self) -> u64 {
        // bits 11..11
        (self.val >> 11) & 0x1
    }

    /// inserts field val into current value
    pub fn eos_1_insert(&mut self, val: u64) {
        // bits 11..11
        self.val = self.val & !(0x1 << 11) | ((val & 0x1) << 11);
    }

    /// extracts field val from current value
    pub fn enrctx_1_extract(&mut self) -> u64 {
        // bits 10..10
        (self.val >> 10) & 0x1
    }

    /// inserts field val into current value
    pub fn enrctx_1_insert(&mut self, val: u64) {
        // bits 10..10
        self.val = self.val & !(0x1 << 10) | ((val & 0x1) << 10);
    }

    /// extracts field val from current value
    pub fn uma_extract(&mut self) -> u64 {
        // bits 9..9
        (self.val >> 9) & 0x1
    }

    /// inserts field val into current value
    pub fn uma_insert(&mut self, val: u64) {
        // bits 9..9
        self.val = self.val & !(0x1 << 9) | ((val & 0x1) << 9);
    }

    /// extracts field val from current value
    pub fn sed_1_extract(&mut self) -> u64 {
        // bits 8..8
        (self.val >> 8) & 0x1
    }

    /// inserts field val into current value
    pub fn sed_1_insert(&mut self, val: u64) {
        // bits 8..8
        self.val = self.val & !(0x1 << 8) | ((val & 0x1) << 8);
    }

    /// extracts field val from current value
    pub fn itd_1_extract(&mut self) -> u64 {
        // bits 7..7
        (self.val >> 7) & 0x1
    }

    /// inserts field val into current value
    pub fn itd_1_insert(&mut self, val: u64) {
        // bits 7..7
        self.val = self.val & !(0x1 << 7) | ((val & 0x1) << 7);
    }

    /// extracts field val from current value
    pub fn naa_1_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }

    /// inserts field val into current value
    pub fn naa_1_insert(&mut self, val: u64) {
        // bits 6..6
        self.val = self.val & !(0x1 << 6) | ((val & 0x1) << 6);
    }

    /// extracts field val from current value
    pub fn cp15ben_1_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn cp15ben_1_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn sa0_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn sa0_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn sa_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }

    /// inserts field val into current value
    pub fn sa_insert(&mut self, val: u64) {
        // bits 3..3
        self.val = self.val & !(0x1 << 3) | ((val & 0x1) << 3);
    }

    /// extracts field val from current value
    pub fn c_extract(&mut self) -> u64 {
        // bits 2..2
        (self.val >> 2) & 0x1
    }

    /// inserts field val into current value
    pub fn c_insert(&mut self, val: u64) {
        // bits 2..2
        self.val = self.val & !(0x1 << 2) | ((val & 0x1) << 2);
    }

    /// extracts field val from current value
    pub fn a_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn a_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }

    /// extracts field val from current value
    pub fn m_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn m_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

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
 * Generated on: 2020-10-05T16:30:11.708598
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
 * Register:    Monitor Debug Configuration Register (EL2) (mdcr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Provides EL2 configuration options for self-hosted debug and
 * the Performance Monitors Extension. File:        AArch64-mdcr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Monitor Debug Configuration Register (EL2) (mdcr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, MDCR_EL2
        llvm_asm!("mrs $0, mdcr_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Monitor Debug Configuration Register (EL2) (mdcr_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR MDCR_EL2, <Xt>
        llvm_asm!("msr mdcr_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn hpmfzs_1_read() -> u64 {
    // bits 36..36
    let val = reg_rawrd();
    (val >> 36) & 0x1
}

/// inserts field val into register
pub fn hpmfzs_1_write(newval: u64) {
    // bits 36..36
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 36) | ((newval & 0x1) << 36));
}

/// reads field val from register
pub fn hpmfzo_1_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

/// inserts field val into register
pub fn hpmfzo_1_write(newval: u64) {
    // bits 29..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 29) | ((newval & 0x1) << 29));
}

/// reads field val from register
pub fn mtpme_1_read() -> u64 {
    // bits 28..28
    let val = reg_rawrd();
    (val >> 28) & 0x1
}

/// inserts field val into register
pub fn mtpme_1_write(newval: u64) {
    // bits 28..28
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 28) | ((newval & 0x1) << 28));
}

/// reads field val from register
pub fn tdcc_1_read() -> u64 {
    // bits 27..27
    let val = reg_rawrd();
    (val >> 27) & 0x1
}

/// inserts field val into register
pub fn tdcc_1_write(newval: u64) {
    // bits 27..27
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 27) | ((newval & 0x1) << 27));
}

/// reads field val from register
pub fn hlp_1_read() -> u64 {
    // bits 26..26
    let val = reg_rawrd();
    (val >> 26) & 0x1
}

/// inserts field val into register
pub fn hlp_1_write(newval: u64) {
    // bits 26..26
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 26) | ((newval & 0x1) << 26));
}

/// reads field val from register
pub fn hccd_1_read() -> u64 {
    // bits 23..23
    let val = reg_rawrd();
    (val >> 23) & 0x1
}

/// inserts field val into register
pub fn hccd_1_write(newval: u64) {
    // bits 23..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 23) | ((newval & 0x1) << 23));
}

/// reads field val from register
pub fn ttrf_1_read() -> u64 {
    // bits 19..19
    let val = reg_rawrd();
    (val >> 19) & 0x1
}

/// inserts field val into register
pub fn ttrf_1_write(newval: u64) {
    // bits 19..19
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 19) | ((newval & 0x1) << 19));
}

/// reads field val from register
pub fn hpmd_1_read() -> u64 {
    // bits 17..17
    let val = reg_rawrd();
    (val >> 17) & 0x1
}

/// inserts field val into register
pub fn hpmd_1_write(newval: u64) {
    // bits 17..17
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 17) | ((newval & 0x1) << 17));
}

/// reads field val from register
pub fn tpms_1_read() -> u64 {
    // bits 14..14
    let val = reg_rawrd();
    (val >> 14) & 0x1
}

/// inserts field val into register
pub fn tpms_1_write(newval: u64) {
    // bits 14..14
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 14) | ((newval & 0x1) << 14));
}

/// reads field val from register
pub fn e2pb_1_read() -> u64 {
    // bits 12..13
    let val = reg_rawrd();
    (val >> 12) & 0x3
}

/// inserts field val into register
pub fn e2pb_1_write(newval: u64) {
    // bits 12..13
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 12) | ((newval & 0x3) << 12));
}

/// reads field val from register
pub fn tdra_read() -> u64 {
    // bits 11..11
    let val = reg_rawrd();
    (val >> 11) & 0x1
}

/// inserts field val into register
pub fn tdra_write(newval: u64) {
    // bits 11..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 11) | ((newval & 0x1) << 11));
}

/// reads field val from register
pub fn tdosa_1_read() -> u64 {
    // bits 10..10
    let val = reg_rawrd();
    (val >> 10) & 0x1
}

/// inserts field val into register
pub fn tdosa_1_write(newval: u64) {
    // bits 10..10
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 10) | ((newval & 0x1) << 10));
}

/// reads field val from register
pub fn tdosa_2_read() -> u64 {
    // bits 10..10
    let val = reg_rawrd();
    (val >> 10) & 0x1
}

/// inserts field val into register
pub fn tdosa_2_write(newval: u64) {
    // bits 10..10
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 10) | ((newval & 0x1) << 10));
}

/// reads field val from register
pub fn tda_read() -> u64 {
    // bits 9..9
    let val = reg_rawrd();
    (val >> 9) & 0x1
}

/// inserts field val into register
pub fn tda_write(newval: u64) {
    // bits 9..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 9) | ((newval & 0x1) << 9));
}

/// reads field val from register
pub fn tde_read() -> u64 {
    // bits 8..8
    let val = reg_rawrd();
    (val >> 8) & 0x1
}

/// inserts field val into register
pub fn tde_write(newval: u64) {
    // bits 8..8
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 8) | ((newval & 0x1) << 8));
}

/// reads field val from register
pub fn hpme_1_read() -> u64 {
    // bits 7..7
    let val = reg_rawrd();
    (val >> 7) & 0x1
}

/// inserts field val into register
pub fn hpme_1_write(newval: u64) {
    // bits 7..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 7) | ((newval & 0x1) << 7));
}

/// reads field val from register
pub fn tpm_1_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

/// inserts field val into register
pub fn tpm_1_write(newval: u64) {
    // bits 6..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 6) | ((newval & 0x1) << 6));
}

/// reads field val from register
pub fn tpmcr_1_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn tpmcr_1_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn hpmn_1_read() -> u64 {
    // bits 0..4
    let val = reg_rawrd();
    (val >> 0) & 0x1f
}

/// inserts field val into register
pub fn hpmn_1_write(newval: u64) {
    // bits 0..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1f << 0) | ((newval & 0x1f) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Monitor Debug Configuration Register (EL2)
/// value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register mdcr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x103c8a7fff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x103c8a7fff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x103c8a7fff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 69735186431;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn hpmfzs_1_extract(&mut self) -> u64 {
        // bits 36..36
        (self.val >> 36) & 0x1
    }

    /// inserts field val into current value
    pub fn hpmfzs_1_insert(&mut self, val: u64) {
        // bits 36..36
        self.val = self.val & !(0x1 << 36) | ((val & 0x1) << 36);
    }

    /// extracts field val from current value
    pub fn hpmfzo_1_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }

    /// inserts field val into current value
    pub fn hpmfzo_1_insert(&mut self, val: u64) {
        // bits 29..29
        self.val = self.val & !(0x1 << 29) | ((val & 0x1) << 29);
    }

    /// extracts field val from current value
    pub fn mtpme_1_extract(&mut self) -> u64 {
        // bits 28..28
        (self.val >> 28) & 0x1
    }

    /// inserts field val into current value
    pub fn mtpme_1_insert(&mut self, val: u64) {
        // bits 28..28
        self.val = self.val & !(0x1 << 28) | ((val & 0x1) << 28);
    }

    /// extracts field val from current value
    pub fn tdcc_1_extract(&mut self) -> u64 {
        // bits 27..27
        (self.val >> 27) & 0x1
    }

    /// inserts field val into current value
    pub fn tdcc_1_insert(&mut self, val: u64) {
        // bits 27..27
        self.val = self.val & !(0x1 << 27) | ((val & 0x1) << 27);
    }

    /// extracts field val from current value
    pub fn hlp_1_extract(&mut self) -> u64 {
        // bits 26..26
        (self.val >> 26) & 0x1
    }

    /// inserts field val into current value
    pub fn hlp_1_insert(&mut self, val: u64) {
        // bits 26..26
        self.val = self.val & !(0x1 << 26) | ((val & 0x1) << 26);
    }

    /// extracts field val from current value
    pub fn hccd_1_extract(&mut self) -> u64 {
        // bits 23..23
        (self.val >> 23) & 0x1
    }

    /// inserts field val into current value
    pub fn hccd_1_insert(&mut self, val: u64) {
        // bits 23..23
        self.val = self.val & !(0x1 << 23) | ((val & 0x1) << 23);
    }

    /// extracts field val from current value
    pub fn ttrf_1_extract(&mut self) -> u64 {
        // bits 19..19
        (self.val >> 19) & 0x1
    }

    /// inserts field val into current value
    pub fn ttrf_1_insert(&mut self, val: u64) {
        // bits 19..19
        self.val = self.val & !(0x1 << 19) | ((val & 0x1) << 19);
    }

    /// extracts field val from current value
    pub fn hpmd_1_extract(&mut self) -> u64 {
        // bits 17..17
        (self.val >> 17) & 0x1
    }

    /// inserts field val into current value
    pub fn hpmd_1_insert(&mut self, val: u64) {
        // bits 17..17
        self.val = self.val & !(0x1 << 17) | ((val & 0x1) << 17);
    }

    /// extracts field val from current value
    pub fn tpms_1_extract(&mut self) -> u64 {
        // bits 14..14
        (self.val >> 14) & 0x1
    }

    /// inserts field val into current value
    pub fn tpms_1_insert(&mut self, val: u64) {
        // bits 14..14
        self.val = self.val & !(0x1 << 14) | ((val & 0x1) << 14);
    }

    /// extracts field val from current value
    pub fn e2pb_1_extract(&mut self) -> u64 {
        // bits 12..13
        (self.val >> 12) & 0x3
    }

    /// inserts field val into current value
    pub fn e2pb_1_insert(&mut self, val: u64) {
        // bits 12..13
        self.val = self.val & !(0x3 << 12) | ((val & 0x3) << 12);
    }

    /// extracts field val from current value
    pub fn tdra_extract(&mut self) -> u64 {
        // bits 11..11
        (self.val >> 11) & 0x1
    }

    /// inserts field val into current value
    pub fn tdra_insert(&mut self, val: u64) {
        // bits 11..11
        self.val = self.val & !(0x1 << 11) | ((val & 0x1) << 11);
    }

    /// extracts field val from current value
    pub fn tdosa_1_extract(&mut self) -> u64 {
        // bits 10..10
        (self.val >> 10) & 0x1
    }

    /// inserts field val into current value
    pub fn tdosa_1_insert(&mut self, val: u64) {
        // bits 10..10
        self.val = self.val & !(0x1 << 10) | ((val & 0x1) << 10);
    }

    /// extracts field val from current value
    pub fn tdosa_2_extract(&mut self) -> u64 {
        // bits 10..10
        (self.val >> 10) & 0x1
    }

    /// inserts field val into current value
    pub fn tdosa_2_insert(&mut self, val: u64) {
        // bits 10..10
        self.val = self.val & !(0x1 << 10) | ((val & 0x1) << 10);
    }

    /// extracts field val from current value
    pub fn tda_extract(&mut self) -> u64 {
        // bits 9..9
        (self.val >> 9) & 0x1
    }

    /// inserts field val into current value
    pub fn tda_insert(&mut self, val: u64) {
        // bits 9..9
        self.val = self.val & !(0x1 << 9) | ((val & 0x1) << 9);
    }

    /// extracts field val from current value
    pub fn tde_extract(&mut self) -> u64 {
        // bits 8..8
        (self.val >> 8) & 0x1
    }

    /// inserts field val into current value
    pub fn tde_insert(&mut self, val: u64) {
        // bits 8..8
        self.val = self.val & !(0x1 << 8) | ((val & 0x1) << 8);
    }

    /// extracts field val from current value
    pub fn hpme_1_extract(&mut self) -> u64 {
        // bits 7..7
        (self.val >> 7) & 0x1
    }

    /// inserts field val into current value
    pub fn hpme_1_insert(&mut self, val: u64) {
        // bits 7..7
        self.val = self.val & !(0x1 << 7) | ((val & 0x1) << 7);
    }

    /// extracts field val from current value
    pub fn tpm_1_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }

    /// inserts field val into current value
    pub fn tpm_1_insert(&mut self, val: u64) {
        // bits 6..6
        self.val = self.val & !(0x1 << 6) | ((val & 0x1) << 6);
    }

    /// extracts field val from current value
    pub fn tpmcr_1_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn tpmcr_1_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn hpmn_1_extract(&mut self) -> u64 {
        // bits 0..4
        (self.val >> 0) & 0x1f
    }

    /// inserts field val into current value
    pub fn hpmn_1_insert(&mut self, val: u64) {
        // bits 0..4
        self.val = self.val & !(0x1f << 0) | ((val & 0x1f) << 0);
    }
}

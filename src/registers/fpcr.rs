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
 * Generated on: 2020-10-05T16:30:11.687920
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
 * Register:    Floating-point Control Register (fpcr)
 * Group:       Floating-point registers
 * Type:        64-bit Register
 * Description: Controls floating-point behavior.
 * File:        AArch64-fpcr.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Floating-point Control Register (fpcr) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, FPCR
        llvm_asm!("mrs $0, fpcr" : "=r"(regval));
    }
    return regval;
}


/// writing the Floating-point Control Register (fpcr) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR FPCR, <Xt>
        llvm_asm!("msr fpcr, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn ahp_read() -> u64 {
    // bits 26..26
    let val = reg_rawrd();
    (val >> 26) & 0x1
}

/// inserts field val into register
pub fn ahp_write(newval: u64) {
    // bits 26..26
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 26) | ((newval & 0x1) << 26));
}

/// reads field val from register
pub fn dn_read() -> u64 {
    // bits 25..25
    let val = reg_rawrd();
    (val >> 25) & 0x1
}

/// inserts field val into register
pub fn dn_write(newval: u64) {
    // bits 25..25
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 25) | ((newval & 0x1) << 25));
}

/// reads field val from register
pub fn fz_read() -> u64 {
    // bits 24..24
    let val = reg_rawrd();
    (val >> 24) & 0x1
}

/// inserts field val into register
pub fn fz_write(newval: u64) {
    // bits 24..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 24) | ((newval & 0x1) << 24));
}

/// reads field val from register
pub fn rmode_read() -> u64 {
    // bits 22..23
    let val = reg_rawrd();
    (val >> 22) & 0x3
}

/// inserts field val into register
pub fn rmode_write(newval: u64) {
    // bits 22..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 22) | ((newval & 0x3) << 22));
}

/// reads field val from register
pub fn stride_read() -> u64 {
    // bits 20..21
    let val = reg_rawrd();
    (val >> 20) & 0x3
}

/// inserts field val into register
pub fn stride_write(newval: u64) {
    // bits 20..21
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 20) | ((newval & 0x3) << 20));
}

/// reads field val from register
pub fn fz16_1_read() -> u64 {
    // bits 19..19
    let val = reg_rawrd();
    (val >> 19) & 0x1
}

/// inserts field val into register
pub fn fz16_1_write(newval: u64) {
    // bits 19..19
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 19) | ((newval & 0x1) << 19));
}

/// reads field val from register
pub fn len_read() -> u64 {
    // bits 16..18
    let val = reg_rawrd();
    (val >> 16) & 0x7
}

/// inserts field val into register
pub fn len_write(newval: u64) {
    // bits 16..18
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7 << 16) | ((newval & 0x7) << 16));
}

/// reads field val from register
pub fn ide_read() -> u64 {
    // bits 15..15
    let val = reg_rawrd();
    (val >> 15) & 0x1
}

/// inserts field val into register
pub fn ide_write(newval: u64) {
    // bits 15..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 15) | ((newval & 0x1) << 15));
}

/// reads field val from register
pub fn ixe_read() -> u64 {
    // bits 12..12
    let val = reg_rawrd();
    (val >> 12) & 0x1
}

/// inserts field val into register
pub fn ixe_write(newval: u64) {
    // bits 12..12
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 12) | ((newval & 0x1) << 12));
}

/// reads field val from register
pub fn ufe_read() -> u64 {
    // bits 11..11
    let val = reg_rawrd();
    (val >> 11) & 0x1
}

/// inserts field val into register
pub fn ufe_write(newval: u64) {
    // bits 11..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 11) | ((newval & 0x1) << 11));
}

/// reads field val from register
pub fn ofe_read() -> u64 {
    // bits 10..10
    let val = reg_rawrd();
    (val >> 10) & 0x1
}

/// inserts field val into register
pub fn ofe_write(newval: u64) {
    // bits 10..10
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 10) | ((newval & 0x1) << 10));
}

/// reads field val from register
pub fn dze_read() -> u64 {
    // bits 9..9
    let val = reg_rawrd();
    (val >> 9) & 0x1
}

/// inserts field val into register
pub fn dze_write(newval: u64) {
    // bits 9..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 9) | ((newval & 0x1) << 9));
}

/// reads field val from register
pub fn ioe_read() -> u64 {
    // bits 8..8
    let val = reg_rawrd();
    (val >> 8) & 0x1
}

/// inserts field val into register
pub fn ioe_write(newval: u64) {
    // bits 8..8
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 8) | ((newval & 0x1) << 8));
}

/// reads field val from register
pub fn nep_1_read() -> u64 {
    // bits 2..2
    let val = reg_rawrd();
    (val >> 2) & 0x1
}

/// inserts field val into register
pub fn nep_1_write(newval: u64) {
    // bits 2..2
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 2) | ((newval & 0x1) << 2));
}

/// reads field val from register
pub fn ah_1_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn ah_1_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}

/// reads field val from register
pub fn fiz_1_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn fiz_1_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Floating-point Control Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register fpcr
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x7ff9f07;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x7ff9f07
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x7ff9f07)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 134192903;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn ahp_extract(&mut self) -> u64 {
        // bits 26..26
        (self.val >> 26) & 0x1
    }

    /// inserts field val into current value
    pub fn ahp_insert(&mut self, val: u64) {
        // bits 26..26
        self.val = self.val & !(0x1 << 26) | ((val & 0x1) << 26);
    }

    /// extracts field val from current value
    pub fn dn_extract(&mut self) -> u64 {
        // bits 25..25
        (self.val >> 25) & 0x1
    }

    /// inserts field val into current value
    pub fn dn_insert(&mut self, val: u64) {
        // bits 25..25
        self.val = self.val & !(0x1 << 25) | ((val & 0x1) << 25);
    }

    /// extracts field val from current value
    pub fn fz_extract(&mut self) -> u64 {
        // bits 24..24
        (self.val >> 24) & 0x1
    }

    /// inserts field val into current value
    pub fn fz_insert(&mut self, val: u64) {
        // bits 24..24
        self.val = self.val & !(0x1 << 24) | ((val & 0x1) << 24);
    }

    /// extracts field val from current value
    pub fn rmode_extract(&mut self) -> u64 {
        // bits 22..23
        (self.val >> 22) & 0x3
    }

    /// inserts field val into current value
    pub fn rmode_insert(&mut self, val: u64) {
        // bits 22..23
        self.val = self.val & !(0x3 << 22) | ((val & 0x3) << 22);
    }

    /// extracts field val from current value
    pub fn stride_extract(&mut self) -> u64 {
        // bits 20..21
        (self.val >> 20) & 0x3
    }

    /// inserts field val into current value
    pub fn stride_insert(&mut self, val: u64) {
        // bits 20..21
        self.val = self.val & !(0x3 << 20) | ((val & 0x3) << 20);
    }

    /// extracts field val from current value
    pub fn fz16_1_extract(&mut self) -> u64 {
        // bits 19..19
        (self.val >> 19) & 0x1
    }

    /// inserts field val into current value
    pub fn fz16_1_insert(&mut self, val: u64) {
        // bits 19..19
        self.val = self.val & !(0x1 << 19) | ((val & 0x1) << 19);
    }

    /// extracts field val from current value
    pub fn len_extract(&mut self) -> u64 {
        // bits 16..18
        (self.val >> 16) & 0x7
    }

    /// inserts field val into current value
    pub fn len_insert(&mut self, val: u64) {
        // bits 16..18
        self.val = self.val & !(0x7 << 16) | ((val & 0x7) << 16);
    }

    /// extracts field val from current value
    pub fn ide_extract(&mut self) -> u64 {
        // bits 15..15
        (self.val >> 15) & 0x1
    }

    /// inserts field val into current value
    pub fn ide_insert(&mut self, val: u64) {
        // bits 15..15
        self.val = self.val & !(0x1 << 15) | ((val & 0x1) << 15);
    }

    /// extracts field val from current value
    pub fn ixe_extract(&mut self) -> u64 {
        // bits 12..12
        (self.val >> 12) & 0x1
    }

    /// inserts field val into current value
    pub fn ixe_insert(&mut self, val: u64) {
        // bits 12..12
        self.val = self.val & !(0x1 << 12) | ((val & 0x1) << 12);
    }

    /// extracts field val from current value
    pub fn ufe_extract(&mut self) -> u64 {
        // bits 11..11
        (self.val >> 11) & 0x1
    }

    /// inserts field val into current value
    pub fn ufe_insert(&mut self, val: u64) {
        // bits 11..11
        self.val = self.val & !(0x1 << 11) | ((val & 0x1) << 11);
    }

    /// extracts field val from current value
    pub fn ofe_extract(&mut self) -> u64 {
        // bits 10..10
        (self.val >> 10) & 0x1
    }

    /// inserts field val into current value
    pub fn ofe_insert(&mut self, val: u64) {
        // bits 10..10
        self.val = self.val & !(0x1 << 10) | ((val & 0x1) << 10);
    }

    /// extracts field val from current value
    pub fn dze_extract(&mut self) -> u64 {
        // bits 9..9
        (self.val >> 9) & 0x1
    }

    /// inserts field val into current value
    pub fn dze_insert(&mut self, val: u64) {
        // bits 9..9
        self.val = self.val & !(0x1 << 9) | ((val & 0x1) << 9);
    }

    /// extracts field val from current value
    pub fn ioe_extract(&mut self) -> u64 {
        // bits 8..8
        (self.val >> 8) & 0x1
    }

    /// inserts field val into current value
    pub fn ioe_insert(&mut self, val: u64) {
        // bits 8..8
        self.val = self.val & !(0x1 << 8) | ((val & 0x1) << 8);
    }

    /// extracts field val from current value
    pub fn nep_1_extract(&mut self) -> u64 {
        // bits 2..2
        (self.val >> 2) & 0x1
    }

    /// inserts field val into current value
    pub fn nep_1_insert(&mut self, val: u64) {
        // bits 2..2
        self.val = self.val & !(0x1 << 2) | ((val & 0x1) << 2);
    }

    /// extracts field val from current value
    pub fn ah_1_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn ah_1_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }

    /// extracts field val from current value
    pub fn fiz_1_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn fiz_1_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

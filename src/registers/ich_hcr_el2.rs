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
 * Generated on: 2020-10-05T16:30:11.697982
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
 * Register:    Interrupt Controller Hyp Control Register (ich_hcr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Controls the environment for VMs.
 * File:        AArch64-ich_hcr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Interrupt Controller Hyp Control Register (ich_hcr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ICH_HCR_EL2
        llvm_asm!("mrs $0, ich_hcr_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Interrupt Controller Hyp Control Register (ich_hcr_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR ICH_HCR_EL2, <Xt>
        llvm_asm!("msr ich_hcr_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn eoicount_read() -> u64 {
    // bits 27..31
    let val = reg_rawrd();
    (val >> 27) & 0x1f
}

/// inserts field val into register
pub fn eoicount_write(newval: u64) {
    // bits 27..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1f << 27) | ((newval & 0x1f) << 27));
}

/// reads field val from register
pub fn dvim_1_read() -> u64 {
    // bits 15..15
    let val = reg_rawrd();
    (val >> 15) & 0x1
}

/// inserts field val into register
pub fn dvim_1_write(newval: u64) {
    // bits 15..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 15) | ((newval & 0x1) << 15));
}

/// reads field val from register
pub fn tdir_read() -> u64 {
    // bits 14..14
    let val = reg_rawrd();
    (val >> 14) & 0x1
}

/// inserts field val into register
pub fn tdir_write(newval: u64) {
    // bits 14..14
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 14) | ((newval & 0x1) << 14));
}

/// reads field val from register
pub fn tsei_read() -> u64 {
    // bits 13..13
    let val = reg_rawrd();
    (val >> 13) & 0x1
}

/// inserts field val into register
pub fn tsei_write(newval: u64) {
    // bits 13..13
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 13) | ((newval & 0x1) << 13));
}

/// reads field val from register
pub fn tall1_read() -> u64 {
    // bits 12..12
    let val = reg_rawrd();
    (val >> 12) & 0x1
}

/// inserts field val into register
pub fn tall1_write(newval: u64) {
    // bits 12..12
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 12) | ((newval & 0x1) << 12));
}

/// reads field val from register
pub fn tall0_read() -> u64 {
    // bits 11..11
    let val = reg_rawrd();
    (val >> 11) & 0x1
}

/// inserts field val into register
pub fn tall0_write(newval: u64) {
    // bits 11..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 11) | ((newval & 0x1) << 11));
}

/// reads field val from register
pub fn tc_read() -> u64 {
    // bits 10..10
    let val = reg_rawrd();
    (val >> 10) & 0x1
}

/// inserts field val into register
pub fn tc_write(newval: u64) {
    // bits 10..10
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 10) | ((newval & 0x1) << 10));
}

/// reads field val from register
pub fn vsgieoicount_1_read() -> u64 {
    // bits 8..8
    let val = reg_rawrd();
    (val >> 8) & 0x1
}

/// inserts field val into register
pub fn vsgieoicount_1_write(newval: u64) {
    // bits 8..8
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 8) | ((newval & 0x1) << 8));
}

/// reads field val from register
pub fn vgrp1die_read() -> u64 {
    // bits 7..7
    let val = reg_rawrd();
    (val >> 7) & 0x1
}

/// inserts field val into register
pub fn vgrp1die_write(newval: u64) {
    // bits 7..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 7) | ((newval & 0x1) << 7));
}

/// reads field val from register
pub fn vgrp1eie_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

/// inserts field val into register
pub fn vgrp1eie_write(newval: u64) {
    // bits 6..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 6) | ((newval & 0x1) << 6));
}

/// reads field val from register
pub fn vgrp0die_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn vgrp0die_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn vgrp0eie_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn vgrp0eie_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn npie_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

/// inserts field val into register
pub fn npie_write(newval: u64) {
    // bits 3..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 3) | ((newval & 0x1) << 3));
}

/// reads field val from register
pub fn lrenpie_read() -> u64 {
    // bits 2..2
    let val = reg_rawrd();
    (val >> 2) & 0x1
}

/// inserts field val into register
pub fn lrenpie_write(newval: u64) {
    // bits 2..2
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 2) | ((newval & 0x1) << 2));
}

/// reads field val from register
pub fn uie_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn uie_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}

/// reads field val from register
pub fn en_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn en_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Interrupt Controller Hyp Control Register value
/// in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register ich_hcr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xf800fdff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xf800fdff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xf800fdff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 4160814591;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn eoicount_extract(&mut self) -> u64 {
        // bits 27..31
        (self.val >> 27) & 0x1f
    }

    /// inserts field val into current value
    pub fn eoicount_insert(&mut self, val: u64) {
        // bits 27..31
        self.val = self.val & !(0x1f << 27) | ((val & 0x1f) << 27);
    }

    /// extracts field val from current value
    pub fn dvim_1_extract(&mut self) -> u64 {
        // bits 15..15
        (self.val >> 15) & 0x1
    }

    /// inserts field val into current value
    pub fn dvim_1_insert(&mut self, val: u64) {
        // bits 15..15
        self.val = self.val & !(0x1 << 15) | ((val & 0x1) << 15);
    }

    /// extracts field val from current value
    pub fn tdir_extract(&mut self) -> u64 {
        // bits 14..14
        (self.val >> 14) & 0x1
    }

    /// inserts field val into current value
    pub fn tdir_insert(&mut self, val: u64) {
        // bits 14..14
        self.val = self.val & !(0x1 << 14) | ((val & 0x1) << 14);
    }

    /// extracts field val from current value
    pub fn tsei_extract(&mut self) -> u64 {
        // bits 13..13
        (self.val >> 13) & 0x1
    }

    /// inserts field val into current value
    pub fn tsei_insert(&mut self, val: u64) {
        // bits 13..13
        self.val = self.val & !(0x1 << 13) | ((val & 0x1) << 13);
    }

    /// extracts field val from current value
    pub fn tall1_extract(&mut self) -> u64 {
        // bits 12..12
        (self.val >> 12) & 0x1
    }

    /// inserts field val into current value
    pub fn tall1_insert(&mut self, val: u64) {
        // bits 12..12
        self.val = self.val & !(0x1 << 12) | ((val & 0x1) << 12);
    }

    /// extracts field val from current value
    pub fn tall0_extract(&mut self) -> u64 {
        // bits 11..11
        (self.val >> 11) & 0x1
    }

    /// inserts field val into current value
    pub fn tall0_insert(&mut self, val: u64) {
        // bits 11..11
        self.val = self.val & !(0x1 << 11) | ((val & 0x1) << 11);
    }

    /// extracts field val from current value
    pub fn tc_extract(&mut self) -> u64 {
        // bits 10..10
        (self.val >> 10) & 0x1
    }

    /// inserts field val into current value
    pub fn tc_insert(&mut self, val: u64) {
        // bits 10..10
        self.val = self.val & !(0x1 << 10) | ((val & 0x1) << 10);
    }

    /// extracts field val from current value
    pub fn vsgieoicount_1_extract(&mut self) -> u64 {
        // bits 8..8
        (self.val >> 8) & 0x1
    }

    /// inserts field val into current value
    pub fn vsgieoicount_1_insert(&mut self, val: u64) {
        // bits 8..8
        self.val = self.val & !(0x1 << 8) | ((val & 0x1) << 8);
    }

    /// extracts field val from current value
    pub fn vgrp1die_extract(&mut self) -> u64 {
        // bits 7..7
        (self.val >> 7) & 0x1
    }

    /// inserts field val into current value
    pub fn vgrp1die_insert(&mut self, val: u64) {
        // bits 7..7
        self.val = self.val & !(0x1 << 7) | ((val & 0x1) << 7);
    }

    /// extracts field val from current value
    pub fn vgrp1eie_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }

    /// inserts field val into current value
    pub fn vgrp1eie_insert(&mut self, val: u64) {
        // bits 6..6
        self.val = self.val & !(0x1 << 6) | ((val & 0x1) << 6);
    }

    /// extracts field val from current value
    pub fn vgrp0die_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn vgrp0die_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn vgrp0eie_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn vgrp0eie_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn npie_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }

    /// inserts field val into current value
    pub fn npie_insert(&mut self, val: u64) {
        // bits 3..3
        self.val = self.val & !(0x1 << 3) | ((val & 0x1) << 3);
    }

    /// extracts field val from current value
    pub fn lrenpie_extract(&mut self) -> u64 {
        // bits 2..2
        (self.val >> 2) & 0x1
    }

    /// inserts field val into current value
    pub fn lrenpie_insert(&mut self, val: u64) {
        // bits 2..2
        self.val = self.val & !(0x1 << 2) | ((val & 0x1) << 2);
    }

    /// extracts field val from current value
    pub fn uie_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn uie_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }

    /// extracts field val from current value
    pub fn en_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn en_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

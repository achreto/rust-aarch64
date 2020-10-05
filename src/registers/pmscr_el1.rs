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
 * Generated on: 2020-10-05T16:30:11.717198
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
 * Register:    Statistical Profiling Control Register (EL1) (pmscr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Provides EL1 controls for Statistical Profiling.
 * File:        AArch64-pmscr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Statistical Profiling Control Register (EL1) (pmscr_el1)
/// register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, PMSCR_EL1
        llvm_asm!("mrs $0, S3_0_C9_C9_0" : "=r"(regval));
    }
    return regval;
}


/// writing the Statistical Profiling Control Register (EL1) (pmscr_el1)
/// register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR PMSCR_EL1, <Xt>
        llvm_asm!("msr S3_0_C9_C9_0, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn pct_1_read() -> u64 {
    // bits 6..7
    let val = reg_rawrd();
    (val >> 6) & 0x3
}

/// inserts field val into register
pub fn pct_1_write(newval: u64) {
    // bits 6..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 6) | ((newval & 0x3) << 6));
}

/// reads field val from register
pub fn pct_2_read() -> u64 {
    // bits 6..7
    let val = reg_rawrd();
    (val >> 6) & 0x3
}

/// inserts field val into register
pub fn pct_2_write(newval: u64) {
    // bits 6..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 6) | ((newval & 0x3) << 6));
}

/// reads field val from register
pub fn ts_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn ts_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn pa_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn pa_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn cx_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

/// inserts field val into register
pub fn cx_write(newval: u64) {
    // bits 3..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 3) | ((newval & 0x1) << 3));
}

/// reads field val from register
pub fn e1spe_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn e1spe_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}

/// reads field val from register
pub fn e0spe_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn e0spe_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Statistical Profiling Control Register (EL1)
/// value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register pmscr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xfb;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xfb
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xfb)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 251;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn pct_1_extract(&mut self) -> u64 {
        // bits 6..7
        (self.val >> 6) & 0x3
    }

    /// inserts field val into current value
    pub fn pct_1_insert(&mut self, val: u64) {
        // bits 6..7
        self.val = self.val & !(0x3 << 6) | ((val & 0x3) << 6);
    }

    /// extracts field val from current value
    pub fn pct_2_extract(&mut self) -> u64 {
        // bits 6..7
        (self.val >> 6) & 0x3
    }

    /// inserts field val into current value
    pub fn pct_2_insert(&mut self, val: u64) {
        // bits 6..7
        self.val = self.val & !(0x3 << 6) | ((val & 0x3) << 6);
    }

    /// extracts field val from current value
    pub fn ts_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn ts_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn pa_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn pa_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn cx_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }

    /// inserts field val into current value
    pub fn cx_insert(&mut self, val: u64) {
        // bits 3..3
        self.val = self.val & !(0x1 << 3) | ((val & 0x1) << 3);
    }

    /// extracts field val from current value
    pub fn e1spe_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn e1spe_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }

    /// extracts field val from current value
    pub fn e0spe_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn e0spe_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

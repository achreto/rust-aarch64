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
 * Generated on: 2020-10-05T16:30:11.681162
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
 * Register:    Cache Size Selection Register (csselr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Selects the current Cache Size ID Register,
 * File:        AArch64-csselr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Cache Size Selection Register (csselr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, CSSELR_EL1
        llvm_asm!("mrs $0, csselr_el1" : "=r"(regval));
    }
    return regval;
}


/// writing the Cache Size Selection Register (csselr_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR CSSELR_EL1, <Xt>
        llvm_asm!("msr csselr_el1, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn tnd_1_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn tnd_1_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn level_read() -> u64 {
    // bits 1..3
    let val = reg_rawrd();
    (val >> 1) & 0x7
}

/// inserts field val into register
pub fn level_write(newval: u64) {
    // bits 1..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7 << 1) | ((newval & 0x7) << 1));
}

/// reads field val from register
pub fn ind_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn ind_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Cache Size Selection Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register csselr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x1f;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x1f
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x1f)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 31;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn tnd_1_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn tnd_1_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn level_extract(&mut self) -> u64 {
        // bits 1..3
        (self.val >> 1) & 0x7
    }

    /// inserts field val into current value
    pub fn level_insert(&mut self, val: u64) {
        // bits 1..3
        self.val = self.val & !(0x7 << 1) | ((val & 0x7) << 1);
    }

    /// extracts field val from current value
    pub fn ind_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn ind_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

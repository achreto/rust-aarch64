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
 * Generated on: 2020-10-05T16:30:11.706975
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
 * Register:    Instruction Fault Status Register (EL2) (ifsr32_el2)
 * Group:       Exception and fault handling registers
 * Type:        64-bit Register
 * Description: Allows access to the AArch32
 * File:        AArch64-ifsr32_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Instruction Fault Status Register (EL2) (ifsr32_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, IFSR32_EL2
        llvm_asm!("mrs $0, ifsr32_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Instruction Fault Status Register (EL2) (ifsr32_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR IFSR32_EL2, <Xt>
        llvm_asm!("msr ifsr32_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn fnv_read() -> u64 {
    // bits 16..16
    let val = reg_rawrd();
    (val >> 16) & 0x1
}

/// inserts field val into register
pub fn fnv_write(newval: u64) {
    // bits 16..16
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 16) | ((newval & 0x1) << 16));
}

/// reads field val from register
pub fn ext_read() -> u64 {
    // bits 12..12
    let val = reg_rawrd();
    (val >> 12) & 0x1
}

/// inserts field val into register
pub fn ext_write(newval: u64) {
    // bits 12..12
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 12) | ((newval & 0x1) << 12));
}

/// reads field val from register
pub fn lpae_read() -> u64 {
    // bits 9..9
    let val = reg_rawrd();
    (val >> 9) & 0x1
}

/// inserts field val into register
pub fn lpae_write(newval: u64) {
    // bits 9..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 9) | ((newval & 0x1) << 9));
}

/// reads field val from register
pub fn status_read() -> u64 {
    // bits 0..5
    let val = reg_rawrd();
    (val >> 0) & 0x3f
}

/// inserts field val into register
pub fn status_write(newval: u64) {
    // bits 0..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3f << 0) | ((newval & 0x3f) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Instruction Fault Status Register (EL2) value
/// in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register ifsr32_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x1123f;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x1123f
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x1123f)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 70207;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn fnv_extract(&mut self) -> u64 {
        // bits 16..16
        (self.val >> 16) & 0x1
    }

    /// inserts field val into current value
    pub fn fnv_insert(&mut self, val: u64) {
        // bits 16..16
        self.val = self.val & !(0x1 << 16) | ((val & 0x1) << 16);
    }

    /// extracts field val from current value
    pub fn ext_extract(&mut self) -> u64 {
        // bits 12..12
        (self.val >> 12) & 0x1
    }

    /// inserts field val into current value
    pub fn ext_insert(&mut self, val: u64) {
        // bits 12..12
        self.val = self.val & !(0x1 << 12) | ((val & 0x1) << 12);
    }

    /// extracts field val from current value
    pub fn lpae_extract(&mut self) -> u64 {
        // bits 9..9
        (self.val >> 9) & 0x1
    }

    /// inserts field val into current value
    pub fn lpae_insert(&mut self, val: u64) {
        // bits 9..9
        self.val = self.val & !(0x1 << 9) | ((val & 0x1) << 9);
    }

    /// extracts field val from current value
    pub fn status_extract(&mut self) -> u64 {
        // bits 0..5
        (self.val >> 0) & 0x3f
    }

    /// inserts field val into current value
    pub fn status_insert(&mut self, val: u64) {
        // bits 0..5
        self.val = self.val & !(0x3f << 0) | ((val & 0x3f) << 0);
    }
}

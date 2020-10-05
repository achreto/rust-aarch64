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
 * Generated on: 2020-10-05T16:30:11.674616
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
 * Register:    Activity Monitors Control Register (amcr_el0)
 * Group:       Activity Monitors registers
 * Type:        64-bit Register
 * Description: Global control register for the activity monitors
 * implementation. AMCR_EL0 is applicable to both the architected and the
 * auxiliary counter groups. File:        AArch64-amcr_el0.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Activity Monitors Control Register (amcr_el0) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, AMCR_EL0
        llvm_asm!("mrs $0, S3_3_C13_C2_0" : "=r"(regval));
    }
    return regval;
}


/// writing the Activity Monitors Control Register (amcr_el0) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR AMCR_EL0, <Xt>
        llvm_asm!("msr S3_3_C13_C2_0, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn cg1rz_1_read() -> u64 {
    // bits 17..17
    let val = reg_rawrd();
    (val >> 17) & 0x1
}

/// inserts field val into register
pub fn cg1rz_1_write(newval: u64) {
    // bits 17..17
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 17) | ((newval & 0x1) << 17));
}

/// reads field val from register
pub fn hdbg_read() -> u64 {
    // bits 10..10
    let val = reg_rawrd();
    (val >> 10) & 0x1
}

/// inserts field val into register
pub fn hdbg_write(newval: u64) {
    // bits 10..10
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 10) | ((newval & 0x1) << 10));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Activity Monitors Control Register value in
/// memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register amcr_el0
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x20400;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x20400
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x20400)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 132096;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn cg1rz_1_extract(&mut self) -> u64 {
        // bits 17..17
        (self.val >> 17) & 0x1
    }

    /// inserts field val into current value
    pub fn cg1rz_1_insert(&mut self, val: u64) {
        // bits 17..17
        self.val = self.val & !(0x1 << 17) | ((val & 0x1) << 17);
    }

    /// extracts field val from current value
    pub fn hdbg_extract(&mut self) -> u64 {
        // bits 10..10
        (self.val >> 10) & 0x1
    }

    /// inserts field val into current value
    pub fn hdbg_insert(&mut self, val: u64) {
        // bits 10..10
        self.val = self.val & !(0x1 << 10) | ((val & 0x1) << 10);
    }
}

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
 * Generated on: 2020-10-05T16:30:11.718884
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
 * Register:    Sampling Interval Counter Register (pmsicr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Software must write zero to PMSICR_EL1 before enabling sample
 * profiling for a sampling session. Software must then treat PMSICR_EL1 as
 * an opaque, 64-bit, read/write register used for context switches only.
 * File:        AArch64-pmsicr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Sampling Interval Counter Register (pmsicr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, PMSICR_EL1
        llvm_asm!("mrs $0, S3_0_C9_C9_2" : "=r"(regval));
    }
    return regval;
}


/// writing the Sampling Interval Counter Register (pmsicr_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR PMSICR_EL1, <Xt>
        llvm_asm!("msr S3_0_C9_C9_2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn ecount_1_read() -> u64 {
    // bits 56..63
    let val = reg_rawrd();
    (val >> 56) & 0xff
}

/// inserts field val into register
pub fn ecount_1_write(newval: u64) {
    // bits 56..63
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 56) | ((newval & 0xff) << 56));
}

/// reads field val from register
pub fn count_read() -> u64 {
    // bits 0..31
    let val = reg_rawrd();
    (val >> 0) & 0xffffffff
}

/// inserts field val into register
pub fn count_write(newval: u64) {
    // bits 0..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0xffffffff << 0) | ((newval & 0xffffffff) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Sampling Interval Counter Register value in
/// memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register pmsicr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xff000000ffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xff000000ffffffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xff000000ffffffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 18374686483966590975;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn ecount_1_extract(&mut self) -> u64 {
        // bits 56..63
        (self.val >> 56) & 0xff
    }

    /// inserts field val into current value
    pub fn ecount_1_insert(&mut self, val: u64) {
        // bits 56..63
        self.val = self.val & !(0xff << 56) | ((val & 0xff) << 56);
    }

    /// extracts field val from current value
    pub fn count_extract(&mut self) -> u64 {
        // bits 0..31
        (self.val >> 0) & 0xffffffff
    }

    /// inserts field val into current value
    pub fn count_insert(&mut self, val: u64) {
        // bits 0..31
        self.val = self.val & !(0xffffffff << 0) | ((val & 0xffffffff) << 0);
    }
}

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
 * Generated on: 2020-10-05T16:49:32.064318
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
 * Register:    Profiling Buffer Limit Address Register (pmblimitr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Defines the upper limit for the profiling buffer, and enables the profiling
 * buffer File:        AArch64-pmblimitr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Profiling Buffer Limit Address Register (pmblimitr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, PMBLIMITR_EL1
        llvm_asm!("mrs $0, S3_0_C9_C10_0" : "=r"(regval));
    }
    return regval;
}


/// writing the Profiling Buffer Limit Address Register (pmblimitr_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR PMBLIMITR_EL1, <Xt>
        llvm_asm!("msr S3_0_C9_C10_0, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn limit_read() -> u64 {
    // bits 12..63
    let val = reg_rawrd();
    (val >> 12) & 0xfffffffffffff
}

/// inserts field val into register
pub fn limit_write(newval: u64) {
    // bits 12..63
    let val = reg_rawrd();
    reg_rawwr(val & !(0xfffffffffffff << 12) | ((newval & 0xfffffffffffff) << 12));
}

/// reads field val from register
pub fn pmfz_1_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn pmfz_1_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn fm_read() -> u64 {
    // bits 1..2
    let val = reg_rawrd();
    (val >> 1) & 0x3
}

/// inserts field val into register
pub fn fm_write(newval: u64) {
    // bits 1..2
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 1) | ((newval & 0x3) << 1));
}

/// reads field val from register
pub fn e_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn e_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Profiling Buffer Limit Address Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register pmblimitr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xfffffffffffff027;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xfffffffffffff027
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xfffffffffffff027)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 18446744073709547559;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn limit_extract(&mut self) -> u64 {
        // bits 12..63
        (self.val >> 12) & 0xfffffffffffff
    }

    /// inserts field val into current value
    pub fn limit_insert(&mut self, val: u64) {
        // bits 12..63
        self.val = self.val & !(0xfffffffffffff << 12) | ((val & 0xfffffffffffff) << 12);
    }

    /// extracts field val from current value
    pub fn pmfz_1_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn pmfz_1_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn fm_extract(&mut self) -> u64 {
        // bits 1..2
        (self.val >> 1) & 0x3
    }

    /// inserts field val into current value
    pub fn fm_insert(&mut self, val: u64) {
        // bits 1..2
        self.val = self.val & !(0x3 << 1) | ((val & 0x3) << 1);
    }

    /// extracts field val from current value
    pub fn e_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn e_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

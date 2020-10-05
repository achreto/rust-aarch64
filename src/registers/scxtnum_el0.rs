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
 * Generated on: 2020-10-05T16:30:11.725985
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
 * Register:    EL0 Read/Write Software Context Number (scxtnum_el0)
 * Group:       Thread and process ID registers
 * Type:        64-bit Register
 * Description: Provides a number that can be used to separate out different
 * context numbers with the EL0 exception level, for the purpose of
 * protecting against side-channels using branch prediction and similar
 * resources. File:        AArch64-scxtnum_el0.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the EL0 Read/Write Software Context Number (scxtnum_el0) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, SCXTNUM_EL0
        llvm_asm!("mrs $0, S3_3_C13_C0_7" : "=r"(regval));
    }
    return regval;
}


/// writing the EL0 Read/Write Software Context Number (scxtnum_el0) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR SCXTNUM_EL0, <Xt>
        llvm_asm!("msr S3_3_C13_C0_7, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn none_63_0_read() -> u64 {
    // bits 0..63
    let val = reg_rawrd();
    (val >> 0) & 0xffffffffffffffff
}

/// inserts field val into register
pub fn none_63_0_write(newval: u64) {
    // bits 0..63
    let val = reg_rawrd();
    reg_rawwr(val & !(0xffffffffffffffff << 0) | ((newval & 0xffffffffffffffff) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the EL0 Read/Write Software Context Number value in
/// memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register scxtnum_el0
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xffffffffffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xffffffffffffffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xffffffffffffffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 18446744073709551615;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn none_63_0_extract(&mut self) -> u64 {
        // bits 0..63
        (self.val >> 0) & 0xffffffffffffffff
    }

    /// inserts field val into current value
    pub fn none_63_0_insert(&mut self, val: u64) {
        // bits 0..63
        self.val = self.val & !(0xffffffffffffffff << 0) | ((val & 0xffffffffffffffff) << 0);
    }
}
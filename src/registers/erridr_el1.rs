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
 * Generated on: 2020-10-05T16:30:11.685402
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
 * Register:    Error Record ID Register (erridr_el1)
 * Group:       RAS registers
 * Type:        64-bit Register
 * Description: Defines the highest numbered index of the error records that
 * can be accessed through the Error Record System registers. File:
 * AArch64-erridr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Error Record ID Register (erridr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ERRIDR_EL1
        llvm_asm!("mrs $0, S3_0_C5_C3_0" : "=r"(regval));
    }
    return regval;
}

// register is not writable. not emitting write accessor

/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn num_read() -> u64 {
    // bits 0..15
    let val = reg_rawrd();
    (val >> 0) & 0xffff
}

// register is not writable, omitting writing to field


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Error Record ID Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register erridr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xffff
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 65535;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn num_extract(&mut self) -> u64 {
        // bits 0..15
        (self.val >> 0) & 0xffff
    }
    // no insert() method for field num
}

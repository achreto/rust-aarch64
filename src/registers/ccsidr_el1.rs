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
 * Generated on: 2020-10-05T16:49:32.027686
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
 * Register:    Current Cache Size ID Register (ccsidr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the architecture of the currently selected cache.
 * File:        AArch64-ccsidr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Current Cache Size ID Register (ccsidr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, CCSIDR_EL1
        llvm_asm!("mrs $0, ccsidr_el1" : "=r"(regval));
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
pub fn numsets_read() -> u64 {
    // bits 32..55
    let val = reg_rawrd();
    (val >> 32) & 0xffffff
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn associativity_read() -> u64 {
    // bits 3..23
    let val = reg_rawrd();
    (val >> 3) & 0x1fffff
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn linesize_read() -> u64 {
    // bits 0..2
    let val = reg_rawrd();
    (val >> 0) & 0x7
}

// register is not writable, omitting writing to field


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Current Cache Size ID Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register ccsidr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xffffff00ffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xffffff00ffffff
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 72057589759737855;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn numsets_extract(&mut self) -> u64 {
        // bits 32..55
        (self.val >> 32) & 0xffffff
    }
    // no insert() method for field numsets
    /// extracts field val from current value
    pub fn associativity_extract(&mut self) -> u64 {
        // bits 3..23
        (self.val >> 3) & 0x1fffff
    }
    // no insert() method for field associativity
    /// extracts field val from current value
    pub fn linesize_extract(&mut self) -> u64 {
        // bits 0..2
        (self.val >> 0) & 0x7
    }
    // no insert() method for field linesize
}

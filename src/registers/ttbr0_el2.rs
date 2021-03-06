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
 * Generated on: 2020-10-05T16:49:32.080504
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
 * Register:    Translation Table Base Register 0 (EL2) (ttbr0_el2)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: When
 * File:        AArch64-ttbr0_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Translation Table Base Register 0 (EL2) (ttbr0_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, TTBR0_EL2
        llvm_asm!("mrs $0, ttbr0_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Translation Table Base Register 0 (EL2) (ttbr0_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR TTBR0_EL2, <Xt>
        llvm_asm!("msr ttbr0_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn asid_1_read() -> u64 {
    // bits 48..63
    let val = reg_rawrd();
    (val >> 48) & 0xffff
}

/// inserts field val into register
pub fn asid_1_write(newval: u64) {
    // bits 48..63
    let val = reg_rawrd();
    reg_rawwr(val & !(0xffff << 48) | ((newval & 0xffff) << 48));
}

/// reads field val from register
pub fn baddr471_read() -> u64 {
    // bits 1..47
    let val = reg_rawrd();
    (val >> 1) & 0x7fffffffffff
}

/// inserts field val into register
pub fn baddr471_write(newval: u64) {
    // bits 1..47
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7fffffffffff << 1) | ((newval & 0x7fffffffffff) << 1));
}

/// reads field val from register
pub fn cnp_1_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn cnp_1_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Translation Table Base Register 0 (EL2) value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register ttbr0_el2
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
    pub fn asid_1_extract(&mut self) -> u64 {
        // bits 48..63
        (self.val >> 48) & 0xffff
    }

    /// inserts field val into current value
    pub fn asid_1_insert(&mut self, val: u64) {
        // bits 48..63
        self.val = self.val & !(0xffff << 48) | ((val & 0xffff) << 48);
    }

    /// extracts field val from current value
    pub fn baddr471_extract(&mut self) -> u64 {
        // bits 1..47
        (self.val >> 1) & 0x7fffffffffff
    }

    /// inserts field val into current value
    pub fn baddr471_insert(&mut self, val: u64) {
        // bits 1..47
        self.val = self.val & !(0x7fffffffffff << 1) | ((val & 0x7fffffffffff) << 1);
    }

    /// extracts field val from current value
    pub fn cnp_1_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn cnp_1_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

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
 * Generated on: 2020-10-05T16:49:32.059906
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
 * Register:    MPAM2 Register (EL2) (mpam2_el2)
 * Group:       Memory Partitioning and Monitoring registers
 * Type:        64-bit Register
 * Description: Holds information to generate MPAM labels for memory requests when executing at
 * EL2. File:        AArch64-mpam2_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the MPAM2 Register (EL2) (mpam2_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, MPAM2_EL2
        llvm_asm!("mrs $0, S3_4_C10_C5_0" : "=r"(regval));
    }
    return regval;
}


/// writing the MPAM2 Register (EL2) (mpam2_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR MPAM2_EL2, <Xt>
        llvm_asm!("msr S3_4_C10_C5_0, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn mpamen_read() -> u64 {
    // bits 63..63
    let val = reg_rawrd();
    (val >> 63) & 0x1
}

/// inserts field val into register
pub fn mpamen_write(newval: u64) {
    // bits 63..63
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 63) | ((newval & 0x1) << 63));
}

/// reads field val from register
pub fn tidr_1_read() -> u64 {
    // bits 58..58
    let val = reg_rawrd();
    (val >> 58) & 0x1
}

/// inserts field val into register
pub fn tidr_1_write(newval: u64) {
    // bits 58..58
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 58) | ((newval & 0x1) << 58));
}

/// reads field val from register
pub fn trapmpam0el1_read() -> u64 {
    // bits 49..49
    let val = reg_rawrd();
    (val >> 49) & 0x1
}

/// inserts field val into register
pub fn trapmpam0el1_write(newval: u64) {
    // bits 49..49
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 49) | ((newval & 0x1) << 49));
}

/// reads field val from register
pub fn trapmpam1el1_read() -> u64 {
    // bits 48..48
    let val = reg_rawrd();
    (val >> 48) & 0x1
}

/// inserts field val into register
pub fn trapmpam1el1_write(newval: u64) {
    // bits 48..48
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 48) | ((newval & 0x1) << 48));
}

/// reads field val from register
pub fn pmg_d_read() -> u64 {
    // bits 40..47
    let val = reg_rawrd();
    (val >> 40) & 0xff
}

/// inserts field val into register
pub fn pmg_d_write(newval: u64) {
    // bits 40..47
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 40) | ((newval & 0xff) << 40));
}

/// reads field val from register
pub fn pmg_i_read() -> u64 {
    // bits 32..39
    let val = reg_rawrd();
    (val >> 32) & 0xff
}

/// inserts field val into register
pub fn pmg_i_write(newval: u64) {
    // bits 32..39
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 32) | ((newval & 0xff) << 32));
}

/// reads field val from register
pub fn partid_d_read() -> u64 {
    // bits 16..31
    let val = reg_rawrd();
    (val >> 16) & 0xffff
}

/// inserts field val into register
pub fn partid_d_write(newval: u64) {
    // bits 16..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0xffff << 16) | ((newval & 0xffff) << 16));
}

/// reads field val from register
pub fn partid_i_read() -> u64 {
    // bits 0..15
    let val = reg_rawrd();
    (val >> 0) & 0xffff
}

/// inserts field val into register
pub fn partid_i_write(newval: u64) {
    // bits 0..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0xffff << 0) | ((newval & 0xffff) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the MPAM2 Register (EL2) value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register mpam2_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x8403ffffffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x8403ffffffffffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x8403ffffffffffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 9512728312913330175;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn mpamen_extract(&mut self) -> u64 {
        // bits 63..63
        (self.val >> 63) & 0x1
    }

    /// inserts field val into current value
    pub fn mpamen_insert(&mut self, val: u64) {
        // bits 63..63
        self.val = self.val & !(0x1 << 63) | ((val & 0x1) << 63);
    }

    /// extracts field val from current value
    pub fn tidr_1_extract(&mut self) -> u64 {
        // bits 58..58
        (self.val >> 58) & 0x1
    }

    /// inserts field val into current value
    pub fn tidr_1_insert(&mut self, val: u64) {
        // bits 58..58
        self.val = self.val & !(0x1 << 58) | ((val & 0x1) << 58);
    }

    /// extracts field val from current value
    pub fn trapmpam0el1_extract(&mut self) -> u64 {
        // bits 49..49
        (self.val >> 49) & 0x1
    }

    /// inserts field val into current value
    pub fn trapmpam0el1_insert(&mut self, val: u64) {
        // bits 49..49
        self.val = self.val & !(0x1 << 49) | ((val & 0x1) << 49);
    }

    /// extracts field val from current value
    pub fn trapmpam1el1_extract(&mut self) -> u64 {
        // bits 48..48
        (self.val >> 48) & 0x1
    }

    /// inserts field val into current value
    pub fn trapmpam1el1_insert(&mut self, val: u64) {
        // bits 48..48
        self.val = self.val & !(0x1 << 48) | ((val & 0x1) << 48);
    }

    /// extracts field val from current value
    pub fn pmg_d_extract(&mut self) -> u64 {
        // bits 40..47
        (self.val >> 40) & 0xff
    }

    /// inserts field val into current value
    pub fn pmg_d_insert(&mut self, val: u64) {
        // bits 40..47
        self.val = self.val & !(0xff << 40) | ((val & 0xff) << 40);
    }

    /// extracts field val from current value
    pub fn pmg_i_extract(&mut self) -> u64 {
        // bits 32..39
        (self.val >> 32) & 0xff
    }

    /// inserts field val into current value
    pub fn pmg_i_insert(&mut self, val: u64) {
        // bits 32..39
        self.val = self.val & !(0xff << 32) | ((val & 0xff) << 32);
    }

    /// extracts field val from current value
    pub fn partid_d_extract(&mut self) -> u64 {
        // bits 16..31
        (self.val >> 16) & 0xffff
    }

    /// inserts field val into current value
    pub fn partid_d_insert(&mut self, val: u64) {
        // bits 16..31
        self.val = self.val & !(0xffff << 16) | ((val & 0xffff) << 16);
    }

    /// extracts field val from current value
    pub fn partid_i_extract(&mut self) -> u64 {
        // bits 0..15
        (self.val >> 0) & 0xffff
    }

    /// inserts field val into current value
    pub fn partid_i_insert(&mut self, val: u64) {
        // bits 0..15
        self.val = self.val & !(0xffff << 0) | ((val & 0xffff) << 0);
    }
}

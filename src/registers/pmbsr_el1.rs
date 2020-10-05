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
 * Generated on: 2020-10-05T16:30:11.714985
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
 * Register:    Profiling Buffer Status/syndrome Register (pmbsr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Provides syndrome information to software when the buffer is
 * disabled because the management interrupt has been raised. File:
 * AArch64-pmbsr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Profiling Buffer Status/syndrome Register (pmbsr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, PMBSR_EL1
        llvm_asm!("mrs $0, S3_0_C9_C10_3" : "=r"(regval));
    }
    return regval;
}


/// writing the Profiling Buffer Status/syndrome Register (pmbsr_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR PMBSR_EL1, <Xt>
        llvm_asm!("msr S3_0_C9_C10_3, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn ec_read() -> u64 {
    // bits 26..31
    let val = reg_rawrd();
    (val >> 26) & 0x3f
}

/// inserts field val into register
pub fn ec_write(newval: u64) {
    // bits 26..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3f << 26) | ((newval & 0x3f) << 26));
}

/// reads field val from register
pub fn dl_read() -> u64 {
    // bits 19..19
    let val = reg_rawrd();
    (val >> 19) & 0x1
}

/// inserts field val into register
pub fn dl_write(newval: u64) {
    // bits 19..19
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 19) | ((newval & 0x1) << 19));
}

/// reads field val from register
pub fn ea_read() -> u64 {
    // bits 18..18
    let val = reg_rawrd();
    (val >> 18) & 0x1
}

/// inserts field val into register
pub fn ea_write(newval: u64) {
    // bits 18..18
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 18) | ((newval & 0x1) << 18));
}

/// reads field val from register
pub fn s_read() -> u64 {
    // bits 17..17
    let val = reg_rawrd();
    (val >> 17) & 0x1
}

/// inserts field val into register
pub fn s_write(newval: u64) {
    // bits 17..17
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 17) | ((newval & 0x1) << 17));
}

/// reads field val from register
pub fn coll_read() -> u64 {
    // bits 16..16
    let val = reg_rawrd();
    (val >> 16) & 0x1
}

/// inserts field val into register
pub fn coll_write(newval: u64) {
    // bits 16..16
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 16) | ((newval & 0x1) << 16));
}

/// reads field val from register
pub fn mss_read() -> u64 {
    // bits 0..15
    let val = reg_rawrd();
    (val >> 0) & 0xffff
}

/// inserts field val into register
pub fn mss_write(newval: u64) {
    // bits 0..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0xffff << 0) | ((newval & 0xffff) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Profiling Buffer Status/syndrome Register value
/// in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register pmbsr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xfc0fffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xfc0fffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xfc0fffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 4228907007;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn ec_extract(&mut self) -> u64 {
        // bits 26..31
        (self.val >> 26) & 0x3f
    }

    /// inserts field val into current value
    pub fn ec_insert(&mut self, val: u64) {
        // bits 26..31
        self.val = self.val & !(0x3f << 26) | ((val & 0x3f) << 26);
    }

    /// extracts field val from current value
    pub fn dl_extract(&mut self) -> u64 {
        // bits 19..19
        (self.val >> 19) & 0x1
    }

    /// inserts field val into current value
    pub fn dl_insert(&mut self, val: u64) {
        // bits 19..19
        self.val = self.val & !(0x1 << 19) | ((val & 0x1) << 19);
    }

    /// extracts field val from current value
    pub fn ea_extract(&mut self) -> u64 {
        // bits 18..18
        (self.val >> 18) & 0x1
    }

    /// inserts field val into current value
    pub fn ea_insert(&mut self, val: u64) {
        // bits 18..18
        self.val = self.val & !(0x1 << 18) | ((val & 0x1) << 18);
    }

    /// extracts field val from current value
    pub fn s_extract(&mut self) -> u64 {
        // bits 17..17
        (self.val >> 17) & 0x1
    }

    /// inserts field val into current value
    pub fn s_insert(&mut self, val: u64) {
        // bits 17..17
        self.val = self.val & !(0x1 << 17) | ((val & 0x1) << 17);
    }

    /// extracts field val from current value
    pub fn coll_extract(&mut self) -> u64 {
        // bits 16..16
        (self.val >> 16) & 0x1
    }

    /// inserts field val into current value
    pub fn coll_insert(&mut self, val: u64) {
        // bits 16..16
        self.val = self.val & !(0x1 << 16) | ((val & 0x1) << 16);
    }

    /// extracts field val from current value
    pub fn mss_extract(&mut self) -> u64 {
        // bits 0..15
        (self.val >> 0) & 0xffff
    }

    /// inserts field val into current value
    pub fn mss_insert(&mut self, val: u64) {
        // bits 0..15
        self.val = self.val & !(0xffff << 0) | ((val & 0xffff) << 0);
    }
}

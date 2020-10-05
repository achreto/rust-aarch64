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
 * Generated on: 2020-10-05T16:30:11.681307
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
 * Register:    Cache Type Register (ctr_el0)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the architecture of the caches.
 * File:        AArch64-ctr_el0.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Cache Type Register (ctr_el0) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, CTR_EL0
        llvm_asm!("mrs $0, ctr_el0" : "=r"(regval));
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
pub fn tminline_1_read() -> u64 {
    // bits 32..37
    let val = reg_rawrd();
    (val >> 32) & 0x3f
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn dic_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn idc_read() -> u64 {
    // bits 28..28
    let val = reg_rawrd();
    (val >> 28) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn cwg_read() -> u64 {
    // bits 24..27
    let val = reg_rawrd();
    (val >> 24) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn erg_read() -> u64 {
    // bits 20..23
    let val = reg_rawrd();
    (val >> 20) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn dminline_read() -> u64 {
    // bits 16..19
    let val = reg_rawrd();
    (val >> 16) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn l1ip_read() -> u64 {
    // bits 14..15
    let val = reg_rawrd();
    (val >> 14) & 0x3
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn iminline_read() -> u64 {
    // bits 0..3
    let val = reg_rawrd();
    (val >> 0) & 0xf
}

// register is not writable, omitting writing to field


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Cache Type Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register ctr_el0
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x3f3fffc00f;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x3f3fffc00f
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 271656665103;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn tminline_1_extract(&mut self) -> u64 {
        // bits 32..37
        (self.val >> 32) & 0x3f
    }
    // no insert() method for field tminline_1
    /// extracts field val from current value
    pub fn dic_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }
    // no insert() method for field dic
    /// extracts field val from current value
    pub fn idc_extract(&mut self) -> u64 {
        // bits 28..28
        (self.val >> 28) & 0x1
    }
    // no insert() method for field idc
    /// extracts field val from current value
    pub fn cwg_extract(&mut self) -> u64 {
        // bits 24..27
        (self.val >> 24) & 0xf
    }
    // no insert() method for field cwg
    /// extracts field val from current value
    pub fn erg_extract(&mut self) -> u64 {
        // bits 20..23
        (self.val >> 20) & 0xf
    }
    // no insert() method for field erg
    /// extracts field val from current value
    pub fn dminline_extract(&mut self) -> u64 {
        // bits 16..19
        (self.val >> 16) & 0xf
    }
    // no insert() method for field dminline
    /// extracts field val from current value
    pub fn l1ip_extract(&mut self) -> u64 {
        // bits 14..15
        (self.val >> 14) & 0x3
    }
    // no insert() method for field l1ip
    /// extracts field val from current value
    pub fn iminline_extract(&mut self) -> u64 {
        // bits 0..3
        (self.val >> 0) & 0xf
    }
    // no insert() method for field iminline
}

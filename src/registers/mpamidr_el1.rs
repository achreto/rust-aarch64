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
 * Generated on: 2020-10-05T16:30:11.710892
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
 * Register:    MPAM ID Register (EL1) (mpamidr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Indicates the presence and maximum PARTID and PMG values
 * supported in the implementation. It also indicates whether the
 * implementation supports MPAM virtualization. File:
 * AArch64-mpamidr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the MPAM ID Register (EL1) (mpamidr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, MPAMIDR_EL1
        llvm_asm!("mrs $0, S3_0_C10_C4_4" : "=r"(regval));
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
pub fn has_sdeflt_read() -> u64 {
    // bits 61..61
    let val = reg_rawrd();
    (val >> 61) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn has_force_ns_read() -> u64 {
    // bits 60..60
    let val = reg_rawrd();
    (val >> 60) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn has_tidr_read() -> u64 {
    // bits 58..58
    let val = reg_rawrd();
    (val >> 58) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn pmg_max_read() -> u64 {
    // bits 32..39
    let val = reg_rawrd();
    (val >> 32) & 0xff
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn vpmr_max_1_read() -> u64 {
    // bits 18..20
    let val = reg_rawrd();
    (val >> 18) & 0x7
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn has_hcr_read() -> u64 {
    // bits 17..17
    let val = reg_rawrd();
    (val >> 17) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn partid_max_read() -> u64 {
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



/// struct holding a copy of the MPAM ID Register (EL1) value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register mpamidr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x340000ff001effff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x340000ff001effff
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 3746995985190944767;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn has_sdeflt_extract(&mut self) -> u64 {
        // bits 61..61
        (self.val >> 61) & 0x1
    }
    // no insert() method for field has_sdeflt
    /// extracts field val from current value
    pub fn has_force_ns_extract(&mut self) -> u64 {
        // bits 60..60
        (self.val >> 60) & 0x1
    }
    // no insert() method for field has_force_ns
    /// extracts field val from current value
    pub fn has_tidr_extract(&mut self) -> u64 {
        // bits 58..58
        (self.val >> 58) & 0x1
    }
    // no insert() method for field has_tidr
    /// extracts field val from current value
    pub fn pmg_max_extract(&mut self) -> u64 {
        // bits 32..39
        (self.val >> 32) & 0xff
    }
    // no insert() method for field pmg_max
    /// extracts field val from current value
    pub fn vpmr_max_1_extract(&mut self) -> u64 {
        // bits 18..20
        (self.val >> 18) & 0x7
    }
    // no insert() method for field vpmr_max_1
    /// extracts field val from current value
    pub fn has_hcr_extract(&mut self) -> u64 {
        // bits 17..17
        (self.val >> 17) & 0x1
    }
    // no insert() method for field has_hcr
    /// extracts field val from current value
    pub fn partid_max_extract(&mut self) -> u64 {
        // bits 0..15
        (self.val >> 0) & 0xffff
    }
    // no insert() method for field partid_max
}

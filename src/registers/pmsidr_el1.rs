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
 * Generated on: 2020-10-05T16:30:11.719108
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
 * Register:    Sampling Profiling ID Register (pmsidr_el1)
 * Group:       Statistical Profiling Extension registers
 * Type:        64-bit Register
 * Description: Describes the Statistical Profiling implementation to
 * software File:        AArch64-pmsidr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Sampling Profiling ID Register (pmsidr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, PMSIDR_EL1
        llvm_asm!("mrs $0, S3_0_C9_C9_7" : "=r"(regval));
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
pub fn format_1_read() -> u64 {
    // bits 20..23
    let val = reg_rawrd();
    (val >> 20) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn countsize_read() -> u64 {
    // bits 16..19
    let val = reg_rawrd();
    (val >> 16) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn maxsize_read() -> u64 {
    // bits 12..15
    let val = reg_rawrd();
    (val >> 12) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn interval_read() -> u64 {
    // bits 8..11
    let val = reg_rawrd();
    (val >> 8) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn fne_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn ernd_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn lds_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn archinst_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn fl_read() -> u64 {
    // bits 2..2
    let val = reg_rawrd();
    (val >> 2) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn ft_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn fe_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

// register is not writable, omitting writing to field


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Sampling Profiling ID Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register pmsidr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xffff7f;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xffff7f
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 16777087;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn format_1_extract(&mut self) -> u64 {
        // bits 20..23
        (self.val >> 20) & 0xf
    }
    // no insert() method for field format_1
    /// extracts field val from current value
    pub fn countsize_extract(&mut self) -> u64 {
        // bits 16..19
        (self.val >> 16) & 0xf
    }
    // no insert() method for field countsize
    /// extracts field val from current value
    pub fn maxsize_extract(&mut self) -> u64 {
        // bits 12..15
        (self.val >> 12) & 0xf
    }
    // no insert() method for field maxsize
    /// extracts field val from current value
    pub fn interval_extract(&mut self) -> u64 {
        // bits 8..11
        (self.val >> 8) & 0xf
    }
    // no insert() method for field interval
    /// extracts field val from current value
    pub fn fne_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }
    // no insert() method for field fne
    /// extracts field val from current value
    pub fn ernd_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }
    // no insert() method for field ernd
    /// extracts field val from current value
    pub fn lds_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }
    // no insert() method for field lds
    /// extracts field val from current value
    pub fn archinst_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }
    // no insert() method for field archinst
    /// extracts field val from current value
    pub fn fl_extract(&mut self) -> u64 {
        // bits 2..2
        (self.val >> 2) & 0x1
    }
    // no insert() method for field fl
    /// extracts field val from current value
    pub fn ft_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }
    // no insert() method for field ft
    /// extracts field val from current value
    pub fn fe_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }
    // no insert() method for field fe
}

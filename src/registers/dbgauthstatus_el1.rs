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
 * Generated on: 2020-10-05T16:30:11.682181
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
 * Register:    Debug Authentication Status register (dbgauthstatus_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Provides information about the state of the
 * File:        AArch64-dbgauthstatus_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Debug Authentication Status register (dbgauthstatus_el1)
/// register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, DBGAUTHSTATUS_EL1
        llvm_asm!("mrs $0, dbgauthstatus_el1" : "=r"(regval));
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
pub fn snid_1_read() -> u64 {
    // bits 6..7
    let val = reg_rawrd();
    (val >> 6) & 0x3
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn snid_2_read() -> u64 {
    // bits 6..7
    let val = reg_rawrd();
    (val >> 6) & 0x3
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn sid_read() -> u64 {
    // bits 4..5
    let val = reg_rawrd();
    (val >> 4) & 0x3
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn nsnid_1_read() -> u64 {
    // bits 2..3
    let val = reg_rawrd();
    (val >> 2) & 0x3
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn nsnid_2_read() -> u64 {
    // bits 2..3
    let val = reg_rawrd();
    (val >> 2) & 0x3
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn nsid_read() -> u64 {
    // bits 0..1
    let val = reg_rawrd();
    (val >> 0) & 0x3
}

// register is not writable, omitting writing to field


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Debug Authentication Status register value in
/// memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register dbgauthstatus_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xff
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 255;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn snid_1_extract(&mut self) -> u64 {
        // bits 6..7
        (self.val >> 6) & 0x3
    }
    // no insert() method for field snid_1
    /// extracts field val from current value
    pub fn snid_2_extract(&mut self) -> u64 {
        // bits 6..7
        (self.val >> 6) & 0x3
    }
    // no insert() method for field snid_2
    /// extracts field val from current value
    pub fn sid_extract(&mut self) -> u64 {
        // bits 4..5
        (self.val >> 4) & 0x3
    }
    // no insert() method for field sid
    /// extracts field val from current value
    pub fn nsnid_1_extract(&mut self) -> u64 {
        // bits 2..3
        (self.val >> 2) & 0x3
    }
    // no insert() method for field nsnid_1
    /// extracts field val from current value
    pub fn nsnid_2_extract(&mut self) -> u64 {
        // bits 2..3
        (self.val >> 2) & 0x3
    }
    // no insert() method for field nsnid_2
    /// extracts field val from current value
    pub fn nsid_extract(&mut self) -> u64 {
        // bits 0..1
        (self.val >> 0) & 0x3
    }
    // no insert() method for field nsid
}

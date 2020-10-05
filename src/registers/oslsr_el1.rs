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
 * Generated on: 2020-10-05T16:30:11.713940
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
 * Register:    OS Lock Status Register (oslsr_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Provides the status of the OS Lock.
 * File:        AArch64-oslsr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the OS Lock Status Register (oslsr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, OSLSR_EL1
        llvm_asm!("mrs $0, oslsr_el1" : "=r"(regval));
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
pub fn oslm_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn ntt_read() -> u64 {
    // bits 2..2
    let val = reg_rawrd();
    (val >> 2) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn oslk_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn oslm_0_0_read() -> u64 {
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



/// struct holding a copy of the OS Lock Status Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register oslsr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xf;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xf
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 15;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn oslm_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }
    // no insert() method for field oslm
    /// extracts field val from current value
    pub fn ntt_extract(&mut self) -> u64 {
        // bits 2..2
        (self.val >> 2) & 0x1
    }
    // no insert() method for field ntt
    /// extracts field val from current value
    pub fn oslk_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }
    // no insert() method for field oslk
    /// extracts field val from current value
    pub fn oslm_0_0_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }
    // no insert() method for field oslm_0_0
}

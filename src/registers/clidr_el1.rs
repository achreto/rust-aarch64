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
 * Generated on: 2020-10-05T16:30:11.676317
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
 * Register:    Cache Level ID Register (clidr_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Identifies the type of cache, or caches, that are implemented
 * at each level and can be managed using the architected cache maintenance
 * instructions that operate by set/way, up to a maximum of seven levels.
 * Also identifies the Level of Coherence (LoC) and Level of Unification
 * (LoU) for the cache hierarchy. File:        AArch64-clidr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Cache Level ID Register (clidr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, CLIDR_EL1
        llvm_asm!("mrs $0, clidr_el1" : "=r"(regval));
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
pub fn ttypen_1_read() -> u64 {
    // bits 33..46
    let val = reg_rawrd();
    (val >> 33) & 0x3fff
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn icb_read() -> u64 {
    // bits 30..32
    let val = reg_rawrd();
    (val >> 30) & 0x7
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn louu_read() -> u64 {
    // bits 27..29
    let val = reg_rawrd();
    (val >> 27) & 0x7
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn loc_read() -> u64 {
    // bits 24..26
    let val = reg_rawrd();
    (val >> 24) & 0x7
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn louis_read() -> u64 {
    // bits 21..23
    let val = reg_rawrd();
    (val >> 21) & 0x7
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn ctypen_read() -> u64 {
    // bits 0..20
    let val = reg_rawrd();
    (val >> 0) & 0x1fffff
}

// register is not writable, omitting writing to field


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Cache Level ID Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register clidr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x7fffffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x7fffffffffff
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 140737488355327;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn ttypen_1_extract(&mut self) -> u64 {
        // bits 33..46
        (self.val >> 33) & 0x3fff
    }
    // no insert() method for field ttypen_1
    /// extracts field val from current value
    pub fn icb_extract(&mut self) -> u64 {
        // bits 30..32
        (self.val >> 30) & 0x7
    }
    // no insert() method for field icb
    /// extracts field val from current value
    pub fn louu_extract(&mut self) -> u64 {
        // bits 27..29
        (self.val >> 27) & 0x7
    }
    // no insert() method for field louu
    /// extracts field val from current value
    pub fn loc_extract(&mut self) -> u64 {
        // bits 24..26
        (self.val >> 24) & 0x7
    }
    // no insert() method for field loc
    /// extracts field val from current value
    pub fn louis_extract(&mut self) -> u64 {
        // bits 21..23
        (self.val >> 21) & 0x7
    }
    // no insert() method for field louis
    /// extracts field val from current value
    pub fn ctypen_extract(&mut self) -> u64 {
        // bits 0..20
        (self.val >> 0) & 0x1fffff
    }
    // no insert() method for field ctypen
}

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
 * Generated on: 2020-10-05T16:49:32.054617
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
 * Register:    AArch32 Instruction Set Attribute Register 5 (id_isar5_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the instruction sets implemented by the PE in AArch32
 * state. File:        AArch64-id_isar5_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the AArch32 Instruction Set Attribute Register 5 (id_isar5_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ID_ISAR5_EL1
        llvm_asm!("mrs $0, id_isar5_el1" : "=r"(regval));
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
pub fn vcma_read() -> u64 {
    // bits 28..31
    let val = reg_rawrd();
    (val >> 28) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn rdm_read() -> u64 {
    // bits 24..27
    let val = reg_rawrd();
    (val >> 24) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn crc32_read() -> u64 {
    // bits 16..19
    let val = reg_rawrd();
    (val >> 16) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn sha2_read() -> u64 {
    // bits 12..15
    let val = reg_rawrd();
    (val >> 12) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn sha1_read() -> u64 {
    // bits 8..11
    let val = reg_rawrd();
    (val >> 8) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn aes_read() -> u64 {
    // bits 4..7
    let val = reg_rawrd();
    (val >> 4) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn sevl_read() -> u64 {
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



/// struct holding a copy of the AArch32 Instruction Set Attribute Register 5 value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register id_isar5_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xff0fffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xff0fffff
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 4279238655;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn vcma_extract(&mut self) -> u64 {
        // bits 28..31
        (self.val >> 28) & 0xf
    }
    // no insert() method for field vcma
    /// extracts field val from current value
    pub fn rdm_extract(&mut self) -> u64 {
        // bits 24..27
        (self.val >> 24) & 0xf
    }
    // no insert() method for field rdm
    /// extracts field val from current value
    pub fn crc32_extract(&mut self) -> u64 {
        // bits 16..19
        (self.val >> 16) & 0xf
    }
    // no insert() method for field crc32
    /// extracts field val from current value
    pub fn sha2_extract(&mut self) -> u64 {
        // bits 12..15
        (self.val >> 12) & 0xf
    }
    // no insert() method for field sha2
    /// extracts field val from current value
    pub fn sha1_extract(&mut self) -> u64 {
        // bits 8..11
        (self.val >> 8) & 0xf
    }
    // no insert() method for field sha1
    /// extracts field val from current value
    pub fn aes_extract(&mut self) -> u64 {
        // bits 4..7
        (self.val >> 4) & 0xf
    }
    // no insert() method for field aes
    /// extracts field val from current value
    pub fn sevl_extract(&mut self) -> u64 {
        // bits 0..3
        (self.val >> 0) & 0xf
    }
    // no insert() method for field sevl
}

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
 * Generated on: 2020-10-05T16:49:32.051913
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
 * Register:    AArch64 Instruction Set Attribute Register 2 (id_aa64isar2_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the features and instructions implemented in AArch64
 * state. File:        AArch64-id_aa64isar2_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the AArch64 Instruction Set Attribute Register 2 (id_aa64isar2_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ID_AA64ISAR2_EL1
        llvm_asm!("mrs $0, S3_0_C0_C6_2" : "=r"(regval));
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
pub fn rpres_read() -> u64 {
    // bits 4..7
    let val = reg_rawrd();
    (val >> 4) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn wfxt_read() -> u64 {
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



/// struct holding a copy of the AArch64 Instruction Set Attribute Register 2 value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register id_aa64isar2_el1
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
    pub fn rpres_extract(&mut self) -> u64 {
        // bits 4..7
        (self.val >> 4) & 0xf
    }
    // no insert() method for field rpres
    /// extracts field val from current value
    pub fn wfxt_extract(&mut self) -> u64 {
        // bits 0..3
        (self.val >> 0) & 0xf
    }
    // no insert() method for field wfxt
}

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
 * Generated on: 2020-10-05T16:49:32.032661
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
 * Register:    Architectural Feature Trap Register (EL2) (cptr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Controls trapping to EL2 of accesses to
 * File:        AArch64-cptr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Architectural Feature Trap Register (EL2) (cptr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, CPTR_EL2
        llvm_asm!("mrs $0, cptr_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Architectural Feature Trap Register (EL2) (cptr_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR CPTR_EL2, <Xt>
        llvm_asm!("msr cptr_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn tcpac_read() -> u64 {
    // bits 31..31
    let val = reg_rawrd();
    (val >> 31) & 0x1
}

/// inserts field val into register
pub fn tcpac_write(newval: u64) {
    // bits 31..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 31) | ((newval & 0x1) << 31));
}

/// reads field val from register
pub fn tam_1_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn tam_1_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn tta_read() -> u64 {
    // bits 28..28
    let val = reg_rawrd();
    (val >> 28) & 0x1
}

/// inserts field val into register
pub fn tta_write(newval: u64) {
    // bits 28..28
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 28) | ((newval & 0x1) << 28));
}

/// reads field val from register
pub fn fpen_read() -> u64 {
    // bits 20..21
    let val = reg_rawrd();
    (val >> 20) & 0x3
}

/// inserts field val into register
pub fn fpen_write(newval: u64) {
    // bits 20..21
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 20) | ((newval & 0x3) << 20));
}

/// reads field val from register
pub fn zen_1_read() -> u64 {
    // bits 16..17
    let val = reg_rawrd();
    (val >> 16) & 0x3
}

/// inserts field val into register
pub fn zen_1_write(newval: u64) {
    // bits 16..17
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 16) | ((newval & 0x3) << 16));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Architectural Feature Trap Register (EL2) value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register cptr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xd0330000;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xd0330000
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xd0330000)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 3493003264;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn tcpac_extract(&mut self) -> u64 {
        // bits 31..31
        (self.val >> 31) & 0x1
    }

    /// inserts field val into current value
    pub fn tcpac_insert(&mut self, val: u64) {
        // bits 31..31
        self.val = self.val & !(0x1 << 31) | ((val & 0x1) << 31);
    }

    /// extracts field val from current value
    pub fn tam_1_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn tam_1_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn tta_extract(&mut self) -> u64 {
        // bits 28..28
        (self.val >> 28) & 0x1
    }

    /// inserts field val into current value
    pub fn tta_insert(&mut self, val: u64) {
        // bits 28..28
        self.val = self.val & !(0x1 << 28) | ((val & 0x1) << 28);
    }

    /// extracts field val from current value
    pub fn fpen_extract(&mut self) -> u64 {
        // bits 20..21
        (self.val >> 20) & 0x3
    }

    /// inserts field val into current value
    pub fn fpen_insert(&mut self, val: u64) {
        // bits 20..21
        self.val = self.val & !(0x3 << 20) | ((val & 0x3) << 20);
    }

    /// extracts field val from current value
    pub fn zen_1_extract(&mut self) -> u64 {
        // bits 16..17
        (self.val >> 16) & 0x3
    }

    /// inserts field val into current value
    pub fn zen_1_insert(&mut self, val: u64) {
        // bits 16..17
        self.val = self.val & !(0x3 << 16) | ((val & 0x3) << 16);
    }
}

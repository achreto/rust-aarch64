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
 * Generated on: 2020-10-05T16:30:11.736620
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
 * Register:    Virtualization Multiprocessor ID Register (vmpidr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Holds the value of the Virtualization Multiprocessor ID. This
 * is the value returned by EL1 reads of File:        AArch64-vmpidr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Virtualization Multiprocessor ID Register (vmpidr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, VMPIDR_EL2
        llvm_asm!("mrs $0, vmpidr_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Virtualization Multiprocessor ID Register (vmpidr_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR VMPIDR_EL2, <Xt>
        llvm_asm!("msr vmpidr_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn aff3_read() -> u64 {
    // bits 32..39
    let val = reg_rawrd();
    (val >> 32) & 0xff
}

/// inserts field val into register
pub fn aff3_write(newval: u64) {
    // bits 32..39
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 32) | ((newval & 0xff) << 32));
}

/// reads field val from register
pub fn u_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn u_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn mt_read() -> u64 {
    // bits 24..24
    let val = reg_rawrd();
    (val >> 24) & 0x1
}

/// inserts field val into register
pub fn mt_write(newval: u64) {
    // bits 24..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 24) | ((newval & 0x1) << 24));
}

/// reads field val from register
pub fn aff2_read() -> u64 {
    // bits 16..23
    let val = reg_rawrd();
    (val >> 16) & 0xff
}

/// inserts field val into register
pub fn aff2_write(newval: u64) {
    // bits 16..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 16) | ((newval & 0xff) << 16));
}

/// reads field val from register
pub fn aff1_read() -> u64 {
    // bits 8..15
    let val = reg_rawrd();
    (val >> 8) & 0xff
}

/// inserts field val into register
pub fn aff1_write(newval: u64) {
    // bits 8..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 8) | ((newval & 0xff) << 8));
}

/// reads field val from register
pub fn aff0_read() -> u64 {
    // bits 0..7
    let val = reg_rawrd();
    (val >> 0) & 0xff
}

/// inserts field val into register
pub fn aff0_write(newval: u64) {
    // bits 0..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 0) | ((newval & 0xff) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Virtualization Multiprocessor ID Register value
/// in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register vmpidr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xff41ffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xff41ffffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xff41ffffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 1096323956735;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn aff3_extract(&mut self) -> u64 {
        // bits 32..39
        (self.val >> 32) & 0xff
    }

    /// inserts field val into current value
    pub fn aff3_insert(&mut self, val: u64) {
        // bits 32..39
        self.val = self.val & !(0xff << 32) | ((val & 0xff) << 32);
    }

    /// extracts field val from current value
    pub fn u_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn u_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn mt_extract(&mut self) -> u64 {
        // bits 24..24
        (self.val >> 24) & 0x1
    }

    /// inserts field val into current value
    pub fn mt_insert(&mut self, val: u64) {
        // bits 24..24
        self.val = self.val & !(0x1 << 24) | ((val & 0x1) << 24);
    }

    /// extracts field val from current value
    pub fn aff2_extract(&mut self) -> u64 {
        // bits 16..23
        (self.val >> 16) & 0xff
    }

    /// inserts field val into current value
    pub fn aff2_insert(&mut self, val: u64) {
        // bits 16..23
        self.val = self.val & !(0xff << 16) | ((val & 0xff) << 16);
    }

    /// extracts field val from current value
    pub fn aff1_extract(&mut self) -> u64 {
        // bits 8..15
        (self.val >> 8) & 0xff
    }

    /// inserts field val into current value
    pub fn aff1_insert(&mut self, val: u64) {
        // bits 8..15
        self.val = self.val & !(0xff << 8) | ((val & 0xff) << 8);
    }

    /// extracts field val from current value
    pub fn aff0_extract(&mut self) -> u64 {
        // bits 0..7
        (self.val >> 0) & 0xff
    }

    /// inserts field val into current value
    pub fn aff0_insert(&mut self, val: u64) {
        // bits 0..7
        self.val = self.val & !(0xff << 0) | ((val & 0xff) << 0);
    }
}

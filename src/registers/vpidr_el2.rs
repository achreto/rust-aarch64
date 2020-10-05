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
 * Generated on: 2020-10-05T16:49:32.082258
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
 * Register:    Virtualization Processor ID Register (vpidr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Holds the value of the Virtualization Processor ID. This is the value returned by
 * EL1 reads of File:        AArch64-vpidr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Virtualization Processor ID Register (vpidr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, VPIDR_EL2
        llvm_asm!("mrs $0, vpidr_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Virtualization Processor ID Register (vpidr_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR VPIDR_EL2, <Xt>
        llvm_asm!("msr vpidr_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn implementer_read() -> u64 {
    // bits 24..31
    let val = reg_rawrd();
    (val >> 24) & 0xff
}

/// inserts field val into register
pub fn implementer_write(newval: u64) {
    // bits 24..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 24) | ((newval & 0xff) << 24));
}

/// reads field val from register
pub fn variant_read() -> u64 {
    // bits 20..23
    let val = reg_rawrd();
    (val >> 20) & 0xf
}

/// inserts field val into register
pub fn variant_write(newval: u64) {
    // bits 20..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0xf << 20) | ((newval & 0xf) << 20));
}

/// reads field val from register
pub fn architecture_read() -> u64 {
    // bits 16..19
    let val = reg_rawrd();
    (val >> 16) & 0xf
}

/// inserts field val into register
pub fn architecture_write(newval: u64) {
    // bits 16..19
    let val = reg_rawrd();
    reg_rawwr(val & !(0xf << 16) | ((newval & 0xf) << 16));
}

/// reads field val from register
pub fn partnum_read() -> u64 {
    // bits 4..15
    let val = reg_rawrd();
    (val >> 4) & 0xfff
}

/// inserts field val into register
pub fn partnum_write(newval: u64) {
    // bits 4..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0xfff << 4) | ((newval & 0xfff) << 4));
}

/// reads field val from register
pub fn revision_read() -> u64 {
    // bits 0..3
    let val = reg_rawrd();
    (val >> 0) & 0xf
}

/// inserts field val into register
pub fn revision_write(newval: u64) {
    // bits 0..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0xf << 0) | ((newval & 0xf) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Virtualization Processor ID Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register vpidr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xffffffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xffffffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 4294967295;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn implementer_extract(&mut self) -> u64 {
        // bits 24..31
        (self.val >> 24) & 0xff
    }

    /// inserts field val into current value
    pub fn implementer_insert(&mut self, val: u64) {
        // bits 24..31
        self.val = self.val & !(0xff << 24) | ((val & 0xff) << 24);
    }

    /// extracts field val from current value
    pub fn variant_extract(&mut self) -> u64 {
        // bits 20..23
        (self.val >> 20) & 0xf
    }

    /// inserts field val into current value
    pub fn variant_insert(&mut self, val: u64) {
        // bits 20..23
        self.val = self.val & !(0xf << 20) | ((val & 0xf) << 20);
    }

    /// extracts field val from current value
    pub fn architecture_extract(&mut self) -> u64 {
        // bits 16..19
        (self.val >> 16) & 0xf
    }

    /// inserts field val into current value
    pub fn architecture_insert(&mut self, val: u64) {
        // bits 16..19
        self.val = self.val & !(0xf << 16) | ((val & 0xf) << 16);
    }

    /// extracts field val from current value
    pub fn partnum_extract(&mut self) -> u64 {
        // bits 4..15
        (self.val >> 4) & 0xfff
    }

    /// inserts field val into current value
    pub fn partnum_insert(&mut self, val: u64) {
        // bits 4..15
        self.val = self.val & !(0xfff << 4) | ((val & 0xfff) << 4);
    }

    /// extracts field val from current value
    pub fn revision_extract(&mut self) -> u64 {
        // bits 0..3
        (self.val >> 0) & 0xf
    }

    /// inserts field val into current value
    pub fn revision_insert(&mut self, val: u64) {
        // bits 0..3
        self.val = self.val & !(0xf << 0) | ((val & 0xf) << 0);
    }
}

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
 * Generated on: 2020-10-05T16:49:32.045111
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
 * Register:    Interrupt Controller Alias Software Generated Interrupt Group 1 Register
 * (icc_asgi1r_el1) Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Generates Group 1 SGIs for the Security state that is not the current Security
 * state. File:        AArch64-icc_asgi1r_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// writing the Interrupt Controller Alias Software Generated Interrupt Group 1 Register
/// (icc_asgi1r_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR ICC_ASGI1R_EL1, <Xt>
        llvm_asm!("msr icc_asgi1r_el1, $0" : : "r"(val));
    }
}

// register is not readable. not emitting read accessor

/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



// register is not readable, omitting reading from field

/// inserts field val into register
pub fn aff3_write(newval: u64) {
    // bits 48..55
    reg_rawwr((newval & 0xff) << 48);
}

// register is not readable, omitting reading from field

/// inserts field val into register
pub fn rs_write(newval: u64) {
    // bits 44..47
    reg_rawwr((newval & 0xf) << 44);
}

// register is not readable, omitting reading from field

/// inserts field val into register
pub fn irm_write(newval: u64) {
    // bits 40..40
    reg_rawwr((newval & 0x1) << 40);
}

// register is not readable, omitting reading from field

/// inserts field val into register
pub fn aff2_write(newval: u64) {
    // bits 32..39
    reg_rawwr((newval & 0xff) << 32);
}

// register is not readable, omitting reading from field

/// inserts field val into register
pub fn intid_write(newval: u64) {
    // bits 24..27
    reg_rawwr((newval & 0xf) << 24);
}

// register is not readable, omitting reading from field

/// inserts field val into register
pub fn aff1_write(newval: u64) {
    // bits 16..23
    reg_rawwr((newval & 0xff) << 16);
}

// register is not readable, omitting reading from field

/// inserts field val into register
pub fn targetlist_write(newval: u64) {
    // bits 0..15
    reg_rawwr((newval & 0xffff) << 0);
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Interrupt Controller Alias Software Generated Interrupt Group 1
/// Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register icc_asgi1r_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }

    // no current() method as it is write only
    // no read() method as it is write only

    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xfff1ff0fffffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 72042196848607231;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }


    // no extract() method for field aff3
    /// inserts field val into current value
    pub fn aff3_insert(&mut self, val: u64) {
        // bits 48..55
        self.val = self.val & !(0xff << 48) | ((val & 0xff) << 48);
    }
    // no extract() method for field rs
    /// inserts field val into current value
    pub fn rs_insert(&mut self, val: u64) {
        // bits 44..47
        self.val = self.val & !(0xf << 44) | ((val & 0xf) << 44);
    }
    // no extract() method for field irm
    /// inserts field val into current value
    pub fn irm_insert(&mut self, val: u64) {
        // bits 40..40
        self.val = self.val & !(0x1 << 40) | ((val & 0x1) << 40);
    }
    // no extract() method for field aff2
    /// inserts field val into current value
    pub fn aff2_insert(&mut self, val: u64) {
        // bits 32..39
        self.val = self.val & !(0xff << 32) | ((val & 0xff) << 32);
    }
    // no extract() method for field intid
    /// inserts field val into current value
    pub fn intid_insert(&mut self, val: u64) {
        // bits 24..27
        self.val = self.val & !(0xf << 24) | ((val & 0xf) << 24);
    }
    // no extract() method for field aff1
    /// inserts field val into current value
    pub fn aff1_insert(&mut self, val: u64) {
        // bits 16..23
        self.val = self.val & !(0xff << 16) | ((val & 0xff) << 16);
    }
    // no extract() method for field targetlist
    /// inserts field val into current value
    pub fn targetlist_insert(&mut self, val: u64) {
        // bits 0..15
        self.val = self.val & !(0xffff << 0) | ((val & 0xffff) << 0);
    }
}

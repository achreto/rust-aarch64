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
 * Generated on: 2020-10-05T16:49:32.063437
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
 * Register:    OS Lock Access Register (oslar_el1)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Used to lock or unlock the OS Lock.
 * File:        AArch64-oslar_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// writing the OS Lock Access Register (oslar_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR OSLAR_EL1, <Xt>
        llvm_asm!("msr oslar_el1, $0" : : "r"(val));
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
pub fn oslk_write(newval: u64) {
    // bits 0..0
    reg_rawwr((newval & 0x1) << 0);
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the OS Lock Access Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register oslar_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }

    // no current() method as it is write only
    // no read() method as it is write only

    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x1)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 1;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }


    // no extract() method for field oslk
    /// inserts field val into current value
    pub fn oslk_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

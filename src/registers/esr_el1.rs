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
 * Generated on: 2020-10-05T16:49:32.038060
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
 * Register:    Exception Syndrome Register (EL1) (esr_el1)
 * Group:       Exception and fault handling registers
 * Type:        64-bit Register
 * Description: Holds syndrome information for an exception taken to EL1.
 * File:        AArch64-esr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Exception Syndrome Register (EL1) (esr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ESR_EL1
        llvm_asm!("mrs $0, esr_el1" : "=r"(regval));
    }
    return regval;
}


/// writing the Exception Syndrome Register (EL1) (esr_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR ESR_EL1, <Xt>
        llvm_asm!("msr esr_el1, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn iss2_1_read() -> u64 {
    // bits 32..36
    let val = reg_rawrd();
    (val >> 32) & 0x1f
}

/// inserts field val into register
pub fn iss2_1_write(newval: u64) {
    // bits 32..36
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1f << 32) | ((newval & 0x1f) << 32));
}

/// reads field val from register
pub fn ec_read() -> u64 {
    // bits 26..31
    let val = reg_rawrd();
    (val >> 26) & 0x3f
}

/// inserts field val into register
pub fn ec_write(newval: u64) {
    // bits 26..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3f << 26) | ((newval & 0x3f) << 26));
}

/// reads field val from register
pub fn il_read() -> u64 {
    // bits 25..25
    let val = reg_rawrd();
    (val >> 25) & 0x1
}

/// inserts field val into register
pub fn il_write(newval: u64) {
    // bits 25..25
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 25) | ((newval & 0x1) << 25));
}

/// reads field val from register
pub fn iss_read() -> u64 {
    // bits 0..24
    let val = reg_rawrd();
    (val >> 0) & 0x1ffffff
}

/// inserts field val into register
pub fn iss_write(newval: u64) {
    // bits 0..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1ffffff << 0) | ((newval & 0x1ffffff) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Exception Syndrome Register (EL1) value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register esr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x1fffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x1fffffffff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x1fffffffff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 137438953471;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn iss2_1_extract(&mut self) -> u64 {
        // bits 32..36
        (self.val >> 32) & 0x1f
    }

    /// inserts field val into current value
    pub fn iss2_1_insert(&mut self, val: u64) {
        // bits 32..36
        self.val = self.val & !(0x1f << 32) | ((val & 0x1f) << 32);
    }

    /// extracts field val from current value
    pub fn ec_extract(&mut self) -> u64 {
        // bits 26..31
        (self.val >> 26) & 0x3f
    }

    /// inserts field val into current value
    pub fn ec_insert(&mut self, val: u64) {
        // bits 26..31
        self.val = self.val & !(0x3f << 26) | ((val & 0x3f) << 26);
    }

    /// extracts field val from current value
    pub fn il_extract(&mut self) -> u64 {
        // bits 25..25
        (self.val >> 25) & 0x1
    }

    /// inserts field val into current value
    pub fn il_insert(&mut self, val: u64) {
        // bits 25..25
        self.val = self.val & !(0x1 << 25) | ((val & 0x1) << 25);
    }

    /// extracts field val from current value
    pub fn iss_extract(&mut self) -> u64 {
        // bits 0..24
        (self.val >> 0) & 0x1ffffff
    }

    /// inserts field val into current value
    pub fn iss_insert(&mut self, val: u64) {
        // bits 0..24
        self.val = self.val & !(0x1ffffff << 0) | ((val & 0x1ffffff) << 0);
    }
}

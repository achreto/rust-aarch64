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
 * Generated on: 2020-10-05T16:49:32.063970
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
 * Register:    Physical Address Register (par_el1)
 * Group:       Address translation instructions
 * Type:        64-bit Register
 * Description: Returns the output address (OA) from an Address translation instruction that executed successfully, or fault information if the instruction did not execute successfully.
 * File:        AArch64-par_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Physical Address Register (par_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, PAR_EL1
        llvm_asm!("mrs $0, par_el1" : "=r"(regval));
    }
    return regval;
}


/// writing the Physical Address Register (par_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR PAR_EL1, <Xt>
        llvm_asm!("msr par_el1, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn implementation_defined_63_56_read() -> u64 {
    // bits 56..63
    let val = reg_rawrd();
    (val >> 56) & 0xff
}

/// inserts field val into register
pub fn implementation_defined_63_56_write(newval: u64) {
    // bits 56..63
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 56) | ((newval & 0xff) << 56));
}

/// reads field val from register
pub fn implementation_defined_55_52_read() -> u64 {
    // bits 52..55
    let val = reg_rawrd();
    (val >> 52) & 0xf
}

/// inserts field val into register
pub fn implementation_defined_55_52_write(newval: u64) {
    // bits 52..55
    let val = reg_rawrd();
    reg_rawwr(val & !(0xf << 52) | ((newval & 0xf) << 52));
}

/// reads field val from register
pub fn implementation_defined_51_48_read() -> u64 {
    // bits 48..51
    let val = reg_rawrd();
    (val >> 48) & 0xf
}

/// inserts field val into register
pub fn implementation_defined_51_48_write(newval: u64) {
    // bits 48..51
    let val = reg_rawrd();
    reg_rawwr(val & !(0xf << 48) | ((newval & 0xf) << 48));
}

/// reads field val from register
pub fn s_read() -> u64 {
    // bits 9..9
    let val = reg_rawrd();
    (val >> 9) & 0x1
}

/// inserts field val into register
pub fn s_write(newval: u64) {
    // bits 9..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 9) | ((newval & 0x1) << 9));
}

/// reads field val from register
pub fn ptw_read() -> u64 {
    // bits 8..8
    let val = reg_rawrd();
    (val >> 8) & 0x1
}

/// inserts field val into register
pub fn ptw_write(newval: u64) {
    // bits 8..8
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 8) | ((newval & 0x1) << 8));
}

/// reads field val from register
pub fn fst_read() -> u64 {
    // bits 1..6
    let val = reg_rawrd();
    (val >> 1) & 0x3f
}

/// inserts field val into register
pub fn fst_write(newval: u64) {
    // bits 1..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3f << 1) | ((newval & 0x3f) << 1));
}

/// reads field val from register
pub fn f_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn f_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Physical Address Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register par_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xffff00000000037f;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xffff00000000037f
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xffff00000000037f)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 18446462598732841855;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn implementation_defined_63_56_extract(&mut self) -> u64 {
        // bits 56..63
        (self.val >> 56) & 0xff
    }

    /// inserts field val into current value
    pub fn implementation_defined_63_56_insert(&mut self, val: u64) {
        // bits 56..63
        self.val = self.val & !(0xff << 56) | ((val & 0xff) << 56);
    }

    /// extracts field val from current value
    pub fn implementation_defined_55_52_extract(&mut self) -> u64 {
        // bits 52..55
        (self.val >> 52) & 0xf
    }

    /// inserts field val into current value
    pub fn implementation_defined_55_52_insert(&mut self, val: u64) {
        // bits 52..55
        self.val = self.val & !(0xf << 52) | ((val & 0xf) << 52);
    }

    /// extracts field val from current value
    pub fn implementation_defined_51_48_extract(&mut self) -> u64 {
        // bits 48..51
        (self.val >> 48) & 0xf
    }

    /// inserts field val into current value
    pub fn implementation_defined_51_48_insert(&mut self, val: u64) {
        // bits 48..51
        self.val = self.val & !(0xf << 48) | ((val & 0xf) << 48);
    }

    /// extracts field val from current value
    pub fn s_extract(&mut self) -> u64 {
        // bits 9..9
        (self.val >> 9) & 0x1
    }

    /// inserts field val into current value
    pub fn s_insert(&mut self, val: u64) {
        // bits 9..9
        self.val = self.val & !(0x1 << 9) | ((val & 0x1) << 9);
    }

    /// extracts field val from current value
    pub fn ptw_extract(&mut self) -> u64 {
        // bits 8..8
        (self.val >> 8) & 0x1
    }

    /// inserts field val into current value
    pub fn ptw_insert(&mut self, val: u64) {
        // bits 8..8
        self.val = self.val & !(0x1 << 8) | ((val & 0x1) << 8);
    }

    /// extracts field val from current value
    pub fn fst_extract(&mut self) -> u64 {
        // bits 1..6
        (self.val >> 1) & 0x3f
    }

    /// inserts field val into current value
    pub fn fst_insert(&mut self, val: u64) {
        // bits 1..6
        self.val = self.val & !(0x3f << 1) | ((val & 0x3f) << 1);
    }

    /// extracts field val from current value
    pub fn f_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn f_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

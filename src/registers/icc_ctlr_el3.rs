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
 * Generated on: 2020-10-05T16:49:32.045751
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
 * Register:    Interrupt Controller Control Register (EL3) (icc_ctlr_el3)
 * Group:       GIC control registers
 * Type:        64-bit Register
 * Description: Controls aspects of the behavior of the GIC CPU interface and provides
 * information about the features implemented. File:        AArch64-icc_ctlr_el3.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Interrupt Controller Control Register (EL3) (icc_ctlr_el3) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ICC_CTLR_EL3
        llvm_asm!("mrs $0, icc_ctlr_el3" : "=r"(regval));
    }
    return regval;
}


/// writing the Interrupt Controller Control Register (EL3) (icc_ctlr_el3) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR ICC_CTLR_EL3, <Xt>
        llvm_asm!("msr icc_ctlr_el3, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn extrange_read() -> u64 {
    // bits 19..19
    let val = reg_rawrd();
    (val >> 19) & 0x1
}

/// inserts field val into register
pub fn extrange_write(newval: u64) {
    // bits 19..19
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 19) | ((newval & 0x1) << 19));
}

/// reads field val from register
pub fn rss_read() -> u64 {
    // bits 18..18
    let val = reg_rawrd();
    (val >> 18) & 0x1
}

/// inserts field val into register
pub fn rss_write(newval: u64) {
    // bits 18..18
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 18) | ((newval & 0x1) << 18));
}

/// reads field val from register
pub fn nds_read() -> u64 {
    // bits 17..17
    let val = reg_rawrd();
    (val >> 17) & 0x1
}

/// inserts field val into register
pub fn nds_write(newval: u64) {
    // bits 17..17
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 17) | ((newval & 0x1) << 17));
}

/// reads field val from register
pub fn a3v_read() -> u64 {
    // bits 15..15
    let val = reg_rawrd();
    (val >> 15) & 0x1
}

/// inserts field val into register
pub fn a3v_write(newval: u64) {
    // bits 15..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 15) | ((newval & 0x1) << 15));
}

/// reads field val from register
pub fn seis_read() -> u64 {
    // bits 14..14
    let val = reg_rawrd();
    (val >> 14) & 0x1
}

/// inserts field val into register
pub fn seis_write(newval: u64) {
    // bits 14..14
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 14) | ((newval & 0x1) << 14));
}

/// reads field val from register
pub fn idbits_read() -> u64 {
    // bits 11..13
    let val = reg_rawrd();
    (val >> 11) & 0x7
}

/// inserts field val into register
pub fn idbits_write(newval: u64) {
    // bits 11..13
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7 << 11) | ((newval & 0x7) << 11));
}

/// reads field val from register
pub fn pribits_read() -> u64 {
    // bits 8..10
    let val = reg_rawrd();
    (val >> 8) & 0x7
}

/// inserts field val into register
pub fn pribits_write(newval: u64) {
    // bits 8..10
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7 << 8) | ((newval & 0x7) << 8));
}

/// reads field val from register
pub fn pmhe_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

/// inserts field val into register
pub fn pmhe_write(newval: u64) {
    // bits 6..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 6) | ((newval & 0x1) << 6));
}

/// reads field val from register
pub fn rm_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn rm_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn eoimode_el1ns_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn eoimode_el1ns_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn eoimode_el1s_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

/// inserts field val into register
pub fn eoimode_el1s_write(newval: u64) {
    // bits 3..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 3) | ((newval & 0x1) << 3));
}

/// reads field val from register
pub fn eoimode_el3_read() -> u64 {
    // bits 2..2
    let val = reg_rawrd();
    (val >> 2) & 0x1
}

/// inserts field val into register
pub fn eoimode_el3_write(newval: u64) {
    // bits 2..2
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 2) | ((newval & 0x1) << 2));
}

/// reads field val from register
pub fn cbpr_el1ns_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn cbpr_el1ns_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}

/// reads field val from register
pub fn cbpr_el1s_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn cbpr_el1s_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Interrupt Controller Control Register (EL3) value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register icc_ctlr_el3
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xeff7f;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xeff7f
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xeff7f)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 982911;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn extrange_extract(&mut self) -> u64 {
        // bits 19..19
        (self.val >> 19) & 0x1
    }

    /// inserts field val into current value
    pub fn extrange_insert(&mut self, val: u64) {
        // bits 19..19
        self.val = self.val & !(0x1 << 19) | ((val & 0x1) << 19);
    }

    /// extracts field val from current value
    pub fn rss_extract(&mut self) -> u64 {
        // bits 18..18
        (self.val >> 18) & 0x1
    }

    /// inserts field val into current value
    pub fn rss_insert(&mut self, val: u64) {
        // bits 18..18
        self.val = self.val & !(0x1 << 18) | ((val & 0x1) << 18);
    }

    /// extracts field val from current value
    pub fn nds_extract(&mut self) -> u64 {
        // bits 17..17
        (self.val >> 17) & 0x1
    }

    /// inserts field val into current value
    pub fn nds_insert(&mut self, val: u64) {
        // bits 17..17
        self.val = self.val & !(0x1 << 17) | ((val & 0x1) << 17);
    }

    /// extracts field val from current value
    pub fn a3v_extract(&mut self) -> u64 {
        // bits 15..15
        (self.val >> 15) & 0x1
    }

    /// inserts field val into current value
    pub fn a3v_insert(&mut self, val: u64) {
        // bits 15..15
        self.val = self.val & !(0x1 << 15) | ((val & 0x1) << 15);
    }

    /// extracts field val from current value
    pub fn seis_extract(&mut self) -> u64 {
        // bits 14..14
        (self.val >> 14) & 0x1
    }

    /// inserts field val into current value
    pub fn seis_insert(&mut self, val: u64) {
        // bits 14..14
        self.val = self.val & !(0x1 << 14) | ((val & 0x1) << 14);
    }

    /// extracts field val from current value
    pub fn idbits_extract(&mut self) -> u64 {
        // bits 11..13
        (self.val >> 11) & 0x7
    }

    /// inserts field val into current value
    pub fn idbits_insert(&mut self, val: u64) {
        // bits 11..13
        self.val = self.val & !(0x7 << 11) | ((val & 0x7) << 11);
    }

    /// extracts field val from current value
    pub fn pribits_extract(&mut self) -> u64 {
        // bits 8..10
        (self.val >> 8) & 0x7
    }

    /// inserts field val into current value
    pub fn pribits_insert(&mut self, val: u64) {
        // bits 8..10
        self.val = self.val & !(0x7 << 8) | ((val & 0x7) << 8);
    }

    /// extracts field val from current value
    pub fn pmhe_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }

    /// inserts field val into current value
    pub fn pmhe_insert(&mut self, val: u64) {
        // bits 6..6
        self.val = self.val & !(0x1 << 6) | ((val & 0x1) << 6);
    }

    /// extracts field val from current value
    pub fn rm_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn rm_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn eoimode_el1ns_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn eoimode_el1ns_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn eoimode_el1s_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }

    /// inserts field val into current value
    pub fn eoimode_el1s_insert(&mut self, val: u64) {
        // bits 3..3
        self.val = self.val & !(0x1 << 3) | ((val & 0x1) << 3);
    }

    /// extracts field val from current value
    pub fn eoimode_el3_extract(&mut self) -> u64 {
        // bits 2..2
        (self.val >> 2) & 0x1
    }

    /// inserts field val into current value
    pub fn eoimode_el3_insert(&mut self, val: u64) {
        // bits 2..2
        self.val = self.val & !(0x1 << 2) | ((val & 0x1) << 2);
    }

    /// extracts field val from current value
    pub fn cbpr_el1ns_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn cbpr_el1ns_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }

    /// extracts field val from current value
    pub fn cbpr_el1s_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn cbpr_el1s_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

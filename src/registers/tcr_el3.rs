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
 * Generated on: 2020-10-05T16:49:32.078465
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
 * Register:    Translation Control Register (EL3) (tcr_el3)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: The control register for stage 1 of the EL3 translation regime.
 * File:        AArch64-tcr_el3.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Translation Control Register (EL3) (tcr_el3) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, TCR_EL3
        llvm_asm!("mrs $0, tcr_el3" : "=r"(regval));
    }
    return regval;
}


/// writing the Translation Control Register (EL3) (tcr_el3) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR TCR_EL3, <Xt>
        llvm_asm!("msr tcr_el3, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn ds_1_read() -> u64 {
    // bits 32..32
    let val = reg_rawrd();
    (val >> 32) & 0x1
}

/// inserts field val into register
pub fn ds_1_write(newval: u64) {
    // bits 32..32
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 32) | ((newval & 0x1) << 32));
}

/// reads field val from register
pub fn tcma_1_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn tcma_1_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn tbid_1_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

/// inserts field val into register
pub fn tbid_1_write(newval: u64) {
    // bits 29..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 29) | ((newval & 0x1) << 29));
}

/// reads field val from register
pub fn hwu62_1_read() -> u64 {
    // bits 28..28
    let val = reg_rawrd();
    (val >> 28) & 0x1
}

/// inserts field val into register
pub fn hwu62_1_write(newval: u64) {
    // bits 28..28
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 28) | ((newval & 0x1) << 28));
}

/// reads field val from register
pub fn hwu61_1_read() -> u64 {
    // bits 27..27
    let val = reg_rawrd();
    (val >> 27) & 0x1
}

/// inserts field val into register
pub fn hwu61_1_write(newval: u64) {
    // bits 27..27
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 27) | ((newval & 0x1) << 27));
}

/// reads field val from register
pub fn hwu60_1_read() -> u64 {
    // bits 26..26
    let val = reg_rawrd();
    (val >> 26) & 0x1
}

/// inserts field val into register
pub fn hwu60_1_write(newval: u64) {
    // bits 26..26
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 26) | ((newval & 0x1) << 26));
}

/// reads field val from register
pub fn hwu59_1_read() -> u64 {
    // bits 25..25
    let val = reg_rawrd();
    (val >> 25) & 0x1
}

/// inserts field val into register
pub fn hwu59_1_write(newval: u64) {
    // bits 25..25
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 25) | ((newval & 0x1) << 25));
}

/// reads field val from register
pub fn hpd_1_read() -> u64 {
    // bits 24..24
    let val = reg_rawrd();
    (val >> 24) & 0x1
}

/// inserts field val into register
pub fn hpd_1_write(newval: u64) {
    // bits 24..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 24) | ((newval & 0x1) << 24));
}

/// reads field val from register
pub fn hd_1_read() -> u64 {
    // bits 22..22
    let val = reg_rawrd();
    (val >> 22) & 0x1
}

/// inserts field val into register
pub fn hd_1_write(newval: u64) {
    // bits 22..22
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 22) | ((newval & 0x1) << 22));
}

/// reads field val from register
pub fn ha_1_read() -> u64 {
    // bits 21..21
    let val = reg_rawrd();
    (val >> 21) & 0x1
}

/// inserts field val into register
pub fn ha_1_write(newval: u64) {
    // bits 21..21
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 21) | ((newval & 0x1) << 21));
}

/// reads field val from register
pub fn tbi_read() -> u64 {
    // bits 20..20
    let val = reg_rawrd();
    (val >> 20) & 0x1
}

/// inserts field val into register
pub fn tbi_write(newval: u64) {
    // bits 20..20
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 20) | ((newval & 0x1) << 20));
}

/// reads field val from register
pub fn ps_read() -> u64 {
    // bits 16..18
    let val = reg_rawrd();
    (val >> 16) & 0x7
}

/// inserts field val into register
pub fn ps_write(newval: u64) {
    // bits 16..18
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7 << 16) | ((newval & 0x7) << 16));
}

/// reads field val from register
pub fn tg0_read() -> u64 {
    // bits 14..15
    let val = reg_rawrd();
    (val >> 14) & 0x3
}

/// inserts field val into register
pub fn tg0_write(newval: u64) {
    // bits 14..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 14) | ((newval & 0x3) << 14));
}

/// reads field val from register
pub fn sh0_read() -> u64 {
    // bits 12..13
    let val = reg_rawrd();
    (val >> 12) & 0x3
}

/// inserts field val into register
pub fn sh0_write(newval: u64) {
    // bits 12..13
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 12) | ((newval & 0x3) << 12));
}

/// reads field val from register
pub fn orgn0_read() -> u64 {
    // bits 10..11
    let val = reg_rawrd();
    (val >> 10) & 0x3
}

/// inserts field val into register
pub fn orgn0_write(newval: u64) {
    // bits 10..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 10) | ((newval & 0x3) << 10));
}

/// reads field val from register
pub fn irgn0_read() -> u64 {
    // bits 8..9
    let val = reg_rawrd();
    (val >> 8) & 0x3
}

/// inserts field val into register
pub fn irgn0_write(newval: u64) {
    // bits 8..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 8) | ((newval & 0x3) << 8));
}

/// reads field val from register
pub fn t0sz_read() -> u64 {
    // bits 0..5
    let val = reg_rawrd();
    (val >> 0) & 0x3f
}

/// inserts field val into register
pub fn t0sz_write(newval: u64) {
    // bits 0..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3f << 0) | ((newval & 0x3f) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Translation Control Register (EL3) value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register tcr_el3
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x17f77ff3f;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x17f77ff3f
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x17f77ff3f)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 6433537855;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn ds_1_extract(&mut self) -> u64 {
        // bits 32..32
        (self.val >> 32) & 0x1
    }

    /// inserts field val into current value
    pub fn ds_1_insert(&mut self, val: u64) {
        // bits 32..32
        self.val = self.val & !(0x1 << 32) | ((val & 0x1) << 32);
    }

    /// extracts field val from current value
    pub fn tcma_1_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn tcma_1_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn tbid_1_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }

    /// inserts field val into current value
    pub fn tbid_1_insert(&mut self, val: u64) {
        // bits 29..29
        self.val = self.val & !(0x1 << 29) | ((val & 0x1) << 29);
    }

    /// extracts field val from current value
    pub fn hwu62_1_extract(&mut self) -> u64 {
        // bits 28..28
        (self.val >> 28) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu62_1_insert(&mut self, val: u64) {
        // bits 28..28
        self.val = self.val & !(0x1 << 28) | ((val & 0x1) << 28);
    }

    /// extracts field val from current value
    pub fn hwu61_1_extract(&mut self) -> u64 {
        // bits 27..27
        (self.val >> 27) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu61_1_insert(&mut self, val: u64) {
        // bits 27..27
        self.val = self.val & !(0x1 << 27) | ((val & 0x1) << 27);
    }

    /// extracts field val from current value
    pub fn hwu60_1_extract(&mut self) -> u64 {
        // bits 26..26
        (self.val >> 26) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu60_1_insert(&mut self, val: u64) {
        // bits 26..26
        self.val = self.val & !(0x1 << 26) | ((val & 0x1) << 26);
    }

    /// extracts field val from current value
    pub fn hwu59_1_extract(&mut self) -> u64 {
        // bits 25..25
        (self.val >> 25) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu59_1_insert(&mut self, val: u64) {
        // bits 25..25
        self.val = self.val & !(0x1 << 25) | ((val & 0x1) << 25);
    }

    /// extracts field val from current value
    pub fn hpd_1_extract(&mut self) -> u64 {
        // bits 24..24
        (self.val >> 24) & 0x1
    }

    /// inserts field val into current value
    pub fn hpd_1_insert(&mut self, val: u64) {
        // bits 24..24
        self.val = self.val & !(0x1 << 24) | ((val & 0x1) << 24);
    }

    /// extracts field val from current value
    pub fn hd_1_extract(&mut self) -> u64 {
        // bits 22..22
        (self.val >> 22) & 0x1
    }

    /// inserts field val into current value
    pub fn hd_1_insert(&mut self, val: u64) {
        // bits 22..22
        self.val = self.val & !(0x1 << 22) | ((val & 0x1) << 22);
    }

    /// extracts field val from current value
    pub fn ha_1_extract(&mut self) -> u64 {
        // bits 21..21
        (self.val >> 21) & 0x1
    }

    /// inserts field val into current value
    pub fn ha_1_insert(&mut self, val: u64) {
        // bits 21..21
        self.val = self.val & !(0x1 << 21) | ((val & 0x1) << 21);
    }

    /// extracts field val from current value
    pub fn tbi_extract(&mut self) -> u64 {
        // bits 20..20
        (self.val >> 20) & 0x1
    }

    /// inserts field val into current value
    pub fn tbi_insert(&mut self, val: u64) {
        // bits 20..20
        self.val = self.val & !(0x1 << 20) | ((val & 0x1) << 20);
    }

    /// extracts field val from current value
    pub fn ps_extract(&mut self) -> u64 {
        // bits 16..18
        (self.val >> 16) & 0x7
    }

    /// inserts field val into current value
    pub fn ps_insert(&mut self, val: u64) {
        // bits 16..18
        self.val = self.val & !(0x7 << 16) | ((val & 0x7) << 16);
    }

    /// extracts field val from current value
    pub fn tg0_extract(&mut self) -> u64 {
        // bits 14..15
        (self.val >> 14) & 0x3
    }

    /// inserts field val into current value
    pub fn tg0_insert(&mut self, val: u64) {
        // bits 14..15
        self.val = self.val & !(0x3 << 14) | ((val & 0x3) << 14);
    }

    /// extracts field val from current value
    pub fn sh0_extract(&mut self) -> u64 {
        // bits 12..13
        (self.val >> 12) & 0x3
    }

    /// inserts field val into current value
    pub fn sh0_insert(&mut self, val: u64) {
        // bits 12..13
        self.val = self.val & !(0x3 << 12) | ((val & 0x3) << 12);
    }

    /// extracts field val from current value
    pub fn orgn0_extract(&mut self) -> u64 {
        // bits 10..11
        (self.val >> 10) & 0x3
    }

    /// inserts field val into current value
    pub fn orgn0_insert(&mut self, val: u64) {
        // bits 10..11
        self.val = self.val & !(0x3 << 10) | ((val & 0x3) << 10);
    }

    /// extracts field val from current value
    pub fn irgn0_extract(&mut self) -> u64 {
        // bits 8..9
        (self.val >> 8) & 0x3
    }

    /// inserts field val into current value
    pub fn irgn0_insert(&mut self, val: u64) {
        // bits 8..9
        self.val = self.val & !(0x3 << 8) | ((val & 0x3) << 8);
    }

    /// extracts field val from current value
    pub fn t0sz_extract(&mut self) -> u64 {
        // bits 0..5
        (self.val >> 0) & 0x3f
    }

    /// inserts field val into current value
    pub fn t0sz_insert(&mut self, val: u64) {
        // bits 0..5
        self.val = self.val & !(0x3f << 0) | ((val & 0x3f) << 0);
    }
}

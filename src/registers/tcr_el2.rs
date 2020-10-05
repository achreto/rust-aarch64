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
 * Generated on: 2020-10-05T16:30:11.731843
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
 * Register:    Translation Control Register (EL2) (tcr_el2)
 * Group:       Virtual memory control registers
 * Type:        64-bit Register
 * Description: The control register for stage 1 of the EL2, or EL2&0,
 * translation regime: File:        AArch64-tcr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Translation Control Register (EL2) (tcr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, TCR_EL2
        llvm_asm!("mrs $0, tcr_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Translation Control Register (EL2) (tcr_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR TCR_EL2, <Xt>
        llvm_asm!("msr tcr_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn ds_1_read() -> u64 {
    // bits 59..59
    let val = reg_rawrd();
    (val >> 59) & 0x1
}

/// inserts field val into register
pub fn ds_1_write(newval: u64) {
    // bits 59..59
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 59) | ((newval & 0x1) << 59));
}

/// reads field val from register
pub fn tcma1_1_read() -> u64 {
    // bits 58..58
    let val = reg_rawrd();
    (val >> 58) & 0x1
}

/// inserts field val into register
pub fn tcma1_1_write(newval: u64) {
    // bits 58..58
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 58) | ((newval & 0x1) << 58));
}

/// reads field val from register
pub fn tcma0_1_read() -> u64 {
    // bits 57..57
    let val = reg_rawrd();
    (val >> 57) & 0x1
}

/// inserts field val into register
pub fn tcma0_1_write(newval: u64) {
    // bits 57..57
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 57) | ((newval & 0x1) << 57));
}

/// reads field val from register
pub fn e0pd1_1_read() -> u64 {
    // bits 56..56
    let val = reg_rawrd();
    (val >> 56) & 0x1
}

/// inserts field val into register
pub fn e0pd1_1_write(newval: u64) {
    // bits 56..56
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 56) | ((newval & 0x1) << 56));
}

/// reads field val from register
pub fn e0pd0_1_read() -> u64 {
    // bits 55..55
    let val = reg_rawrd();
    (val >> 55) & 0x1
}

/// inserts field val into register
pub fn e0pd0_1_write(newval: u64) {
    // bits 55..55
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 55) | ((newval & 0x1) << 55));
}

/// reads field val from register
pub fn nfd1_1_read() -> u64 {
    // bits 54..54
    let val = reg_rawrd();
    (val >> 54) & 0x1
}

/// inserts field val into register
pub fn nfd1_1_write(newval: u64) {
    // bits 54..54
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 54) | ((newval & 0x1) << 54));
}

/// reads field val from register
pub fn nfd0_1_read() -> u64 {
    // bits 53..53
    let val = reg_rawrd();
    (val >> 53) & 0x1
}

/// inserts field val into register
pub fn nfd0_1_write(newval: u64) {
    // bits 53..53
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 53) | ((newval & 0x1) << 53));
}

/// reads field val from register
pub fn tbid1_1_read() -> u64 {
    // bits 52..52
    let val = reg_rawrd();
    (val >> 52) & 0x1
}

/// inserts field val into register
pub fn tbid1_1_write(newval: u64) {
    // bits 52..52
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 52) | ((newval & 0x1) << 52));
}

/// reads field val from register
pub fn tbid0_1_read() -> u64 {
    // bits 51..51
    let val = reg_rawrd();
    (val >> 51) & 0x1
}

/// inserts field val into register
pub fn tbid0_1_write(newval: u64) {
    // bits 51..51
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 51) | ((newval & 0x1) << 51));
}

/// reads field val from register
pub fn hwu162_1_read() -> u64 {
    // bits 50..50
    let val = reg_rawrd();
    (val >> 50) & 0x1
}

/// inserts field val into register
pub fn hwu162_1_write(newval: u64) {
    // bits 50..50
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 50) | ((newval & 0x1) << 50));
}

/// reads field val from register
pub fn hwu161_1_read() -> u64 {
    // bits 49..49
    let val = reg_rawrd();
    (val >> 49) & 0x1
}

/// inserts field val into register
pub fn hwu161_1_write(newval: u64) {
    // bits 49..49
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 49) | ((newval & 0x1) << 49));
}

/// reads field val from register
pub fn hwu160_1_read() -> u64 {
    // bits 48..48
    let val = reg_rawrd();
    (val >> 48) & 0x1
}

/// inserts field val into register
pub fn hwu160_1_write(newval: u64) {
    // bits 48..48
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 48) | ((newval & 0x1) << 48));
}

/// reads field val from register
pub fn hwu159_1_read() -> u64 {
    // bits 47..47
    let val = reg_rawrd();
    (val >> 47) & 0x1
}

/// inserts field val into register
pub fn hwu159_1_write(newval: u64) {
    // bits 47..47
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 47) | ((newval & 0x1) << 47));
}

/// reads field val from register
pub fn hwu062_1_read() -> u64 {
    // bits 46..46
    let val = reg_rawrd();
    (val >> 46) & 0x1
}

/// inserts field val into register
pub fn hwu062_1_write(newval: u64) {
    // bits 46..46
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 46) | ((newval & 0x1) << 46));
}

/// reads field val from register
pub fn hwu061_1_read() -> u64 {
    // bits 45..45
    let val = reg_rawrd();
    (val >> 45) & 0x1
}

/// inserts field val into register
pub fn hwu061_1_write(newval: u64) {
    // bits 45..45
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 45) | ((newval & 0x1) << 45));
}

/// reads field val from register
pub fn hwu060_1_read() -> u64 {
    // bits 44..44
    let val = reg_rawrd();
    (val >> 44) & 0x1
}

/// inserts field val into register
pub fn hwu060_1_write(newval: u64) {
    // bits 44..44
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 44) | ((newval & 0x1) << 44));
}

/// reads field val from register
pub fn hwu059_1_read() -> u64 {
    // bits 43..43
    let val = reg_rawrd();
    (val >> 43) & 0x1
}

/// inserts field val into register
pub fn hwu059_1_write(newval: u64) {
    // bits 43..43
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 43) | ((newval & 0x1) << 43));
}

/// reads field val from register
pub fn hpd1_1_read() -> u64 {
    // bits 42..42
    let val = reg_rawrd();
    (val >> 42) & 0x1
}

/// inserts field val into register
pub fn hpd1_1_write(newval: u64) {
    // bits 42..42
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 42) | ((newval & 0x1) << 42));
}

/// reads field val from register
pub fn hpd0_1_read() -> u64 {
    // bits 41..41
    let val = reg_rawrd();
    (val >> 41) & 0x1
}

/// inserts field val into register
pub fn hpd0_1_write(newval: u64) {
    // bits 41..41
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 41) | ((newval & 0x1) << 41));
}

/// reads field val from register
pub fn hd_1_read() -> u64 {
    // bits 40..40
    let val = reg_rawrd();
    (val >> 40) & 0x1
}

/// inserts field val into register
pub fn hd_1_write(newval: u64) {
    // bits 40..40
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 40) | ((newval & 0x1) << 40));
}

/// reads field val from register
pub fn ha_1_read() -> u64 {
    // bits 39..39
    let val = reg_rawrd();
    (val >> 39) & 0x1
}

/// inserts field val into register
pub fn ha_1_write(newval: u64) {
    // bits 39..39
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 39) | ((newval & 0x1) << 39));
}

/// reads field val from register
pub fn tbi1_read() -> u64 {
    // bits 38..38
    let val = reg_rawrd();
    (val >> 38) & 0x1
}

/// inserts field val into register
pub fn tbi1_write(newval: u64) {
    // bits 38..38
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 38) | ((newval & 0x1) << 38));
}

/// reads field val from register
pub fn tbi0_read() -> u64 {
    // bits 37..37
    let val = reg_rawrd();
    (val >> 37) & 0x1
}

/// inserts field val into register
pub fn tbi0_write(newval: u64) {
    // bits 37..37
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 37) | ((newval & 0x1) << 37));
}

/// reads field val from register
pub fn as_read() -> u64 {
    // bits 36..36
    let val = reg_rawrd();
    (val >> 36) & 0x1
}

/// inserts field val into register
pub fn as_write(newval: u64) {
    // bits 36..36
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 36) | ((newval & 0x1) << 36));
}

/// reads field val from register
pub fn ips_read() -> u64 {
    // bits 32..34
    let val = reg_rawrd();
    (val >> 32) & 0x7
}

/// inserts field val into register
pub fn ips_write(newval: u64) {
    // bits 32..34
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7 << 32) | ((newval & 0x7) << 32));
}

/// reads field val from register
pub fn tg1_read() -> u64 {
    // bits 30..31
    let val = reg_rawrd();
    (val >> 30) & 0x3
}

/// inserts field val into register
pub fn tg1_write(newval: u64) {
    // bits 30..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 30) | ((newval & 0x3) << 30));
}

/// reads field val from register
pub fn sh1_read() -> u64 {
    // bits 28..29
    let val = reg_rawrd();
    (val >> 28) & 0x3
}

/// inserts field val into register
pub fn sh1_write(newval: u64) {
    // bits 28..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 28) | ((newval & 0x3) << 28));
}

/// reads field val from register
pub fn orgn1_read() -> u64 {
    // bits 26..27
    let val = reg_rawrd();
    (val >> 26) & 0x3
}

/// inserts field val into register
pub fn orgn1_write(newval: u64) {
    // bits 26..27
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 26) | ((newval & 0x3) << 26));
}

/// reads field val from register
pub fn irgn1_read() -> u64 {
    // bits 24..25
    let val = reg_rawrd();
    (val >> 24) & 0x3
}

/// inserts field val into register
pub fn irgn1_write(newval: u64) {
    // bits 24..25
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 24) | ((newval & 0x3) << 24));
}

/// reads field val from register
pub fn epd1_read() -> u64 {
    // bits 23..23
    let val = reg_rawrd();
    (val >> 23) & 0x1
}

/// inserts field val into register
pub fn epd1_write(newval: u64) {
    // bits 23..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 23) | ((newval & 0x1) << 23));
}

/// reads field val from register
pub fn a1_read() -> u64 {
    // bits 22..22
    let val = reg_rawrd();
    (val >> 22) & 0x1
}

/// inserts field val into register
pub fn a1_write(newval: u64) {
    // bits 22..22
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 22) | ((newval & 0x1) << 22));
}

/// reads field val from register
pub fn t1sz_read() -> u64 {
    // bits 16..21
    let val = reg_rawrd();
    (val >> 16) & 0x3f
}

/// inserts field val into register
pub fn t1sz_write(newval: u64) {
    // bits 16..21
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3f << 16) | ((newval & 0x3f) << 16));
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
pub fn epd0_read() -> u64 {
    // bits 7..7
    let val = reg_rawrd();
    (val >> 7) & 0x1
}

/// inserts field val into register
pub fn epd0_write(newval: u64) {
    // bits 7..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 7) | ((newval & 0x1) << 7));
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



/// struct holding a copy of the Translation Control Register (EL2) value in
/// memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register tcr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xffffff7ffffffbf;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xffffff7ffffffbf
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xffffff7ffffffbf)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 1152921470247108543;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn ds_1_extract(&mut self) -> u64 {
        // bits 59..59
        (self.val >> 59) & 0x1
    }

    /// inserts field val into current value
    pub fn ds_1_insert(&mut self, val: u64) {
        // bits 59..59
        self.val = self.val & !(0x1 << 59) | ((val & 0x1) << 59);
    }

    /// extracts field val from current value
    pub fn tcma1_1_extract(&mut self) -> u64 {
        // bits 58..58
        (self.val >> 58) & 0x1
    }

    /// inserts field val into current value
    pub fn tcma1_1_insert(&mut self, val: u64) {
        // bits 58..58
        self.val = self.val & !(0x1 << 58) | ((val & 0x1) << 58);
    }

    /// extracts field val from current value
    pub fn tcma0_1_extract(&mut self) -> u64 {
        // bits 57..57
        (self.val >> 57) & 0x1
    }

    /// inserts field val into current value
    pub fn tcma0_1_insert(&mut self, val: u64) {
        // bits 57..57
        self.val = self.val & !(0x1 << 57) | ((val & 0x1) << 57);
    }

    /// extracts field val from current value
    pub fn e0pd1_1_extract(&mut self) -> u64 {
        // bits 56..56
        (self.val >> 56) & 0x1
    }

    /// inserts field val into current value
    pub fn e0pd1_1_insert(&mut self, val: u64) {
        // bits 56..56
        self.val = self.val & !(0x1 << 56) | ((val & 0x1) << 56);
    }

    /// extracts field val from current value
    pub fn e0pd0_1_extract(&mut self) -> u64 {
        // bits 55..55
        (self.val >> 55) & 0x1
    }

    /// inserts field val into current value
    pub fn e0pd0_1_insert(&mut self, val: u64) {
        // bits 55..55
        self.val = self.val & !(0x1 << 55) | ((val & 0x1) << 55);
    }

    /// extracts field val from current value
    pub fn nfd1_1_extract(&mut self) -> u64 {
        // bits 54..54
        (self.val >> 54) & 0x1
    }

    /// inserts field val into current value
    pub fn nfd1_1_insert(&mut self, val: u64) {
        // bits 54..54
        self.val = self.val & !(0x1 << 54) | ((val & 0x1) << 54);
    }

    /// extracts field val from current value
    pub fn nfd0_1_extract(&mut self) -> u64 {
        // bits 53..53
        (self.val >> 53) & 0x1
    }

    /// inserts field val into current value
    pub fn nfd0_1_insert(&mut self, val: u64) {
        // bits 53..53
        self.val = self.val & !(0x1 << 53) | ((val & 0x1) << 53);
    }

    /// extracts field val from current value
    pub fn tbid1_1_extract(&mut self) -> u64 {
        // bits 52..52
        (self.val >> 52) & 0x1
    }

    /// inserts field val into current value
    pub fn tbid1_1_insert(&mut self, val: u64) {
        // bits 52..52
        self.val = self.val & !(0x1 << 52) | ((val & 0x1) << 52);
    }

    /// extracts field val from current value
    pub fn tbid0_1_extract(&mut self) -> u64 {
        // bits 51..51
        (self.val >> 51) & 0x1
    }

    /// inserts field val into current value
    pub fn tbid0_1_insert(&mut self, val: u64) {
        // bits 51..51
        self.val = self.val & !(0x1 << 51) | ((val & 0x1) << 51);
    }

    /// extracts field val from current value
    pub fn hwu162_1_extract(&mut self) -> u64 {
        // bits 50..50
        (self.val >> 50) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu162_1_insert(&mut self, val: u64) {
        // bits 50..50
        self.val = self.val & !(0x1 << 50) | ((val & 0x1) << 50);
    }

    /// extracts field val from current value
    pub fn hwu161_1_extract(&mut self) -> u64 {
        // bits 49..49
        (self.val >> 49) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu161_1_insert(&mut self, val: u64) {
        // bits 49..49
        self.val = self.val & !(0x1 << 49) | ((val & 0x1) << 49);
    }

    /// extracts field val from current value
    pub fn hwu160_1_extract(&mut self) -> u64 {
        // bits 48..48
        (self.val >> 48) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu160_1_insert(&mut self, val: u64) {
        // bits 48..48
        self.val = self.val & !(0x1 << 48) | ((val & 0x1) << 48);
    }

    /// extracts field val from current value
    pub fn hwu159_1_extract(&mut self) -> u64 {
        // bits 47..47
        (self.val >> 47) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu159_1_insert(&mut self, val: u64) {
        // bits 47..47
        self.val = self.val & !(0x1 << 47) | ((val & 0x1) << 47);
    }

    /// extracts field val from current value
    pub fn hwu062_1_extract(&mut self) -> u64 {
        // bits 46..46
        (self.val >> 46) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu062_1_insert(&mut self, val: u64) {
        // bits 46..46
        self.val = self.val & !(0x1 << 46) | ((val & 0x1) << 46);
    }

    /// extracts field val from current value
    pub fn hwu061_1_extract(&mut self) -> u64 {
        // bits 45..45
        (self.val >> 45) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu061_1_insert(&mut self, val: u64) {
        // bits 45..45
        self.val = self.val & !(0x1 << 45) | ((val & 0x1) << 45);
    }

    /// extracts field val from current value
    pub fn hwu060_1_extract(&mut self) -> u64 {
        // bits 44..44
        (self.val >> 44) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu060_1_insert(&mut self, val: u64) {
        // bits 44..44
        self.val = self.val & !(0x1 << 44) | ((val & 0x1) << 44);
    }

    /// extracts field val from current value
    pub fn hwu059_1_extract(&mut self) -> u64 {
        // bits 43..43
        (self.val >> 43) & 0x1
    }

    /// inserts field val into current value
    pub fn hwu059_1_insert(&mut self, val: u64) {
        // bits 43..43
        self.val = self.val & !(0x1 << 43) | ((val & 0x1) << 43);
    }

    /// extracts field val from current value
    pub fn hpd1_1_extract(&mut self) -> u64 {
        // bits 42..42
        (self.val >> 42) & 0x1
    }

    /// inserts field val into current value
    pub fn hpd1_1_insert(&mut self, val: u64) {
        // bits 42..42
        self.val = self.val & !(0x1 << 42) | ((val & 0x1) << 42);
    }

    /// extracts field val from current value
    pub fn hpd0_1_extract(&mut self) -> u64 {
        // bits 41..41
        (self.val >> 41) & 0x1
    }

    /// inserts field val into current value
    pub fn hpd0_1_insert(&mut self, val: u64) {
        // bits 41..41
        self.val = self.val & !(0x1 << 41) | ((val & 0x1) << 41);
    }

    /// extracts field val from current value
    pub fn hd_1_extract(&mut self) -> u64 {
        // bits 40..40
        (self.val >> 40) & 0x1
    }

    /// inserts field val into current value
    pub fn hd_1_insert(&mut self, val: u64) {
        // bits 40..40
        self.val = self.val & !(0x1 << 40) | ((val & 0x1) << 40);
    }

    /// extracts field val from current value
    pub fn ha_1_extract(&mut self) -> u64 {
        // bits 39..39
        (self.val >> 39) & 0x1
    }

    /// inserts field val into current value
    pub fn ha_1_insert(&mut self, val: u64) {
        // bits 39..39
        self.val = self.val & !(0x1 << 39) | ((val & 0x1) << 39);
    }

    /// extracts field val from current value
    pub fn tbi1_extract(&mut self) -> u64 {
        // bits 38..38
        (self.val >> 38) & 0x1
    }

    /// inserts field val into current value
    pub fn tbi1_insert(&mut self, val: u64) {
        // bits 38..38
        self.val = self.val & !(0x1 << 38) | ((val & 0x1) << 38);
    }

    /// extracts field val from current value
    pub fn tbi0_extract(&mut self) -> u64 {
        // bits 37..37
        (self.val >> 37) & 0x1
    }

    /// inserts field val into current value
    pub fn tbi0_insert(&mut self, val: u64) {
        // bits 37..37
        self.val = self.val & !(0x1 << 37) | ((val & 0x1) << 37);
    }

    /// extracts field val from current value
    pub fn as_extract(&mut self) -> u64 {
        // bits 36..36
        (self.val >> 36) & 0x1
    }

    /// inserts field val into current value
    pub fn as_insert(&mut self, val: u64) {
        // bits 36..36
        self.val = self.val & !(0x1 << 36) | ((val & 0x1) << 36);
    }

    /// extracts field val from current value
    pub fn ips_extract(&mut self) -> u64 {
        // bits 32..34
        (self.val >> 32) & 0x7
    }

    /// inserts field val into current value
    pub fn ips_insert(&mut self, val: u64) {
        // bits 32..34
        self.val = self.val & !(0x7 << 32) | ((val & 0x7) << 32);
    }

    /// extracts field val from current value
    pub fn tg1_extract(&mut self) -> u64 {
        // bits 30..31
        (self.val >> 30) & 0x3
    }

    /// inserts field val into current value
    pub fn tg1_insert(&mut self, val: u64) {
        // bits 30..31
        self.val = self.val & !(0x3 << 30) | ((val & 0x3) << 30);
    }

    /// extracts field val from current value
    pub fn sh1_extract(&mut self) -> u64 {
        // bits 28..29
        (self.val >> 28) & 0x3
    }

    /// inserts field val into current value
    pub fn sh1_insert(&mut self, val: u64) {
        // bits 28..29
        self.val = self.val & !(0x3 << 28) | ((val & 0x3) << 28);
    }

    /// extracts field val from current value
    pub fn orgn1_extract(&mut self) -> u64 {
        // bits 26..27
        (self.val >> 26) & 0x3
    }

    /// inserts field val into current value
    pub fn orgn1_insert(&mut self, val: u64) {
        // bits 26..27
        self.val = self.val & !(0x3 << 26) | ((val & 0x3) << 26);
    }

    /// extracts field val from current value
    pub fn irgn1_extract(&mut self) -> u64 {
        // bits 24..25
        (self.val >> 24) & 0x3
    }

    /// inserts field val into current value
    pub fn irgn1_insert(&mut self, val: u64) {
        // bits 24..25
        self.val = self.val & !(0x3 << 24) | ((val & 0x3) << 24);
    }

    /// extracts field val from current value
    pub fn epd1_extract(&mut self) -> u64 {
        // bits 23..23
        (self.val >> 23) & 0x1
    }

    /// inserts field val into current value
    pub fn epd1_insert(&mut self, val: u64) {
        // bits 23..23
        self.val = self.val & !(0x1 << 23) | ((val & 0x1) << 23);
    }

    /// extracts field val from current value
    pub fn a1_extract(&mut self) -> u64 {
        // bits 22..22
        (self.val >> 22) & 0x1
    }

    /// inserts field val into current value
    pub fn a1_insert(&mut self, val: u64) {
        // bits 22..22
        self.val = self.val & !(0x1 << 22) | ((val & 0x1) << 22);
    }

    /// extracts field val from current value
    pub fn t1sz_extract(&mut self) -> u64 {
        // bits 16..21
        (self.val >> 16) & 0x3f
    }

    /// inserts field val into current value
    pub fn t1sz_insert(&mut self, val: u64) {
        // bits 16..21
        self.val = self.val & !(0x3f << 16) | ((val & 0x3f) << 16);
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
    pub fn epd0_extract(&mut self) -> u64 {
        // bits 7..7
        (self.val >> 7) & 0x1
    }

    /// inserts field val into current value
    pub fn epd0_insert(&mut self, val: u64) {
        // bits 7..7
        self.val = self.val & !(0x1 << 7) | ((val & 0x1) << 7);
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

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
 * Generated on: 2020-10-05T16:30:11.719907
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
 * Register:    Sampling Inverted Event Filter Register (pmsnevfr_el1)
 * Group:       A group mapping that does not have a known primary
 * Type:        64-bit Register
 * Description: Controls sample filtering by events. The overall filter is
 * the logical AND of these filters. For example, if E[3] and E[5] are both
 * set to File:        AArch64-pmsnevfr_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Sampling Inverted Event Filter Register (pmsnevfr_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, PMSNEVFR_EL1
        llvm_asm!("mrs $0, S3_0_C9_C9_1" : "=r"(regval));
    }
    return regval;
}


/// writing the Sampling Inverted Event Filter Register (pmsnevfr_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR PMSNEVFR_EL1, <Xt>
        llvm_asm!("msr S3_0_C9_C9_1, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn ex_read() -> u64 {
    // bits 48..63
    let val = reg_rawrd();
    (val >> 48) & 0xffff
}

/// inserts field val into register
pub fn ex_write(newval: u64) {
    // bits 48..63
    let val = reg_rawrd();
    reg_rawwr(val & !(0xffff << 48) | ((newval & 0xffff) << 48));
}

/// reads field val from register
pub fn e63_63_63_read() -> u64 {
    // bits 63..63
    let val = reg_rawrd();
    (val >> 63) & 0x1
}

/// inserts field val into register
pub fn e63_63_63_write(newval: u64) {
    // bits 63..63
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 63) | ((newval & 0x1) << 63));
}

/// reads field val from register
pub fn e62_62_62_read() -> u64 {
    // bits 62..62
    let val = reg_rawrd();
    (val >> 62) & 0x1
}

/// inserts field val into register
pub fn e62_62_62_write(newval: u64) {
    // bits 62..62
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 62) | ((newval & 0x1) << 62));
}

/// reads field val from register
pub fn e61_61_61_read() -> u64 {
    // bits 61..61
    let val = reg_rawrd();
    (val >> 61) & 0x1
}

/// inserts field val into register
pub fn e61_61_61_write(newval: u64) {
    // bits 61..61
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 61) | ((newval & 0x1) << 61));
}

/// reads field val from register
pub fn e60_60_60_read() -> u64 {
    // bits 60..60
    let val = reg_rawrd();
    (val >> 60) & 0x1
}

/// inserts field val into register
pub fn e60_60_60_write(newval: u64) {
    // bits 60..60
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 60) | ((newval & 0x1) << 60));
}

/// reads field val from register
pub fn e59_59_59_read() -> u64 {
    // bits 59..59
    let val = reg_rawrd();
    (val >> 59) & 0x1
}

/// inserts field val into register
pub fn e59_59_59_write(newval: u64) {
    // bits 59..59
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 59) | ((newval & 0x1) << 59));
}

/// reads field val from register
pub fn e58_58_58_read() -> u64 {
    // bits 58..58
    let val = reg_rawrd();
    (val >> 58) & 0x1
}

/// inserts field val into register
pub fn e58_58_58_write(newval: u64) {
    // bits 58..58
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 58) | ((newval & 0x1) << 58));
}

/// reads field val from register
pub fn e57_57_57_read() -> u64 {
    // bits 57..57
    let val = reg_rawrd();
    (val >> 57) & 0x1
}

/// inserts field val into register
pub fn e57_57_57_write(newval: u64) {
    // bits 57..57
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 57) | ((newval & 0x1) << 57));
}

/// reads field val from register
pub fn e56_56_56_read() -> u64 {
    // bits 56..56
    let val = reg_rawrd();
    (val >> 56) & 0x1
}

/// inserts field val into register
pub fn e56_56_56_write(newval: u64) {
    // bits 56..56
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 56) | ((newval & 0x1) << 56));
}

/// reads field val from register
pub fn e55_55_55_read() -> u64 {
    // bits 55..55
    let val = reg_rawrd();
    (val >> 55) & 0x1
}

/// inserts field val into register
pub fn e55_55_55_write(newval: u64) {
    // bits 55..55
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 55) | ((newval & 0x1) << 55));
}

/// reads field val from register
pub fn e54_54_54_read() -> u64 {
    // bits 54..54
    let val = reg_rawrd();
    (val >> 54) & 0x1
}

/// inserts field val into register
pub fn e54_54_54_write(newval: u64) {
    // bits 54..54
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 54) | ((newval & 0x1) << 54));
}

/// reads field val from register
pub fn e53_53_53_read() -> u64 {
    // bits 53..53
    let val = reg_rawrd();
    (val >> 53) & 0x1
}

/// inserts field val into register
pub fn e53_53_53_write(newval: u64) {
    // bits 53..53
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 53) | ((newval & 0x1) << 53));
}

/// reads field val from register
pub fn e52_52_52_read() -> u64 {
    // bits 52..52
    let val = reg_rawrd();
    (val >> 52) & 0x1
}

/// inserts field val into register
pub fn e52_52_52_write(newval: u64) {
    // bits 52..52
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 52) | ((newval & 0x1) << 52));
}

/// reads field val from register
pub fn e51_51_51_read() -> u64 {
    // bits 51..51
    let val = reg_rawrd();
    (val >> 51) & 0x1
}

/// inserts field val into register
pub fn e51_51_51_write(newval: u64) {
    // bits 51..51
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 51) | ((newval & 0x1) << 51));
}

/// reads field val from register
pub fn e50_50_50_read() -> u64 {
    // bits 50..50
    let val = reg_rawrd();
    (val >> 50) & 0x1
}

/// inserts field val into register
pub fn e50_50_50_write(newval: u64) {
    // bits 50..50
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 50) | ((newval & 0x1) << 50));
}

/// reads field val from register
pub fn e49_49_49_read() -> u64 {
    // bits 49..49
    let val = reg_rawrd();
    (val >> 49) & 0x1
}

/// inserts field val into register
pub fn e49_49_49_write(newval: u64) {
    // bits 49..49
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 49) | ((newval & 0x1) << 49));
}

/// reads field val from register
pub fn e48_48_48_read() -> u64 {
    // bits 48..48
    let val = reg_rawrd();
    (val >> 48) & 0x1
}

/// inserts field val into register
pub fn e48_48_48_write(newval: u64) {
    // bits 48..48
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 48) | ((newval & 0x1) << 48));
}

/// reads field val from register
pub fn e31_31_31_read() -> u64 {
    // bits 31..31
    let val = reg_rawrd();
    (val >> 31) & 0x1
}

/// inserts field val into register
pub fn e31_31_31_write(newval: u64) {
    // bits 31..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 31) | ((newval & 0x1) << 31));
}

/// reads field val from register
pub fn e30_30_30_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn e30_30_30_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn e29_29_29_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

/// inserts field val into register
pub fn e29_29_29_write(newval: u64) {
    // bits 29..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 29) | ((newval & 0x1) << 29));
}

/// reads field val from register
pub fn e28_28_28_read() -> u64 {
    // bits 28..28
    let val = reg_rawrd();
    (val >> 28) & 0x1
}

/// inserts field val into register
pub fn e28_28_28_write(newval: u64) {
    // bits 28..28
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 28) | ((newval & 0x1) << 28));
}

/// reads field val from register
pub fn e27_27_27_read() -> u64 {
    // bits 27..27
    let val = reg_rawrd();
    (val >> 27) & 0x1
}

/// inserts field val into register
pub fn e27_27_27_write(newval: u64) {
    // bits 27..27
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 27) | ((newval & 0x1) << 27));
}

/// reads field val from register
pub fn e26_26_26_read() -> u64 {
    // bits 26..26
    let val = reg_rawrd();
    (val >> 26) & 0x1
}

/// inserts field val into register
pub fn e26_26_26_write(newval: u64) {
    // bits 26..26
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 26) | ((newval & 0x1) << 26));
}

/// reads field val from register
pub fn e25_25_25_read() -> u64 {
    // bits 25..25
    let val = reg_rawrd();
    (val >> 25) & 0x1
}

/// inserts field val into register
pub fn e25_25_25_write(newval: u64) {
    // bits 25..25
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 25) | ((newval & 0x1) << 25));
}

/// reads field val from register
pub fn e24_24_24_read() -> u64 {
    // bits 24..24
    let val = reg_rawrd();
    (val >> 24) & 0x1
}

/// inserts field val into register
pub fn e24_24_24_write(newval: u64) {
    // bits 24..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 24) | ((newval & 0x1) << 24));
}

/// reads field val from register
pub fn e18_1_read() -> u64 {
    // bits 18..18
    let val = reg_rawrd();
    (val >> 18) & 0x1
}

/// inserts field val into register
pub fn e18_1_write(newval: u64) {
    // bits 18..18
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 18) | ((newval & 0x1) << 18));
}

/// reads field val from register
pub fn e17_1_read() -> u64 {
    // bits 17..17
    let val = reg_rawrd();
    (val >> 17) & 0x1
}

/// inserts field val into register
pub fn e17_1_write(newval: u64) {
    // bits 17..17
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 17) | ((newval & 0x1) << 17));
}

/// reads field val from register
pub fn e15_15_15_read() -> u64 {
    // bits 15..15
    let val = reg_rawrd();
    (val >> 15) & 0x1
}

/// inserts field val into register
pub fn e15_15_15_write(newval: u64) {
    // bits 15..15
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 15) | ((newval & 0x1) << 15));
}

/// reads field val from register
pub fn e14_14_14_read() -> u64 {
    // bits 14..14
    let val = reg_rawrd();
    (val >> 14) & 0x1
}

/// inserts field val into register
pub fn e14_14_14_write(newval: u64) {
    // bits 14..14
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 14) | ((newval & 0x1) << 14));
}

/// reads field val from register
pub fn e13_13_13_read() -> u64 {
    // bits 13..13
    let val = reg_rawrd();
    (val >> 13) & 0x1
}

/// inserts field val into register
pub fn e13_13_13_write(newval: u64) {
    // bits 13..13
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 13) | ((newval & 0x1) << 13));
}

/// reads field val from register
pub fn e12_12_12_read() -> u64 {
    // bits 12..12
    let val = reg_rawrd();
    (val >> 12) & 0x1
}

/// inserts field val into register
pub fn e12_12_12_write(newval: u64) {
    // bits 12..12
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 12) | ((newval & 0x1) << 12));
}

/// reads field val from register
pub fn e11_1_read() -> u64 {
    // bits 11..11
    let val = reg_rawrd();
    (val >> 11) & 0x1
}

/// inserts field val into register
pub fn e11_1_write(newval: u64) {
    // bits 11..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 11) | ((newval & 0x1) << 11));
}

/// reads field val from register
pub fn e7_read() -> u64 {
    // bits 7..7
    let val = reg_rawrd();
    (val >> 7) & 0x1
}

/// inserts field val into register
pub fn e7_write(newval: u64) {
    // bits 7..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 7) | ((newval & 0x1) << 7));
}

/// reads field val from register
pub fn e6_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

/// inserts field val into register
pub fn e6_write(newval: u64) {
    // bits 6..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 6) | ((newval & 0x1) << 6));
}

/// reads field val from register
pub fn e5_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn e5_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn e3_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

/// inserts field val into register
pub fn e3_write(newval: u64) {
    // bits 3..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 3) | ((newval & 0x1) << 3));
}

/// reads field val from register
pub fn e1_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn e1_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Sampling Inverted Event Filter Register value
/// in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register pmsnevfr_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xffff0000ff06f8ea;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xffff0000ff06f8ea
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xffff0000ff06f8ea)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 18446462603011487978;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn ex_extract(&mut self) -> u64 {
        // bits 48..63
        (self.val >> 48) & 0xffff
    }

    /// inserts field val into current value
    pub fn ex_insert(&mut self, val: u64) {
        // bits 48..63
        self.val = self.val & !(0xffff << 48) | ((val & 0xffff) << 48);
    }

    /// extracts field val from current value
    pub fn e63_63_63_extract(&mut self) -> u64 {
        // bits 63..63
        (self.val >> 63) & 0x1
    }

    /// inserts field val into current value
    pub fn e63_63_63_insert(&mut self, val: u64) {
        // bits 63..63
        self.val = self.val & !(0x1 << 63) | ((val & 0x1) << 63);
    }

    /// extracts field val from current value
    pub fn e62_62_62_extract(&mut self) -> u64 {
        // bits 62..62
        (self.val >> 62) & 0x1
    }

    /// inserts field val into current value
    pub fn e62_62_62_insert(&mut self, val: u64) {
        // bits 62..62
        self.val = self.val & !(0x1 << 62) | ((val & 0x1) << 62);
    }

    /// extracts field val from current value
    pub fn e61_61_61_extract(&mut self) -> u64 {
        // bits 61..61
        (self.val >> 61) & 0x1
    }

    /// inserts field val into current value
    pub fn e61_61_61_insert(&mut self, val: u64) {
        // bits 61..61
        self.val = self.val & !(0x1 << 61) | ((val & 0x1) << 61);
    }

    /// extracts field val from current value
    pub fn e60_60_60_extract(&mut self) -> u64 {
        // bits 60..60
        (self.val >> 60) & 0x1
    }

    /// inserts field val into current value
    pub fn e60_60_60_insert(&mut self, val: u64) {
        // bits 60..60
        self.val = self.val & !(0x1 << 60) | ((val & 0x1) << 60);
    }

    /// extracts field val from current value
    pub fn e59_59_59_extract(&mut self) -> u64 {
        // bits 59..59
        (self.val >> 59) & 0x1
    }

    /// inserts field val into current value
    pub fn e59_59_59_insert(&mut self, val: u64) {
        // bits 59..59
        self.val = self.val & !(0x1 << 59) | ((val & 0x1) << 59);
    }

    /// extracts field val from current value
    pub fn e58_58_58_extract(&mut self) -> u64 {
        // bits 58..58
        (self.val >> 58) & 0x1
    }

    /// inserts field val into current value
    pub fn e58_58_58_insert(&mut self, val: u64) {
        // bits 58..58
        self.val = self.val & !(0x1 << 58) | ((val & 0x1) << 58);
    }

    /// extracts field val from current value
    pub fn e57_57_57_extract(&mut self) -> u64 {
        // bits 57..57
        (self.val >> 57) & 0x1
    }

    /// inserts field val into current value
    pub fn e57_57_57_insert(&mut self, val: u64) {
        // bits 57..57
        self.val = self.val & !(0x1 << 57) | ((val & 0x1) << 57);
    }

    /// extracts field val from current value
    pub fn e56_56_56_extract(&mut self) -> u64 {
        // bits 56..56
        (self.val >> 56) & 0x1
    }

    /// inserts field val into current value
    pub fn e56_56_56_insert(&mut self, val: u64) {
        // bits 56..56
        self.val = self.val & !(0x1 << 56) | ((val & 0x1) << 56);
    }

    /// extracts field val from current value
    pub fn e55_55_55_extract(&mut self) -> u64 {
        // bits 55..55
        (self.val >> 55) & 0x1
    }

    /// inserts field val into current value
    pub fn e55_55_55_insert(&mut self, val: u64) {
        // bits 55..55
        self.val = self.val & !(0x1 << 55) | ((val & 0x1) << 55);
    }

    /// extracts field val from current value
    pub fn e54_54_54_extract(&mut self) -> u64 {
        // bits 54..54
        (self.val >> 54) & 0x1
    }

    /// inserts field val into current value
    pub fn e54_54_54_insert(&mut self, val: u64) {
        // bits 54..54
        self.val = self.val & !(0x1 << 54) | ((val & 0x1) << 54);
    }

    /// extracts field val from current value
    pub fn e53_53_53_extract(&mut self) -> u64 {
        // bits 53..53
        (self.val >> 53) & 0x1
    }

    /// inserts field val into current value
    pub fn e53_53_53_insert(&mut self, val: u64) {
        // bits 53..53
        self.val = self.val & !(0x1 << 53) | ((val & 0x1) << 53);
    }

    /// extracts field val from current value
    pub fn e52_52_52_extract(&mut self) -> u64 {
        // bits 52..52
        (self.val >> 52) & 0x1
    }

    /// inserts field val into current value
    pub fn e52_52_52_insert(&mut self, val: u64) {
        // bits 52..52
        self.val = self.val & !(0x1 << 52) | ((val & 0x1) << 52);
    }

    /// extracts field val from current value
    pub fn e51_51_51_extract(&mut self) -> u64 {
        // bits 51..51
        (self.val >> 51) & 0x1
    }

    /// inserts field val into current value
    pub fn e51_51_51_insert(&mut self, val: u64) {
        // bits 51..51
        self.val = self.val & !(0x1 << 51) | ((val & 0x1) << 51);
    }

    /// extracts field val from current value
    pub fn e50_50_50_extract(&mut self) -> u64 {
        // bits 50..50
        (self.val >> 50) & 0x1
    }

    /// inserts field val into current value
    pub fn e50_50_50_insert(&mut self, val: u64) {
        // bits 50..50
        self.val = self.val & !(0x1 << 50) | ((val & 0x1) << 50);
    }

    /// extracts field val from current value
    pub fn e49_49_49_extract(&mut self) -> u64 {
        // bits 49..49
        (self.val >> 49) & 0x1
    }

    /// inserts field val into current value
    pub fn e49_49_49_insert(&mut self, val: u64) {
        // bits 49..49
        self.val = self.val & !(0x1 << 49) | ((val & 0x1) << 49);
    }

    /// extracts field val from current value
    pub fn e48_48_48_extract(&mut self) -> u64 {
        // bits 48..48
        (self.val >> 48) & 0x1
    }

    /// inserts field val into current value
    pub fn e48_48_48_insert(&mut self, val: u64) {
        // bits 48..48
        self.val = self.val & !(0x1 << 48) | ((val & 0x1) << 48);
    }

    /// extracts field val from current value
    pub fn e31_31_31_extract(&mut self) -> u64 {
        // bits 31..31
        (self.val >> 31) & 0x1
    }

    /// inserts field val into current value
    pub fn e31_31_31_insert(&mut self, val: u64) {
        // bits 31..31
        self.val = self.val & !(0x1 << 31) | ((val & 0x1) << 31);
    }

    /// extracts field val from current value
    pub fn e30_30_30_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn e30_30_30_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn e29_29_29_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }

    /// inserts field val into current value
    pub fn e29_29_29_insert(&mut self, val: u64) {
        // bits 29..29
        self.val = self.val & !(0x1 << 29) | ((val & 0x1) << 29);
    }

    /// extracts field val from current value
    pub fn e28_28_28_extract(&mut self) -> u64 {
        // bits 28..28
        (self.val >> 28) & 0x1
    }

    /// inserts field val into current value
    pub fn e28_28_28_insert(&mut self, val: u64) {
        // bits 28..28
        self.val = self.val & !(0x1 << 28) | ((val & 0x1) << 28);
    }

    /// extracts field val from current value
    pub fn e27_27_27_extract(&mut self) -> u64 {
        // bits 27..27
        (self.val >> 27) & 0x1
    }

    /// inserts field val into current value
    pub fn e27_27_27_insert(&mut self, val: u64) {
        // bits 27..27
        self.val = self.val & !(0x1 << 27) | ((val & 0x1) << 27);
    }

    /// extracts field val from current value
    pub fn e26_26_26_extract(&mut self) -> u64 {
        // bits 26..26
        (self.val >> 26) & 0x1
    }

    /// inserts field val into current value
    pub fn e26_26_26_insert(&mut self, val: u64) {
        // bits 26..26
        self.val = self.val & !(0x1 << 26) | ((val & 0x1) << 26);
    }

    /// extracts field val from current value
    pub fn e25_25_25_extract(&mut self) -> u64 {
        // bits 25..25
        (self.val >> 25) & 0x1
    }

    /// inserts field val into current value
    pub fn e25_25_25_insert(&mut self, val: u64) {
        // bits 25..25
        self.val = self.val & !(0x1 << 25) | ((val & 0x1) << 25);
    }

    /// extracts field val from current value
    pub fn e24_24_24_extract(&mut self) -> u64 {
        // bits 24..24
        (self.val >> 24) & 0x1
    }

    /// inserts field val into current value
    pub fn e24_24_24_insert(&mut self, val: u64) {
        // bits 24..24
        self.val = self.val & !(0x1 << 24) | ((val & 0x1) << 24);
    }

    /// extracts field val from current value
    pub fn e18_1_extract(&mut self) -> u64 {
        // bits 18..18
        (self.val >> 18) & 0x1
    }

    /// inserts field val into current value
    pub fn e18_1_insert(&mut self, val: u64) {
        // bits 18..18
        self.val = self.val & !(0x1 << 18) | ((val & 0x1) << 18);
    }

    /// extracts field val from current value
    pub fn e17_1_extract(&mut self) -> u64 {
        // bits 17..17
        (self.val >> 17) & 0x1
    }

    /// inserts field val into current value
    pub fn e17_1_insert(&mut self, val: u64) {
        // bits 17..17
        self.val = self.val & !(0x1 << 17) | ((val & 0x1) << 17);
    }

    /// extracts field val from current value
    pub fn e15_15_15_extract(&mut self) -> u64 {
        // bits 15..15
        (self.val >> 15) & 0x1
    }

    /// inserts field val into current value
    pub fn e15_15_15_insert(&mut self, val: u64) {
        // bits 15..15
        self.val = self.val & !(0x1 << 15) | ((val & 0x1) << 15);
    }

    /// extracts field val from current value
    pub fn e14_14_14_extract(&mut self) -> u64 {
        // bits 14..14
        (self.val >> 14) & 0x1
    }

    /// inserts field val into current value
    pub fn e14_14_14_insert(&mut self, val: u64) {
        // bits 14..14
        self.val = self.val & !(0x1 << 14) | ((val & 0x1) << 14);
    }

    /// extracts field val from current value
    pub fn e13_13_13_extract(&mut self) -> u64 {
        // bits 13..13
        (self.val >> 13) & 0x1
    }

    /// inserts field val into current value
    pub fn e13_13_13_insert(&mut self, val: u64) {
        // bits 13..13
        self.val = self.val & !(0x1 << 13) | ((val & 0x1) << 13);
    }

    /// extracts field val from current value
    pub fn e12_12_12_extract(&mut self) -> u64 {
        // bits 12..12
        (self.val >> 12) & 0x1
    }

    /// inserts field val into current value
    pub fn e12_12_12_insert(&mut self, val: u64) {
        // bits 12..12
        self.val = self.val & !(0x1 << 12) | ((val & 0x1) << 12);
    }

    /// extracts field val from current value
    pub fn e11_1_extract(&mut self) -> u64 {
        // bits 11..11
        (self.val >> 11) & 0x1
    }

    /// inserts field val into current value
    pub fn e11_1_insert(&mut self, val: u64) {
        // bits 11..11
        self.val = self.val & !(0x1 << 11) | ((val & 0x1) << 11);
    }

    /// extracts field val from current value
    pub fn e7_extract(&mut self) -> u64 {
        // bits 7..7
        (self.val >> 7) & 0x1
    }

    /// inserts field val into current value
    pub fn e7_insert(&mut self, val: u64) {
        // bits 7..7
        self.val = self.val & !(0x1 << 7) | ((val & 0x1) << 7);
    }

    /// extracts field val from current value
    pub fn e6_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }

    /// inserts field val into current value
    pub fn e6_insert(&mut self, val: u64) {
        // bits 6..6
        self.val = self.val & !(0x1 << 6) | ((val & 0x1) << 6);
    }

    /// extracts field val from current value
    pub fn e5_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn e5_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn e3_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }

    /// inserts field val into current value
    pub fn e3_insert(&mut self, val: u64) {
        // bits 3..3
        self.val = self.val & !(0x1 << 3) | ((val & 0x1) << 3);
    }

    /// extracts field val from current value
    pub fn e1_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn e1_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }
}

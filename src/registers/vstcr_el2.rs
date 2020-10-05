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
 * Generated on: 2020-10-05T16:30:11.737687
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
 * Register:    Virtualization Secure Translation Control Register
 * (vstcr_el2) Group:       Generic System Control
 * Type:        64-bit Register
 * Description: The control register for stage 2 of the Secure EL1&0
 * translation regime. File:        AArch64-vstcr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Virtualization Secure Translation Control Register (vstcr_el2)
/// register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, VSTCR_EL2
        llvm_asm!("mrs $0, S3_4_C2_C6_2" : "=r"(regval));
    }
    return regval;
}


/// writing the Virtualization Secure Translation Control Register (vstcr_el2)
/// register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR VSTCR_EL2, <Xt>
        llvm_asm!("msr S3_4_C2_C6_2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn sl2_1_read() -> u64 {
    // bits 33..33
    let val = reg_rawrd();
    (val >> 33) & 0x1
}

/// inserts field val into register
pub fn sl2_1_write(newval: u64) {
    // bits 33..33
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 33) | ((newval & 0x1) << 33));
}

/// reads field val from register
pub fn sa_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn sa_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn sw_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

/// inserts field val into register
pub fn sw_write(newval: u64) {
    // bits 29..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 29) | ((newval & 0x1) << 29));
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
pub fn sl0_1_read() -> u64 {
    // bits 6..7
    let val = reg_rawrd();
    (val >> 6) & 0x3
}

/// inserts field val into register
pub fn sl0_1_write(newval: u64) {
    // bits 6..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 6) | ((newval & 0x3) << 6));
}

/// reads field val from register
pub fn sl0_2_read() -> u64 {
    // bits 6..7
    let val = reg_rawrd();
    (val >> 6) & 0x3
}

/// inserts field val into register
pub fn sl0_2_write(newval: u64) {
    // bits 6..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 6) | ((newval & 0x3) << 6));
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



/// struct holding a copy of the Virtualization Secure Translation Control
/// Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register vstcr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x26000c0ff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x26000c0ff
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x26000c0ff)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 10200596735;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn sl2_1_extract(&mut self) -> u64 {
        // bits 33..33
        (self.val >> 33) & 0x1
    }

    /// inserts field val into current value
    pub fn sl2_1_insert(&mut self, val: u64) {
        // bits 33..33
        self.val = self.val & !(0x1 << 33) | ((val & 0x1) << 33);
    }

    /// extracts field val from current value
    pub fn sa_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn sa_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn sw_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }

    /// inserts field val into current value
    pub fn sw_insert(&mut self, val: u64) {
        // bits 29..29
        self.val = self.val & !(0x1 << 29) | ((val & 0x1) << 29);
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
    pub fn sl0_1_extract(&mut self) -> u64 {
        // bits 6..7
        (self.val >> 6) & 0x3
    }

    /// inserts field val into current value
    pub fn sl0_1_insert(&mut self, val: u64) {
        // bits 6..7
        self.val = self.val & !(0x3 << 6) | ((val & 0x3) << 6);
    }

    /// extracts field val from current value
    pub fn sl0_2_extract(&mut self) -> u64 {
        // bits 6..7
        (self.val >> 6) & 0x3
    }

    /// inserts field val into current value
    pub fn sl0_2_insert(&mut self, val: u64) {
        // bits 6..7
        self.val = self.val & !(0x3 << 6) | ((val & 0x3) << 6);
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

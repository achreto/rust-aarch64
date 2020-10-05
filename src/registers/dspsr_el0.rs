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
 * Generated on: 2020-10-05T16:30:11.684308
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
 * Register:    Debug Saved Program Status Register (dspsr_el0)
 * Group:       Debug registers
 * Type:        64-bit Register
 * Description: Holds the saved process state for Debug state. On entering
 * Debug state, PSTATE information is written to this register. On exiting
 * Debug state, values are copied from this register to PSTATE. File:
 * AArch64-dspsr_el0.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Debug Saved Program Status Register (dspsr_el0) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, DSPSR_EL0
        llvm_asm!("mrs $0, dspsr_el0" : "=r"(regval));
    }
    return regval;
}


/// writing the Debug Saved Program Status Register (dspsr_el0) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR DSPSR_EL0, <Xt>
        llvm_asm!("msr dspsr_el0, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn n_read() -> u64 {
    // bits 31..31
    let val = reg_rawrd();
    (val >> 31) & 0x1
}

/// inserts field val into register
pub fn n_write(newval: u64) {
    // bits 31..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 31) | ((newval & 0x1) << 31));
}

/// reads field val from register
pub fn z_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn z_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn c_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

/// inserts field val into register
pub fn c_write(newval: u64) {
    // bits 29..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 29) | ((newval & 0x1) << 29));
}

/// reads field val from register
pub fn v_read() -> u64 {
    // bits 28..28
    let val = reg_rawrd();
    (val >> 28) & 0x1
}

/// inserts field val into register
pub fn v_write(newval: u64) {
    // bits 28..28
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 28) | ((newval & 0x1) << 28));
}

/// reads field val from register
pub fn tco_1_read() -> u64 {
    // bits 25..25
    let val = reg_rawrd();
    (val >> 25) & 0x1
}

/// inserts field val into register
pub fn tco_1_write(newval: u64) {
    // bits 25..25
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 25) | ((newval & 0x1) << 25));
}

/// reads field val from register
pub fn dit_1_read() -> u64 {
    // bits 24..24
    let val = reg_rawrd();
    (val >> 24) & 0x1
}

/// inserts field val into register
pub fn dit_1_write(newval: u64) {
    // bits 24..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 24) | ((newval & 0x1) << 24));
}

/// reads field val from register
pub fn uao_1_read() -> u64 {
    // bits 23..23
    let val = reg_rawrd();
    (val >> 23) & 0x1
}

/// inserts field val into register
pub fn uao_1_write(newval: u64) {
    // bits 23..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 23) | ((newval & 0x1) << 23));
}

/// reads field val from register
pub fn pan_1_read() -> u64 {
    // bits 22..22
    let val = reg_rawrd();
    (val >> 22) & 0x1
}

/// inserts field val into register
pub fn pan_1_write(newval: u64) {
    // bits 22..22
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 22) | ((newval & 0x1) << 22));
}

/// reads field val from register
pub fn ss_read() -> u64 {
    // bits 21..21
    let val = reg_rawrd();
    (val >> 21) & 0x1
}

/// inserts field val into register
pub fn ss_write(newval: u64) {
    // bits 21..21
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 21) | ((newval & 0x1) << 21));
}

/// reads field val from register
pub fn il_read() -> u64 {
    // bits 20..20
    let val = reg_rawrd();
    (val >> 20) & 0x1
}

/// inserts field val into register
pub fn il_write(newval: u64) {
    // bits 20..20
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 20) | ((newval & 0x1) << 20));
}

/// reads field val from register
pub fn ssbs_1_read() -> u64 {
    // bits 12..12
    let val = reg_rawrd();
    (val >> 12) & 0x1
}

/// inserts field val into register
pub fn ssbs_1_write(newval: u64) {
    // bits 12..12
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 12) | ((newval & 0x1) << 12));
}

/// reads field val from register
pub fn btype_1_read() -> u64 {
    // bits 10..11
    let val = reg_rawrd();
    (val >> 10) & 0x3
}

/// inserts field val into register
pub fn btype_1_write(newval: u64) {
    // bits 10..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x3 << 10) | ((newval & 0x3) << 10));
}

/// reads field val from register
pub fn d_read() -> u64 {
    // bits 9..9
    let val = reg_rawrd();
    (val >> 9) & 0x1
}

/// inserts field val into register
pub fn d_write(newval: u64) {
    // bits 9..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 9) | ((newval & 0x1) << 9));
}

/// reads field val from register
pub fn a_read() -> u64 {
    // bits 8..8
    let val = reg_rawrd();
    (val >> 8) & 0x1
}

/// inserts field val into register
pub fn a_write(newval: u64) {
    // bits 8..8
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 8) | ((newval & 0x1) << 8));
}

/// reads field val from register
pub fn i_read() -> u64 {
    // bits 7..7
    let val = reg_rawrd();
    (val >> 7) & 0x1
}

/// inserts field val into register
pub fn i_write(newval: u64) {
    // bits 7..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 7) | ((newval & 0x1) << 7));
}

/// reads field val from register
pub fn f_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

/// inserts field val into register
pub fn f_write(newval: u64) {
    // bits 6..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 6) | ((newval & 0x1) << 6));
}

/// reads field val from register
pub fn m4_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn m4_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn m30_read() -> u64 {
    // bits 0..3
    let val = reg_rawrd();
    (val >> 0) & 0xf
}

/// inserts field val into register
pub fn m30_write(newval: u64) {
    // bits 0..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0xf << 0) | ((newval & 0xf) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Debug Saved Program Status Register value in
/// memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register dspsr_el0
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xf3f01fdf;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xf3f01fdf
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xf3f01fdf)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 4092600287;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn n_extract(&mut self) -> u64 {
        // bits 31..31
        (self.val >> 31) & 0x1
    }

    /// inserts field val into current value
    pub fn n_insert(&mut self, val: u64) {
        // bits 31..31
        self.val = self.val & !(0x1 << 31) | ((val & 0x1) << 31);
    }

    /// extracts field val from current value
    pub fn z_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn z_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn c_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }

    /// inserts field val into current value
    pub fn c_insert(&mut self, val: u64) {
        // bits 29..29
        self.val = self.val & !(0x1 << 29) | ((val & 0x1) << 29);
    }

    /// extracts field val from current value
    pub fn v_extract(&mut self) -> u64 {
        // bits 28..28
        (self.val >> 28) & 0x1
    }

    /// inserts field val into current value
    pub fn v_insert(&mut self, val: u64) {
        // bits 28..28
        self.val = self.val & !(0x1 << 28) | ((val & 0x1) << 28);
    }

    /// extracts field val from current value
    pub fn tco_1_extract(&mut self) -> u64 {
        // bits 25..25
        (self.val >> 25) & 0x1
    }

    /// inserts field val into current value
    pub fn tco_1_insert(&mut self, val: u64) {
        // bits 25..25
        self.val = self.val & !(0x1 << 25) | ((val & 0x1) << 25);
    }

    /// extracts field val from current value
    pub fn dit_1_extract(&mut self) -> u64 {
        // bits 24..24
        (self.val >> 24) & 0x1
    }

    /// inserts field val into current value
    pub fn dit_1_insert(&mut self, val: u64) {
        // bits 24..24
        self.val = self.val & !(0x1 << 24) | ((val & 0x1) << 24);
    }

    /// extracts field val from current value
    pub fn uao_1_extract(&mut self) -> u64 {
        // bits 23..23
        (self.val >> 23) & 0x1
    }

    /// inserts field val into current value
    pub fn uao_1_insert(&mut self, val: u64) {
        // bits 23..23
        self.val = self.val & !(0x1 << 23) | ((val & 0x1) << 23);
    }

    /// extracts field val from current value
    pub fn pan_1_extract(&mut self) -> u64 {
        // bits 22..22
        (self.val >> 22) & 0x1
    }

    /// inserts field val into current value
    pub fn pan_1_insert(&mut self, val: u64) {
        // bits 22..22
        self.val = self.val & !(0x1 << 22) | ((val & 0x1) << 22);
    }

    /// extracts field val from current value
    pub fn ss_extract(&mut self) -> u64 {
        // bits 21..21
        (self.val >> 21) & 0x1
    }

    /// inserts field val into current value
    pub fn ss_insert(&mut self, val: u64) {
        // bits 21..21
        self.val = self.val & !(0x1 << 21) | ((val & 0x1) << 21);
    }

    /// extracts field val from current value
    pub fn il_extract(&mut self) -> u64 {
        // bits 20..20
        (self.val >> 20) & 0x1
    }

    /// inserts field val into current value
    pub fn il_insert(&mut self, val: u64) {
        // bits 20..20
        self.val = self.val & !(0x1 << 20) | ((val & 0x1) << 20);
    }

    /// extracts field val from current value
    pub fn ssbs_1_extract(&mut self) -> u64 {
        // bits 12..12
        (self.val >> 12) & 0x1
    }

    /// inserts field val into current value
    pub fn ssbs_1_insert(&mut self, val: u64) {
        // bits 12..12
        self.val = self.val & !(0x1 << 12) | ((val & 0x1) << 12);
    }

    /// extracts field val from current value
    pub fn btype_1_extract(&mut self) -> u64 {
        // bits 10..11
        (self.val >> 10) & 0x3
    }

    /// inserts field val into current value
    pub fn btype_1_insert(&mut self, val: u64) {
        // bits 10..11
        self.val = self.val & !(0x3 << 10) | ((val & 0x3) << 10);
    }

    /// extracts field val from current value
    pub fn d_extract(&mut self) -> u64 {
        // bits 9..9
        (self.val >> 9) & 0x1
    }

    /// inserts field val into current value
    pub fn d_insert(&mut self, val: u64) {
        // bits 9..9
        self.val = self.val & !(0x1 << 9) | ((val & 0x1) << 9);
    }

    /// extracts field val from current value
    pub fn a_extract(&mut self) -> u64 {
        // bits 8..8
        (self.val >> 8) & 0x1
    }

    /// inserts field val into current value
    pub fn a_insert(&mut self, val: u64) {
        // bits 8..8
        self.val = self.val & !(0x1 << 8) | ((val & 0x1) << 8);
    }

    /// extracts field val from current value
    pub fn i_extract(&mut self) -> u64 {
        // bits 7..7
        (self.val >> 7) & 0x1
    }

    /// inserts field val into current value
    pub fn i_insert(&mut self, val: u64) {
        // bits 7..7
        self.val = self.val & !(0x1 << 7) | ((val & 0x1) << 7);
    }

    /// extracts field val from current value
    pub fn f_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }

    /// inserts field val into current value
    pub fn f_insert(&mut self, val: u64) {
        // bits 6..6
        self.val = self.val & !(0x1 << 6) | ((val & 0x1) << 6);
    }

    /// extracts field val from current value
    pub fn m4_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn m4_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn m30_extract(&mut self) -> u64 {
        // bits 0..3
        (self.val >> 0) & 0xf
    }

    /// inserts field val into current value
    pub fn m30_insert(&mut self, val: u64) {
        // bits 0..3
        self.val = self.val & !(0xf << 0) | ((val & 0xf) << 0);
    }
}

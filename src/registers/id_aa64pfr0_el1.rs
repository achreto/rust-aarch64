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
 * Generated on: 2020-10-05T16:30:11.702919
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
 * Register:    AArch64 Processor Feature Register 0 (id_aa64pfr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides additional information about implemented PE features
 * in AArch64 state. File:        AArch64-id_aa64pfr0_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the AArch64 Processor Feature Register 0 (id_aa64pfr0_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ID_AA64PFR0_EL1
        llvm_asm!("mrs $0, id_aa64pfr0_el1" : "=r"(regval));
    }
    return regval;
}

// register is not writable. not emitting write accessor

/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn csv3_read() -> u64 {
    // bits 60..63
    let val = reg_rawrd();
    (val >> 60) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn csv2_read() -> u64 {
    // bits 56..59
    let val = reg_rawrd();
    (val >> 56) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn dit_read() -> u64 {
    // bits 48..51
    let val = reg_rawrd();
    (val >> 48) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn amu_read() -> u64 {
    // bits 44..47
    let val = reg_rawrd();
    (val >> 44) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn mpam_read() -> u64 {
    // bits 40..43
    let val = reg_rawrd();
    (val >> 40) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn sel2_read() -> u64 {
    // bits 36..39
    let val = reg_rawrd();
    (val >> 36) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn sve_read() -> u64 {
    // bits 32..35
    let val = reg_rawrd();
    (val >> 32) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn ras_read() -> u64 {
    // bits 28..31
    let val = reg_rawrd();
    (val >> 28) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn gic_read() -> u64 {
    // bits 24..27
    let val = reg_rawrd();
    (val >> 24) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn advsimd_read() -> u64 {
    // bits 20..23
    let val = reg_rawrd();
    (val >> 20) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn fp_read() -> u64 {
    // bits 16..19
    let val = reg_rawrd();
    (val >> 16) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn el3_read() -> u64 {
    // bits 12..15
    let val = reg_rawrd();
    (val >> 12) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn el2_read() -> u64 {
    // bits 8..11
    let val = reg_rawrd();
    (val >> 8) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn el1_read() -> u64 {
    // bits 4..7
    let val = reg_rawrd();
    (val >> 4) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn el0_read() -> u64 {
    // bits 0..3
    let val = reg_rawrd();
    (val >> 0) & 0xf
}

// register is not writable, omitting writing to field


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the AArch64 Processor Feature Register 0 value in
/// memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register id_aa64pfr0_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xff0fffffffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xff0fffffffffffff
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 18379190079298994175;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn csv3_extract(&mut self) -> u64 {
        // bits 60..63
        (self.val >> 60) & 0xf
    }
    // no insert() method for field csv3
    /// extracts field val from current value
    pub fn csv2_extract(&mut self) -> u64 {
        // bits 56..59
        (self.val >> 56) & 0xf
    }
    // no insert() method for field csv2
    /// extracts field val from current value
    pub fn dit_extract(&mut self) -> u64 {
        // bits 48..51
        (self.val >> 48) & 0xf
    }
    // no insert() method for field dit
    /// extracts field val from current value
    pub fn amu_extract(&mut self) -> u64 {
        // bits 44..47
        (self.val >> 44) & 0xf
    }
    // no insert() method for field amu
    /// extracts field val from current value
    pub fn mpam_extract(&mut self) -> u64 {
        // bits 40..43
        (self.val >> 40) & 0xf
    }
    // no insert() method for field mpam
    /// extracts field val from current value
    pub fn sel2_extract(&mut self) -> u64 {
        // bits 36..39
        (self.val >> 36) & 0xf
    }
    // no insert() method for field sel2
    /// extracts field val from current value
    pub fn sve_extract(&mut self) -> u64 {
        // bits 32..35
        (self.val >> 32) & 0xf
    }
    // no insert() method for field sve
    /// extracts field val from current value
    pub fn ras_extract(&mut self) -> u64 {
        // bits 28..31
        (self.val >> 28) & 0xf
    }
    // no insert() method for field ras
    /// extracts field val from current value
    pub fn gic_extract(&mut self) -> u64 {
        // bits 24..27
        (self.val >> 24) & 0xf
    }
    // no insert() method for field gic
    /// extracts field val from current value
    pub fn advsimd_extract(&mut self) -> u64 {
        // bits 20..23
        (self.val >> 20) & 0xf
    }
    // no insert() method for field advsimd
    /// extracts field val from current value
    pub fn fp_extract(&mut self) -> u64 {
        // bits 16..19
        (self.val >> 16) & 0xf
    }
    // no insert() method for field fp
    /// extracts field val from current value
    pub fn el3_extract(&mut self) -> u64 {
        // bits 12..15
        (self.val >> 12) & 0xf
    }
    // no insert() method for field el3
    /// extracts field val from current value
    pub fn el2_extract(&mut self) -> u64 {
        // bits 8..11
        (self.val >> 8) & 0xf
    }
    // no insert() method for field el2
    /// extracts field val from current value
    pub fn el1_extract(&mut self) -> u64 {
        // bits 4..7
        (self.val >> 4) & 0xf
    }
    // no insert() method for field el1
    /// extracts field val from current value
    pub fn el0_extract(&mut self) -> u64 {
        // bits 0..3
        (self.val >> 0) & 0xf
    }
    // no insert() method for field el0
}

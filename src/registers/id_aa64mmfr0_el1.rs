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
 * Generated on: 2020-10-05T16:30:11.702171
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
 * Register:    AArch64 Memory Model Feature Register 0 (id_aa64mmfr0_el1)
 * Group:       Identification registers
 * Type:        64-bit Register
 * Description: Provides information about the implemented memory model and
 * memory management support in AArch64 state. File:
 * AArch64-id_aa64mmfr0_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the AArch64 Memory Model Feature Register 0 (id_aa64mmfr0_el1)
/// register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ID_AA64MMFR0_EL1
        llvm_asm!("mrs $0, id_aa64mmfr0_el1" : "=r"(regval));
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
pub fn ecv_read() -> u64 {
    // bits 60..63
    let val = reg_rawrd();
    (val >> 60) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn fgt_read() -> u64 {
    // bits 56..59
    let val = reg_rawrd();
    (val >> 56) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn exs_read() -> u64 {
    // bits 44..47
    let val = reg_rawrd();
    (val >> 44) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn tgran4_2_read() -> u64 {
    // bits 40..43
    let val = reg_rawrd();
    (val >> 40) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn tgran64_2_read() -> u64 {
    // bits 36..39
    let val = reg_rawrd();
    (val >> 36) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn tgran16_2_read() -> u64 {
    // bits 32..35
    let val = reg_rawrd();
    (val >> 32) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn tgran4_read() -> u64 {
    // bits 28..31
    let val = reg_rawrd();
    (val >> 28) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn tgran64_read() -> u64 {
    // bits 24..27
    let val = reg_rawrd();
    (val >> 24) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn tgran16_read() -> u64 {
    // bits 20..23
    let val = reg_rawrd();
    (val >> 20) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn bigendel0_read() -> u64 {
    // bits 16..19
    let val = reg_rawrd();
    (val >> 16) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn snsmem_read() -> u64 {
    // bits 12..15
    let val = reg_rawrd();
    (val >> 12) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn bigend_read() -> u64 {
    // bits 8..11
    let val = reg_rawrd();
    (val >> 8) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn asidbits_read() -> u64 {
    // bits 4..7
    let val = reg_rawrd();
    (val >> 4) & 0xf
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn parange_read() -> u64 {
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



/// struct holding a copy of the AArch64 Memory Model Feature Register 0 value
/// in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register id_aa64mmfr0_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xff00ffffffffffff;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xff00ffffffffffff
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 18374967954648334335;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn ecv_extract(&mut self) -> u64 {
        // bits 60..63
        (self.val >> 60) & 0xf
    }
    // no insert() method for field ecv
    /// extracts field val from current value
    pub fn fgt_extract(&mut self) -> u64 {
        // bits 56..59
        (self.val >> 56) & 0xf
    }
    // no insert() method for field fgt
    /// extracts field val from current value
    pub fn exs_extract(&mut self) -> u64 {
        // bits 44..47
        (self.val >> 44) & 0xf
    }
    // no insert() method for field exs
    /// extracts field val from current value
    pub fn tgran4_2_extract(&mut self) -> u64 {
        // bits 40..43
        (self.val >> 40) & 0xf
    }
    // no insert() method for field tgran4_2
    /// extracts field val from current value
    pub fn tgran64_2_extract(&mut self) -> u64 {
        // bits 36..39
        (self.val >> 36) & 0xf
    }
    // no insert() method for field tgran64_2
    /// extracts field val from current value
    pub fn tgran16_2_extract(&mut self) -> u64 {
        // bits 32..35
        (self.val >> 32) & 0xf
    }
    // no insert() method for field tgran16_2
    /// extracts field val from current value
    pub fn tgran4_extract(&mut self) -> u64 {
        // bits 28..31
        (self.val >> 28) & 0xf
    }
    // no insert() method for field tgran4
    /// extracts field val from current value
    pub fn tgran64_extract(&mut self) -> u64 {
        // bits 24..27
        (self.val >> 24) & 0xf
    }
    // no insert() method for field tgran64
    /// extracts field val from current value
    pub fn tgran16_extract(&mut self) -> u64 {
        // bits 20..23
        (self.val >> 20) & 0xf
    }
    // no insert() method for field tgran16
    /// extracts field val from current value
    pub fn bigendel0_extract(&mut self) -> u64 {
        // bits 16..19
        (self.val >> 16) & 0xf
    }
    // no insert() method for field bigendel0
    /// extracts field val from current value
    pub fn snsmem_extract(&mut self) -> u64 {
        // bits 12..15
        (self.val >> 12) & 0xf
    }
    // no insert() method for field snsmem
    /// extracts field val from current value
    pub fn bigend_extract(&mut self) -> u64 {
        // bits 8..11
        (self.val >> 8) & 0xf
    }
    // no insert() method for field bigend
    /// extracts field val from current value
    pub fn asidbits_extract(&mut self) -> u64 {
        // bits 4..7
        (self.val >> 4) & 0xf
    }
    // no insert() method for field asidbits
    /// extracts field val from current value
    pub fn parange_extract(&mut self) -> u64 {
        // bits 0..3
        (self.val >> 0) & 0xf
    }
    // no insert() method for field parange
}

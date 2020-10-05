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
 * Generated on: 2020-10-05T16:49:32.049044
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
 * Register:    Interrupt Controller VGIC Type Register (ich_vtr_el2)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Reports supported GIC virtualization features.
 * File:        AArch64-ich_vtr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Interrupt Controller VGIC Type Register (ich_vtr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ICH_VTR_EL2
        llvm_asm!("mrs $0, ich_vtr_el2" : "=r"(regval));
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
pub fn pribits_read() -> u64 {
    // bits 29..31
    let val = reg_rawrd();
    (val >> 29) & 0x7
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn prebits_read() -> u64 {
    // bits 26..28
    let val = reg_rawrd();
    (val >> 26) & 0x7
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn idbits_read() -> u64 {
    // bits 23..25
    let val = reg_rawrd();
    (val >> 23) & 0x7
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn seis_read() -> u64 {
    // bits 22..22
    let val = reg_rawrd();
    (val >> 22) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn a3v_read() -> u64 {
    // bits 21..21
    let val = reg_rawrd();
    (val >> 21) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn nv4_read() -> u64 {
    // bits 20..20
    let val = reg_rawrd();
    (val >> 20) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn tds_read() -> u64 {
    // bits 19..19
    let val = reg_rawrd();
    (val >> 19) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn dvim_read() -> u64 {
    // bits 18..18
    let val = reg_rawrd();
    (val >> 18) & 0x1
}

// register is not writable, omitting writing to field

/// reads field val from register
pub fn listregs_read() -> u64 {
    // bits 0..4
    let val = reg_rawrd();
    (val >> 0) & 0x1f
}

// register is not writable, omitting writing to field


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Interrupt Controller VGIC Type Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register ich_vtr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xfffc001f;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xfffc001f
    }

    // no write() method as it is read only

    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 4294705183;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn pribits_extract(&mut self) -> u64 {
        // bits 29..31
        (self.val >> 29) & 0x7
    }
    // no insert() method for field pribits
    /// extracts field val from current value
    pub fn prebits_extract(&mut self) -> u64 {
        // bits 26..28
        (self.val >> 26) & 0x7
    }
    // no insert() method for field prebits
    /// extracts field val from current value
    pub fn idbits_extract(&mut self) -> u64 {
        // bits 23..25
        (self.val >> 23) & 0x7
    }
    // no insert() method for field idbits
    /// extracts field val from current value
    pub fn seis_extract(&mut self) -> u64 {
        // bits 22..22
        (self.val >> 22) & 0x1
    }
    // no insert() method for field seis
    /// extracts field val from current value
    pub fn a3v_extract(&mut self) -> u64 {
        // bits 21..21
        (self.val >> 21) & 0x1
    }
    // no insert() method for field a3v
    /// extracts field val from current value
    pub fn nv4_extract(&mut self) -> u64 {
        // bits 20..20
        (self.val >> 20) & 0x1
    }
    // no insert() method for field nv4
    /// extracts field val from current value
    pub fn tds_extract(&mut self) -> u64 {
        // bits 19..19
        (self.val >> 19) & 0x1
    }
    // no insert() method for field tds
    /// extracts field val from current value
    pub fn dvim_extract(&mut self) -> u64 {
        // bits 18..18
        (self.val >> 18) & 0x1
    }
    // no insert() method for field dvim
    /// extracts field val from current value
    pub fn listregs_extract(&mut self) -> u64 {
        // bits 0..4
        (self.val >> 0) & 0x1f
    }
    // no insert() method for field listregs
}

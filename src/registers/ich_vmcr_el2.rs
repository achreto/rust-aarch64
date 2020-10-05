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
 * Generated on: 2020-10-05T16:30:11.698516
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
 * Register:    Interrupt Controller Virtual Machine Control Register
 * (ich_vmcr_el2) Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Enables the hypervisor to save and restore the virtual
 * machine view of the GIC state. File:        AArch64-ich_vmcr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Interrupt Controller Virtual Machine Control Register
/// (ich_vmcr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ICH_VMCR_EL2
        llvm_asm!("mrs $0, ich_vmcr_el2" : "=r"(regval));
    }
    return regval;
}


/// writing the Interrupt Controller Virtual Machine Control Register
/// (ich_vmcr_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR ICH_VMCR_EL2, <Xt>
        llvm_asm!("msr ich_vmcr_el2, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn vpmr_read() -> u64 {
    // bits 24..31
    let val = reg_rawrd();
    (val >> 24) & 0xff
}

/// inserts field val into register
pub fn vpmr_write(newval: u64) {
    // bits 24..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0xff << 24) | ((newval & 0xff) << 24));
}

/// reads field val from register
pub fn vbpr0_read() -> u64 {
    // bits 21..23
    let val = reg_rawrd();
    (val >> 21) & 0x7
}

/// inserts field val into register
pub fn vbpr0_write(newval: u64) {
    // bits 21..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7 << 21) | ((newval & 0x7) << 21));
}

/// reads field val from register
pub fn vbpr1_read() -> u64 {
    // bits 18..20
    let val = reg_rawrd();
    (val >> 18) & 0x7
}

/// inserts field val into register
pub fn vbpr1_write(newval: u64) {
    // bits 18..20
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7 << 18) | ((newval & 0x7) << 18));
}

/// reads field val from register
pub fn veoim_read() -> u64 {
    // bits 9..9
    let val = reg_rawrd();
    (val >> 9) & 0x1
}

/// inserts field val into register
pub fn veoim_write(newval: u64) {
    // bits 9..9
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 9) | ((newval & 0x1) << 9));
}

/// reads field val from register
pub fn vcbpr_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn vcbpr_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn vfiqen_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

/// inserts field val into register
pub fn vfiqen_write(newval: u64) {
    // bits 3..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 3) | ((newval & 0x1) << 3));
}

/// reads field val from register
pub fn vackctl_read() -> u64 {
    // bits 2..2
    let val = reg_rawrd();
    (val >> 2) & 0x1
}

/// inserts field val into register
pub fn vackctl_write(newval: u64) {
    // bits 2..2
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 2) | ((newval & 0x1) << 2));
}

/// reads field val from register
pub fn veng1_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn veng1_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}

/// reads field val from register
pub fn veng0_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn veng0_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Interrupt Controller Virtual Machine Control
/// Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register ich_vmcr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0xfffc021f;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0xfffc021f
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0xfffc021f)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 4294705695;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn vpmr_extract(&mut self) -> u64 {
        // bits 24..31
        (self.val >> 24) & 0xff
    }

    /// inserts field val into current value
    pub fn vpmr_insert(&mut self, val: u64) {
        // bits 24..31
        self.val = self.val & !(0xff << 24) | ((val & 0xff) << 24);
    }

    /// extracts field val from current value
    pub fn vbpr0_extract(&mut self) -> u64 {
        // bits 21..23
        (self.val >> 21) & 0x7
    }

    /// inserts field val into current value
    pub fn vbpr0_insert(&mut self, val: u64) {
        // bits 21..23
        self.val = self.val & !(0x7 << 21) | ((val & 0x7) << 21);
    }

    /// extracts field val from current value
    pub fn vbpr1_extract(&mut self) -> u64 {
        // bits 18..20
        (self.val >> 18) & 0x7
    }

    /// inserts field val into current value
    pub fn vbpr1_insert(&mut self, val: u64) {
        // bits 18..20
        self.val = self.val & !(0x7 << 18) | ((val & 0x7) << 18);
    }

    /// extracts field val from current value
    pub fn veoim_extract(&mut self) -> u64 {
        // bits 9..9
        (self.val >> 9) & 0x1
    }

    /// inserts field val into current value
    pub fn veoim_insert(&mut self, val: u64) {
        // bits 9..9
        self.val = self.val & !(0x1 << 9) | ((val & 0x1) << 9);
    }

    /// extracts field val from current value
    pub fn vcbpr_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn vcbpr_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn vfiqen_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }

    /// inserts field val into current value
    pub fn vfiqen_insert(&mut self, val: u64) {
        // bits 3..3
        self.val = self.val & !(0x1 << 3) | ((val & 0x1) << 3);
    }

    /// extracts field val from current value
    pub fn vackctl_extract(&mut self) -> u64 {
        // bits 2..2
        (self.val >> 2) & 0x1
    }

    /// inserts field val into current value
    pub fn vackctl_insert(&mut self, val: u64) {
        // bits 2..2
        self.val = self.val & !(0x1 << 2) | ((val & 0x1) << 2);
    }

    /// extracts field val from current value
    pub fn veng1_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn veng1_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }

    /// extracts field val from current value
    pub fn veng0_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn veng0_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

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
 * Generated on: 2020-10-05T16:49:32.049362
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
 * Register:    Interrupt Controller Virtual Binary Point Register 1 (icv_bpr1_el1)
 * Group:       Virtualization registers
 * Type:        64-bit Register
 * Description: Defines the point at which the priority value fields split into two parts, the
 * group priority field and the subpriority field. The group priority field determines virtual
 * Group 1 interrupt preemption. File:        AArch64-icv_bpr1_el1.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Interrupt Controller Virtual Binary Point Register 1 (icv_bpr1_el1) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, ICC_BPR1_EL1
        llvm_asm!("mrs $0, S3_0_C12_C12_3" : "=r"(regval));
    }
    return regval;
}


/// writing the Interrupt Controller Virtual Binary Point Register 1 (icv_bpr1_el1) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR ICC_BPR1_EL1, <Xt>
        llvm_asm!("msr S3_0_C12_C12_3, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn binarypoint_read() -> u64 {
    // bits 0..2
    let val = reg_rawrd();
    (val >> 0) & 0x7
}

/// inserts field val into register
pub fn binarypoint_write(newval: u64) {
    // bits 0..2
    let val = reg_rawrd();
    reg_rawwr(val & !(0x7 << 0) | ((newval & 0x7) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Interrupt Controller Virtual Binary Point Register 1 value in
/// memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register icv_bpr1_el1
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x7;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x7
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x7)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 7;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn binarypoint_extract(&mut self) -> u64 {
        // bits 0..2
        (self.val >> 0) & 0x7
    }

    /// inserts field val into current value
    pub fn binarypoint_insert(&mut self, val: u64) {
        // bits 0..2
        self.val = self.val & !(0x7 << 0) | ((val & 0x7) << 0);
    }
}

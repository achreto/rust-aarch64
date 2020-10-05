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
 * Generated on: 2020-10-05T16:49:32.044167
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
 * Register:    Hypervisor Fine-Grained Write Trap Register (hfgwtr_el2)
 * Group:       A group mapping that does not have a known primary
 * Type:        64-bit Register
 * Description: Provides controls for traps of
 * File:        AArch64-hfgwtr_el2.xml
 */



/*
 * ================================================================================================
 * Register Read/Write Functions
 * ================================================================================================
 */



/// reading the Hypervisor Fine-Grained Write Trap Register (hfgwtr_el2) register
pub fn reg_rawrd() -> u64 {
    let mut regval: u64;
    unsafe {
        // MRS <Xt>, HFGWTR_EL2
        llvm_asm!("mrs $0, S3_4_C1_C1_5" : "=r"(regval));
    }
    return regval;
}


/// writing the Hypervisor Fine-Grained Write Trap Register (hfgwtr_el2) register
pub fn reg_rawwr(val: u64) {
    unsafe {
        // MSR HFGWTR_EL2, <Xt>
        llvm_asm!("msr S3_4_C1_C1_5, $0" : : "r"(val));
    }
}



/*
 * ================================================================================================
 * Register Fields Read/Write Functions
 * ================================================================================================
 */



/// reads field val from register
pub fn naccdata_el1_1_read() -> u64 {
    // bits 50..50
    let val = reg_rawrd();
    (val >> 50) & 0x1
}

/// inserts field val into register
pub fn naccdata_el1_1_write(newval: u64) {
    // bits 50..50
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 50) | ((newval & 0x1) << 50));
}

/// reads field val from register
pub fn erxaddr_el1_1_read() -> u64 {
    // bits 49..49
    let val = reg_rawrd();
    (val >> 49) & 0x1
}

/// inserts field val into register
pub fn erxaddr_el1_1_write(newval: u64) {
    // bits 49..49
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 49) | ((newval & 0x1) << 49));
}

/// reads field val from register
pub fn erxpfgcdn_el1_1_read() -> u64 {
    // bits 48..48
    let val = reg_rawrd();
    (val >> 48) & 0x1
}

/// inserts field val into register
pub fn erxpfgcdn_el1_1_write(newval: u64) {
    // bits 48..48
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 48) | ((newval & 0x1) << 48));
}

/// reads field val from register
pub fn erxpfgctl_el1_1_read() -> u64 {
    // bits 47..47
    let val = reg_rawrd();
    (val >> 47) & 0x1
}

/// inserts field val into register
pub fn erxpfgctl_el1_1_write(newval: u64) {
    // bits 47..47
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 47) | ((newval & 0x1) << 47));
}

/// reads field val from register
pub fn erxmiscn_el1_1_read() -> u64 {
    // bits 45..45
    let val = reg_rawrd();
    (val >> 45) & 0x1
}

/// inserts field val into register
pub fn erxmiscn_el1_1_write(newval: u64) {
    // bits 45..45
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 45) | ((newval & 0x1) << 45));
}

/// reads field val from register
pub fn erxstatus_el1_1_read() -> u64 {
    // bits 44..44
    let val = reg_rawrd();
    (val >> 44) & 0x1
}

/// inserts field val into register
pub fn erxstatus_el1_1_write(newval: u64) {
    // bits 44..44
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 44) | ((newval & 0x1) << 44));
}

/// reads field val from register
pub fn erxctlr_el1_1_read() -> u64 {
    // bits 43..43
    let val = reg_rawrd();
    (val >> 43) & 0x1
}

/// inserts field val into register
pub fn erxctlr_el1_1_write(newval: u64) {
    // bits 43..43
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 43) | ((newval & 0x1) << 43));
}

/// reads field val from register
pub fn errselr_el1_1_read() -> u64 {
    // bits 41..41
    let val = reg_rawrd();
    (val >> 41) & 0x1
}

/// inserts field val into register
pub fn errselr_el1_1_write(newval: u64) {
    // bits 41..41
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 41) | ((newval & 0x1) << 41));
}

/// reads field val from register
pub fn icc_igrpenn_el1_1_read() -> u64 {
    // bits 39..39
    let val = reg_rawrd();
    (val >> 39) & 0x1
}

/// inserts field val into register
pub fn icc_igrpenn_el1_1_write(newval: u64) {
    // bits 39..39
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 39) | ((newval & 0x1) << 39));
}

/// reads field val from register
pub fn vbar_el1_read() -> u64 {
    // bits 38..38
    let val = reg_rawrd();
    (val >> 38) & 0x1
}

/// inserts field val into register
pub fn vbar_el1_write(newval: u64) {
    // bits 38..38
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 38) | ((newval & 0x1) << 38));
}

/// reads field val from register
pub fn ttbr1_el1_read() -> u64 {
    // bits 37..37
    let val = reg_rawrd();
    (val >> 37) & 0x1
}

/// inserts field val into register
pub fn ttbr1_el1_write(newval: u64) {
    // bits 37..37
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 37) | ((newval & 0x1) << 37));
}

/// reads field val from register
pub fn ttbr0_el1_read() -> u64 {
    // bits 36..36
    let val = reg_rawrd();
    (val >> 36) & 0x1
}

/// inserts field val into register
pub fn ttbr0_el1_write(newval: u64) {
    // bits 36..36
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 36) | ((newval & 0x1) << 36));
}

/// reads field val from register
pub fn tpidr_el0_read() -> u64 {
    // bits 35..35
    let val = reg_rawrd();
    (val >> 35) & 0x1
}

/// inserts field val into register
pub fn tpidr_el0_write(newval: u64) {
    // bits 35..35
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 35) | ((newval & 0x1) << 35));
}

/// reads field val from register
pub fn tpidrro_el0_read() -> u64 {
    // bits 34..34
    let val = reg_rawrd();
    (val >> 34) & 0x1
}

/// inserts field val into register
pub fn tpidrro_el0_write(newval: u64) {
    // bits 34..34
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 34) | ((newval & 0x1) << 34));
}

/// reads field val from register
pub fn tpidr_el1_read() -> u64 {
    // bits 33..33
    let val = reg_rawrd();
    (val >> 33) & 0x1
}

/// inserts field val into register
pub fn tpidr_el1_write(newval: u64) {
    // bits 33..33
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 33) | ((newval & 0x1) << 33));
}

/// reads field val from register
pub fn tcr_el1_read() -> u64 {
    // bits 32..32
    let val = reg_rawrd();
    (val >> 32) & 0x1
}

/// inserts field val into register
pub fn tcr_el1_write(newval: u64) {
    // bits 32..32
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 32) | ((newval & 0x1) << 32));
}

/// reads field val from register
pub fn scxtnum_el0_1_read() -> u64 {
    // bits 31..31
    let val = reg_rawrd();
    (val >> 31) & 0x1
}

/// inserts field val into register
pub fn scxtnum_el0_1_write(newval: u64) {
    // bits 31..31
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 31) | ((newval & 0x1) << 31));
}

/// reads field val from register
pub fn scxtnum_el1_1_read() -> u64 {
    // bits 30..30
    let val = reg_rawrd();
    (val >> 30) & 0x1
}

/// inserts field val into register
pub fn scxtnum_el1_1_write(newval: u64) {
    // bits 30..30
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 30) | ((newval & 0x1) << 30));
}

/// reads field val from register
pub fn sctlr_el1_read() -> u64 {
    // bits 29..29
    let val = reg_rawrd();
    (val >> 29) & 0x1
}

/// inserts field val into register
pub fn sctlr_el1_write(newval: u64) {
    // bits 29..29
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 29) | ((newval & 0x1) << 29));
}

/// reads field val from register
pub fn par_el1_read() -> u64 {
    // bits 27..27
    let val = reg_rawrd();
    (val >> 27) & 0x1
}

/// inserts field val into register
pub fn par_el1_write(newval: u64) {
    // bits 27..27
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 27) | ((newval & 0x1) << 27));
}

/// reads field val from register
pub fn mair_el1_read() -> u64 {
    // bits 24..24
    let val = reg_rawrd();
    (val >> 24) & 0x1
}

/// inserts field val into register
pub fn mair_el1_write(newval: u64) {
    // bits 24..24
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 24) | ((newval & 0x1) << 24));
}

/// reads field val from register
pub fn lorsa_el1_1_read() -> u64 {
    // bits 23..23
    let val = reg_rawrd();
    (val >> 23) & 0x1
}

/// inserts field val into register
pub fn lorsa_el1_1_write(newval: u64) {
    // bits 23..23
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 23) | ((newval & 0x1) << 23));
}

/// reads field val from register
pub fn lorn_el1_1_read() -> u64 {
    // bits 22..22
    let val = reg_rawrd();
    (val >> 22) & 0x1
}

/// inserts field val into register
pub fn lorn_el1_1_write(newval: u64) {
    // bits 22..22
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 22) | ((newval & 0x1) << 22));
}

/// reads field val from register
pub fn lorea_el1_1_read() -> u64 {
    // bits 20..20
    let val = reg_rawrd();
    (val >> 20) & 0x1
}

/// inserts field val into register
pub fn lorea_el1_1_write(newval: u64) {
    // bits 20..20
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 20) | ((newval & 0x1) << 20));
}

/// reads field val from register
pub fn lorc_el1_1_read() -> u64 {
    // bits 19..19
    let val = reg_rawrd();
    (val >> 19) & 0x1
}

/// inserts field val into register
pub fn lorc_el1_1_write(newval: u64) {
    // bits 19..19
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 19) | ((newval & 0x1) << 19));
}

/// reads field val from register
pub fn far_el1_read() -> u64 {
    // bits 17..17
    let val = reg_rawrd();
    (val >> 17) & 0x1
}

/// inserts field val into register
pub fn far_el1_write(newval: u64) {
    // bits 17..17
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 17) | ((newval & 0x1) << 17));
}

/// reads field val from register
pub fn esr_el1_read() -> u64 {
    // bits 16..16
    let val = reg_rawrd();
    (val >> 16) & 0x1
}

/// inserts field val into register
pub fn esr_el1_write(newval: u64) {
    // bits 16..16
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 16) | ((newval & 0x1) << 16));
}

/// reads field val from register
pub fn csselr_el1_read() -> u64 {
    // bits 13..13
    let val = reg_rawrd();
    (val >> 13) & 0x1
}

/// inserts field val into register
pub fn csselr_el1_write(newval: u64) {
    // bits 13..13
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 13) | ((newval & 0x1) << 13));
}

/// reads field val from register
pub fn cpacr_el1_read() -> u64 {
    // bits 12..12
    let val = reg_rawrd();
    (val >> 12) & 0x1
}

/// inserts field val into register
pub fn cpacr_el1_write(newval: u64) {
    // bits 12..12
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 12) | ((newval & 0x1) << 12));
}

/// reads field val from register
pub fn contextidr_el1_read() -> u64 {
    // bits 11..11
    let val = reg_rawrd();
    (val >> 11) & 0x1
}

/// inserts field val into register
pub fn contextidr_el1_write(newval: u64) {
    // bits 11..11
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 11) | ((newval & 0x1) << 11));
}

/// reads field val from register
pub fn apibkey_1_read() -> u64 {
    // bits 8..8
    let val = reg_rawrd();
    (val >> 8) & 0x1
}

/// inserts field val into register
pub fn apibkey_1_write(newval: u64) {
    // bits 8..8
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 8) | ((newval & 0x1) << 8));
}

/// reads field val from register
pub fn apiakey_1_read() -> u64 {
    // bits 7..7
    let val = reg_rawrd();
    (val >> 7) & 0x1
}

/// inserts field val into register
pub fn apiakey_1_write(newval: u64) {
    // bits 7..7
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 7) | ((newval & 0x1) << 7));
}

/// reads field val from register
pub fn apgakey_1_read() -> u64 {
    // bits 6..6
    let val = reg_rawrd();
    (val >> 6) & 0x1
}

/// inserts field val into register
pub fn apgakey_1_write(newval: u64) {
    // bits 6..6
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 6) | ((newval & 0x1) << 6));
}

/// reads field val from register
pub fn apdbkey_1_read() -> u64 {
    // bits 5..5
    let val = reg_rawrd();
    (val >> 5) & 0x1
}

/// inserts field val into register
pub fn apdbkey_1_write(newval: u64) {
    // bits 5..5
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 5) | ((newval & 0x1) << 5));
}

/// reads field val from register
pub fn apdakey_1_read() -> u64 {
    // bits 4..4
    let val = reg_rawrd();
    (val >> 4) & 0x1
}

/// inserts field val into register
pub fn apdakey_1_write(newval: u64) {
    // bits 4..4
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 4) | ((newval & 0x1) << 4));
}

/// reads field val from register
pub fn amair_el1_read() -> u64 {
    // bits 3..3
    let val = reg_rawrd();
    (val >> 3) & 0x1
}

/// inserts field val into register
pub fn amair_el1_write(newval: u64) {
    // bits 3..3
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 3) | ((newval & 0x1) << 3));
}

/// reads field val from register
pub fn afsr1_el1_read() -> u64 {
    // bits 1..1
    let val = reg_rawrd();
    (val >> 1) & 0x1
}

/// inserts field val into register
pub fn afsr1_el1_write(newval: u64) {
    // bits 1..1
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 1) | ((newval & 0x1) << 1));
}

/// reads field val from register
pub fn afsr0_el1_read() -> u64 {
    // bits 0..0
    let val = reg_rawrd();
    (val >> 0) & 0x1
}

/// inserts field val into register
pub fn afsr0_el1_write(newval: u64) {
    // bits 0..0
    let val = reg_rawrd();
    reg_rawwr(val & !(0x1 << 0) | ((newval & 0x1) << 0));
}


/*
 * ================================================================================================
 * Data Structure Definitions
 * ================================================================================================
 */



/// struct holding a copy of the Hypervisor Fine-Grained Write Trap Register value in memory
pub struct RegVal {
    val: u64,
}


/// struct implementation for accessing the fields of register hfgwtr_el2
impl RegVal {
    // creates a new default value
    pub fn default() -> RegVal {
        RegVal { val: 0 }
    }


    /// inserts field val into current value
    pub fn current(&mut self) -> RegVal {
        let curval = reg_rawrd() & 0x7baffe9db39fb;
        RegVal { val: curval }
    }


    /// extracts field val from current value
    pub fn read(&mut self) {
        self.val = reg_rawrd() & 0x7baffe9db39fb
    }


    /// inserts field val into current value
    pub fn write(&self) {
        reg_rawwr(self.val & 0x7baffe9db39fb)
    }


    // sets the value of the struct
    pub fn set(&mut self, newval: u64) {
        self.val = newval & 2175933139859963;
    }

    // gets the value of the struct
    pub fn get(&self) -> u64 {
        self.val
    }



    /// extracts field val from current value
    pub fn naccdata_el1_1_extract(&mut self) -> u64 {
        // bits 50..50
        (self.val >> 50) & 0x1
    }

    /// inserts field val into current value
    pub fn naccdata_el1_1_insert(&mut self, val: u64) {
        // bits 50..50
        self.val = self.val & !(0x1 << 50) | ((val & 0x1) << 50);
    }

    /// extracts field val from current value
    pub fn erxaddr_el1_1_extract(&mut self) -> u64 {
        // bits 49..49
        (self.val >> 49) & 0x1
    }

    /// inserts field val into current value
    pub fn erxaddr_el1_1_insert(&mut self, val: u64) {
        // bits 49..49
        self.val = self.val & !(0x1 << 49) | ((val & 0x1) << 49);
    }

    /// extracts field val from current value
    pub fn erxpfgcdn_el1_1_extract(&mut self) -> u64 {
        // bits 48..48
        (self.val >> 48) & 0x1
    }

    /// inserts field val into current value
    pub fn erxpfgcdn_el1_1_insert(&mut self, val: u64) {
        // bits 48..48
        self.val = self.val & !(0x1 << 48) | ((val & 0x1) << 48);
    }

    /// extracts field val from current value
    pub fn erxpfgctl_el1_1_extract(&mut self) -> u64 {
        // bits 47..47
        (self.val >> 47) & 0x1
    }

    /// inserts field val into current value
    pub fn erxpfgctl_el1_1_insert(&mut self, val: u64) {
        // bits 47..47
        self.val = self.val & !(0x1 << 47) | ((val & 0x1) << 47);
    }

    /// extracts field val from current value
    pub fn erxmiscn_el1_1_extract(&mut self) -> u64 {
        // bits 45..45
        (self.val >> 45) & 0x1
    }

    /// inserts field val into current value
    pub fn erxmiscn_el1_1_insert(&mut self, val: u64) {
        // bits 45..45
        self.val = self.val & !(0x1 << 45) | ((val & 0x1) << 45);
    }

    /// extracts field val from current value
    pub fn erxstatus_el1_1_extract(&mut self) -> u64 {
        // bits 44..44
        (self.val >> 44) & 0x1
    }

    /// inserts field val into current value
    pub fn erxstatus_el1_1_insert(&mut self, val: u64) {
        // bits 44..44
        self.val = self.val & !(0x1 << 44) | ((val & 0x1) << 44);
    }

    /// extracts field val from current value
    pub fn erxctlr_el1_1_extract(&mut self) -> u64 {
        // bits 43..43
        (self.val >> 43) & 0x1
    }

    /// inserts field val into current value
    pub fn erxctlr_el1_1_insert(&mut self, val: u64) {
        // bits 43..43
        self.val = self.val & !(0x1 << 43) | ((val & 0x1) << 43);
    }

    /// extracts field val from current value
    pub fn errselr_el1_1_extract(&mut self) -> u64 {
        // bits 41..41
        (self.val >> 41) & 0x1
    }

    /// inserts field val into current value
    pub fn errselr_el1_1_insert(&mut self, val: u64) {
        // bits 41..41
        self.val = self.val & !(0x1 << 41) | ((val & 0x1) << 41);
    }

    /// extracts field val from current value
    pub fn icc_igrpenn_el1_1_extract(&mut self) -> u64 {
        // bits 39..39
        (self.val >> 39) & 0x1
    }

    /// inserts field val into current value
    pub fn icc_igrpenn_el1_1_insert(&mut self, val: u64) {
        // bits 39..39
        self.val = self.val & !(0x1 << 39) | ((val & 0x1) << 39);
    }

    /// extracts field val from current value
    pub fn vbar_el1_extract(&mut self) -> u64 {
        // bits 38..38
        (self.val >> 38) & 0x1
    }

    /// inserts field val into current value
    pub fn vbar_el1_insert(&mut self, val: u64) {
        // bits 38..38
        self.val = self.val & !(0x1 << 38) | ((val & 0x1) << 38);
    }

    /// extracts field val from current value
    pub fn ttbr1_el1_extract(&mut self) -> u64 {
        // bits 37..37
        (self.val >> 37) & 0x1
    }

    /// inserts field val into current value
    pub fn ttbr1_el1_insert(&mut self, val: u64) {
        // bits 37..37
        self.val = self.val & !(0x1 << 37) | ((val & 0x1) << 37);
    }

    /// extracts field val from current value
    pub fn ttbr0_el1_extract(&mut self) -> u64 {
        // bits 36..36
        (self.val >> 36) & 0x1
    }

    /// inserts field val into current value
    pub fn ttbr0_el1_insert(&mut self, val: u64) {
        // bits 36..36
        self.val = self.val & !(0x1 << 36) | ((val & 0x1) << 36);
    }

    /// extracts field val from current value
    pub fn tpidr_el0_extract(&mut self) -> u64 {
        // bits 35..35
        (self.val >> 35) & 0x1
    }

    /// inserts field val into current value
    pub fn tpidr_el0_insert(&mut self, val: u64) {
        // bits 35..35
        self.val = self.val & !(0x1 << 35) | ((val & 0x1) << 35);
    }

    /// extracts field val from current value
    pub fn tpidrro_el0_extract(&mut self) -> u64 {
        // bits 34..34
        (self.val >> 34) & 0x1
    }

    /// inserts field val into current value
    pub fn tpidrro_el0_insert(&mut self, val: u64) {
        // bits 34..34
        self.val = self.val & !(0x1 << 34) | ((val & 0x1) << 34);
    }

    /// extracts field val from current value
    pub fn tpidr_el1_extract(&mut self) -> u64 {
        // bits 33..33
        (self.val >> 33) & 0x1
    }

    /// inserts field val into current value
    pub fn tpidr_el1_insert(&mut self, val: u64) {
        // bits 33..33
        self.val = self.val & !(0x1 << 33) | ((val & 0x1) << 33);
    }

    /// extracts field val from current value
    pub fn tcr_el1_extract(&mut self) -> u64 {
        // bits 32..32
        (self.val >> 32) & 0x1
    }

    /// inserts field val into current value
    pub fn tcr_el1_insert(&mut self, val: u64) {
        // bits 32..32
        self.val = self.val & !(0x1 << 32) | ((val & 0x1) << 32);
    }

    /// extracts field val from current value
    pub fn scxtnum_el0_1_extract(&mut self) -> u64 {
        // bits 31..31
        (self.val >> 31) & 0x1
    }

    /// inserts field val into current value
    pub fn scxtnum_el0_1_insert(&mut self, val: u64) {
        // bits 31..31
        self.val = self.val & !(0x1 << 31) | ((val & 0x1) << 31);
    }

    /// extracts field val from current value
    pub fn scxtnum_el1_1_extract(&mut self) -> u64 {
        // bits 30..30
        (self.val >> 30) & 0x1
    }

    /// inserts field val into current value
    pub fn scxtnum_el1_1_insert(&mut self, val: u64) {
        // bits 30..30
        self.val = self.val & !(0x1 << 30) | ((val & 0x1) << 30);
    }

    /// extracts field val from current value
    pub fn sctlr_el1_extract(&mut self) -> u64 {
        // bits 29..29
        (self.val >> 29) & 0x1
    }

    /// inserts field val into current value
    pub fn sctlr_el1_insert(&mut self, val: u64) {
        // bits 29..29
        self.val = self.val & !(0x1 << 29) | ((val & 0x1) << 29);
    }

    /// extracts field val from current value
    pub fn par_el1_extract(&mut self) -> u64 {
        // bits 27..27
        (self.val >> 27) & 0x1
    }

    /// inserts field val into current value
    pub fn par_el1_insert(&mut self, val: u64) {
        // bits 27..27
        self.val = self.val & !(0x1 << 27) | ((val & 0x1) << 27);
    }

    /// extracts field val from current value
    pub fn mair_el1_extract(&mut self) -> u64 {
        // bits 24..24
        (self.val >> 24) & 0x1
    }

    /// inserts field val into current value
    pub fn mair_el1_insert(&mut self, val: u64) {
        // bits 24..24
        self.val = self.val & !(0x1 << 24) | ((val & 0x1) << 24);
    }

    /// extracts field val from current value
    pub fn lorsa_el1_1_extract(&mut self) -> u64 {
        // bits 23..23
        (self.val >> 23) & 0x1
    }

    /// inserts field val into current value
    pub fn lorsa_el1_1_insert(&mut self, val: u64) {
        // bits 23..23
        self.val = self.val & !(0x1 << 23) | ((val & 0x1) << 23);
    }

    /// extracts field val from current value
    pub fn lorn_el1_1_extract(&mut self) -> u64 {
        // bits 22..22
        (self.val >> 22) & 0x1
    }

    /// inserts field val into current value
    pub fn lorn_el1_1_insert(&mut self, val: u64) {
        // bits 22..22
        self.val = self.val & !(0x1 << 22) | ((val & 0x1) << 22);
    }

    /// extracts field val from current value
    pub fn lorea_el1_1_extract(&mut self) -> u64 {
        // bits 20..20
        (self.val >> 20) & 0x1
    }

    /// inserts field val into current value
    pub fn lorea_el1_1_insert(&mut self, val: u64) {
        // bits 20..20
        self.val = self.val & !(0x1 << 20) | ((val & 0x1) << 20);
    }

    /// extracts field val from current value
    pub fn lorc_el1_1_extract(&mut self) -> u64 {
        // bits 19..19
        (self.val >> 19) & 0x1
    }

    /// inserts field val into current value
    pub fn lorc_el1_1_insert(&mut self, val: u64) {
        // bits 19..19
        self.val = self.val & !(0x1 << 19) | ((val & 0x1) << 19);
    }

    /// extracts field val from current value
    pub fn far_el1_extract(&mut self) -> u64 {
        // bits 17..17
        (self.val >> 17) & 0x1
    }

    /// inserts field val into current value
    pub fn far_el1_insert(&mut self, val: u64) {
        // bits 17..17
        self.val = self.val & !(0x1 << 17) | ((val & 0x1) << 17);
    }

    /// extracts field val from current value
    pub fn esr_el1_extract(&mut self) -> u64 {
        // bits 16..16
        (self.val >> 16) & 0x1
    }

    /// inserts field val into current value
    pub fn esr_el1_insert(&mut self, val: u64) {
        // bits 16..16
        self.val = self.val & !(0x1 << 16) | ((val & 0x1) << 16);
    }

    /// extracts field val from current value
    pub fn csselr_el1_extract(&mut self) -> u64 {
        // bits 13..13
        (self.val >> 13) & 0x1
    }

    /// inserts field val into current value
    pub fn csselr_el1_insert(&mut self, val: u64) {
        // bits 13..13
        self.val = self.val & !(0x1 << 13) | ((val & 0x1) << 13);
    }

    /// extracts field val from current value
    pub fn cpacr_el1_extract(&mut self) -> u64 {
        // bits 12..12
        (self.val >> 12) & 0x1
    }

    /// inserts field val into current value
    pub fn cpacr_el1_insert(&mut self, val: u64) {
        // bits 12..12
        self.val = self.val & !(0x1 << 12) | ((val & 0x1) << 12);
    }

    /// extracts field val from current value
    pub fn contextidr_el1_extract(&mut self) -> u64 {
        // bits 11..11
        (self.val >> 11) & 0x1
    }

    /// inserts field val into current value
    pub fn contextidr_el1_insert(&mut self, val: u64) {
        // bits 11..11
        self.val = self.val & !(0x1 << 11) | ((val & 0x1) << 11);
    }

    /// extracts field val from current value
    pub fn apibkey_1_extract(&mut self) -> u64 {
        // bits 8..8
        (self.val >> 8) & 0x1
    }

    /// inserts field val into current value
    pub fn apibkey_1_insert(&mut self, val: u64) {
        // bits 8..8
        self.val = self.val & !(0x1 << 8) | ((val & 0x1) << 8);
    }

    /// extracts field val from current value
    pub fn apiakey_1_extract(&mut self) -> u64 {
        // bits 7..7
        (self.val >> 7) & 0x1
    }

    /// inserts field val into current value
    pub fn apiakey_1_insert(&mut self, val: u64) {
        // bits 7..7
        self.val = self.val & !(0x1 << 7) | ((val & 0x1) << 7);
    }

    /// extracts field val from current value
    pub fn apgakey_1_extract(&mut self) -> u64 {
        // bits 6..6
        (self.val >> 6) & 0x1
    }

    /// inserts field val into current value
    pub fn apgakey_1_insert(&mut self, val: u64) {
        // bits 6..6
        self.val = self.val & !(0x1 << 6) | ((val & 0x1) << 6);
    }

    /// extracts field val from current value
    pub fn apdbkey_1_extract(&mut self) -> u64 {
        // bits 5..5
        (self.val >> 5) & 0x1
    }

    /// inserts field val into current value
    pub fn apdbkey_1_insert(&mut self, val: u64) {
        // bits 5..5
        self.val = self.val & !(0x1 << 5) | ((val & 0x1) << 5);
    }

    /// extracts field val from current value
    pub fn apdakey_1_extract(&mut self) -> u64 {
        // bits 4..4
        (self.val >> 4) & 0x1
    }

    /// inserts field val into current value
    pub fn apdakey_1_insert(&mut self, val: u64) {
        // bits 4..4
        self.val = self.val & !(0x1 << 4) | ((val & 0x1) << 4);
    }

    /// extracts field val from current value
    pub fn amair_el1_extract(&mut self) -> u64 {
        // bits 3..3
        (self.val >> 3) & 0x1
    }

    /// inserts field val into current value
    pub fn amair_el1_insert(&mut self, val: u64) {
        // bits 3..3
        self.val = self.val & !(0x1 << 3) | ((val & 0x1) << 3);
    }

    /// extracts field val from current value
    pub fn afsr1_el1_extract(&mut self) -> u64 {
        // bits 1..1
        (self.val >> 1) & 0x1
    }

    /// inserts field val into current value
    pub fn afsr1_el1_insert(&mut self, val: u64) {
        // bits 1..1
        self.val = self.val & !(0x1 << 1) | ((val & 0x1) << 1);
    }

    /// extracts field val from current value
    pub fn afsr0_el1_extract(&mut self) -> u64 {
        // bits 0..0
        (self.val >> 0) & 0x1
    }

    /// inserts field val into current value
    pub fn afsr0_el1_insert(&mut self, val: u64) {
        // bits 0..0
        self.val = self.val & !(0x1 << 0) | ((val & 0x1) << 0);
    }
}

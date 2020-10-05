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
 * Generated on: 2020-10-05T16:49:32.021874
 * Version: Armv8.7-A-2020-09
 * Source: https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture/2020-09/SysReg_xml_v87A-2020-09.tar.gz
 *
 * !!!! WARNING: THIS FILE IS AUTO GENERATED. ANY CHANGES MAY BE OVERWRITTEN !!!!
 *
 **********************************************************************************************
 * * */


/*
 * ================================================================================================
 * A group mapping that does not have a known primary
 * ================================================================================================
 */


// Accelerator Data
pub mod accdata_el1;

// Hypervisor Activity Monitors Fine-Grained Read Trap Register
pub mod hafgrtr_el2;

// Hypervisor Debug Fine-Grained Read Trap Register
pub mod hdfgrtr_el2;

// Hypervisor Debug Fine-Grained Write Trap Register
pub mod hdfgwtr_el2;

// Hypervisor Fine-Grained Instruction Trap Register
pub mod hfgitr_el2;

// Hypervisor Fine-Grained Read Trap Register
pub mod hfgrtr_el2;

// Hypervisor Fine-Grained Write Trap Register
pub mod hfgwtr_el2;

// Sampling Inverted Event Filter Register
pub mod pmsnevfr_el1;



/*
 * ================================================================================================
 * Activity Monitors registers
 * ================================================================================================
 */


// Activity Monitors Configuration Register
pub mod amcfgr_el0;

// Activity Monitors Counter Group 1 Identification Register
pub mod amcg1idr_el0;

// Activity Monitors Counter Group Configuration Register
pub mod amcgcr_el0;

// Activity Monitors Count Enable Clear Register 0
pub mod amcntenclr0_el0;

// Activity Monitors Count Enable Clear Register 1
pub mod amcntenclr1_el0;

// Activity Monitors Count Enable Set Register 0
pub mod amcntenset0_el0;

// Activity Monitors Count Enable Set Register 1
pub mod amcntenset1_el0;

// Activity Monitors Control Register
pub mod amcr_el0;

// Activity Monitors User Enable Register
pub mod amuserenr_el0;



/*
 * ================================================================================================
 * Address translation instructions
 * ================================================================================================
 */


// Physical Address Register
pub mod par_el1;



/*
 * ================================================================================================
 * Debug registers
 * ================================================================================================
 */


// Debug Authentication Status register
pub mod dbgauthstatus_el1;

// Debug CLAIM Tag Clear register
pub mod dbgclaimclr_el1;

// Debug CLAIM Tag Set register
pub mod dbgclaimset_el1;

// Debug Data Transfer Register, half-duplex
pub mod dbgdtr_el0;

// Debug Data Transfer Register, Receive
pub mod dbgdtrrx_el0;

// Debug Data Transfer Register, Transmit
pub mod dbgdtrtx_el0;

// Debug Power Control Register
pub mod dbgprcr_el1;

// Debug Vector Catch Register
pub mod dbgvcr32_el2;

// Debug Link Register
pub mod dlr_el0;

// Debug Saved Program Status Register
pub mod dspsr_el0;

// Monitor DCC Interrupt Enable Register
pub mod mdccint_el1;

// Monitor DCC Status Register
pub mod mdccsr_el0;

// Monitor Debug ROM Address Register
pub mod mdrar_el1;

// Monitor Debug System Control Register
pub mod mdscr_el1;

// OS Double Lock Register
pub mod osdlr_el1;

// OS Lock Data Transfer Register, Receive
pub mod osdtrrx_el1;

// OS Lock Data Transfer Register, Transmit
pub mod osdtrtx_el1;

// OS Lock Exception Catch Control Register
pub mod oseccr_el1;

// OS Lock Access Register
pub mod oslar_el1;

// OS Lock Status Register
pub mod oslsr_el1;

// Trace Filter Control Register (EL1)
pub mod trfcr_el1;

// Trace Filter Control Register (EL2)
pub mod trfcr_el2;



/*
 * ================================================================================================
 * Exception and fault handling registers
 * ================================================================================================
 */


// Auxiliary Fault Status Register 0 (EL1)
pub mod afsr0_el1;

// Auxiliary Fault Status Register 0 (EL2)
pub mod afsr0_el2;

// Auxiliary Fault Status Register 1 (EL1)
pub mod afsr1_el1;

// Auxiliary Fault Status Register 1 (EL2)
pub mod afsr1_el2;

// Exception Syndrome Register (EL1)
pub mod esr_el1;

// Exception Syndrome Register (EL2)
pub mod esr_el2;

// Exception Syndrome Register (EL3)
pub mod esr_el3;

// Fault Address Register (EL1)
pub mod far_el1;

// Fault Address Register (EL2)
pub mod far_el2;

// Fault Address Register (EL3)
pub mod far_el3;

// Hypervisor IPA Fault Address Register
pub mod hpfar_el2;

// Instruction Fault Status Register (EL2)
pub mod ifsr32_el2;

// Interrupt Status Register
pub mod isr_el1;

// Vector Base Address Register (EL1)
pub mod vbar_el1;

// Vector Base Address Register (EL2)
pub mod vbar_el2;



/*
 * ================================================================================================
 * Floating-point registers
 * ================================================================================================
 */


// Floating-point Control Register
pub mod fpcr;

// Floating-Point Exception Control register
pub mod fpexc32_el2;

// Floating-point Status Register
pub mod fpsr;

// AArch32 Media and VFP Feature Register 0
pub mod mvfr0_el1;

// AArch32 Media and VFP Feature Register 1
pub mod mvfr1_el1;

// AArch32 Media and VFP Feature Register 2
pub mod mvfr2_el1;



/*
 * ================================================================================================
 * GIC control registers
 * ================================================================================================
 */


// Interrupt Controller Control Register (EL3)
pub mod icc_ctlr_el3;

// Interrupt Controller System Register Enable register (EL3)
pub mod icc_sre_el3;



/*
 * ================================================================================================
 * Generic System Control
 * ================================================================================================
 */


// Tag Control Register.
pub mod gcr_el1;

// Random Allocation Tag Seed Register.
pub mod rgsr_el1;

// Random Number
pub mod rndr;

// Reseeded Random Number
pub mod rndrrs;

// AArch32 Secure Debug Enable Register
pub mod sder32_el2;

// Tag Fault Status Register (EL1)
pub mod tfsr_el1;

// Tag Fault Status Register (EL2)
pub mod tfsr_el2;

// Tag Fault Status Register (EL3)
pub mod tfsr_el3;

// Tag Fault Status Register (EL0).
pub mod tfsre0_el1;

// Virtual Nested Control Register
pub mod vncr_el2;

// Virtualization Secure Translation Control Register
pub mod vstcr_el2;

// Virtualization Secure Translation Table Base Register
pub mod vsttbr_el2;



/*
 * ================================================================================================
 * Generic Timer registers
 * ================================================================================================
 */


// Counter-timer Frequency register
pub mod cntfrq_el0;

// Counter-timer Virtual Timer Control register (EL2)
pub mod cnthv_ctl_el2;

// Counter-timer Virtual Timer CompareValue register (EL2)
pub mod cnthv_cval_el2;

// Counter-timer Virtual Timer TimerValue Register (EL2)
pub mod cnthv_tval_el2;

// Counter-timer Secure Virtual Timer Control register (EL2)
pub mod cnthvs_ctl_el2;

// Counter-timer Secure Virtual Timer CompareValue register (EL2)
pub mod cnthvs_cval_el2;

// Counter-timer Secure Virtual Timer TimerValue register (EL2)
pub mod cnthvs_tval_el2;

// Counter-timer Kernel Control register
pub mod cntkctl_el1;

// Counter-timer Physical Timer Control register
pub mod cntp_ctl_el0;

// Counter-timer Physical Timer CompareValue register
pub mod cntp_cval_el0;

// Counter-timer Physical Timer TimerValue register
pub mod cntp_tval_el0;

// Counter-timer Physical Count register
pub mod cntpct_el0;

// Counter-timer Self-Synchronized Physical Count register
pub mod cntpctss_el0;

// Counter-timer Physical Offset register
pub mod cntpoff_el2;

// Counter-timer Physical Secure Timer Control register
pub mod cntps_ctl_el1;

// Counter-timer Physical Secure Timer CompareValue register
pub mod cntps_cval_el1;

// Counter-timer Physical Secure Timer TimerValue register
pub mod cntps_tval_el1;

// Counter-timer Virtual Timer Control register
pub mod cntv_ctl_el0;

// Counter-timer Virtual Timer CompareValue register
pub mod cntv_cval_el0;

// Counter-timer Virtual Timer TimerValue register
pub mod cntv_tval_el0;

// Counter-timer Virtual Count register
pub mod cntvct_el0;

// Counter-timer Self-Synchronized Virtual Count register
pub mod cntvctss_el0;



/*
 * ================================================================================================
 * IMPLEMENTATION DEFINED
 * ================================================================================================
 */


// Auxiliary Control Register (EL1)
pub mod actlr_el1;

// Auxiliary ID Register
pub mod aidr_el1;



/*
 * ================================================================================================
 * Identification registers
 * ================================================================================================
 */


// Current Cache Size ID Register 2
pub mod ccsidr2_el1;

// Current Cache Size ID Register
pub mod ccsidr_el1;

// Cache Level ID Register
pub mod clidr_el1;

// Cache Size Selection Register
pub mod csselr_el1;

// Cache Type Register
pub mod ctr_el0;

// Data Cache Zero ID register
pub mod dczid_el0;

//  Multiple tag transfer ID register
pub mod gmid_el1;

// AArch64 Auxiliary Feature Register 0
pub mod id_aa64afr0_el1;

// AArch64 Auxiliary Feature Register 1
pub mod id_aa64afr1_el1;

// AArch64 Debug Feature Register 0
pub mod id_aa64dfr0_el1;

// AArch64 Debug Feature Register 1
pub mod id_aa64dfr1_el1;

// AArch64 Instruction Set Attribute Register 0
pub mod id_aa64isar0_el1;

// AArch64 Instruction Set Attribute Register 1
pub mod id_aa64isar1_el1;

// AArch64 Instruction Set Attribute Register 2
pub mod id_aa64isar2_el1;

// AArch64 Memory Model Feature Register 0
pub mod id_aa64mmfr0_el1;

// AArch64 Memory Model Feature Register 1
pub mod id_aa64mmfr1_el1;

// AArch64 Memory Model Feature Register 2
pub mod id_aa64mmfr2_el1;

// AArch64 Processor Feature Register 0
pub mod id_aa64pfr0_el1;

// AArch64 Processor Feature Register 1
pub mod id_aa64pfr1_el1;

// SVE Feature ID register 0
pub mod id_aa64zfr0_el1;

// AArch32 Auxiliary Feature Register 0
pub mod id_afr0_el1;

// AArch32 Debug Feature Register 0
pub mod id_dfr0_el1;

// Debug Feature Register 1
pub mod id_dfr1_el1;

// AArch32 Instruction Set Attribute Register 0
pub mod id_isar0_el1;

// AArch32 Instruction Set Attribute Register 1
pub mod id_isar1_el1;

// AArch32 Instruction Set Attribute Register 2
pub mod id_isar2_el1;

// AArch32 Instruction Set Attribute Register 3
pub mod id_isar3_el1;

// AArch32 Instruction Set Attribute Register 4
pub mod id_isar4_el1;

// AArch32 Instruction Set Attribute Register 5
pub mod id_isar5_el1;

// AArch32 Instruction Set Attribute Register 6
pub mod id_isar6_el1;

// AArch32 Memory Model Feature Register 0
pub mod id_mmfr0_el1;

// AArch32 Memory Model Feature Register 1
pub mod id_mmfr1_el1;

// AArch32 Memory Model Feature Register 2
pub mod id_mmfr2_el1;

// AArch32 Memory Model Feature Register 3
pub mod id_mmfr3_el1;

// AArch32 Memory Model Feature Register 4
pub mod id_mmfr4_el1;

// AArch32 Memory Model Feature Register 5
pub mod id_mmfr5_el1;

// AArch32 Processor Feature Register 0
pub mod id_pfr0_el1;

// AArch32 Processor Feature Register 1
pub mod id_pfr1_el1;

// AArch32 Processor Feature Register 2
pub mod id_pfr2_el1;

// Main ID Register
pub mod midr_el1;

// MPAM ID Register (EL1)
pub mod mpamidr_el1;

// Multiprocessor Affinity Register
pub mod mpidr_el1;

// Revision ID Register
pub mod revidr_el1;



/*
 * ================================================================================================
 * Memory Partitioning and Monitoring registers
 * ================================================================================================
 */


// MPAM0 Register (EL1)
pub mod mpam0_el1;

// MPAM1 Register (EL1)
pub mod mpam1_el1;

// MPAM2 Register (EL2)
pub mod mpam2_el2;

// MPAM3 Register (EL3)
pub mod mpam3_el3;

// MPAM Hypervisor Control Register (EL2)
pub mod mpamhcr_el2;

// MPAM Virtual PARTID Mapping Register 0
pub mod mpamvpm0_el2;

// MPAM Virtual PARTID Mapping Register 1
pub mod mpamvpm1_el2;

// MPAM Virtual PARTID Mapping Register 2
pub mod mpamvpm2_el2;

// MPAM Virtual PARTID Mapping Register 3
pub mod mpamvpm3_el2;

// MPAM Virtual PARTID Mapping Register 4
pub mod mpamvpm4_el2;

// MPAM Virtual PARTID Mapping Register 5
pub mod mpamvpm5_el2;

// MPAM Virtual PARTID Mapping Register 6
pub mod mpamvpm6_el2;

// MPAM Virtual PARTID Mapping Register 7
pub mod mpamvpm7_el2;

// MPAM Virtual Partition Mapping Valid Register
pub mod mpamvpmv_el2;



/*
 * ================================================================================================
 * Other system control registers
 * ================================================================================================
 */


// Architectural Feature Access Control Register
pub mod cpacr_el1;

// System Control Register (EL1)
pub mod sctlr_el1;

// System Control Register (EL3)
pub mod sctlr_el3;

// SVE Control Register for EL1
pub mod zcr_el1;

// SVE Control Register for EL2
pub mod zcr_el2;

// SVE Control Register for EL3
pub mod zcr_el3;



/*
 * ================================================================================================
 * Performance Monitors registers
 * ================================================================================================
 */


// Performance Monitors Cycle Count Filter Register
pub mod pmccfiltr_el0;

// Performance Monitors Cycle Count Register
pub mod pmccntr_el0;

// Performance Monitors Common Event Identification register 0
pub mod pmceid0_el0;

// Performance Monitors Common Event Identification register 1
pub mod pmceid1_el0;

// Performance Monitors Count Enable Clear register
pub mod pmcntenclr_el0;

// Performance Monitors Count Enable Set register
pub mod pmcntenset_el0;

// Performance Monitors Control Register
pub mod pmcr_el0;

// Performance Monitors Interrupt Enable Clear register
pub mod pmintenclr_el1;

// Performance Monitors Interrupt Enable Set register
pub mod pmintenset_el1;

// Performance Monitors Machine Identification Register
pub mod pmmir_el1;

// Performance Monitors Overflow Flag Status Clear Register
pub mod pmovsclr_el0;

// Performance Monitors Overflow Flag Status Set register
pub mod pmovsset_el0;

// Performance Monitors Event Counter Selection Register
pub mod pmselr_el0;

// Performance Monitors Software Increment register
pub mod pmswinc_el0;

// Performance Monitors User Enable Register
pub mod pmuserenr_el0;

// Performance Monitors Selected Event Count Register
pub mod pmxevcntr_el0;

// Performance Monitors Selected Event Type Register
pub mod pmxevtyper_el0;



/*
 * ================================================================================================
 * Pointer authentication
 * ================================================================================================
 */


// Pointer Authentication Key A for Data (bits[127:64])
pub mod apdakeyhi_el1;

// Pointer Authentication Key A for Data (bits[63:0])
pub mod apdakeylo_el1;

// Pointer Authentication Key B for Data (bits[127:64])
pub mod apdbkeyhi_el1;

// Pointer Authentication Key B for Data (bits[63:0])
pub mod apdbkeylo_el1;

// Pointer Authentication Key A for Code (bits[127:64])
pub mod apgakeyhi_el1;

// Pointer Authentication Key A for Code (bits[63:0])
pub mod apgakeylo_el1;

// Pointer Authentication Key A for Instruction (bits[127:64])
pub mod apiakeyhi_el1;

// Pointer Authentication Key A for Instruction (bits[63:0])
pub mod apiakeylo_el1;

// Pointer Authentication Key B for Instruction (bits[127:64])
pub mod apibkeyhi_el1;

// Pointer Authentication Key B for Instruction (bits[63:0])
pub mod apibkeylo_el1;



/*
 * ================================================================================================
 * Process state registers
 * ================================================================================================
 */


// Current Exception Level
pub mod currentel;

// Interrupt Mask Bits
pub mod daif;

// Data Independent Timing
pub mod dit;

// Condition Flags
pub mod nzcv;

// Privileged Access Never
pub mod pan;

// Stack Pointer Select
pub mod spsel;

// Speculative Store Bypass Safe
pub mod ssbs;

// Tag Check Override
pub mod tco;

// User Access Override
pub mod uao;



/*
 * ================================================================================================
 * RAS registers
 * ================================================================================================
 */


// Deferred Interrupt Status Register
pub mod disr_el1;

// Error Record ID Register
pub mod erridr_el1;

// Error Record Select Register
pub mod errselr_el1;

// Selected Error Record Address Register
pub mod erxaddr_el1;

// Selected Error Record Control Register
pub mod erxctlr_el1;

// Selected Error Record Feature Register
pub mod erxfr_el1;

// Selected Error Record Miscellaneous Register 0
pub mod erxmisc0_el1;

// Selected Error Record Miscellaneous Register 1
pub mod erxmisc1_el1;

// Selected Error Record Miscellaneous Register 2
pub mod erxmisc2_el1;

// Selected Error Record Miscellaneous Register 3
pub mod erxmisc3_el1;

// Selected Pseudo-fault Generation Countdown register
pub mod erxpfgcdn_el1;

// Selected Pseudo-fault Generation Control register
pub mod erxpfgctl_el1;

// Selected Pseudo-fault Generation Feature register
pub mod erxpfgf_el1;

// Selected Error Record Primary Status Register
pub mod erxstatus_el1;

// Virtual Deferred Interrupt Status Register
pub mod vdisr_el2;

// Virtual SError Exception Syndrome Register
pub mod vsesr_el2;



/*
 * ================================================================================================
 * Reset management registers
 * ================================================================================================
 */


// Reset Management Register (EL1)
pub mod rmr_el1;

// Reset Management Register (EL2)
pub mod rmr_el2;

// Reset Management Register (EL3)
pub mod rmr_el3;

// Reset Vector Base Address Register (if EL2 and EL3 not implemented)
pub mod rvbar_el1;

// Reset Vector Base Address Register (if EL3 not implemented)
pub mod rvbar_el2;

// Reset Vector Base Address Register (if EL3 implemented)
pub mod rvbar_el3;



/*
 * ================================================================================================
 * Security registers
 * ================================================================================================
 */


// Auxiliary Control Register (EL3)
pub mod actlr_el3;

// Architectural Feature Trap Register (EL3)
pub mod cptr_el3;

// Monitor Debug Configuration Register (EL3)
pub mod mdcr_el3;

// Secure Configuration Register
pub mod scr_el3;

// AArch32 Secure Debug Enable Register
pub mod sder32_el3;



/*
 * ================================================================================================
 * Special-purpose registers
 * ================================================================================================
 */


// Exception Link Register (EL1)
pub mod elr_el1;

// Exception Link Register (EL2)
pub mod elr_el2;

// Exception Link Register (EL3)
pub mod elr_el3;

// Stack Pointer (EL0)
pub mod sp_el0;

// Stack Pointer (EL1)
pub mod sp_el1;

// Stack Pointer (EL2)
pub mod sp_el2;

// Saved Program Status Register (Abort mode)
pub mod spsr_abt;

// Saved Program Status Register (EL1)
pub mod spsr_el1;

// Saved Program Status Register (EL2)
pub mod spsr_el2;

// Saved Program Status Register (EL3)
pub mod spsr_el3;

// Saved Program Status Register (FIQ mode)
pub mod spsr_fiq;

// Saved Program Status Register (IRQ mode)
pub mod spsr_irq;

// Saved Program Status Register (Undefined mode)
pub mod spsr_und;



/*
 * ================================================================================================
 * Statistical Profiling Extension registers
 * ================================================================================================
 */


// Profiling Buffer ID Register
pub mod pmbidr_el1;

// Profiling Buffer Limit Address Register
pub mod pmblimitr_el1;

// Profiling Buffer Write Pointer Register
pub mod pmbptr_el1;

// Profiling Buffer Status/syndrome Register
pub mod pmbsr_el1;

// Statistical Profiling Control Register (EL1)
pub mod pmscr_el1;

// Statistical Profiling Control Register (EL2)
pub mod pmscr_el2;

// Sampling Event Filter Register
pub mod pmsevfr_el1;

// Sampling Filter Control Register
pub mod pmsfcr_el1;

// Sampling Interval Counter Register
pub mod pmsicr_el1;

// Sampling Profiling ID Register
pub mod pmsidr_el1;

// Sampling Interval Reload Register
pub mod pmsirr_el1;

// Sampling Latency Filter Register
pub mod pmslatfr_el1;



/*
 * ================================================================================================
 * Thread and process ID registers
 * ================================================================================================
 */


// EL0 Read/Write Software Context Number
pub mod scxtnum_el0;

// EL1 Read/Write Software Context Number
pub mod scxtnum_el1;

// EL2 Read/Write Software Context Number
pub mod scxtnum_el2;

// EL3 Read/Write Software Context Number
pub mod scxtnum_el3;

// EL0 Read/Write Software Thread ID Register
pub mod tpidr_el0;

// EL1 Software Thread ID Register
pub mod tpidr_el1;

// EL2 Software Thread ID Register
pub mod tpidr_el2;

// EL3 Software Thread ID Register
pub mod tpidr_el3;

// EL0 Read-Only Software Thread ID Register
pub mod tpidrro_el0;



/*
 * ================================================================================================
 * Virtual memory control registers
 * ================================================================================================
 */


// Auxiliary Memory Attribute Indirection Register (EL1)
pub mod amair_el1;

// Auxiliary Memory Attribute Indirection Register (EL2)
pub mod amair_el2;

// Context ID Register (EL1)
pub mod contextidr_el1;

// Context ID Register (EL2)
pub mod contextidr_el2;

// Domain Access Control Register
pub mod dacr32_el2;

// LORegion Control (EL1)
pub mod lorc_el1;

// LORegion End Address (EL1)
pub mod lorea_el1;

// LORegionID (EL1)
pub mod lorid_el1;

// LORegion Number (EL1)
pub mod lorn_el1;

// LORegion Start Address (EL1)
pub mod lorsa_el1;

// Memory Attribute Indirection Register (EL1)
pub mod mair_el1;

// Memory Attribute Indirection Register (EL2)
pub mod mair_el2;

// Memory Attribute Indirection Register (EL3)
pub mod mair_el3;

// Translation Control Register (EL1)
pub mod tcr_el1;

// Translation Control Register (EL2)
pub mod tcr_el2;

// Translation Control Register (EL3)
pub mod tcr_el3;

// Translation Table Base Register 0 (EL1)
pub mod ttbr0_el1;

// Translation Table Base Register 0 (EL2)
pub mod ttbr0_el2;

// Translation Table Base Register 0 (EL3)
pub mod ttbr0_el3;

// Translation Table Base Register 1 (EL1)
pub mod ttbr1_el1;

// Translation Table Base Register 1 (EL2)
pub mod ttbr1_el2;

// Virtualization Translation Control Register
pub mod vtcr_el2;

// Virtualization Translation Table Base Register
pub mod vttbr_el2;



/*
 * ================================================================================================
 * Virtualization registers
 * ================================================================================================
 */


// Auxiliary Control Register (EL2)
pub mod actlr_el2;

// Auxiliary Fault Status Register 0 (EL3)
pub mod afsr0_el3;

// Auxiliary Fault Status Register 1 (EL3)
pub mod afsr1_el3;

// Auxiliary Memory Attribute Indirection Register (EL3)
pub mod amair_el3;

// Counter-timer Hypervisor Control register
pub mod cnthctl_el2;

// Counter-timer Hypervisor Physical Timer Control register
pub mod cnthp_ctl_el2;

// Counter-timer Physical Timer CompareValue register (EL2)
pub mod cnthp_cval_el2;

// Counter-timer Physical Timer TimerValue register (EL2)
pub mod cnthp_tval_el2;

// Counter-timer Secure Physical Timer Control register (EL2)
pub mod cnthps_ctl_el2;

// Counter-timer Secure Physical Timer CompareValue register (EL2)
pub mod cnthps_cval_el2;

// Counter-timer Secure Physical Timer TimerValue register (EL2)
pub mod cnthps_tval_el2;

// Counter-timer Virtual Offset register
pub mod cntvoff_el2;

// Architectural Feature Trap Register (EL2)
pub mod cptr_el2;

// Hypervisor Auxiliary Control Register
pub mod hacr_el2;

// Hypervisor Configuration Register
pub mod hcr_el2;

// Extended Hypervisor Configuration Register
pub mod hcrx_el2;

// Hypervisor System Trap Register
pub mod hstr_el2;

// Interrupt Controller Alias Software Generated Interrupt Group 1 Register
pub mod icc_asgi1r_el1;

// Interrupt Controller Binary Point Register 0
pub mod icc_bpr0_el1;

// Interrupt Controller Binary Point Register 1
pub mod icc_bpr1_el1;

// Interrupt Controller Control Register (EL1)
pub mod icc_ctlr_el1;

// Interrupt Controller Deactivate Interrupt Register
pub mod icc_dir_el1;

// Interrupt Controller End Of Interrupt Register 0
pub mod icc_eoir0_el1;

// Interrupt Controller End Of Interrupt Register 1
pub mod icc_eoir1_el1;

// Interrupt Controller Highest Priority Pending Interrupt Register 0
pub mod icc_hppir0_el1;

// Interrupt Controller Highest Priority Pending Interrupt Register 1
pub mod icc_hppir1_el1;

// Interrupt Controller Interrupt Acknowledge Register 0
pub mod icc_iar0_el1;

// Interrupt Controller Interrupt Acknowledge Register 1
pub mod icc_iar1_el1;

// Interrupt Controller Interrupt Group 0 Enable register
pub mod icc_igrpen0_el1;

// Interrupt Controller Interrupt Group 1 Enable register
pub mod icc_igrpen1_el1;

// Interrupt Controller Interrupt Group 1 Enable register (EL3)
pub mod icc_igrpen1_el3;

// Interrupt Controller Interrupt Priority Mask Register
pub mod icc_pmr_el1;

// Interrupt Controller Running Priority Register
pub mod icc_rpr_el1;

// Interrupt Controller Software Generated Interrupt Group 0 Register
pub mod icc_sgi0r_el1;

// Interrupt Controller Software Generated Interrupt Group 1 Register
pub mod icc_sgi1r_el1;

// Interrupt Controller System Register Enable register (EL1)
pub mod icc_sre_el1;

// Interrupt Controller System Register Enable register (EL2)
pub mod icc_sre_el2;

// Interrupt Controller End of Interrupt Status Register
pub mod ich_eisr_el2;

// Interrupt Controller Empty List Register Status Register
pub mod ich_elrsr_el2;

// Interrupt Controller Hyp Control Register
pub mod ich_hcr_el2;

// Interrupt Controller Maintenance Interrupt State Register
pub mod ich_misr_el2;

// Interrupt Controller Virtual Machine Control Register
pub mod ich_vmcr_el2;

// Interrupt Controller VGIC Type Register
pub mod ich_vtr_el2;

// Interrupt Controller Virtual Binary Point Register 0
pub mod icv_bpr0_el1;

// Interrupt Controller Virtual Binary Point Register 1
pub mod icv_bpr1_el1;

// Interrupt Controller Virtual Control Register
pub mod icv_ctlr_el1;

// Interrupt Controller Deactivate Virtual Interrupt Register
pub mod icv_dir_el1;

// Interrupt Controller Virtual End Of Interrupt Register 0
pub mod icv_eoir0_el1;

// Interrupt Controller Virtual End Of Interrupt Register 1
pub mod icv_eoir1_el1;

// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 0
pub mod icv_hppir0_el1;

// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 1
pub mod icv_hppir1_el1;

// Interrupt Controller Virtual Interrupt Acknowledge Register 0
pub mod icv_iar0_el1;

// Interrupt Controller Virtual Interrupt Acknowledge Register 1
pub mod icv_iar1_el1;

// Interrupt Controller Virtual Interrupt Group 0 Enable register
pub mod icv_igrpen0_el1;

// Interrupt Controller Virtual Interrupt Group 1 Enable register
pub mod icv_igrpen1_el1;

// Interrupt Controller Virtual Interrupt Priority Mask Register
pub mod icv_pmr_el1;

// Interrupt Controller Virtual Running Priority Register
pub mod icv_rpr_el1;

// Monitor Debug Configuration Register (EL2)
pub mod mdcr_el2;

// System Control Register (EL2)
pub mod sctlr_el2;

// Vector Base Address Register (EL3)
pub mod vbar_el3;

// Virtualization Multiprocessor ID Register
pub mod vmpidr_el2;

// Virtualization Processor ID Register
pub mod vpidr_el2;



// end of file

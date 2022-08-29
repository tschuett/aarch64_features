#![doc(html_root_url = "https://docs.rs/aarch64_features/0.1.0")]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub,
    future_incompatible
)]

//! This crate checks for available features of AArch64 cores. It
//! strives for completeness instead of focussing on the favorite
//! features.  The features cover a wide range from floating
//! operations, atomics, operations on caches, operations for
//! virtualization, and cryptography. It supports AArch64 on Linux and
//! macOS.
//!
//! Despite striving for completeness, the Linux kernel only exposes a subset of the features to userspace, see [feature registers](https://github.com/torvalds/linux/blob/master/Documentation/arm64/cpu-feature-registers.rst).
//!
//!
//!# ZCR_EL1
//!
//! AArch64 has various feature registers, which can be read with the `MRS` assembler command.
//! ```bash
//! MRS  X0, ZCR_EL1
//! ```
//! The `ZCR_EL1` feature register describes the vector length for SVE: `(LEN+1)x128 bits`.
//!
//!| 63:9            | 8:4              | LEN, 3:0              |
//!|-----------------|------------------|-----------------------|
//!| Reserved, RES0  | Reserved, RAZ/WI |SVE Vector Length (VL) |
//!
//!
//!# ID_AA64ISAR0_EL1
//!
//! One of the greatest features, the large system extension (`FEAT_LSE`), was introduced with Armv8.1-A. If the bits 20:23 of the register `ID_AA64ISAR0_EL1` are `0b0010`, then the  core has atomic read-modify-write operations, e.g. `CAS` and `CASP`.
//!
//!| 63:60   | 59:56 | 55:52 | 51:48 | 47:44  | 43:40 | 39:36 | 35:32 | 31:28 | 27:24 | 20:23   | 19:16 | 15:12 | 11:8 | 7:4 | 0:3 |
//!|---------|-------|-------|-------|--------|-------|-------|-------|-------|-------|---------|-------|-------|------|-----|-----|
//!| RNDR    |   TLB | TS    |   FHM |  DP    |  SM4  |  SM3  | SHA3  | RDM   | RES0  | Atomics | CRC32 | SHA2  | SHA1 | AES | RES0|
//!
//!
//!```rust
//!#[cfg(target_arch = "aarch64")]
//!fn has_feat_lse() -> bool {
//!    use core::arch::aarch64::__rsr64;
//!    ((__rsr64("ID_AA64ISAR0_EL1") >> 20) & 0b1111) == 0b0010
//!}
//!```
//!
//! Our crate let's you check for it like this:
//! ```rust
//! use aarch64_features::{check_features, Feature};
//!
//! fn main() {
//!   let features = check_features();
//!
//!   if features.contains(&Feature::FEAT_LSE) {
//!     println!("happy");
//!   }
//!}
//! ```
//!
//! It removes the need to understand the details of low-level AArch64 system programming.
//!
//!# Status
//!
//! - [x] AdvSIMD
//! - [x] Armv8.0-A
//! - [x] Armv8.1-A
//! - [x] Armv8.2-A
//! - [x] Armv8.3-A
//! - [x] Armv8.4-A
//! - [x] Armv8.5-A
//! - [x] Armv8.6-A
//! - [x] Armv8.7-A
//! - [x] Armv8.8-A
//! - [x] Armv9.2-A
//! - [x] Armv9.3-A
//! - [ ] SVE
//! - [ ] SVE2
//! - [ ] Optional and Mandatory
//!
//! # Usage
//! This crate is [on crates.io](https://crates.io/crates/aarch64_features) and can be
//! used by adding `aarch64_features` to your dependencies in your project's `Cargo.toml`.
//! ```toml
//! [dependencies]
//! aarch64_features = "0.1.0"
//! ```
//!
//!# Example: xxx
//!
//! A small example highlighting some of the features of this crate.
//! ```rust
//! use aarch64_features::{check_features, Feature, get_description};
//!
//! fn main() {
//!   let features = check_features();
//!
//!   if features.contains(&Feature::FEAT_LSE) {
//!     println!("happy");
//!   }
//!
//!   if features.contains(&Feature::FEAT_BF16) {
//!     println!("^");
//!   }
//!
//!   if features.contains(&Feature::FEAT_LRCPC) {
//!     println!("really happy");
//!
//!   }
//!
//!   if features.contains(&Feature::FEAT_AdvSIMD) {
//!     println!("no surprise");
//!   }
//!
//!   if features.contains(&Feature::FEAT_FCMA) {
//!     println!("too complex for me");
//!   }
//!
//!   if features.contains(&Feature::FEAT_LRCPC2) {
//!     println!("atomic");
//!   }
//!
//!   if features.contains(&Feature::FEAT_LSE2) {
//!     println!("blows my mind");
//!   }
//!
//!   // Limited ordering regions
//!   println!("FEAT_LOR: {:?}", get_description(&Feature::FEAT_LOR));
//! }
//! ```
//!
//!# Tests
//!
//!
//! There are various tests for different cores.
//!```bash
//!> cargo test test_apple_silicone_m1
//!> cargo test test_apple_silicone_m2
//!> cargo test test_a72 (EC2 A1)
//!> cargo test test_neoverse_n1
//!> cargo test test_neoverse_v1
//!> cargo test test_neoverse_n2
//!```
//!# References
//!
//! Most of the features checks are based on information from the [ARM Architecure Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/) 22 January 2021 G.a, 22 July 2021 G.b, and 4 February 2022 H.a.
//!
//! [ELF Hardware caps](https://github.com/torvalds/linux/blob/master/Documentation/arm64/elf_hwcaps.rst) file from the Linux kernel documents some feature registers.
//!
//! [The cpu registers file from the Linux kernel](https://github.com/torvalds/linux/blob/master/Documentation/arm64/cpu-feature-registers.rst) provides documentation of the SVE feature registers.
//!
//! ARMv9 features are from the [Arm Architecture Reference Manual Supplement Armv9, for Armv9-A architecture profile](https://developer.arm.com/documentation/ddi0608/latest)  Issue A.a.
//!
//!# Caveats
//!
//!The Linux kernel only exports a subset of the feature to userspace, see [features](https://github.com/torvalds/linux/blob/master/Documentation/arm64/cpu-feature-registers.rst). The exported features change over time. Thus the reported features of this crate may change between releases, see [exposed features](crate::linux_exported_features::EXPORTED_FEATURES).
//!
//!
//!
//!# Test your core
//!
//! We are happy to extend the test suite.
//!
//!```shell
//! git clone https://github.com/tschuett/aarch64_features.git
//! cd aarch64_features
//! cargo run
//!```
//!
//! Output:
//!
//!```rust,ignore
//! let features = vec![
//!                      Feature::FEAT_AdvSIMD,
//!                     ];
//!
//!```

use crate::features_list::get_features;
use std::collections::HashSet;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Eq)]
#[non_exhaustive]
/// aarch64 features
pub enum Feature {
    // Armv8.0
    /// Speculation Barrier
    FEAT_SB,
    /// Speculative Store Bypass Safe
    FEAT_SSBS,
    /// Cache Speculation Variant 2
    FEAT_CSV2,
    /// Cache Speculation Variant 2
    FEAT_CSV2_2,
    /// Cache Speculation Variant 2
    FEAT_CSV2_1p1,
    /// Cache Speculation Variant 2
    FEAT_CSV2_1p2,
    /// Cache Speculation Variant 3
    FEAT_CSV3,
    /// Speculation restriction instructions
    FEAT_SPECRES,
    /// CP15SDISABLE2
    FEAT_CP15SDISABLE2,
    /// Double Lock
    FEAT_DoubleLock,
    /// Data Gathering Hint
    FEAT_DGH,
    /// Enhanced Translation Synchronization
    FEAT_ETS,
    /// Intermediate caching of translation table walks
    FEAT_nTLBPA,
    /// PC Sample-based Profiling Extension
    FEAT_PCSRv8,
    /// Advanced SIMD AES instructions
    FEAT_AES,
    /// Advanced SIMD PMULL instructions
    FEAT_PMULL,
    /// Advanced SIMD SHA1 instructions
    FEAT_SHA1,
    /// Advanced SIMD SHA256 instructions
    FEAT_SHA256,

    // Armv8.1
    /// Large System Extensions
    FEAT_LSE,
    /// Advanced SIMD rounding double multiply accumulate instructions
    FEAT_RDM,
    /// Limited ordering regions
    FEAT_LOR,
    /// Hierarchical permission disables
    FEAT_HPDS,
    ///  Hardware management of the Access flag and dirty state
    FEAT_HAFDBS,
    /// Privileged access never
    FEAT_PAN,
    /// 16-bit VMID
    FEAT_VMID16,
    /// Virtualization Host Extensions
    FEAT_VHE,
    /// PMU Extensions v3.1
    FEAT_PMUv3p1,
    /// Support for SCTLR_ELx.EPAN
    FEAT_PAN3,

    // Armv8.2
    /// Advanced SIMD SHA512 instructions
    FEAT_SHA512,
    /// Advanced SIMD SHA3 instructions
    FEAT_SHA3,
    /// Advanced SIMD SM3 instructions
    FEAT_SM3,
    /// Advanced SIMD SM4 instructions
    FEAT_SM4,
    /// Armv8.2 changes to the A64 ISA
    FEAT_ASMv8p2,
    /// AT S1E1R and AT S1E1W instruction variants affected by PSTATE.PAN
    FEAT_PAN2,
    /// Half-precision floating-point data processing
    FEAT_FP16,
    /// Advanced SIMD dot product instructions
    FEAT_DotProd,
    /// Floating-point half-precision multiplication instructions
    FEAT_FHM,
    /// Unprivileged Access Override control
    FEAT_UAO,
    /// DC CVAP instruction
    FEAT_DPB,
    /// VMID-aware PIPT instruction cache
    FEAT_VPIPT,
    /// Translation table page-based hardware attributes
    FEAT_HPDS2,
    /// Large PA and IPA support
    FEAT_LPA,
    /// Large VA support
    FEAT_LVA,
    /// Translation table Common not private translations
    FEAT_TTCNP,
    /// Translation table stage 2 Unprivileged Execute-never
    FEAT_XNX,
    /// Debug v8.2
    FEAT_Debugv8p2,
    /// PC Sample-based profiling
    FEAT_PCSRv8p2,
    /// Implicit Error Synchronization event
    FEAT_IESB,
    /// Single-precision Matrix Multiplication
    FEAT_F32MM,
    /// Single-precision Matrix Multiplication,
    FEAT_F64MM,

    // added later
    /// Enhanced Virtualization Traps
    FEAT_EVT,
    /// DC CVADP instruction
    FEAT_DPB2,
    /// BFloat16 instructions
    FEAT_BF16,
    /// Int8 matrix multiplication instructions
    FEAT_I8MM,
    /// Scalable Vector Extension (SVE)
    FEAT_SVE,

    // Armv8.3
    /// Floating-point complex number instructions
    FEAT_FCMA,
    /// JavaScript conversion instructions
    FEAT_JSCVT,
    /// Load-acquire RCpc instructions
    FEAT_LRCPC,
    /// Nested virtualization support
    FEAT_NV,
    /// Extended cache index
    FEAT_CCIDX,
    /// Pointer authentication
    FEAT_PAuth,

    // added later
    /// Armv8.3 Statistical Profiling Extensions
    FEAT_SPEv1p1,
    /// Debug over Powerdown
    FEAT_DoPD,
    /// Enhancements to pointer authentication
    FEAT_PAuth2,
    /// Faulting on AUT* instructions and combined pointer authentication instructions
    FEAT_FPAC,
    /// Faulting on AUT* instructions and combined pointer authentication instructions
    FEAT_FPACCOMBINE,
    /// Pointer authentication - QARMA3 algorithm
    FEAT_PACQARMA3,
    /// PAC algorithm enhancement
    FEAT_CONSTPACFIELD,

    // Armv8.4
    /// Data Independent Timing instructions
    FEAT_DIT,
    /// Flag manipulation instructions v2
    FEAT_FlagM,
    /// Load-acquire RCpc instructions v2
    FEAT_LRCPC2,
    /// Large System Extensions v2
    FEAT_LSE2,
    /// TLB invalidate instructions in Outer Shareable domain
    FEAT_TLBIOS,
    /// TLB invalidate range instructions
    FEAT_TLBIRANGE,
    /// Translation Table Level
    FEAT_TTL,
    /// Stage 2 forced Write-Back
    FEAT_S2FWB,
    /// Small translation tables
    FEAT_TTST,
    /// Translation table break-before-make levels
    FEAT_BBM,
    /// Secure EL2
    FEAT_SEL2,
    /// Enhanced nested virtualization support
    FEAT_NV2,
    /// ID space trap handling
    FEAT_IDST,
    /// Generic Counter Scaling
    FEAT_CNTSC,
    /// Debug v8.4
    FEAT_Debugv8p4,
    /// Self-hosted Trace Extensions
    FEAT_TRF,
    /// PMU Extensions v3.4
    FEAT_PMUv3p4,
    /// RAS Extension v1.1
    FEAT_RASv1p1,
    /// Double Fault Extension
    FEAT_DoubleFault,

    // Armv8.5
    /// Enhancements to flag manipulation instructions
    FEAT_FlagM2,
    /// Floating-point to integer instructions
    FEAT_FRINTTS,
    /// Context synchronization and exception handling
    FEAT_ExS,
    /// Guest translation granule size
    FEAT_GTG,
    /// Branch Target Identification
    FEAT_BTI,
    /// Preventing EL0 access to halves of address maps
    FEAT_E0PD,
    /// Random number generator
    FEAT_RNG,
    /// Memory Tagging Extension
    FEAT_MTE,
    /// Memory Tagging Extension
    FEAT_MTE2,
    /// PMU Extensions v3.5
    FEAT_PMUv3p5,

    // added later
    /// MTE Asymmetric Fault Handling
    FEAT_MTE3,
    /// Trapping support for RNDR/RNDRRS
    FEAT_RNG_TRAP,

    // Armv8.6
    /// Enhanced Counter Virtualization
    FEAT_ECV,
    /// Fine Grain Traps
    FEAT_FGT,
    /// Delayed Trapping of WFE
    FEAT_TWED,
    /// AMU Extensions v1.1
    FEAT_AMUv1p1,
    /// Multi-threaded PMU Extensions
    FEAT_MTPMU,

    // Armv8.7
    /// Alternate floating-point behavior
    FEAT_AFP,
    /// Increased precision of Reciprocal Estimate and Reciprocal Square Root Estimate
    FEAT_RPRES,
    /// Support for 64 byte loads/stores
    FEAT_LS64,
    /// Support for 64 byte loads/stores
    FEAT_LS64_V,
    /// Support for 64 byte loads/stores
    FEAT_LS64_ACCDATA,
    /// WFE and WFI instructions with timeout
    FEAT_WFxT,
    ////// WFE and WFI instructions with timeout
    //FEAT_WFxT2,
    /// Support for the HCRX_EL2 register
    FEAT_HCX,
    /// Larger physical address for 4KB and 16KB translation granules
    FEAT_LPA2,
    /// XS attribute
    FEAT_XS,
    /// Armv8.7 PMU extensions
    FEAT_PMUv3p7,
    /// Armv8.7 SPE features
    FEAT_SPEv1p2,

    // Armv8.8
    /// Standardization of memory operations
    FEAT_MOPS,
    /// Hinted conditional branches
    FEAT_HBC,
    /// Non-maskable Interrupts
    FEAT_NMI,
    /// EL0 use of IMPLEMENTATION DEFINED functionality
    FEAT_TIDCP1,
    /// Control for cache maintenance permission
    FEAT_CMOW,
    /// Armv8.8 PMU extensions
    FEAT_PMUv3p8,
    /// Setting of MDCR_EL2.HPMN to zero
    FEAT_HPMN0,
    /// Event counting threshold
    FEAT_PMUv3_TH,
    /// Statistical Profiling Extensions
    FEAT_SPEv1p3,
    /// Debug v8.8
    FEAT_Debugv8p8,

    // Armv9.0
    /// Scalable Vector Extension version 2
    FEAT_SVE2,
    /// Scalable Vector AES instructions
    FEAT_SVE_AES,
    /// Scalable Vector Bit Permutes instructions
    FEAT_SVE_BitPerm,
    /// Scalable Vector PMULL instructions
    FEAT_SVE_PMULL128,
    ///  Scalable Vector SHA3 instructions
    FEAT_SVE_SHA3,
    /// Scalable Vector SM4 instructions
    FEAT_SVE_SM4,
    /// Embedded Trace Extension
    FEAT_ETE,
    /// Trace Buffer Extension
    FEAT_TRBE,

    // Armv9.1
    /// Embedded Trace Extension
    FEAT_ETEv1p1,

    // Armv9.2
    /// Branch Record Buffer Extension
    FEAT_BRBE,
    /// Embedded Trace Extension
    FEAT_ETEv1p2,
    /// Realm Management Extension
    FEAT_RME,
    /// Scalable Matrix Extension
    FEAT_SME,
    /// Full Streaming SVE mode instructions
    FEAT_SME_FA64,
    /// AArch64 Extended BFloat16 instructions
    FEAT_EBF16,
    /// Double-precision floating-point outer product instructions
    FEAT_SME_F64F64,
    /// 16-bit to 64-bit integer widening outer product instructions
    FEAT_SME_I16I64,

    // Armv9.3
    /// Branch Record Buffer Extension
    FEAT_BRBEv1p1,

    /// Reliability, Availability, and Serviceability Extension
    FEAT_RAS,
    /// Statistical Profiling Extension
    FEAT_SPE,

    /// Advanced SIMD Extension
    FEAT_AdvSIMD,
}

/// test for all aarch64 features
pub fn check_features() -> HashSet<Feature> {
    if !check_availability() {
        eprintln!("warning unsupported os or cpu");
        return HashSet::new();
    }

    get_features()
}

use crate::features_list::get_feature_description;

/// find a description of the given feature, see [`Feature`]
///
/// ```rust
///use aarch64_features::{Feature, get_description};
///
/// // AdvSIMD"
/// let description = get_description(&Feature::FEAT_AdvSIMD);
///```
pub fn get_description(feature: &Feature) -> Option<String> {
    get_feature_description(feature)
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
use crate::macos_aarch64::cpu::{check_availability, is_exposed_to_userspace};

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
use crate::linux_aarch64::cpu::{check_availability, is_exposed_to_userspace};

#[cfg(not(target_arch = "aarch64"))]
use crate::generic::cpu::{check_availability, is_exposed_to_userspace};

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
mod linux_aarch64 {
    pub(crate) mod cpu;
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
mod macos_aarch64 {
    pub(crate) mod cpu;
}

/// detector for core kinds
pub mod cpu_type;

mod aarch64;
mod cpu_info;
mod features_list;

/// The list of features that are exported by the kernel to userspace.
pub mod linux_exported_features;

#[allow(unused)]
#[derive(Hash, Eq, PartialEq)]
enum Category {
    Atomics,
    Float,
    Caches,
    Crypto,
    Sme,
    Sve,
    Virtual,
    Profiling,
    Security,
    Debug,
    Memory,
    Jump,
    Unknown,
}

#[allow(non_camel_case_types)]
#[allow(unused)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum ARMVersion {
    Armv8_0,
    Armv8_1,
    Armv8_2,
    Armv8_3,
    Armv8_4,
    Armv8_5,
    Armv8_6,
    Armv8_7,
    Armv8_8,
    Armv9_0,
    Armv9_1,
    Armv9_2,
    Armv9_3,
}

#[cfg(not(target_arch = "aarch64"))]
mod generic {
    pub(crate) mod cpu;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_neoverse_n1() {
        //let neoverse_n1_features = vec![];

        //test_features(&neoverse_n1_features); // FIXME
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_neoverse_n2() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_apple_m1() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_apple_m2() {
        assert_eq!(2 + 2, 4);
    }

    #[allow(unused)]
    fn test_features(test_features: &Vec<Feature>) {
        let features = check_features();
        let test_hashset: HashSet<Feature> = test_features.iter().cloned().collect();
        let intersection = features.intersection(&test_hashset).collect::<Vec<_>>();

        assert_eq!(intersection.len(), features.len());
        assert_eq!(intersection.len(), test_features.len());
        assert_eq!(2 + 2, 4);
    }
}

// [AArch64][SME] Add system registers and related instructions
// https://reviews.llvm.org/D105576

// https://developer.arm.com/architectures/cpu-architecture/a-profile/exploration-tools/armv9-a-a64-instruction-set-architecture-release-notes

// MIDR_EL1

// https://developer.arm.com/architectures/cpu-architecture/a-profile/exploration-tools/feature-names-for-a-profile

// https://developer.arm.com/documentation/101427/0101/Register-descriptions/AArch64-system-registers/MIDR-EL1--Main-ID-Register--EL1

// https://github.com/rust-lang/rust/blob/master/src/tools/rustfmt/tests/source/cfg_if/detect/os/aarch64.rs

// https://developer.arm.com/documentation/ddi0601/2021-06/AArch64-Registers/ID-AA64SMFR0-EL1--SME-Feature-ID-register-0

// Armv8.1-M

//! This an opinionated cpu type detector for AArch64 cores. It won't detect any core.
//! The coverage is biased.
//!
//! It focuses on modern data-center cores from Arm, Apple cores, and other high-performance cores.
//!
//! [The MIDR for the Neoverse N2](https://developer.arm.com/documentation/102099/0000/AArch64-registers/AArch64-identification-registers/MIDR-EL1--Main-ID-Register) describes the contents of MIDR_EL1 registern on Arm Neoverse N2 cores.
//! The gcc [aarch64 cores](https://github.com/gcc-mirror/gcc/blob/master/gcc/config/aarch64/aarch64-cores.def) has a elaborate list of cores and partial MIDR_EL1 definitions.

use crate::midr::Implementer;
use crate::midr::Midr;

#[non_exhaustive]
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
/// Core kind
pub enum Core {
    /// Arm Neoverse N1 core
    NeoverseN1,
    /// Arm Neoverse N2 core
    NeoverseN2,
    /// Arm Neoverse V1 core
    NeoverseV1,
    /// Arm Neoverse V2 core
    NeoverseV2,
    /// Fujitsu A64FX
    A64FX,
    /// Apple M1
    AppleM1,
    /// Apple M1 Pro
    AppleM1Pro,
    /// Apple M1 Max
    AppleM1Max,
    /// Ampere 1
    Ampere1,
}

impl TryFrom<Midr> for Core {
    type Error = &'static str;

    #[cfg(target_arch = "aarch64")]
    /// try to detect the current core
    fn try_from(value: Midr) -> Result<Self, Self::Error> {
        for core_description in CORES {
            if value.check_implementer(core_description.implementer)
                && core_description.variant.check_match(&value)
            {
                return Ok(core_description.core);
            }
        }

        Err("unknown core")
    }

    #[cfg(not(target_arch = "aarch64"))]
    /// try to detect the current core
    fn try_from(_value: Midr) -> Result<Self, Self::Error> {
        Err("Unsupported arch")
    }
}

/// arm/cpuid.h (mobile phone?)
const APPLE_ICESTORM_PART_NUM: u64 = 0x20;
const APPLE_FIRESTORM_PART_NUM: u64 = 0x21;

const APPLE_M1_ICESTORM_PART_NUM: u64 = 0x22;
const APPLE_M1_FIRESTORM_PART_NUM: u64 = 0x23;

const APPLE_M1_ICESTORM_PRO_PART_NUM: u64 = 0x24;
const APPLE_M1_FIRESTORM_PRO_PART_NUM: u64 = 0x25;

const APPLE_M1_ICESTORM_MAX_PART_NUM: u64 = 0x28;
const APPLE_M1_FIRESTORM_MAX_PART_NUM: u64 = 0x29;

/// arm/cpuid.h (mobile phone?)
const APPLE_BLIZZARD_PART_NUM: u64 = 0x30;
const APPLE_AVALANCHE_PART_NUM: u64 = 0x31;

const ARM_NEOVERSE_N1_PART_NUM: u64 = 0xD0C;
const ARM_NEOVERSE_N2_PART_NUM: u64 = 0xD49;
const ARM_NEOVERSE_V1_PART_NUM: u64 = 0xD40;
const ARM_NEOVERSE_V2_PART_NUM: u64 = 0xD4F;

const AMPERE_1_PART_NUM: u64 = 0xac3;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::midr::MidrBuilder;

    #[test]
    fn test_apple_m1() {
        let midr = MidrBuilder::new()
            .implementer(Implementer::Apple)
            .part_num(APPLE_M1_FIRESTORM_PART_NUM)
            .build();

        //assert!(is_apple_m1(&midr));

        let midr = MidrBuilder::new()
            .implementer(Implementer::Apple)
            .part_num(APPLE_M1_ICESTORM_PART_NUM)
            .build();

        //assert!(is_apple_m1(&midr));
    }
}

// https://developer.arm.com/documentation/ddi0595/2020-12/AArch64-Registers/MIDR-EL1--Main-ID-Register

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Support/Host.cpp

// https://github.com/torvalds/linux/blob/master/arch/arm64/include/asm/cputype.h

// https://github.com/gcc-mirror/gcc/blob/master/gcc/config/aarch64/aarch64-cores.def

// for recent Apple CPUs
// https://reviews.llvm.org/D134351

// V2
// https://reviews.llvm.org/D134352

// 4 Februar
// https://github.com/apple-oss-distributions/xnu/blob/main/osfmk/arm/cpuid.c

#[allow(unused)]
#[derive(Eq, PartialEq)]
struct CoreDescription {
    core: Core,
    implementer: Implementer,
    variant: VariantMatcher,
}

#[allow(unused)]
#[derive(Eq, PartialEq)]
enum VariantMatcher {
    One(u64),
    Or(u64, u64),
}

impl VariantMatcher {
    fn check_match(&self, midr: &Midr) -> bool {
        match self {
            VariantMatcher::One(one) => midr.check_variant(*one),
            VariantMatcher::Or(one, two) => midr.check_variant(*one) || midr.check_variant(*two),
        }
    }
}

macro_rules! declare_cores {
    ($(
        ($core:ident, $implementer:ident, $variant:expr),
    )+) => {
        /// My favorite cores
        use crate::cpu_type::VariantMatcher::*;
        use crate::midr::Implementer::*;
        use crate::cpu_type::Core::*;
        const CORES: &[CoreDescription] = &[
            $(
                CoreDescription{
                    core: $core,
                    implementer: $implementer,
                    variant: $variant,
                }
            ),+
        ];
    }
}

#[rustfmt::skip]
declare_cores!(
    (NeoverseN1, Arm,     One(ARM_NEOVERSE_N1_PART_NUM)),
    (NeoverseN2, Arm,     One(ARM_NEOVERSE_N2_PART_NUM)),
    (NeoverseV1, Arm,     One(ARM_NEOVERSE_V1_PART_NUM)),
    (NeoverseV2, Arm,     One(ARM_NEOVERSE_V2_PART_NUM)),
    (Ampere1,    Ampere,  One(AMPERE_1_PART_NUM)),
    (AppleM1,    Apple,   Or(APPLE_M1_FIRESTORM_PART_NUM, APPLE_M1_ICESTORM_PART_NUM)),
    (AppleM1Pro, Apple,   Or(APPLE_M1_FIRESTORM_PRO_PART_NUM, APPLE_M1_ICESTORM_PRO_PART_NUM)),
    (AppleM1Max, Apple,   Or(APPLE_M1_FIRESTORM_MAX_PART_NUM, APPLE_M1_ICESTORM_MAX_PART_NUM)),
    (A64FX,      Fujitsu, One(0x1)),
);

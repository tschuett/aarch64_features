//! This an opinionated cpu type detector for AArch64 cores. It won't detect any core.
//! The coverage is biased.
//!
//! It focuses on modern data-center cores from Arm, Apple cores, and other high-performance cores.
//!
//! [The MIDR for the Neoverse N2](https://developer.arm.com/documentation/102099/0000/AArch64-registers/AArch64-identification-registers/MIDR-EL1--Main-ID-Register) describes the contents of MIDR_EL1 registern on Arm Neoverse N2 cores.
//! The gcc [aarch64 cores](https://github.com/gcc-mirror/gcc/blob/master/gcc/config/aarch64/aarch64-cores.def) has a elaborate list of cores and partial MIDR_EL1 definitions.

use crate::midr::Architecture;
use crate::midr::Implementer;
use crate::midr::Midr;

#[derive(Debug)]
#[non_exhaustive]
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
        if is_neoverse_n1(&value) {
            return Ok(Core::NeoverseN1);
        } else if is_neoverse_n2(&value) {
            return Ok(Core::NeoverseN2);
        } else if is_neoverse_v1(&value) {
            return Ok(Core::NeoverseV1);
        } else if is_neoverse_v2(&value) {
            return Ok(Core::NeoverseV2);
        } else if is_a64fx(&value) {
            return Ok(Core::A64FX);
        } else if is_apple_m1(&value) {
            return Ok(Core::AppleM1);
        } else if is_apple_m1_pro(&value) {
            return Ok(Core::AppleM1Pro);
        } else if is_apple_m1_max(&value) {
            return Ok(Core::AppleM1Max);
        } else if is_ampere_1(&value) {
            return Ok(Core::Ampere1);
        }

        Err("Unknown core")
    }

    #[cfg(not(target_arch = "aarch64"))]
    /// try to detect the current core
    fn try_from(_value: Midr) -> Result<Self, Self::Error> {
        Err("Unsupported arch")
    }
}

fn is_neoverse_n1(midr: &Midr) -> bool {
    midr.is_arm() // arm
        && midr.check_variant(0x4)
        && midr.check_architecture(Architecture::IDRegisters)
        && midr.check_part_num(ARM_NEOVERSE_N1_PART_NUM) // N1
        && midr.check_revision(0x0) // r4p0
}

// https://developer.arm.com/documentation/102099/0000/AArch64-registers/AArch64-identification-registers/MIDR-EL1--Main-ID-Register
fn is_neoverse_n2(midr: &Midr) -> bool {
    midr.is_arm() // arm
        && midr.check_variant(0x0) // r0p0
        && midr.check_architecture(Architecture::IDRegisters)
        && midr.check_part_num(ARM_NEOVERSE_N2_PART_NUM) // N2
        && midr.check_revision(0x0) // r0p0
}

fn is_neoverse_v1(midr: &Midr) -> bool {
    midr.is_arm() // arm
        && midr.check_variant(0x1) // r1p1
        && midr.check_architecture(Architecture::IDRegisters)
        && midr.check_part_num(ARM_NEOVERSE_V1_PART_NUM) // V1
        && midr.check_revision(0x1) // r1p1
}

fn is_neoverse_v2(midr: &Midr) -> bool {
    midr.is_arm() // arm
        && midr.check_part_num(ARM_NEOVERSE_V2_PART_NUM) // V2
}

fn is_a64fx(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Fujitsu) && midr.check_part_num(0x1)
}

fn is_apple_m1(midr: &Midr) -> bool {
    midr.is_apple()
        && midr.check_part_num_or(APPLE_M1_FIRESTORM_PART_NUM, APPLE_M1_ICESTORM_PART_NUM)
}

fn is_apple_m1_pro(midr: &Midr) -> bool {
    midr.is_apple()
        && midr.check_part_num_or(
            APPLE_M1_FIRESTORM_PRO_PART_NUM,
            APPLE_M1_ICESTORM_PRO_PART_NUM,
        )
}

fn is_apple_m1_max(midr: &Midr) -> bool {
    midr.is_apple()
        && midr.check_part_num_or(
            APPLE_M1_FIRESTORM_MAX_PART_NUM,
            APPLE_M1_ICESTORM_MAX_PART_NUM,
        )
}

fn is_ampere_1(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Ampere) // ampere
        && midr.check_part_num(AMPERE_1_PART_NUM) // ampere 1
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

        assert!(is_apple_m1(&midr));

        let midr = MidrBuilder::new()
            .implementer(Implementer::Apple)
            .part_num(APPLE_M1_ICESTORM_PART_NUM)
            .build();

        assert!(is_apple_m1(&midr));
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

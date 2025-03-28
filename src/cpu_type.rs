//! This an opinionated cpu type detector for AArch64 cores. It won't detect any core.
//! The coverage is biased.
//!
//! It focuses on modern data-center cores from Arm, Apple cores, and other high-performance cores.
//!
//! [The MIDR for the Neoverse N2](https://developer.arm.com/documentation/102099/0000/AArch64-registers/AArch64-identification-registers/MIDR-EL1--Main-ID-Register) describes the contents of MIDR_EL1 registern on Arm Neoverse N2 cores.
//! The gcc [aarch64 cores](https://github.com/gcc-mirror/gcc/blob/master/gcc/config/aarch64/aarch64-cores.def) has a elaborate list of cores and partial MIDR_EL1 definitions.

use crate::midr::Implementer;
use crate::midr::MidrEL1;

#[non_exhaustive]
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
/// Core kind
pub enum Core {
    /// Arm Neoverse E1 core
    NeoverseE1,
    /// Arm Neoverse N1 core
    NeoverseN1,
    /// Arm Neoverse N2 core
    NeoverseN2,
    /// Arm Neoverse N3 core
    NeoverseN3,
    /// Arm Neoverse V1 core
    NeoverseV1,
    /// Arm Neoverse V2 core
    NeoverseV2,
    /// Arm Neoverse V3 core
    NeoverseV3,
    /// Fujitsu A64FX
    A64FX,
    /// FUJITSU
    MONAKA,
    /// Apple M1
    AppleM1,
    /// Apple M1 Pro
    AppleM1Pro,
    /// Apple M1 Max
    AppleM1Max,
    /// Apple M2
    AppleM2,
    /// Apple M2 Pro
    AppleM2Pro,
    /// Apple M2 Max
    AppleM2Max,
    /// Apple M3
    AppleM3,
    /// Apple M3 Pro
    AppleM3Pro,
    /// Apple M3 Max
    AppleM3Max,
    /// Apple M4
    AppleM4,
    /// Apple M4 Max
    AppleM4Max,
    /// Ampere 1
    Ampere1,
    /// Ampere 1A
    Ampere1A,
    /// Ampere 1B
    Ampere1B,
    /// Apple A14
    AppleA14,
    /// Apple A15
    AppleA15,
    /// Apple A16
    AppleA16,
    /// Apple A18
    AppleA18,
    /// Apple A18 Pro
    AppleA18Pro,
    /// Microsoft Azure Cobalt 100 (custom N2)
    MicrosoftAzureCobalt100,
    /// Nvidia Olympus
    NvidiaOlympus,
}

impl TryFrom<MidrEL1> for Core {
    type Error = &'static str;

    /// try to detect the current core
    fn try_from(value: MidrEL1) -> Result<Self, Self::Error> {
        match value {
            MidrEL1::Unknown => Err("Unsupported arch"),
            MidrEL1::Known(ref midr) => {
                for core_description in CORES {
                    if midr.check_implementer(core_description.implementer)
                        && core_description.variant.check_match(&value)
                    {
                        return Ok(core_description.core);
                    }
                }

                Err("unknown core")
            }
        }
    }
}

// https://github.com/llvm/llvm-project/pull/67591/files

/// arm/cpuid.h (mobile phone?)
const APPLE_ICESTORM_PART_NUM: u64 = 0x20; // energy-efficient
const APPLE_FIRESTORM_PART_NUM: u64 = 0x21; // high-performance maybe A14

/// arm/cpuid.h
const APPLE_BLIZZARD_PART_NUM: u64 = 0x30; // A15 energy-efficient or M2
const APPLE_AVALANCHE_PART_NUM: u64 = 0x31; // A15 high-performance or M2

/// arm/cpuid.h
/// https://github.com/torvalds/linux/blob/master/arch/arm64/include/asm/cputype.h
const APPLE_CPU_PART_M1_ICESTORM: u64 = 0x022;
const APPLE_CPU_PART_M1_FIRESTORM: u64 = 0x023;
const APPLE_CPU_PART_M1_ICESTORM_PRO: u64 = 0x024;
const APPLE_CPU_PART_M1_FIRESTORM_PRO: u64 = 0x025;
const APPLE_CPU_PART_M1_ICESTORM_MAX: u64 = 0x028;
const APPLE_CPU_PART_M1_FIRESTORM_MAX: u64 = 0x029;
const APPLE_CPU_PART_M2_BLIZZARD: u64 = 0x032;
const APPLE_CPU_PART_M2_AVALANCHE: u64 = 0x033;
const APPLE_CPU_PART_M2_BLIZZARD_PRO: u64 = 0x034;
const APPLE_CPU_PART_M2_AVALANCHE_PRO: u64 = 0x035;
const APPLE_CPU_PART_M2_BLIZZARD_MAX: u64 = 0x038;
const APPLE_CPU_PART_M2_AVALANCHE_MAX: u64 = 0x039;

/// arm/cpuid.h

/* H15 e-Core */
const APPLE_SAWTOOTH_PART_NUM: u64 = 0x40; // maybe A16 energy-efficient core
                                           /* H15 p-Core */
const APPLE_EVEREST_PART_NUM: u64 = 0x41; // maybe A16 high-performance core

// ibiza, lobos, and palma are M3s

/* H15 Ibiza e-Core */
const APPLE_ECORE_IBIZA_PART_NUM: u64 = 0x42;
/* H15 Ibiza p-Core */
const APPLE_PCORE_IBIZA_PART_NUM: u64 = 0x43;

/* H15 Lobos e-Core. */
const APPLE_ECORE_LOBOS_PART_NUM: u64 = 0x44;
/* H15 Lobos p-Core. */
const APPLE_PCORE_LOBOS_PART_NUM: u64 = 0x45;

/* M11 e-Core */
const APPLE_SAWTOOTH_M11_PART_NUM: u64 = 0x46;

/* H15 Palma e-Core. */
const APPLE_ECORE_PALMA_PART_NUM: u64 = 0x48;
/* H15 Palma p-Core. */
const APPLE_PCORE_PALMA_PART_NUM: u64 = 0x49;

/* H15 Coll e-Core. */
const APPLE_ECORE_COLL_PART_NUM: u64 = 0x50; // maybe A17
                                             /* H15 Coll p-Core. */
const APPLE_PCORE_COLL_PART_NUM: u64 = 0x51;

/* H16G Donan e-Core. */
const APPLE_ECORE_DONAN_PART_NUM: u64 = 0x52; // M4 small

/* H16H Donan p-Core. */
const APPLE_PCORE_DONAN_PART_NUM: u64 = 0x53;

/* H17P Tahiti e-Core. */
const APPLE_ECORE_TAHITI_PART_NUM: u64 = 0x60; // A18 Pro

/* H17P Tahiti p-Core. */
const APPLE_PCORE_TAHITI_PART_NUM: u64 = 0x61; // A18 Pro

/* H17A Tupai e-Core. */
const APPLE_ECORE_TUPAI_PART_NUM: u64 = 0x6a; // A18

/* H17A Tupai p-Core. */
const APPLE_PCORE_TUPAI_PART_NUM: u64 = 0x6b; // A18

/* H16S Brava S e-Core. */
const APPLE_ECORE_BRAVA_S: u64 = 0x54;

/* H16S Brava S p-Core. */
const APPLE_PCORE_BRAVA_S: u64 = 0x55;

/* H16C Brava C e-Core. */
const APPLE_ECORE_BRAVA_C: u64 = 0x58;

/* H16C Brava C p-Core. */
const APPLE_PCORE_BRAVA_C: u64 = 0x59;

const ARM_NEOVERSE_E1_PART_NUM: u64 = 0xD4A;
const ARM_NEOVERSE_N1_PART_NUM: u64 = 0xD0C;
const ARM_NEOVERSE_N2_PART_NUM: u64 = 0xD49;
const ARM_NEOVERSE_N3_PART_NUM: u64 = 0xD8E;
const ARM_NEOVERSE_V1_PART_NUM: u64 = 0xD40;
const ARM_NEOVERSE_V2_PART_NUM: u64 = 0xD4F;
const ARM_NEOVERSE_V3_PART_NUM: u64 = 0xD84;

const AMPERE_1_PART_NUM: u64 = 0xac3;
const AMPERE_1A_PART_NUM: u64 = 0xac4;
const AMPERE_1B_PART_NUM: u64 = 0xac5;

const FUJITSU_A64FX_PART_NUM: u64 = 0x001;
const FUJITSU_MONAKA_PART_NUM: u64 = 0x003;

const MICROSOFT_AZURE_COBALT100_PART_NUM: u64 = 0xd49;

const NVIDIA_OLYMPUS_PART_NUM: u64 = 0x010;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::midr::{Midr, MidrBuilder};

    fn try_from(value: &MidrEL1) -> Option<Core> {
        match value {
            MidrEL1::Unknown => None,
            MidrEL1::Known(midr) => {
                for core_description in CORES {
                    if midr.check_implementer(core_description.implementer)
                        && core_description.variant.check_match(&value)
                    {
                        return Some(core_description.core);
                    }
                }
                None
            }
        }
    }

    #[test]
    fn test_apple_m1() {
        let midr = MidrBuilder::new()
            .implementer(Implementer::Apple)
            .part_num(APPLE_FIRESTORM_PART_NUM)
            .build();
        let midr_el1 = MidrEL1::Known(midr.clone());

        assert!(midr.check_part_num(APPLE_FIRESTORM_PART_NUM));
        assert!(midr.check_implementer(Apple));

        assert!(Or(APPLE_FIRESTORM_PART_NUM, APPLE_CPU_PART_M1_ICESTORM).check_match(&midr_el1));

        let core_option = try_from(&midr_el1);
        assert!(core_option.is_some());

        assert!(core_option.unwrap_or(AppleM1Max) == AppleM1);

        let midr = MidrBuilder::new()
            .implementer(Implementer::Apple)
            .part_num(APPLE_CPU_PART_M1_ICESTORM)
            .build();

        let core_option = try_from(&midr_el1);
        assert!(core_option.is_some());
        assert!(core_option.unwrap_or(AppleM1Max) == AppleM1);

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

// https://github.com/rust-lang/rust/blob/master/src/tools/rustfmt/tests/source/cfg_if/detect/os/aarch64.rs

#[allow(unused)]
#[derive(Eq, PartialEq)]
struct CoreDescription {
    core: Core,
    implementer: Implementer,
    variant: PartNumMatcher,
}

#[allow(unused)]
#[derive(Eq, PartialEq)]
enum PartNumMatcher {
    One(u64),
    Or(u64, u64),
}

impl PartNumMatcher {
    fn check_match(&self, value: &MidrEL1) -> bool {
        match value {
            MidrEL1::Unknown => false,
            MidrEL1::Known(midr) => match self {
                PartNumMatcher::One(one) => midr.check_part_num(*one),
                PartNumMatcher::Or(one, two) => {
                    midr.check_part_num(*one) || midr.check_part_num(*two)
                }
            },
        }
    }
}

macro_rules! declare_cores {
    ($(
        ($core:ident, $implementer:ident, $variant:expr),
    )+) => {
        /// My favorite cores
        use crate::cpu_type::PartNumMatcher::*;
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
    (NeoverseE1, Arm,     One(ARM_NEOVERSE_E1_PART_NUM)),
    (NeoverseN1, Arm,     One(ARM_NEOVERSE_N1_PART_NUM)),
    (NeoverseN2, Arm,     One(ARM_NEOVERSE_N2_PART_NUM)),
    (NeoverseN3, Arm,     One(ARM_NEOVERSE_N3_PART_NUM)),
    (NeoverseV1, Arm,     One(ARM_NEOVERSE_V1_PART_NUM)),
    (NeoverseV2, Arm,     One(ARM_NEOVERSE_V2_PART_NUM)),
    (NeoverseV3, Arm,     One(ARM_NEOVERSE_V3_PART_NUM)),
    (Ampere1,    Ampere,  One(AMPERE_1_PART_NUM)),
    (Ampere1A,   Ampere,  One(AMPERE_1A_PART_NUM)),
    (Ampere1B,   Ampere,  One(AMPERE_1B_PART_NUM)),
    (AppleM1,    Apple,   Or(APPLE_CPU_PART_M1_ICESTORM, APPLE_CPU_PART_M1_FIRESTORM)),
    (AppleM1Pro, Apple,   Or(APPLE_CPU_PART_M1_ICESTORM_PRO, APPLE_CPU_PART_M1_FIRESTORM_PRO)),
    (AppleM1Max, Apple,   Or(APPLE_CPU_PART_M1_ICESTORM_MAX, APPLE_CPU_PART_M1_FIRESTORM_MAX)),
    (AppleM2,    Apple,   Or(APPLE_CPU_PART_M2_BLIZZARD, APPLE_CPU_PART_M2_AVALANCHE)),
    (AppleM2Pro, Apple,   Or(APPLE_CPU_PART_M2_BLIZZARD_PRO, APPLE_CPU_PART_M2_AVALANCHE_PRO)),
    (AppleM2Max, Apple,   Or(APPLE_CPU_PART_M2_BLIZZARD_MAX, APPLE_CPU_PART_M2_AVALANCHE_MAX)),
    (AppleM3,    Apple,   Or(APPLE_ECORE_IBIZA_PART_NUM, APPLE_PCORE_IBIZA_PART_NUM)),
    (AppleM3Pro, Apple,   Or(APPLE_ECORE_LOBOS_PART_NUM, APPLE_PCORE_LOBOS_PART_NUM)),
    (AppleM3Max, Apple,   Or(APPLE_ECORE_PALMA_PART_NUM, APPLE_PCORE_PALMA_PART_NUM)),
    (AppleM4,    Apple,   Or(APPLE_ECORE_DONAN_PART_NUM, APPLE_PCORE_DONAN_PART_NUM)),
    (AppleA14,   Apple,   Or(APPLE_ICESTORM_PART_NUM, APPLE_FIRESTORM_PART_NUM)),
    (AppleA15,   Apple,   Or(APPLE_BLIZZARD_PART_NUM, APPLE_AVALANCHE_PART_NUM)),
    (AppleA16,   Apple,   Or(APPLE_SAWTOOTH_PART_NUM, APPLE_EVEREST_PART_NUM)),
    (AppleA18,   Apple,   Or(APPLE_ECORE_TUPAI_PART_NUM, APPLE_PCORE_TUPAI_PART_NUM)),
    (AppleA18Pro,   Apple,   Or(APPLE_ECORE_TAHITI_PART_NUM, APPLE_PCORE_TAHITI_PART_NUM)),
    (A64FX,      Fujitsu, One(FUJITSU_A64FX_PART_NUM)),
    (MONAKA,     Fujitsu, One(FUJITSU_MONAKA_PART_NUM)),
    (MicrosoftAzureCobalt100, Microsoft, One(MICROSOFT_AZURE_COBALT100_PART_NUM)),
    (NvidiaOlympus, Nvidia, One(NVIDIA_OLYMPUS_PART_NUM)),
);

// https://www.anandtech.com/show/21116/apple-announces-m3-soc-family-m3-m3-pro-and-m3-max-make-their-marks

// Grace: https://github.com/openucx/ucx/pull/9479/files

// https://github.com/llvm/llvm-project/pull/77793

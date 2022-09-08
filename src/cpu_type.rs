use crate::midr::Architecture;
use crate::midr::Implementer;
use crate::midr::Midr;

// https://developer.arm.com/documentation/ddi0595/2020-12/AArch64-Registers/MIDR-EL1--Main-ID-Register

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Support/Host.cpp

// https://github.com/torvalds/linux/blob/master/arch/arm64/include/asm/cputype.h

#[derive(Debug)]
#[non_exhaustive]
/// Core kind
pub enum Core {
    /// ARM Neoverse N1 core
    NeoverseN1,
    /// ARM Neoverse N2 core
    NeoverseN2,
    /// ARM Neoverse V1 core
    NeoverseV1,
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
    /// unknown core
    Unknown,
}

#[cfg(target_arch = "aarch64")]
/// try to detect the current core
pub fn detect_core() -> Core {
    let midr = Midr::new();

    if is_neoverse_n1(&midr) {
        Core::NeoverseN1
    } else if is_neoverse_n2(&midr) {
        Core::NeoverseN2
    } else if is_neoverse_v1(&midr) {
        Core::NeoverseV1
    } else if is_a64fx(&midr) {
        Core::A64FX
    } else if is_apple_m1(&midr) {
        Core::AppleM1
    } else if is_apple_m1_pro(&midr) {
        Core::AppleM1Pro
    } else if is_apple_m1_max(&midr) {
        Core::AppleM1Max
    } else if is_ampere_1(&midr) {
        Core::Ampere1
    } else {
        Core::Unknown
    }
}

#[cfg(not(target_arch = "aarch64"))]
/// try to detect the current core
pub fn detect_core() {}

fn is_neoverse_n1(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Arm) // arm
        && midr.check_variant(0x4)
        && midr.check_architecture(Architecture::IDRegisters)
        && midr.check_part_num(ARM_NEOVERSE_N1_PART_NUM) // N1
        && midr.check_revision(0x0) // r4p0
}

// https://developer.arm.com/documentation/102099/0000/AArch64-registers/AArch64-identification-registers/MIDR-EL1--Main-ID-Register
fn is_neoverse_n2(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Arm) // arm
        && midr.check_variant(0x0) // r0p0
        && midr.check_architecture(Architecture::IDRegisters)
        && midr.check_part_num(ARM_NEOVERSE_N2_PART_NUM) // N2
        && midr.check_revision(0x0) // r0p0
}

fn is_neoverse_v1(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Arm) // arm
        && midr.check_variant(0x1) // r1p1
        && midr.check_architecture(Architecture::IDRegisters)
        && midr.check_part_num(ARM_NEOVERSE_V1_PART_NUM) // V1
        && midr.check_revision(0x1) // r1p1
}

fn is_a64fx(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Fujitsu) && midr.check_part_num(0x1)
}

fn is_apple_m1(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Apple)
        && midr.check_part_num_or(APPLE_M1_FIRESTORM_PART_NUM, APPLE_M1_ICESTORM_PART_NUM)
}

fn is_apple_m1_pro(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Apple)
        && midr.check_part_num_or(
            APPLE_M1_FIRESTORM_PRO_PART_NUM,
            APPLE_M1_ICESTORM_PRO_PART_NUM,
        )
}

fn is_apple_m1_max(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Apple)
        && midr.check_part_num_or(
            APPLE_M1_FIRESTORM_MAX_PART_NUM,
            APPLE_M1_ICESTORM_MAX_PART_NUM,
        )
}

fn is_ampere_1(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Ampere) // ampere
        && midr.check_part_num(AMPERE_1_PART_NUM) // ampere 1
}

const APPLE_M1_FIRESTORM_PART_NUM: u64 = 0x22;
const APPLE_M1_ICESTORM_PART_NUM: u64 = 0x23;

const APPLE_M1_FIRESTORM_PRO_PART_NUM: u64 = 0x24;
const APPLE_M1_ICESTORM_PRO_PART_NUM: u64 = 0x25;

const APPLE_M1_FIRESTORM_MAX_PART_NUM: u64 = 0x28;
const APPLE_M1_ICESTORM_MAX_PART_NUM: u64 = 0x29;

const ARM_NEOVERSE_N1_PART_NUM: u64 = 0xD0C;
const ARM_NEOVERSE_N2_PART_NUM: u64 = 0xD49;
const ARM_NEOVERSE_V1_PART_NUM: u64 = 0xD40;

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

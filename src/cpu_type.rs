//MIDR-EL1

// https://developer.arm.com/documentation/ddi0595/2020-12/AArch64-Registers/MIDR-EL1--Main-ID-Register

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Support/Host.cpp

// https://github.com/torvalds/linux/blob/master/arch/arm64/include/asm/cputype.h

enum Implementer {
    Arm = 0x41,
    Fujitsu = 0x46,
    Apple = 0x61,
}

#[derive(Debug)]
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
    /// unknown core
    Unknown,
}

#[cfg(target_arch = "aarch64")]
#[deprecated(note = "experimental")]
/// try to detect the current core
pub fn detect_core() -> Core {
    use std::arch::asm;
    let mut tmp: u64;

    unsafe {
        asm!("mrs {tmp}, MIDR_EL1", tmp = out(reg) tmp);
    }

    let implementer = (tmp >> 24) & 0b11111111;
    let variant = (tmp >> 20) & 0b1111;
    let architecture = (tmp >> 16) & 0b1111;
    let part_num = (tmp >> 4) & 0b111111111111; // FIXME
    let revision = tmp & 0b1111;

    let midr = MIDR {
        implementer,
        variant,
        architecture,
        part_num,
        revision,
    };

    if is_neoverse_n1(&midr) {
        return Core::NeoverseN1;
    } else if is_neoverse_n2(&midr) {
        return Core::NeoverseN2;
    } else if is_neoverse_v1(&midr) {
        return Core::NeoverseV1;
    } else if is_a64fx(&midr) {
        return Core::A64FX;
    } else {
        return Core::Unknown;
    }
}

#[cfg(not(target_arch = "aarch64"))]
/// try to detect the current core
pub fn detect_core() {}

struct MIDR {
    implementer: u64,
    variant: u64,
    architecture: u64,
    part_num: u64,
    revision: u64,
}

fn is_neoverse_n1(midr: &MIDR) -> bool {
    midr.implementer == Implementer::Arm as u64// arm
        && midr.variant == 0x4
        && midr.architecture == 0xf
        && midr.part_num == 0xd0c // N1
        && midr.revision == 0x0 // r4p0
}

fn is_neoverse_n2(midr: &MIDR) -> bool {
    midr.implementer == Implementer::Arm as u64 // arm
        && midr.variant == 0x0 // r0p0
        && midr.architecture == 0xf
        && midr.part_num == 0xd49 // N2
        && midr.revision == 0x0 // r0p0
}

fn is_neoverse_v1(midr: &MIDR) -> bool {
    midr.implementer == Implementer::Arm as u64 // arm
        && midr.variant == 0x1 // r1p1
        && midr.architecture == 0xf
        && midr.part_num == 0xd40 // V1
        && midr.revision == 0x1 // r1p1
}

fn is_a64fx(midr: &MIDR) -> bool {
    return midr.implementer == Implementer::Fujitsu as u64 && midr.part_num == 0x1;
}

//#define APPLE_CPU_PART_M1_ICESTORM	0x022
//#define APPLE_CPU_PART_M1_FIRESTORM	0x023
//#define APPLE_CPU_PART_M1_ICESTORM_PRO	0x024
//#define APPLE_CPU_PART_M1_FIRESTORM_PRO	0x025
//#define APPLE_CPU_PART_M1_ICESTORM_MAX	0x028
//#define APPLE_CPU_PART_M1_FIRESTORM_MAX	0x029

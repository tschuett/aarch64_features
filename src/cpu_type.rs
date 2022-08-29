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

    let midr = Midr {
        implementer,
        variant,
        architecture,
        part_num,
        revision,
    };

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
    } else {
        Core::Unknown
    }
}

#[cfg(not(target_arch = "aarch64"))]
/// try to detect the current core
pub fn detect_core() {}

struct Midr {
    implementer: u64,
    variant: u64,
    architecture: u64,
    part_num: u64,
    revision: u64,
}

impl Midr {
    fn check_implementer(&self, im: Implementer) -> bool {
        self.implementer == im as u64
    }

    fn check_part_num(&self, part_num: u64) -> bool {
        self.part_num == part_num
    }

    fn check_part_num_or(&self, part_num0: u64, part_num1: u64) -> bool {
        self.part_num == part_num0 || self.part_num == part_num1
    }
}

fn is_neoverse_n1(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Arm)
        && midr.variant == 0x4
        && midr.architecture == 0xf
        && midr.check_part_num(ARM_NEOVERSE_N1_PART_NUM) // N1
        && midr.revision == 0x0 // r4p0
}

fn is_neoverse_n2(midr: &Midr) -> bool {
    midr.implementer == Implementer::Arm as u64 // arm
        && midr.variant == 0x0 // r0p0
        && midr.architecture == 0xf
        && midr.check_part_num(ARM_NEOVERSE_N2_PART_NUM) // N2
        && midr.revision == 0x0 // r0p0
}

fn is_neoverse_v1(midr: &Midr) -> bool {
    midr.implementer == Implementer::Arm as u64 // arm
        && midr.variant == 0x1 // r1p1
        && midr.architecture == 0xf
        && midr.check_part_num(ARM_NEOVERSE_V1_PART_NUM) // V1
        && midr.revision == 0x1 // r1p1
}

fn is_a64fx(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Fujitsu) && midr.part_num == 0x1
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

const APPLE_M1_FIRESTORM_PART_NUM: u64 = 0x22;
const APPLE_M1_ICESTORM_PART_NUM: u64 = 0x23;

const APPLE_M1_FIRESTORM_PRO_PART_NUM: u64 = 0x24;
const APPLE_M1_ICESTORM_PRO_PART_NUM: u64 = 0x25;

const APPLE_M1_FIRESTORM_MAX_PART_NUM: u64 = 0x28;
const APPLE_M1_ICESTORM_MAX_PART_NUM: u64 = 0x29;

const ARM_NEOVERSE_N1_PART_NUM: u64 = 0xD0C;
const ARM_NEOVERSE_N2_PART_NUM: u64 = 0xD49;
const ARM_NEOVERSE_V1_PART_NUM: u64 = 0xD40;

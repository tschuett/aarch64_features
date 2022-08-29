//MIDR-EL1

// https://developer.arm.com/documentation/ddi0595/2020-12/AArch64-Registers/MIDR-EL1--Main-ID-Register

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Support/Host.cpp

// https://github.com/torvalds/linux/blob/master/arch/arm64/include/asm/cputype.h

enum Implementer {
    Arm = 0x41,
    Fujitsu = 0x46,
    Apple = 0x61,
    Ampere = 0xc0,
}

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

struct Midr {
    implementer: u64,
    variant: u64,
    architecture: u64,
    part_num: u64,
    revision: u64,
}

impl Midr {
    fn new() -> Self {
        use std::arch::asm;
        let mut midr: u64;

        unsafe {
            asm!("mrs {midr}, MIDR_EL1", midr = out(reg) midr);
        }

        let implementer = extract(midr, MIDR_IMPLEMENTOR_SHIFT, MIDR_IMPLEMENTOR_MASK);
        let variant = extract(midr, MIDR_VARIANT_SHIFT, MIDR_VARIANT_MASK);
        let architecture = extract(midr, MIDR_ARCHITECTURE_SHIFT, MIDR_ARCHITECTURE_MASK);
        let part_num = extract(midr, MIDR_PART_NUM_SHIFT, MIDR_PART_NUM_MASK);
        let revision = extract(midr, MIDR_REVISION_SHIFT, MIDR_REVISION_MASK);

        Midr {
            implementer,
            variant,
            architecture,
            part_num,
            revision,
        }
    }

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
    midr.check_implementer(Implementer::Arm) // arm
        && midr.variant == 0x4
        && midr.architecture == 0xf
        && midr.check_part_num(ARM_NEOVERSE_N1_PART_NUM) // N1
        && midr.revision == 0x0 // r4p0
}

fn is_neoverse_n2(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Arm) // arm
        && midr.variant == 0x0 // r0p0
        && midr.architecture == 0xf
        && midr.check_part_num(ARM_NEOVERSE_N2_PART_NUM) // N2
        && midr.revision == 0x0 // r0p0
}

fn is_neoverse_v1(midr: &Midr) -> bool {
    midr.check_implementer(Implementer::Arm) // arm
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

const MIDR_IMPLEMENTOR_SHIFT: u64 = 24;
const MIDR_VARIANT_SHIFT: u64 = 24;
const MIDR_ARCHITECTURE_SHIFT: u64 = 24;
const MIDR_PART_NUM_SHIFT: u64 = 4;
const MIDR_REVISION_SHIFT: u64 = 0;

const MIDR_REVISION_MASK: u64 = 0xf;
const MIDR_PART_NUM_MASK: u64 = 0xfff << MIDR_PART_NUM_SHIFT;
const MIDR_ARCHITECTURE_MASK: u64 = 0xf << MIDR_ARCHITECTURE_SHIFT;
const MIDR_VARIANT_MASK: u64 = 0xf << MIDR_VARIANT_SHIFT;
const MIDR_IMPLEMENTOR_MASK: u64 = 0xff << MIDR_IMPLEMENTOR_SHIFT;

fn extract(midr: u64, shift: u64, mask: u64) -> u64 {
    (midr >> shift) & mask
}

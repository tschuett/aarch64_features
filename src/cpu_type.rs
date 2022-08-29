//MIDR-EL1

// https://developer.arm.com/documentation/ddi0595/2020-12/AArch64-Registers/MIDR-EL1--Main-ID-Register

// https://github.com/llvm/llvm-project/blob/main/llvm/lib/Support/Host.cpp

// https://github.com/torvalds/linux/blob/master/arch/arm64/include/asm/cputype.h

enum Implementers {
    Arm = 0x41,
    Apple = 0x61,
}

#[cfg(target_arch = "aarch64")]
fn cpu_detection() {
    use std::arch::asm;
    let mut tmp: u64;
    asm!("mrs {tmp}, MIDR_EL1", tmp = out(reg) tmp);

    let implementer = (tmp >> 24) & 0b11111111;
    let variant = (tmp >> 20) & 0b1111;
    let architecture = (tmp >> 16) & 0b1111;
    let part_num = (tmp >> 4) & 0b111111111111; // FIXME
    let revision = tmp & 0b1111;
}

#[cfg(not(target_arch = "aarch64"))]
fn cpu_detection() {}

struct MIDR {
    implementer: u32,
    variant: u32,
    architecture: u32,
    part_num: u32,
    revision: u32,
}

fn is_neoverse_n1(midr: &MIDR) -> bool {
    midr.implementer == Implementers::Arm as u32// arm
        && midr.variant == 0x4
        && midr.architecture == 0xf
        && midr.part_num == 0xd0c // N1
        && midr.revision == 0x0 // r4p0
}

fn is_neoverse_n2(midr: &MIDR) -> bool {
    midr.implementer == Implementers::Arm as u32 // arm
        && midr.variant == 0x0 // r0p0
        && midr.architecture == 0xf
        && midr.part_num == 0xd49 // N2
        && midr.revision == 0x0 // r0p0
}

fn is_neoverse_v1(midr: &MIDR) -> bool {
    midr.implementer == Implementers::Arm as u32 // arm
        && midr.variant == 0x1 // r1p1
        && midr.architecture == 0xf
        && midr.part_num == 0xd40 // V1
        && midr.revision == 0x1 // r1p1
}

//#define APPLE_CPU_PART_M1_ICESTORM	0x022
//#define APPLE_CPU_PART_M1_FIRESTORM	0x023
//#define APPLE_CPU_PART_M1_ICESTORM_PRO	0x024
//#define APPLE_CPU_PART_M1_FIRESTORM_PRO	0x025
//#define APPLE_CPU_PART_M1_ICESTORM_MAX	0x028
//#define APPLE_CPU_PART_M1_FIRESTORM_MAX	0x029

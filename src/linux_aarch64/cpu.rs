use crate::Feature;
use libc::AT_HWCAP;
use libc::HWCAP_CPUID;
// MIDR_EL1
use crate::linux_exported_features::EXPORTED_FEATURES;

/// see https://www.kernel.org/doc/html/latest/arm64/cpu-feature-registers.html
pub(crate) fn check_availability() -> bool {
    let caps = unsafe { libc::getauxval(AT_HWCAP) };
    if (caps & HWCAP_CPUID) != 0 {
        return false;
    }
    true
}

pub(crate) fn is_exposed_to_userspace(feat: Feature) -> bool {
    EXPORTED_FEATURES.contains(&feat)
}

// https://developer.arm.com/documentation/102099/0000/AArch64-registers/AArch64-identification-registers/MIDR-EL1--Main-ID-Register

// https://developer.arm.com/documentation/100616/0301/register-descriptions/aarch64-system-registers/midr-el1--main-id-register--el1

// https://developer.arm.com/documentation/ddi0595/2021-06/External-Registers/MIDR-EL1--Main-ID-Register

// https://mirage-rs.github.io/libtegra/src/cortex_a/regs/midr_el1.rs.html

// https://docs.rs/aarch64/0.0.6/aarch64/regs/MIDR_EL1/index.html

// https://github.com/klauspost/cpuid/blob/master/detect_arm64.go

// https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/zircon/kernel/arch/arm64/feature.cc

// https://github.com/torvalds/linux/blob/master/arch/arm64/kernel/cpuinfo.c

// https://github.com/ARM-software/acle/pull/21

// ID_AA64PFR1_EL1

// https://blog.rust-lang.org/2022/02/24/Rust-1.59.0.html

// https://github.com/torvalds/linux/blob/master/Documentation/arm64/elf_hwcaps.rst

// https://github.com/torvalds/linux/blob/master/Documentation/arch/arm64/cpu-feature-registers.rst

// https://github.com/torvalds/linux/blob/master/Documentation/arm64/sve.rst

// https://lwn.net/Articles/871473/

// https://git.kernel.org/pub/scm/linux/kernel/git/arm64/linux.git/tree/Documentation/arm64/sme.rst?h=for-next/sme

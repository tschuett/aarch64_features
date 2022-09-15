use crate::registers_info::Register;

// let aa64isar0: u64;
// unsafe { asm!("mrs $0, ID_AA64ISAR0_EL1" : "=r"(aa64isar0)); }

#[cfg(target_arch = "aarch64")]
pub(crate) fn read_register(register: Register) -> u64 {
    use std::arch::asm;
    let mut tmp: u64;

    match register {
        Register::ID_AA64DFR0_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64DFR0_EL1", tmp = out(reg) tmp);
        },
        Register::ID_AA64ISAR0_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64ISAR0_EL1_EL1", tmp = out(reg) tmp);
        },
        Register::ID_AA64ISAR1_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64ISAR1_EL1", tmp = out(reg) tmp);
        },
        Register::ID_AA64MMFR0_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64MMFR0_EL1", tmp = out(reg) tmp);
        },
        Register::ID_AA64MMFR1_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64MMFR1_EL1", tmp = out(reg) tmp);
        },
        Register::ID_AA64PFR1_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64PFR1_EL1", tmp = out(reg) tmp);
        },
        Register::ID_AA64PFR0_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64PFR0_EL1", tmp = out(reg) tmp);
        },
        Register::ID_AA64MMFR2_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64MMFR2_EL1", tmp = out(reg) tmp);
        },
        Register::EDDEVID => unsafe {
            asm!("mrs {tmp}, EDDEVID", tmp = out(reg) tmp);
        },
        Register::ID_AA64SMFR0_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64SMFR0_EL1", tmp = out(reg) tmp);
        },
        Register::ID_AA64ISAR2_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64ISAR2_EL1", tmp = out(reg) tmp);
        },
        Register::CNTID => unsafe {
            asm!("mrs {tmp}, CNTID", tmp = out(reg) tmp);
        },
        Register::ID_AA64ZFR0_EL1 => unsafe {
            asm!("mrs {tmp}, ID_AA64ZFR0_EL1", tmp = out(reg) tmp);
        },
        Register::TRCDEVARCH => unsafe {
            asm!("mrs {tmp}, TRCDEVARCH", tmp = out(reg) tmp);
        },
        Register::CTR_EL0 => unsafe {
            asm!("mrs {tmp}, CTR_EL0", tmp = out(reg) tmp);
        },
    }

    tmp
}

#[cfg(not(target_arch = "aarch64"))]
pub(crate) fn read_register(_register: Register) -> u64 {
    0
}

// FIXME: audit switch statement and asm!s

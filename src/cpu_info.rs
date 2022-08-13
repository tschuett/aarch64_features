use std::ops::Index;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub(crate) enum Register {
    ID_AA64DFR0_EL1,
    // ID_AA64DFR1_EL1,
    ID_AA64ISAR0_EL1,
    ID_AA64ISAR1_EL1,
    ID_AA64MMFR0_EL1,
    ID_AA64MMFR1_EL1,
    ID_AA64MMFR2_EL1,
    ID_AA64PFR0_EL1,
    ID_AA64PFR1_EL1,
    //ID_AA64ZFR0_EL1,
    EDDEVID,
    CTR_EL0,
    ID_AA64SMFR0_EL1,
    CNTID,
    ID_AA64ISAR2_EL1,
    TRCDEVARCH,
    ID_AA64ZFR0_EL1,
}

pub(crate) struct CPUInfo {
    id_aa64dfr0: u64,
    //id_aa64dfr1: u64,
    id_aa64isar0: u64,
    id_aa64isar1: u64,
    id_aa64mmfr0: u64,
    id_aa64mmfr1: u64,
    id_aa64mmfr2: u64,
    id_aa64pfr0: u64,
    id_aa64pfr1: u64,
    //id_aa64zfr0: u64,
    //zcr: u64,
    ctr_el0: u64,
    eddevid: u64,
    id_aa64smfr0_el1: u64,
    cntid: u64,
    id_aa64isar2_el1: u64,
    trcdevarch: u64,
    id_aa64zfr0: u64,
}

impl CPUInfo {
    pub(crate) fn new() -> CPUInfo {
        CPUInfo {
            id_aa64dfr0: CPUInfo::read_register("ID_DFR0_EL1"),
            //id_aa64dfr1: CPUInfo::read_register("ID_AA64DFR1_EL1"),
            id_aa64isar0: CPUInfo::read_register("ID_AA64ISAR0_EL1"),
            id_aa64isar1: CPUInfo::read_register("ID_AA64ISAR1_EL1"),
            id_aa64mmfr0: CPUInfo::read_register("ID_AA64MMFR0_EL1"),
            id_aa64mmfr1: CPUInfo::read_register("ID_AA64MMFR1_EL1"),
            id_aa64mmfr2: CPUInfo::read_register("ID_AA64MMFR2_EL1"),
            id_aa64pfr0: CPUInfo::read_register("ID_AA64PFR0_EL1"),
            id_aa64pfr1: CPUInfo::read_register("ID_AA64PFR1_EL1"),
            //id_aa64zfr0: CPUInfo::read_register("ID_AA64ZFR0_EL1"),
            //zcr: u64,
            ctr_el0: CPUInfo::read_register("CTR_EL0"),
            eddevid: CPUInfo::read_register("EDDEVID"),
            id_aa64smfr0_el1: CPUInfo::read_register("ID_AA64SMFR0_EL1"),
            cntid: CPUInfo::read_register("CNTID"),
            id_aa64isar2_el1: CPUInfo::read_register("ID_AA64ISAR2_EL1"),
            trcdevarch: CPUInfo::read_register("TRCDEVARCH"),
            id_aa64zfr0: CPUInfo::read_register("ID_AA64ZFR0_EL1"),
        }
    }

    #[cfg(target_arch = "aarch64")]
    fn read_register(reg: &str) -> u64 {
        use core::arch::aarch64::__rsr64;
        __rsr64(reg)
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn read_register(_reg: &str) -> u64 {
        0
    }
}

impl Index<Register> for CPUInfo {
    type Output = u64;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::ID_AA64DFR0_EL1 => &self.id_aa64dfr0,
            //Register::ID_AA64DFR1_EL1 => &self.id_aa64dfr1,
            Register::ID_AA64ISAR0_EL1 => &self.id_aa64isar0,
            Register::ID_AA64ISAR1_EL1 => &self.id_aa64isar1,
            Register::ID_AA64MMFR0_EL1 => &self.id_aa64mmfr0,
            Register::ID_AA64MMFR1_EL1 => &self.id_aa64mmfr1,
            Register::ID_AA64MMFR2_EL1 => &self.id_aa64mmfr2,
            Register::ID_AA64PFR0_EL1 => &self.id_aa64pfr0,
            Register::ID_AA64PFR1_EL1 => &self.id_aa64pfr1,
            //Register::ID_AA64ZFR0_EL1 => &self.id_aa64zfr0,
            Register::EDDEVID => &self.eddevid,
            Register::CTR_EL0 => &self.ctr_el0,
            Register::ID_AA64SMFR0_EL1 => &self.id_aa64smfr0_el1,
            Register::CNTID => &self.cntid,
            Register::ID_AA64ISAR2_EL1 => &self.id_aa64isar2_el1,
            Register::TRCDEVARCH => &self.trcdevarch,
            Register::ID_AA64ZFR0_EL1 => &self.id_aa64zfr0,
        }
    }
}

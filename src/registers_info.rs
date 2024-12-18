use crate::aarch64::read_register;
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
    ID_AA64ISAR3_EL1,
    //ID_AA64ZFR0_EL1,
    EDDEVID,
    CTR_EL0,
    ID_AA64SMFR0_EL1,
    CNTID,
    ID_AA64ISAR2_EL1,
    TRCDEVARCH,
    ID_AA64ZFR0_EL1,
    NoRegister,
}

pub(crate) struct RegistersInfo {
    id_aa64dfr0: u64,
    //id_aa64dfr1: u64,
    id_aa64isar0: u64,
    id_aa64isar1: u64,
    id_aa64mmfr0: u64,
    id_aa64mmfr1: u64,
    id_aa64mmfr2: u64,
    id_aa64pfr0: u64,
    id_aa64pfr1: u64,
    id_aa64isar3_el1: u64,
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

impl RegistersInfo {
    pub(crate) fn new() -> RegistersInfo {
        RegistersInfo {
            id_aa64dfr0: read_register(Register::ID_AA64DFR0_EL1),
            //id_aa64dfr1: CPUInfo::read_register(Register::ID_AA64DFR1_EL1),
            id_aa64isar0: read_register(Register::ID_AA64ISAR0_EL1),
            id_aa64isar1: read_register(Register::ID_AA64ISAR1_EL1),
            id_aa64mmfr0: read_register(Register::ID_AA64MMFR0_EL1),
            id_aa64mmfr1: read_register(Register::ID_AA64MMFR1_EL1),
            id_aa64mmfr2: read_register(Register::ID_AA64MMFR2_EL1),
            id_aa64pfr0: read_register(Register::ID_AA64PFR0_EL1),
            id_aa64pfr1: read_register(Register::ID_AA64PFR1_EL1),
            id_aa64isar3_el1: read_register(Register::ID_AA64ISAR3_EL1),
            //id_aa64zfr0: CPUInfo::read_register(Register::ID_AA64ZFR0_EL1),
            //zcr: u64,
            ctr_el0: read_register(Register::CTR_EL0),
            eddevid: read_register(Register::EDDEVID),
            id_aa64smfr0_el1: read_register(Register::ID_AA64SMFR0_EL1),
            cntid: read_register(Register::CNTID),
            id_aa64isar2_el1: read_register(Register::ID_AA64ISAR2_EL1),
            trcdevarch: read_register(Register::TRCDEVARCH),
            id_aa64zfr0: read_register(Register::ID_AA64ZFR0_EL1),
        }
    }
}

impl Index<Register> for RegistersInfo {
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
            Register::ID_AA64ISAR3_EL1 => &self.id_aa64isar3_el1,
            //Register::ID_AA64ZFR0_EL1 => &self.id_aa64zfr0,
            Register::EDDEVID => &self.eddevid,
            Register::CTR_EL0 => &self.ctr_el0,
            Register::ID_AA64SMFR0_EL1 => &self.id_aa64smfr0_el1,
            Register::CNTID => &self.cntid,
            Register::ID_AA64ISAR2_EL1 => &self.id_aa64isar2_el1,
            Register::TRCDEVARCH => &self.trcdevarch,
            Register::ID_AA64ZFR0_EL1 => &self.id_aa64zfr0,
            Register::NoRegister => todo!(),
        }
    }
}

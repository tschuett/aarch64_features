use crate::cpu_type::Implementer;

pub(crate) struct Midr {
    implementer: u64,
    variant: u64,
    architecture: u64,
    part_num: u64,
    revision: u64,
}

impl Midr {
    pub(crate) fn new() -> Self {
        use std::arch::asm;
        let mut midr: u64;

        unsafe {
            asm!("mrs {midr}, MIDR_EL1", midr = out(reg) midr);
        }

        Self::extract_parts(midr)
    }

    //    #[cfg(test)]
    //    fn new_test(midr: u64) -> Self {
    //        println!("new_test: {}", midr);
    //        let result = Self::extract_parts(midr);
    //        result.dump();
    //        result
    //    }

    fn extract_parts(midr: u64) -> Self {
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

    pub(crate) fn check_implementer(&self, im: Implementer) -> bool {
        if cfg!(test) {
            // do test stuff
        } else {
            // do non-test stuff
        }
        self.implementer == im as u64
    }

    pub(crate) fn check_variant(&self, im: u64) -> bool {
        self.variant == im as u64
    }

    pub(crate) fn check_architecture(&self, im: u64) -> bool {
        self.architecture == im as u64
    }

    pub(crate) fn check_part_num(&self, part_num: u64) -> bool {
        self.part_num == part_num
    }

    pub(crate) fn check_part_num_or(&self, part_num0: u64, part_num1: u64) -> bool {
        self.part_num == part_num0 || self.part_num == part_num1
    }

    pub(crate) fn check_revision(&self, im: u64) -> bool {
        self.revision == im as u64
    }

    fn dump(&self) {
        println!("implementer : {}", self.implementer);
        println!("variant     : {}", self.variant);
        println!("architecture: {}", self.architecture);
        println!("part_num    : {}", self.part_num);
        println!("revision    : {}", self.revision);
    }
}

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

pub(crate) struct MidrBuilder {
    implementer: Option<u64>,
    variant: Option<u64>,
    architecture: Option<u64>,
    part_num: Option<u64>,
    revision: Option<u64>,
}

impl MidrBuilder {
    pub(crate) fn new() -> Self {
        MidrBuilder {
            implementer: None,
            variant: None,
            architecture: None,
            part_num: None,
            revision: None,
        }
    }

    pub(crate) fn implementer(mut self, im: Implementer) -> MidrBuilder {
        self.implementer = Some(im as u64);
        self
    }

    pub(crate) fn variant(mut self, var: u64) -> MidrBuilder {
        self.variant = Some(var);
        self
    }

    pub(crate) fn architecture(mut self, arch: u64) -> MidrBuilder {
        self.architecture = Some(arch);
        self
    }

    pub(crate) fn part_num(mut self, part: u64) -> MidrBuilder {
        self.part_num = Some(part);
        self
    }

    pub(crate) fn revision(mut self, rev: u64) -> MidrBuilder {
        self.revision = Some(rev);
        self
    }

    pub(crate) fn build(&self) -> Midr {
        Midr {
            implementer: self.implementer.unwrap_or(0x0),
            variant: self.variant.unwrap_or(0x0),
            architecture: self.architecture.unwrap_or(0x0),
            part_num: self.part_num.unwrap_or(0x0),
            revision: self.revision.unwrap_or(0x0),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_arm_implementer() {
        let midr = MidrBuilder::new().implementer(Implementer::Arm).build();

        midr.dump();
        assert!(midr.check_implementer(Implementer::Arm));
        assert!(!midr.check_implementer(Implementer::Ampere));
    }
}

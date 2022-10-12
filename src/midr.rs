use std::fmt;

#[non_exhaustive]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub(crate) enum Implementer {
    Arm = 0x41,
    Fujitsu = 0x46,
    Apple = 0x61,
    Ampere = 0xc0,
    Unknown,
}

impl TryFrom<u64> for Implementer {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0x41 {
            Ok(Implementer::Arm)
        } else if value == 0x46 {
            Ok(Implementer::Fujitsu)
        } else if value == 0x61 {
            Ok(Implementer::Apple)
        } else if value == 0xc0 {
            Ok(Implementer::Ampere)
        } else {
            Err("unknown implementer")
        }
    }
}

impl fmt::Display for Implementer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Implementer::Arm => {
                write!(f, "Arm")
            }
            Implementer::Fujitsu => {
                write!(f, "Fujitsu")
            }
            Implementer::Apple => {
                write!(f, "Apple")
            }
            Implementer::Ampere => {
                write!(f, "Ampere")
            }
            Implementer::Unknown => {
                write!(f, "Unknown")
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Architecture {
    Armv4 = 0x01,
    Armv4T = 0x02,
    Armv5 = 0x03,
    Armv5T = 0x04,
    Armv5TE = 0x05,
    Armv5TEJ = 0x06,
    IDRegisters = 0x07,
    Unknown,
}

impl TryFrom<u64> for Architecture {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0x01 {
            return Ok(Architecture::Armv4);
        } else if value == 0x02 {
            return Ok(Architecture::Armv4T);
        } else if value == 0x03 {
            return Ok(Architecture::Armv5);
        } else if value == 0x04 {
            return Ok(Architecture::Armv5T);
        } else if value == 0x05 {
            return Ok(Architecture::Armv5TE);
        } else if value == 0x06 {
            return Ok(Architecture::Armv5TEJ);
        } else if value == 0x07 {
            return Ok(Architecture::IDRegisters);
        }
        Err("Value greater than 0x07")
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::Armv4 => {
                write!(f, "Armv4")
            }
            Architecture::Armv4T => {
                write!(f, "Armv4T")
            }
            Architecture::Armv5 => {
                write!(f, "Armv5")
            }
            Architecture::Armv5T => {
                write!(f, "Armv5T")
            }
            Architecture::Armv5TE => {
                write!(f, "Armv5TE")
            }
            Architecture::Armv5TEJ => {
                write!(f, "Armv5TEJ")
            }
            Architecture::IDRegisters => {
                write!(f, "ID Registers")
            }
            Architecture::Unknown => {
                write!(f, "unknown")
            }
        }
    }
}

#[derive(Debug)]
/// A MIDR_EL1 register
pub struct Midr {
    implementer: Implementer,
    variant: u64,
    architecture: Architecture,
    part_num: u64,
    revision: u64,
}

impl Midr {
    /// Create a new Midr
    pub fn new() -> Self {
        #[cfg(target_arch = "aarch64")]
        {
            let mut midr: u64;
            use std::arch::asm;
            unsafe {
                asm!("mrs {midr}, MIDR_EL1", midr = out(reg) midr);
            }

            Self::extract_parts(midr)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            let midr = 0;

            Self::extract_parts(midr)
        }
    }

    fn extract_parts(midr: u64) -> Self {
        let implementer = extract(midr, MIDR_IMPLEMENTOR_SHIFT, MIDR_IMPLEMENTOR_MASK);
        let variant = extract(midr, MIDR_VARIANT_SHIFT, MIDR_VARIANT_MASK);
        let architecture = extract(midr, MIDR_ARCHITECTURE_SHIFT, MIDR_ARCHITECTURE_MASK);
        let part_num = extract(midr, MIDR_PART_NUM_SHIFT, MIDR_PART_NUM_MASK);
        let revision = extract(midr, MIDR_REVISION_SHIFT, MIDR_REVISION_MASK);

        Midr {
            implementer: Implementer::try_from(implementer).unwrap_or(Implementer::Unknown),
            variant,
            architecture: Architecture::try_from(architecture).unwrap_or(Architecture::Unknown),
            part_num,
            revision,
        }
    }

    pub(crate) fn check_implementer(&self, im: Implementer) -> bool {
        self.implementer == im
    }

    pub(crate) fn is_arm(&self) -> bool {
        self.implementer == Implementer::Arm
    }

    pub(crate) fn is_apple(&self) -> bool {
        self.implementer == Implementer::Apple
    }

    pub(crate) fn check_variant(&self, im: u64) -> bool {
        self.variant == im
    }

    pub(crate) fn check_architecture(&self, arch: Architecture) -> bool {
        self.architecture == arch
    }

    pub(crate) fn check_part_num(&self, part_num: u64) -> bool {
        self.part_num == part_num
    }

    pub(crate) fn check_part_num_or(&self, part_num0: u64, part_num1: u64) -> bool {
        self.part_num == part_num0 || self.part_num == part_num1
    }

    pub(crate) fn check_revision(&self, im: u64) -> bool {
        self.revision == im
    }

    pub(crate) fn dump(&self) {
        println!("implementer : {}", self.implementer);
        println!("variant     : {}", self.variant);
        println!("architecture: {}", self.architecture);
        println!("part_num    : {:#0x}", self.part_num);
        println!("revision    : {}", self.revision);
    }
}

impl Default for Midr {
    fn default() -> Self {
        Self::new()
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
    implementer: Option<Implementer>,
    variant: Option<u64>,
    architecture: Option<Architecture>,
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
        self.implementer = Some(im);
        self
    }

    pub(crate) fn variant(mut self, var: u64) -> MidrBuilder {
        self.variant = Some(var);
        self
    }

    pub(crate) fn architecture(mut self, arch: Architecture) -> MidrBuilder {
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
            implementer: self.implementer.unwrap_or(Implementer::Unknown),
            variant: self.variant.unwrap_or(0x0),
            architecture: self.architecture.unwrap_or(Architecture::Unknown),
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

        assert!(midr.check_implementer(Implementer::Arm));
        assert!(!midr.check_implementer(Implementer::Ampere));
    }

    #[test]
    fn test_variant() {
        let midr = MidrBuilder::new().variant(0x7).build();

        assert!(midr.check_variant(0x7));

        let midr = MidrBuilder::new().build();

        assert!(midr.check_variant(0x0));
    }
}

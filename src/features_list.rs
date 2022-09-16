use crate::is_exposed_to_userspace;
use crate::{
    registers_info::{Register, RegistersInfo},
    Feature,
};

pub(super) fn get_feature_description(feature: &Feature) -> Option<String> {
    match AARCH64_FEATURES.binary_search_by(|mat| mat.get_feature().cmp(feature)) {
        Result::Ok(idx) => Some(AARCH64_FEATURES[idx].get_description().to_string()),
        _ => None,
    }
}

pub(super) fn get_features() -> HashSet<Feature> {
    let mut features = HashSet::new();

    let cpu_info = RegistersInfo::new();

    for feature in AARCH64_FEATURES {
        if let Some(feat) = feature.matches(&cpu_info) {
            if is_exposed_to_userspace(feat) {
                features.insert(feat);
            }
        }
    }

    features
}

#[allow(unused)]
#[derive(Hash, Eq, PartialEq)]
struct FeatureDescription {
    feature: Feature,
    version: crate::ARMVersion,
    //register: &'static str,
    register: Register,
    matcher: RegisterMatches,
    matcher2: RegisterMatches,
    category: crate::Category,
    description: &'static str,
}

impl FeatureDescription {
    pub(super) fn matches(&self, cpu_info: &RegistersInfo) -> Option<Feature> {
        let register = cpu_info[self.register];

        if self.matcher.check_match(register) || self.matcher2.check_match(register) {
            return Some(self.feature);
        }

        None
    }

    pub(super) fn get_feature(&self) -> Feature {
        self.feature
    }

    pub(super) fn get_description(&self) -> &str {
        self.description
    }
}

use std::{collections::HashSet, ops::RangeInclusive};

#[allow(unused)]
#[derive(Hash, Eq, PartialEq)]
enum ValueMatcher {
    Value(u64),
    Or(u64, u64),
    Any(u64, u64, u64),
}

impl ValueMatcher {
    fn check_match(&self, bits: u64) -> bool {
        match self {
            ValueMatcher::Value(val) => bits == *val,
            ValueMatcher::Or(a, b) => bits == *a || bits == *b,
            ValueMatcher::Any(a, b, c) => bits == *a || bits == *b || bits == *c,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
enum RegisterMatches {
    Fill,
    RegisterMatch(RangeInclusive<u64>, ValueMatcher),
}

impl RegisterMatches {
    fn check_match(&self, register: u64) -> bool {
        match self {
            RegisterMatches::Fill => false,
            RegisterMatches::RegisterMatch(range, value) => {
                let bits = self.extract_bits(range, register);
                value.check_match(bits)
            }
        }
    }

    fn extract_bits(&self, range: &RangeInclusive<u64>, register: u64) -> u64 {
        let shifted_bits = register >> range.start();

        match range.end() - range.start() + 1 {
            4 => shifted_bits & 0b1111,
            2 => shifted_bits & 0b0011,
            1 => shifted_bits & 0b0001,
            value => {
                panic!("unknown {}: {:?}", value, range);
            }
        }
    }
}

macro_rules! declare_features {
    ($(
        ($feature:ident, $version:ident, $register:ident, $matcher:expr, $matcher2:expr, $category:expr, $description:expr),
    )+) => {
        /// The features of AARCH64 from the ARMARM
        use crate::features_list::ValueMatcher::{Value, Or, Any};
        use crate::features_list::RegisterMatches::{RegisterMatch, Fill};
        use crate::Category::*;
        use crate::ARMVersion::*;
        use crate::registers_info::Register::*;
        const AARCH64_FEATURES: &[FeatureDescription] = &[
            $(
                FeatureDescription{
                    feature: Feature::$feature,
                    version: $version,
                    register: $register,
                    matcher: $matcher,
                    matcher2: $matcher2,
                    category: $category,
                    description: $description,
                }
            ),+
        ];
    }
}

#[rustfmt::skip]
declare_features!(
    (FEAT_AdvSIMD,     Armv8_0, ID_AA64PFR0_EL1,  RegisterMatch(20..=23,  Or(0b000 ,0b0001)),   Fill,                                      Float,     "AdvSIMD"),
    // Armv8.0
    (FEAT_SB,          Armv8_0, ID_AA64ISAR1_EL1, RegisterMatch(36..=39,  Value(0b0001)),       Fill,                                      Unknown,   "Speculation Barrier"),
    (FEAT_SSBS,        Armv8_0, ID_AA64PFR1_EL1,  RegisterMatch( 4..= 7,  Or(0b0001, 0b0010)),  Fill,                                      Unknown,   "Speculative Store Bypass Safe"),
    (FEAT_CSV2,        Armv8_0, ID_AA64PFR0_EL1,  RegisterMatch(56..=59,  Value(0b0001)),       Fill,                                      Caches,    "Cache Speculation Variant 2"),
    (FEAT_CSV2_2,      Armv8_0, ID_AA64PFR0_EL1,  RegisterMatch(56..=59,  Value(0b0010)),       Fill,                                      Caches,    "Cache Speculation Variant 2"),
    (FEAT_CSV2_1p1,    Armv8_0, ID_AA64PFR1_EL1,  RegisterMatch(32..=35,  Value(0b0001)),       Fill,                                      Caches,    "Cache Speculation Variant 2"),
    (FEAT_CSV2_1p2,    Armv8_0, ID_AA64PFR1_EL1,  RegisterMatch(32..=35,  Value(0b0010)),       Fill,                                      Caches,    "Cache Speculation Variant 2"),
    (FEAT_CSV3,        Armv8_0, ID_AA64PFR0_EL1,  RegisterMatch(60..=63,  Value(0b0001)),       Fill,                                      Caches,    "Cache Speculation Variant 3"),
    (FEAT_SPECRES,     Armv8_0, ID_AA64ISAR1_EL1, RegisterMatch(40..=43,  Value(0b0001)),       Fill,                                      Unknown,   "Speculation restriction instructions"),
    (FEAT_DoubleLock,  Armv8_0, ID_AA64DFR0_EL1,  RegisterMatch(36..=39,  Value(0b0000)),       Fill,                                      Unknown,   "Double Lock"),
    (FEAT_DGH,         Armv8_0, ID_AA64ISAR1_EL1, RegisterMatch(48..=51,  Value(0b0001)),       Fill,                                      Unknown,   "Data Gathering Hint"),
    (FEAT_ETS,         Armv8_0, ID_AA64MMFR1_EL1, RegisterMatch(36..=39,  Value(0b0001)),       Fill,                                      Unknown,   "Enhanced Translation Synchronization"),
    (FEAT_nTLBPA,      Armv8_0, ID_AA64MMFR1_EL1, RegisterMatch(48..=51,  Value(0b0001)),       Fill,                                      Unknown,   "Intermediate caching of translation table walks"),
    (FEAT_AES,         Armv8_0, ID_AA64ISAR0_EL1, RegisterMatch( 4..= 7,  Value(0b0001)),       Fill,                                      Crypto,    "Advanced SIMD AES instructions"),
    (FEAT_PMULL,       Armv8_0, ID_AA64ISAR0_EL1, RegisterMatch( 4..= 7,  Value(0b0010)),       Fill,                                      Unknown,   "Advanced SIMD PMULL instructions"),
    (FEAT_SHA1,        Armv8_0, ID_AA64ISAR0_EL1, RegisterMatch( 8..=11,  Value(0b0001)),       Fill,                                      Crypto,    "Advanced SIMD SHA1 instructions"),
    (FEAT_SHA256,      Armv8_0, ID_AA64ISAR0_EL1, RegisterMatch(12..=15,  Value(0b0001)),       Fill,                                      Crypto,    "Advanced SIMD SHA256 instructions"),
    // Armv8.1
    (FEAT_LSE,         Armv8_1, ID_AA64ISAR0_EL1, RegisterMatch(20..=23,  Value(0b0010)),       Fill,                                      Atomics,   "Large System Extensions"),
    (FEAT_RDM,         Armv8_1, ID_AA64ISAR0_EL1, RegisterMatch(28..=31,  Value(0b0001)),       Fill,                                      Float,     "Advanced SIMD rounding double multiply accumulate instructions"),
    (FEAT_LOR,         Armv8_1, ID_AA64MMFR1_EL1, RegisterMatch(16..=19,  Value(0b0001)),       Fill,                                      Atomics,   "Limited ordering regions"),
    (FEAT_HAFDBS,      Armv8_1, ID_AA64MMFR1_EL1, RegisterMatch(12..=15,  Value(0b0001)),       Fill,                                      Unknown,   "Hardware management of the Access flag and dirty state"),
    (FEAT_PAN,         Armv8_1, ID_AA64MMFR1_EL1, RegisterMatch(20..=23,  Value(0b0001)),       Fill,                                      Unknown,   "Privileged access never"),
    (FEAT_VMID16,      Armv8_1, ID_AA64MMFR1_EL1, RegisterMatch( 4..= 7,  Value(0b0010)),       Fill,                                      Unknown,   "16-bit VMID"),
    (FEAT_VHE,         Armv8_1, ID_AA64DFR0_EL1,  RegisterMatch( 8..=11,  Value(0b0001)),       Fill,                                      Virtual,   "Virtualization Host Extensions"),
    (FEAT_PMUv3p1,     Armv8_1, ID_AA64DFR0_EL1,  RegisterMatch( 8..=11,  Value(0b0010)),       Fill,                                      Unknown,   "PMU Extensions v3.1"),
    (FEAT_PAN3,        Armv8_1, ID_AA64MMFR1_EL1, RegisterMatch(20..=23,  Value(0b0011)),       Fill,                                      Unknown,   "Support for SCTLR_ELx.EPAN"),
    // Armv8.2
    (FEAT_SHA512,      Armv8_2, ID_AA64ISAR0_EL1, RegisterMatch(12..=15,  Value(0b0010)),       Fill,                                      Crypto,    "Advanced SIMD SHA512 instructions"),
    (FEAT_SHA3,        Armv8_2, ID_AA64ISAR0_EL1, RegisterMatch(32..=35,  Value(0b0001)),       Fill,                                      Crypto,    "Advanced SIMD SHA3 instructions"),
    (FEAT_SM3,         Armv8_2, ID_AA64ISAR0_EL1, RegisterMatch(36..=39,  Value(0b0001)),       Fill,                                      Crypto,    "Advanced SIMD SM3 instructions"),
    (FEAT_SM4,         Armv8_2, ID_AA64ISAR0_EL1, RegisterMatch(40..=43,  Value(0b0001)),       Fill,                                      Crypto,    "Advanced SIMD SM4 instructions"),
    (FEAT_PAN2,        Armv8_2, ID_AA64MMFR1_EL1, RegisterMatch(20..=23,  Value(0b0010)),       Fill,                                      Unknown,   "AT S1E1R and AT S1E1W instruction variants affected by PSTATE.PAN"),
    (FEAT_FP16,        Armv8_2, ID_AA64PFR0_EL1,  RegisterMatch(16..=19,  Value(0b0001)),       Fill,                                      Float,     "Half-precision floating-point data processing"),
    (FEAT_DotProd,     Armv8_2, ID_AA64ISAR0_EL1, RegisterMatch(44..=47,  Value(0b0001)),       Fill,                                      Float,     "Advanced SIMD dot product instructions"),
    (FEAT_FHM,         Armv8_2, ID_AA64ISAR0_EL1, RegisterMatch(48..=51,  Value(0b0001)),       Fill,                                      Float,     "Floating-point half-precision multiplication instructions"),
    (FEAT_UAO,         Armv8_2, ID_AA64MMFR2_EL1, RegisterMatch( 4..= 7,  Value(0b0001)),       Fill,                                      Unknown,   "Unprivileged Access Override control"),
    (FEAT_DPB,         Armv8_2, ID_AA64ISAR1_EL1, RegisterMatch( 0..= 3,  Value(0b0001)),       Fill,                                      Caches,    "DC CVAP instruction"),
    (FEAT_VPIPT,       Armv8_2, CTR_EL0,          RegisterMatch(14..=15,  Value(0b0000)),       Fill,                                      Unknown,   "VMID-aware PIPT instruction cache"),
    (FEAT_HPDS2,       Armv8_2, ID_AA64MMFR1_EL1, RegisterMatch(12..=15,  Value(0b0010)),       Fill,                                      Unknown,   "Translation table page-based hardware attributes"),
    (FEAT_LPA,         Armv8_2, ID_AA64MMFR0_EL1, RegisterMatch( 0..= 3,  Value(0b0110)),       Fill,                                      Unknown,   "Large PA and IPA support"),
    (FEAT_LVA,         Armv8_2, ID_AA64MMFR2_EL1, RegisterMatch(16..=19,  Value(0b0001)),       Fill,                                      Unknown,   "Large VA support"),
    (FEAT_TTCNP,       Armv8_2, ID_AA64MMFR2_EL1, RegisterMatch( 0..= 3,  Value(0b0001)),       Fill,                                      Unknown,   "Translation table Common not private translations"),
    (FEAT_XNX,         Armv8_2, ID_AA64MMFR1_EL1, RegisterMatch(28..=31,  Value(0b0001)),       Fill,                                      Unknown,   "Translation table stage 2 Unprivileged Execute-never"),
    (FEAT_Debugv8p2,   Armv8_2, ID_AA64DFR0_EL1,  RegisterMatch( 0..= 3,  Value(0b1000)),       Fill,                                      Unknown,   "Debug v8.2"),
    (FEAT_PCSRv8p2,    Armv8_2, EDDEVID,          RegisterMatch( 0..= 3,  Value(0b0000)),       Fill,                                      Unknown,   "PC Sample-based profiling"),
    (FEAT_IESB,        Armv8_2, ID_AA64MMFR2_EL1, RegisterMatch(12..=15,  Value(0b0001)),       Fill,                                      Unknown,   "Implicit Error Synchronization event"),
    // added later
    (FEAT_EVT,         Armv8_2, ID_AA64MMFR2_EL1, RegisterMatch(56..=59,  Value(0b0010)),       Fill,                                      Unknown,   "Enhanced Virtualization Traps"),
    (FEAT_DPB2,        Armv8_2, ID_AA64ISAR1_EL1, RegisterMatch( 0..= 3,  Value(0b0010)),       Fill,                                      Caches,    "DC CVADP instruction"),
    (FEAT_BF16,        Armv8_2, ID_AA64ISAR1_EL1, RegisterMatch(44..=47,  Value(0b0001)),       Fill,                                      Float,     "BFloat16 instructions"),
    (FEAT_I8MM,        Armv8_2, ID_AA64ISAR1_EL1, RegisterMatch(52..=55,  Value(0b0001)),       Fill,                                      Float,     "Int8 matrix multiplication instructions"),
    // Armv8.3
    (FEAT_FCMA,        Armv8_3, ID_AA64ISAR1_EL1, RegisterMatch(16..=19,  Value(0b0001)),       Fill,                                      Float,     "Floating-point complex number instructions"),
    (FEAT_JSCVT,       Armv8_3, ID_AA64ISAR1_EL1, RegisterMatch(12..=15,  Value(0b0001)),       Fill,                                      Float,     "JavaScript conversion instructions"),
    (FEAT_LRCPC,       Armv8_3, ID_AA64ISAR1_EL1, RegisterMatch(20..=23,  Value(0b0001)),       Fill,                                      Atomics,   "Load-acquire RCpc instructions"),
    (FEAT_NV,          Armv8_3, ID_AA64MMFR2_EL1, RegisterMatch(24..=27,  Value(0b0001)),       Fill,                                      Virtual,   "Nested virtualization support"),
    (FEAT_CCIDX,       Armv8_3, ID_AA64MMFR2_EL1, RegisterMatch(20..=23,  Value(0b0001)),       Fill,                                      Caches,    "Extended cache index"),
    (FEAT_PAuth,       Armv8_3, ID_AA64ISAR1_EL1, RegisterMatch( 4..= 7,  Or(0b0001, 0b0010)),  RegisterMatch(8..=11, Or(0b0001, 0b0010)), Security,  "Pointer authentication"),
    // added later
    (FEAT_SPEv1p1,     Armv8_3, ID_AA64DFR0_EL1,  RegisterMatch(32..=35,  Value(0b0010)),       Fill,                                      Profiling, "Armv8.3 Statistical Profiling Extensions"),
    (FEAT_DoPD,        Armv8_3, EDDEVID,          RegisterMatch(  4..=7,  Value(0b0001)),       Fill,                                      Debug,     "Debug over Powerdown"),
    (FEAT_PAuth2,      Armv8_3, ID_AA64ISAR1_EL1, RegisterMatch(  4..=7,  Value(0b0011)),       RegisterMatch(8..=11, Value(0b0011)),      Security,  "Enhancements to pointer authentication"),
    (FEAT_FPAC,        Armv8_3, ID_AA64ISAR1_EL1, RegisterMatch(  4..=7,  Or(0b0100, 0b0101)),  RegisterMatch(8..=11, Or(0b0100, 0b0101)), Security,  "Faulting on AUT* instructions"),
    // Armv8.4
    (FEAT_DIT,         Armv8_4, ID_AA64PFR0_EL1,  RegisterMatch(48..=51,  Value(0b0010)),       Fill,                                      Unknown,   "Data Independent Timing instructions"),
    (FEAT_FlagM,       Armv8_4, ID_AA64ISAR0_EL1, RegisterMatch(52..=55,  Value(0b0001)),       Fill,                                      Unknown,   "Flag manipulation instructions v2"),
    (FEAT_LRCPC2,      Armv8_4, ID_AA64ISAR1_EL1, RegisterMatch(20..=23,  Value(0b0010)),       Fill,                                      Atomics,   "Load-Acquire RCpc instructions v2"),
    (FEAT_LSE2,        Armv8_4, ID_AA64MMFR2_EL1, RegisterMatch(32..=35,  Value(0b0001)),       Fill,                                      Atomics,   "Large System Extensions v2"),
    (FEAT_TLBIOS,      Armv8_4, ID_AA64ISAR0_EL1, RegisterMatch(56..=59,  Or(0b0001, 0b0010)),  Fill,                                      Unknown,   "TLB invalidate instructions in Outer Shareable domain"),
    (FEAT_TLBIRANGE,   Armv8_4, ID_AA64ISAR0_EL1, RegisterMatch(56..=59,  Value(0b0010)),       Fill,                                      Unknown,   "TLB invalidate range instructions"),
    (FEAT_TTL,         Armv8_4, ID_AA64MMFR2_EL1, RegisterMatch(48..=51,  Value(0b0001)),       Fill,                                      Unknown,   "Translation Table Level"),
    (FEAT_S2FWB,       Armv8_4, ID_AA64MMFR2_EL1, RegisterMatch(40..=43,  Value(0b0001)),       Fill,                                      Unknown,   "Stage 2 forced Write-Back"),
    (FEAT_TTST,        Armv8_4, ID_AA64MMFR2_EL1, RegisterMatch(28..=31,  Value(0b0001)),       Fill,                                      Unknown,   "Small translation tables"),
    (FEAT_BBM,         Armv8_4, ID_AA64MMFR2_EL1, RegisterMatch(52..=55,  Any(0b0000, 0b0001, 0b0010)),        Fill,                       Unknown,   "Translation table break-before-make levels"),
    (FEAT_SEL2,        Armv8_4, ID_AA64PFR0_EL1,  RegisterMatch(36..=39,  Value(0b0001)),       Fill,                                      Unknown,   "Secure EL2"),
    (FEAT_NV2,         Armv8_4, ID_AA64MMFR2_EL1, RegisterMatch(24..=27,  Value(0b0010)),       Fill,                                      Virtual,   "Enhanced nested virtualization support"),
    (FEAT_IDST,        Armv8_4, ID_AA64MMFR2_EL1, RegisterMatch(36..=39,  Value(0b0001)),       Fill,                                      Unknown,   "ID space trap handling"),
    (FEAT_CNTSC,       Armv8_4, CNTID,            RegisterMatch( 0..= 3,  Value(0b0001)),       Fill,                                      Unknown,   "Generic Counter Scaling"),
    (FEAT_Debugv8p4,   Armv8_4, ID_AA64DFR0_EL1,  RegisterMatch( 0..= 3,  Value(0b1001)),       Fill,                                      Unknown,   "Debug v8.4"),
    (FEAT_TRF,         Armv8_4, ID_AA64DFR0_EL1,  RegisterMatch(40..=43,  Value(0b0001)),       Fill,                                      Unknown,   "Self-hosted Trace Extensions"),
    (FEAT_PMUv3p4,     Armv8_4, ID_AA64DFR0_EL1,  RegisterMatch( 8..=11,  Value(0b0101)),       Fill,                                      Unknown,   "PMU Extensions v3.4"),
    (FEAT_RASv1p1,     Armv8_4, ID_AA64PFR0_EL1,  RegisterMatch(28..=31,  Value(0b0010)),       Fill,                                      Unknown,   "RAS Extension v1.1"),
    (FEAT_DoubleFault, Armv8_4, ID_AA64PFR0_EL1,  RegisterMatch(28..=31,  Value(0b0010)),       Fill,                                      Unknown,   "Double Fault Extension"),
    // Armv8.5
    (FEAT_FlagM2,      Armv8_5,  ID_AA64ISAR0_EL1, RegisterMatch(52..=55, Value(0b0010)),       Fill,                                      Unknown,   "Enhancements to flag manipulation instructions"),
    (FEAT_FRINTTS,     Armv8_5,  ID_AA64ISAR1_EL1, RegisterMatch(32..=35 ,Value(0b0001)),       Fill,                                      Float,     "Floating-point to integer instructions"),
    (FEAT_ExS,         Armv8_5,  ID_AA64MMFR0_EL1, RegisterMatch(44..=47 ,Value(0b0001)),       Fill,                                      Unknown,   "Context synchronization and exception handling"),
   // (FEAT_GTG,         Armv8_5,  ID_AA64MMFR0_EL1, RegisterMatch(        ,Value()), Fill,                                                  Unknown,   "Guest translation granule size"),
    (FEAT_BTI,         Armv8_5,  ID_AA64PFR1_EL1,  RegisterMatch( 0..= 3, Value(0b0001)),       Fill,                                      Security,  "Branch Target Identification"),
    (FEAT_E0PD,        Armv8_5,  ID_AA64MMFR2_EL1, RegisterMatch(60..=63, Value(0b0001)),       Fill,                                      Unknown,   "Preventing EL0 access to halves of address maps"),
    (FEAT_RNG,         Armv8_5,  ID_AA64ISAR0_EL1, RegisterMatch(60..=63, Value(0b0001)),       Fill,                                      Unknown,   "Random number generator"),
    (FEAT_MTE,         Armv8_5,  ID_AA64PFR1_EL1,  RegisterMatch( 8..=11, Value(0b0001)),       Fill,                                      Security,  "Memory Tagging Extension"),
    (FEAT_MTE2,        Armv8_5,  ID_AA64PFR1_EL1,  RegisterMatch( 8..=11, Value(0b0010)),       Fill,                                      Security,  "Memory Tagging Extension"),
    (FEAT_PMUv3p5,     Armv8_5,  ID_AA64DFR0_EL1,  RegisterMatch( 8..=11, Value(0b0110)),       Fill,                                      Unknown,   "PMU Extensions v3.5"),
    // added later
    (FEAT_MTE3,        Armv8_5,  ID_AA64PFR1_EL1,  RegisterMatch( 8..=11, Value(0b0011)),       Fill,                                      Unknown,   "MTE Asymmetric Fault Handling"),
    (FEAT_RNG_TRAP,    Armv8_5,  ID_AA64PFR1_EL1,  RegisterMatch(28..=31, Value(0b0001)),       Fill,                                      Unknown,   "Trapping support for RNDR/RNDRRS"),
    // Armv8.6
    (FEAT_ECV,         Armv8_6,  ID_AA64MMFR0_EL1, RegisterMatch(60..=63, Or(0b0001, 0b0010)),  Fill,                                      Virtual,   "Enhanced Counter Virtualization"),
    (FEAT_FGT,         Armv8_6,  ID_AA64MMFR0_EL1, RegisterMatch(56..=59, Value(0b0001)),       Fill,                                      Unknown,   "Fine Grain Traps"),
    (FEAT_TWED,        Armv8_6,  ID_AA64MMFR1_EL1, RegisterMatch(32..=35, Value(0b0001)),       Fill,                                      Unknown,   "Delayed Trapping of WFE"),
    (FEAT_AMUv1p1,     Armv8_6,  ID_AA64PFR0_EL1,  RegisterMatch(44..=47, Value(0b0010)),       Fill,                                      Virtual,   "AMU Extensions v1.1"),
    (FEAT_MTPMU,       Armv8_6,  ID_AA64DFR0_EL1,  RegisterMatch(48..=51, Value(0b0001)),       Fill,                                      Unknown,   "Multi-threaded PMU Extensions"),
    // Armv8.7
    (FEAT_AFP,          Armv8_7, ID_AA64MMFR1_EL1, RegisterMatch(44..=47, Value(0b0001)),       Fill,                                      Float,     "Alternate floating-point behavior"),
    (FEAT_RPRES,        Armv8_7, ID_AA64ISAR2_EL1, RegisterMatch( 4..=7,  Value(0b0001)),       Fill,                                      Float,     "Increased precision of Reciprocal Estimate and Reciprocal Square Root Estimate"),
    (FEAT_LS64,         Armv8_7, ID_AA64ISAR1_EL1, RegisterMatch(60..=63, Value(0b0001)),       Fill,                                      Unknown,   "Support for 64 byte loads/stores"),
    (FEAT_LS64_V,       Armv8_7, ID_AA64ISAR1_EL1, RegisterMatch(60..=63, Value(0b0010)),       Fill,                                      Unknown,   "Support for 64 byte loads/stores"),
    (FEAT_LS64_ACCDATA, Armv8_7, ID_AA64ISAR1_EL1, RegisterMatch(60..=63, Value(0b0011)),       Fill,                                      Unknown,   "Support for 64 byte loads/stores"),
    (FEAT_WFxT,         Armv8_7, ID_AA64ISAR2_EL1, RegisterMatch( 0..=3,  Value(0b0010)),       Fill,                                      Unknown,   "WFE and WFI instructions with timeout"),
    //(FEAT_WFxT2,        Armv8_7, ID_AA64ISAR2_EL1, RegisterMatch(), Fill, Float, "WFE and WFI instructions with timeout"),
    (FEAT_HCX,          Armv8_7, ID_AA64MMFR1_EL1, RegisterMatch(40..=43, Value(0b0001)),       Fill,                                      Unknown,   "Support for the HCRX_EL2 register"),
    //(FEAT_LPA2,         Armv8_7, ID_AA64MMFR0_EL1, RegisterMatch(), Fill, Float, "Larger physical address for 4KB and 16KB translation granules"),
    (FEAT_XS,           Armv8_7, ID_AA64ISAR1_EL1, RegisterMatch(56..=59, Value(0b0001)),       Fill,                                      Unknown,   "XS attribute"),
    (FEAT_PMUv3p7,      Armv8_7, ID_AA64DFR0_EL1,  RegisterMatch( 8..=11, Value(0b0111)),       Fill,                                      Unknown,   "Armv8.7 PMU extensions"),
    (FEAT_SPEv1p2,      Armv8_7, ID_AA64DFR0_EL1,  RegisterMatch(32..=35, Value(0b0011)),       Fill,                                      Unknown,   "Armv8.7 SPE extensions"),
    // Armv8.8
    (FEAT_MOPS,        Armv8_8, ID_AA64ISAR2_EL1,  RegisterMatch(16..=19, Value(0b0001)),       Fill,                                      Memory,    "Standardization of memory operations"),
    (FEAT_HBC,         Armv8_8, ID_AA64ISAR2_EL1,  RegisterMatch(20..=23, Value(0b0001)),       Fill,                                      Unknown,   "Hinted conditional branches"),
    (FEAT_NMI,         Armv8_8, ID_AA64PFR1_EL1,   RegisterMatch(36..=39, Value(0b0001)),       Fill,                                      Unknown,   "Non-maskable Interrupts"),
    (FEAT_TIDCP1,      Armv8_8, ID_AA64MMFR1_EL1,  RegisterMatch(52..=55, Value(0b0001)),       Fill,                                      Unknown,   "EL0 use of IMPLEMENTATION DEFINED functionality"),
    (FEAT_CMOW,        Armv8_8, ID_AA64MMFR1_EL1,  RegisterMatch(56..=59, Value(0b0001)),       Fill,                                      Unknown,   "Control for cache maintenance permission"),
    (FEAT_PMUv3p8,     Armv8_8, ID_AA64DFR0_EL1,   RegisterMatch( 8..=11, Value(0b1000)),       Fill,                                      Unknown,   "Armv8.8 PMU extensions"),
    (FEAT_HPMN0,       Armv8_8, ID_AA64DFR0_EL1,   RegisterMatch(60..=63, Value(0b0001)),       Fill,                                      Unknown,   "Setting of MDCR_EL2.HPMN to zero"),
    //(FEAT_PMUv3_TH,    Armv8_8, PMMIR_EL1,         RegisterMatch(20..=23, Value()), Fill, Unknown, "Event counting threshold"),
    (FEAT_SPEv1p3,     Armv8_8, ID_AA64DFR0_EL1,   RegisterMatch(32..=35, Value(0b0100)),       Fill,                                      Unknown,   "Armv8.8 Statistical Profiling Extensions"),
    (FEAT_Debugv8p8,   Armv8_8, ID_AA64DFR0_EL1,   RegisterMatch( 0..=3,  Value(0b1010)),       Fill,                                      Unknown,   "Debug v8.8"),

    // Armv9.0
    (FEAT_SVE2,        Armv9_0, ID_AA64ZFR0_EL1,   RegisterMatch( 0..= 3, Value(0b0001)),       Fill,                                      Float,     "Scalable Vector Extension version 2"),
    (FEAT_SVE_AES,     Armv9_0, ID_AA64ZFR0_EL1,   RegisterMatch( 4..= 7, Value(0b0001)),       Fill,                                      Float,     "Scalable Vector AES instructions"),
    (FEAT_SVE_BitPerm, Armv9_0, ID_AA64ZFR0_EL1,   RegisterMatch(16..=19, Value(0b0001)),       Fill,                                      Unknown,   "Scalable Vector Bit Permutes instructions"),
    (FEAT_SVE_PMULL128,Armv9_0, ID_AA64ZFR0_EL1,   RegisterMatch( 4..= 7, Value(0b0010)),       Fill,                                      Unknown,   "Scalable Vector PMULL instructions"),
    (FEAT_SVE_SHA3,    Armv9_0, ID_AA64ZFR0_EL1,   RegisterMatch(32..=35, Value(0b0001)),       Fill,                                      Unknown,   "Scalable Vector SHA3 instructions"),
    (FEAT_SVE_SM4,     Armv9_0, ID_AA64ZFR0_EL1,   RegisterMatch(40..=43, Value(0b0001)),       Fill,                                      Unknown,   "Scalable Vector SM4 instructions"),
    (FEAT_ETE,         Armv9_0, ID_AA64DFR0_EL1,   RegisterMatch( 4..= 7, Value(0b0001)),       Fill,                                      Unknown,   "Embedded Trace Extension"),
    (FEAT_TRBE,        Armv9_0, ID_AA64DFR0_EL1,   RegisterMatch(44..=47, Value(0b0001)),       Fill,                                      Unknown,   "Trace Buffer Extension"),

    // Armv9.1
    (FEAT_ETEv1p1,     Armv9_1, TRCDEVARCH,        RegisterMatch(16..=19, Value(0b0001)),       Fill,                                      Unknown,   "Embedded Trace Extension"),

    // Armv9.2
    (FEAT_BRBE,        Armv9_2, ID_AA64DFR0_EL1,   RegisterMatch(52..=55, Value(0b0001)),       Fill,                                      Jump,      "Branch Record Buffer Extension"),
    (FEAT_ETEv1p2,     Armv9_2, TRCDEVARCH,        RegisterMatch(16..=19, Value(0b0010)),       Fill,                                      Unknown,   "Embedded Trace Extension"),
    //(FEAT_RME,         Armv9_2, ,        RegisterMatch(16..=19, Value(0b0010)),       Fill,                                      Unknown,   "Realm Management Extension"),
    (FEAT_SME,         Armv9_2, ID_AA64PFR1_EL1,   RegisterMatch(24..=27, Value(0b0001)),       Fill,                                      Sme,       "Scalable Matrix Extension"),
    (FEAT_SME_FA64,    Armv9_2, ID_AA64SMFR0_EL1,  RegisterMatch(63..=63, Value(0b1)),          Fill,                                      Sme,       "Full Streaming SVE mode instructions"),
    (FEAT_EBF16,       Armv9_2, ID_AA64ISAR1_EL1,  RegisterMatch(44..=47, Value(0b0010)),       Fill,                                      Float,     "AArch64 Extended BFloat16 instructions"),
    (FEAT_SME_F64F64,  Armv9_2, ID_AA64SMFR0_EL1,  RegisterMatch(48..=48, Value(0b1)),          Fill,                                      Sme,       "Double-precision floating-point outer product instructions"),
    (FEAT_SME_I16I64,  Armv9_2, ID_AA64SMFR0_EL1,  RegisterMatch(52..=55, Value(0b1111)),       Fill,                                      Sme,       "16-bit to 64-bit integer widening outer product instructions"),

    // Armv9.3
    (FEAT_BRBEv1p1,    Armv9_3,  ID_AA64DFR0_EL1,  RegisterMatch(52..=55, Value(0b0010)),       Fill,                                      Jump,      "Branch Record Buffer Extension"),
);

// FIXME: missing CRC32 [19-16]

// CRC32 instructions have no features

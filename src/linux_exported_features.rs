use crate::Feature;

/// These are the features exported by the kernel to userspace, see [kernel](https://github.com/torvalds/linux/blob/master/Documentation/arm64/cpu-feature-registers.rst).
/// The list was last updated at 16.8.2022 (non-exhaustive).
pub(crate) const EXPORTED_FEATURES: &[Feature] = &[
    // ID_AA64ISAR0_EL1
    Feature::FEAT_RNG,     // RNDR
    Feature::FEAT_FlagM2,  // TS
    Feature::FEAT_FHM,     // FHM
    Feature::FEAT_DotProd, // DP
    Feature::FEAT_SM4,     // SM4
    Feature::FEAT_SM3,     // SM3
    Feature::FEAT_SHA3,    // SHA3
    Feature::FEAT_RDM,     // RDM
    Feature::FEAT_LSE,     // ATOMICS
    // CRC32 instructions have no features
    Feature::FEAT_SHA256, // SHA2
    Feature::FEAT_SHA1,   // SHA1
    Feature::FEAT_PMULL,  // AES
    Feature::FEAT_AES,    // AES
    // ID_AA64PFR0_EL1
    Feature::FEAT_DIT,     // DIR
    Feature::FEAT_SVE,     // SVE
    Feature::FEAT_AdvSIMD, // AdvSIMD
    // ID_AA64PFR1_EL1
    Feature::FEAT_MTE, // MTE
                       // MIDR_EL1
];

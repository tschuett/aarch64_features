use crate::Feature;

// MIDR_EL1
pub(crate) fn check_availability() -> bool {
    false
}

pub(crate) fn is_exposed_to_userspace(_feat: Feature) -> bool {
    true
}

// AArch64: Add initial apple-m1 target.
// https://reviews.llvm.org/D92619

// [AArch64] Add support for -march=native for Apple M1 CPU
// https://reviews.llvm.org/D119788

// sysctl -a | fgrep hw.optional.arm.

// [AArch64] Add apple-m1 CPU, and default to it for macOS.
// https://reviews.llvm.org/rGa8a3a43792472c9775c60fa79b9357033d47ce40

//fn read_register(reg: &str) -> u64 {
//    use core::arch::aarch64::__rsr64;
//    __rsr64(reg)
//}

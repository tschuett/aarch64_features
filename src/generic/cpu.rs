use crate::Feature;

pub(crate) fn is_exposed_to_userspace(_feat: Feature) -> bool {
    false
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
pub(crate) fn check_availability() -> bool {
    true
}

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub(crate) fn check_availability() -> bool {
    true
}

#[cfg(all(target_arch = "aarch64", target_os = "windows"))]
pub(crate) fn check_availability() -> bool {
    false
}

/// default implementation
#[cfg(not(target_arch = "aarch64"))]
pub(crate) fn check_availability() -> bool {
    false
}

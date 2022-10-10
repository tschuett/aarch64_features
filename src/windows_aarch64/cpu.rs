use crate::Feature;

/// default implementation
pub(crate) fn check_availability() -> bool {
    false
}

pub(crate) fn is_exposed_to_userspace(_feat: Feature) -> bool {
    false
}

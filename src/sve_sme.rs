pub(super) fn has_sve() -> bool {
    use std::arch::asm;
    let tmp: u64;
    unsafe {
        asm!("mrs {tmp}, ID_AA64PFR0_EL1", tmp = out(reg) tmp);
    };
    // SVE, bits [35:32]
    let shifted_bits = tmp >> 32;
    let shifted_bits = shifted_bits & 0b1111;
    shifted_bits == 0b0001
}

pub(super) fn has_sme() -> bool {
    use std::arch::asm;
    let tmp: u64;
    unsafe {
        asm!("mrs {tmp}, ID_AA64PFR0_EL1", tmp = out(reg) tmp);
    };
    // SME, bits [27:24]
    let shifted_bits = tmp >> 24;
    let shifted_bits = shifted_bits & 0b1111;
    shifted_bits == 0b0001
}

pub(super) fn is_streaming_sve_mode() -> bool {
    use std::arch::asm;
    let tmp: u64;
    unsafe {
        asm!("mrs {tmp}, SVCR", tmp = out(reg) tmp);
    };
    // SM, bit [0]
    let shifted_bits = tmp & 0b0001;
    shifted_bits == 0b1
}

pub(super) fn get_za_size() -> Option<u64> {
    if !is_streaming_sve_mode() {
        return None;
    }

    if let Some(sve_len) = get_sve_len() {
        return Some((sve_len / 8) * (sve_len / 8));
    }

    None
}

pub(super) fn get_sve_len() -> Option<u64> {
    if !has_sve() {
        return None;
    }

    use std::arch::asm;

    let tmp: u64;
    unsafe {
        asm!("mrs {tmp}, ZCR_EL3", tmp = out(reg) tmp);
    };

    // LEN, bits [3:0]

    let len = tmp & 0b1111;
    Some((len + 1) * 128)
}

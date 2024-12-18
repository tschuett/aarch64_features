pub(super) fn has_sve() -> bool {
    use std::arch::asm;
    let tmp: u64;
    unsafe {
        asm!("mrs {tmp}, ID_AA64PFR0_EL1", tmp = out(reg) tmp);
    };
    // SVE, bits [35:32]
    let shifted_bits = tmp >> 32;
    let shifted_bits = shifted_bits & 0b1111;
    return shifted_bits == 0b0001;
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
    return Some((len + 1) * 128);
}

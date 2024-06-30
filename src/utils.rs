pub fn sign_extend(value: u16, bit_count: u16) -> u16 {
    let mut sign_extended_value = value;
    if (sign_extended_value >> (bit_count - 1)) & 1 == 1 {
        sign_extended_value |= 0xFFFF << bit_count;
    }
    return sign_extended_value;
}

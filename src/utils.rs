use std::io;
use std::io::{Read};
use std::os::fd::{AsFd};
use nix::sys::select::{select, FdSet};
use nix::sys::time::TimeVal;

pub fn sign_extend(value: u16, bit_count: u16) -> u16 {
    let mut sign_extended_value = value;
    if (sign_extended_value >> (bit_count - 1)) & 1 == 1 {
        sign_extended_value |= 0xFFFF << bit_count;
    }
    return sign_extended_value;
}

pub fn get_char_byte() -> io::Result<u8> {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer)?;
    Ok(buffer[0])
}

pub fn check_key() -> bool {
    let mut fd = FdSet::new();
    let stdin = io::stdin();
    fd.insert(stdin.as_fd());
    let mut timeout = TimeVal::new(0, 0);
    match select(1, &mut fd, None, None, &mut timeout) {
        Err(_) => false,
        _ => true,
    }
}

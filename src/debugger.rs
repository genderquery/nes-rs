use crate::addressing_mode::AddressingMode;
use std::fmt;

pub struct Decoded {
    byte_code: Vec<u8>,
    opcode: u8,
    mnemonic: &'static str,
    addressing_mode: AddressingMode,
}

impl fmt::Display for Decoded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

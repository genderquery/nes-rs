#[macro_use]
extern crate bitflags;

extern crate derive_more;

pub mod addressing_mode;
pub mod bus;
pub mod console;
pub mod cpu;
pub mod debugger;
pub mod ines;
pub mod instructions;
pub mod mapper;
pub mod mappers;
pub mod ppu;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

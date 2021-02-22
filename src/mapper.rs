use crate::ines;
use crate::mappers::nrom::Nrom;
use crate::mappers::uxrom::Uxrom;
use crate::Result;
use core::fmt;
use std::fs;
use std::path::Path;

pub trait Mapper {
    fn id(&self) -> u8;
    fn cpu_read(&mut self, address: u16) -> u8;
    fn cpu_write(&mut self, address: u16, _data: u8);
    fn ppu_read(&mut self, address: u16) -> u8;
    fn ppu_write(&mut self, address: u16, _data: u8);
}

impl dyn Mapper {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Box<dyn Mapper>> {
        let bytes = fs::read(path)?;
        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Result<Box<dyn Mapper>> {
        let bytes = bytes.into();
        let (header, bytes) = bytes.split_at(16);
        let header = ines::parse_header(header).unwrap();
        let (_trainer, file) = if header.has_trainer {
            bytes.split_at(512)
        } else {
            bytes.split_at(0)
        };
        let (prg_rom, file) = file.split_at(header.prg_rom_size);
        let (chr_rom, _) = file.split_at(header.chr_rom_size);

        let mapper: Box<dyn Mapper> = match header.mapper_id {
            0 => Box::new(Nrom::new(prg_rom, chr_rom)),
            2 | 94 | 180 => Box::new(Uxrom::new(prg_rom, chr_rom)),
            _ => unimplemented!(),
        };
        Ok(mapper)
    }
}

impl fmt::Debug for dyn Mapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mapper {}", self.id())
    }
}

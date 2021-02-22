use crate::mapper::Mapper;

#[derive(Debug, Clone)]
pub struct Nrom {
    prg_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Nrom {
    pub fn new<V>(prg_rom: V, chr_rom: V) -> Nrom
    where
        V: Into<Vec<u8>>,
    {
        Nrom {
            prg_rom: prg_rom.into(),
            chr_rom: chr_rom.into(),
            prg_ram: vec![0; 8 * 1024],
        }
    }
}

impl Mapper for Nrom {
    fn id(&self) -> u8 {
        0
    }

    fn cpu_read(&mut self, address: u16) -> u8 {
        match address {
            0x6000..=0x7fff => {
                let address = address % self.prg_ram.len() as u16;
                self.prg_ram[address as usize]
            }
            0x8000..=0xffff => {
                let address = address % self.prg_rom.len() as u16;
                self.prg_rom[address as usize]
            }
            _ => 0,
        }
    }

    fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            0x6000..=0x7fff => {
                let address = address % self.prg_ram.len() as u16;
                self.prg_ram[address as usize] = data
            }
            0x8000..=0xffff => {
                let address = address % self.prg_rom.len() as u16;
                self.prg_rom[address as usize] = data
            }
            _ => (),
        }
    }

    fn ppu_read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1fff => {
                let address = address % self.chr_rom.len() as u16;
                self.chr_rom[address as usize]
            }
            _ => 0,
        }
    }

    fn ppu_write(&mut self, _address: u16, _data: u8) {}
}

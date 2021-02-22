use crate::mapper::Mapper;

#[derive(Debug, Clone)]
pub struct Uxrom {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    bank: usize,
}

impl Uxrom {
    const BANK_SIZE: usize = 16 * 1024; // 16 kB

    pub fn new<V>(prg_rom: V, chr_rom: V) -> Uxrom
    where
        V: Into<Vec<u8>>,
    {
        Uxrom {
            prg_rom: prg_rom.into(),
            chr_rom: chr_rom.into(),
            bank: 0,
        }
    }
}

impl Mapper for Uxrom {
    fn id(&self) -> u8 {
        2
    }

    fn cpu_read(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0xbfff => {
                let index = (address - 0x8000) as usize + Self::BANK_SIZE * self.bank;
                self.prg_rom[index]
            }
            0xc000..=0xffff => {
                let last_bank = self.prg_rom.len() - Self::BANK_SIZE;
                let index = last_bank + (address % 0xc000) as usize;
                self.prg_rom[index]
            }
            _ => 0,
        }
    }

    fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0xffff => {
                self.bank = data as usize;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_switching() {
        let mut prg_rom = Vec::with_capacity(256 * 1024); // 256 kB
        let chr_rom = vec![0; 8 * 1024]; // 8 kB

        for bank in 0..16 {
            for _ in 0..Uxrom::BANK_SIZE {
                prg_rom.push(bank);
            }
        }

        let mut mapper = Uxrom::new(prg_rom, chr_rom);

        // read from fixed bank
        assert_eq!(mapper.cpu_read(0xc000), 0x0f);

        // switch to bank 0
        mapper.cpu_write(0x8000, 0x00);
        assert_eq!(mapper.cpu_read(0x8000), 0x00);

        // switch to bank 1
        mapper.cpu_write(0x8000, 0x01);

        // fixed bank should not have changed
        assert_eq!(mapper.cpu_read(0xc000), 0x0f);

        // should be reading from bank 1
        assert_eq!(mapper.cpu_read(0x8000), 0x01);
    }
}

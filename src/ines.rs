use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileFormat {
    /// iNES
    INes,
    /// NES 2.0
    Nes20,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Header {
    pub format: FileFormat,
    pub prg_rom_size: usize,
    pub chr_rom_size: usize,
    pub mapper_id: u16,
    pub mirroring: Mirroring,
    pub has_trainer: bool,
    pub has_battery: bool,
}

// Flags 6
const MIRRORING_VERTICAL_MASK: u8 = 0b0000_0001;
const MIRRORING_FOUR_SCREEN_MASK: u8 = 0b0000_1000;
const HAS_BATTERY_MASK: u8 = 0b0000_0010;
const HAS_TRAINER_MASK: u8 = 0b0000_0100;

pub fn parse_header(header: &[u8]) -> Result<Header, Box<dyn Error>> {
    let magic = &header[0..4];
    if magic != b"NES\x1a" {
        return Err("bad format".into());
    }

    // Bits 3-4 are "10" for NES 2.0
    let format = if header[7] & 0b0000_1100 == 0b0000_1000 {
        FileFormat::Nes20
    } else {
        FileFormat::INes
    };

    let prg_rom_size = {
        let multiplier = 16 * 1024; // 16 kB
        match format {
            FileFormat::INes => header[4] as usize * multiplier,
            FileFormat::Nes20 => {
                let size_lsb = header[4] as usize;
                let size_msb = (header[9] as usize & 0b0000_1111) << 8;
                if size_msb == 0b1111_0000_0000 {
                    let multiplier = size_lsb & 0b0000_0011;
                    let exponent = size_lsb & 0b1111_1100;
                    2 ^ exponent * (multiplier * 2 + 1)
                } else {
                    (size_msb | size_lsb) * multiplier
                }
            }
        }
    };

    let chr_rom_size = {
        let multiplier = 8 * 1024; // 8 kB
        match format {
            FileFormat::INes => header[5] as usize * multiplier,
            FileFormat::Nes20 => {
                let size_lsb = header[5] as usize;
                let size_msb = (header[9] as usize & 0b1111_0000) << 4;
                if size_msb == 0b1111_0000_0000 {
                    let multiplier = size_lsb & 0b0000_0011;
                    let exponent = size_lsb & 0b1111_1100;
                    2 ^ exponent * (multiplier * 2 + 1)
                } else {
                    (size_msb | size_lsb) * multiplier
                }
            }
        }
    };

    let mirroring = if header[6] & MIRRORING_FOUR_SCREEN_MASK != 0 {
        Mirroring::FourScreen
    } else if header[6] & MIRRORING_VERTICAL_MASK != 0 {
        Mirroring::Vertical
    } else {
        Mirroring::Horizontal
    };

    let has_battery = header[6] & HAS_BATTERY_MASK != 0;
    let has_trainer = header[6] & HAS_TRAINER_MASK != 0;

    let (mapper_id, _submapper_id) = match format {
        FileFormat::INes => {
            let bits_0_3 = (header[6] & 0b1111_0000) as u16 >> 4;
            let bits_4_7 = (header[7] & 0b1111_0000) as u16;
            (bits_4_7 | bits_0_3, 0)
        }
        FileFormat::Nes20 => {
            let bits_0_3 = ((header[6] & 0b1111_0000) as u16) >> 4;
            let bits_4_7 = (header[7] & 0b1111_0000) as u16;
            let bits_8_11 = ((header[8] & 0b0000_1111) as u16) << 8;
            let mapper_id = bits_8_11 | bits_4_7 | bits_0_3;
            let submapper_id = ((header[8] & 0b1111_0000) as u8) >> 4;
            (mapper_id, submapper_id)
        }
    };

    Ok(Header {
        format,
        prg_rom_size,
        chr_rom_size,
        mapper_id,
        mirroring,
        has_trainer,
        has_battery,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    #[should_panic(expected = "bad format")]
    fn err_on_bad_header() {
        let header = hex::decode("00000000000000000000000000000000").unwrap();
        parse_header(&header).unwrap();
    }

    #[test]
    fn ines_mapper_0_prg_16_chr_8_horz() {
        let header = hex::decode("4E45531A010100000000000000000000").unwrap();
        let header = parse_header(&header).unwrap();
        assert_eq!(
            header,
            Header {
                format: FileFormat::INes,
                mapper_id: 0,
                prg_rom_size: 16 * 1024,
                chr_rom_size: 8 * 1024,
                mirroring: Mirroring::Horizontal,
                has_trainer: false,
                has_battery: false,
            }
        )
    }
}

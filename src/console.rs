use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::mapper::Mapper;
use crate::ppu::Ppu;
use crate::Result;
use std::cell::RefCell;
use std::ops;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct CpuBus {
    wram: Vec<u8>,
    mapper: Rc<RefCell<Box<dyn Mapper>>>,
    ppu: Rc<RefCell<Ppu<PpuBus>>>,
}

impl Bus for CpuBus {
    fn read(&mut self, address: u16) -> u8 {
        match address {
            // 2 kB work RAM
            0x0000..=0x1fff => {
                let index = address as usize % self.wram.len();
                self.wram[index]
            }
            // PPU
            0x2000..=0x3fff => {
                unimplemented!()
            }
            // APU and I/O
            0x4000..=0x401f => {
                unimplemented!()
            }
            // Cartridge
            0x4020..=0xffff => self.mapper.borrow_mut().cpu_read(address),
        }
    }
    fn write(&mut self, address: u16, data: u8) {
        match address {
            // 2 kB RAM
            0x0000..=0x1fff => {
                let index = address as usize % self.wram.len();
                self.wram[index] = data
            }
            // PPU
            0x2000..=0x3fff => self.ppu.borrow_mut().write(address, data),
            // APU and I/O
            0x4000..=0x401f => unimplemented!(),
            // Cartridge
            0x4020..=0xffff => self.mapper.borrow_mut().cpu_write(address, data),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PpuBus {
    vram: Vec<u8>,
    mapper: Rc<RefCell<Box<dyn Mapper>>>,
}

impl Bus for PpuBus {
    fn read(&mut self, address: u16) -> u8 {
        todo!();
    }
    fn write(&mut self, address: u16, data: u8) {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct Console {
    cpu: Cpu<CpuBus>,
    ppu: Rc<RefCell<Ppu<PpuBus>>>,
}

impl Console {
    pub fn from_file(path: impl AsRef<Path> + 'static) -> Result<Console> {
        let mapper = Mapper::from_file(path)?;
        let mapper = Rc::new(RefCell::new(mapper));

        let ppu_bus = PpuBus {
            vram: vec![0; 2 * 1024], // 2 kB
            mapper: mapper.clone(),
        };

        let ppu = Ppu::new(ppu_bus);
        let ppu = Rc::new(RefCell::new(ppu));

        let cpu_bus = CpuBus {
            wram: vec![0; 2 * 1024], // 2 kB
            mapper: mapper.clone(),
            ppu: ppu.clone(),
        };

        let cpu = Cpu::new(cpu_bus);

        Ok(Console {
            cpu,
            ppu: ppu.clone(),
        })
    }

    pub fn read_range<R: ops::RangeBounds<u16>>(&mut self, range: R) -> Vec<u8> {
        self.cpu.bus.read_range(range)
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn step(&mut self) {
        self.cpu.step();
        self.ppu.borrow_mut().step();
    }
}

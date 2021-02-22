use crate::bus::Bus;

#[derive(Debug, Clone, Copy)]
pub struct Ppu<B: Bus> {
    pub(crate) bus: B,
}

impl<B: Bus> Ppu<B> {
    pub fn new(bus: B) -> Ppu<B> {
        Ppu { bus }
    }

    pub fn reset(&mut self) {}
    pub fn step(&mut self) {}

    pub fn read(&mut self, address: u16) -> u8 {
        0
    }

    pub fn write(&mut self, address: u16, data: u8) {}
}

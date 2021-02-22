use crate::addressing_mode::AddressingMode;
use crate::bus::Bus;
use crate::instructions::Instruction;
use std::fmt;

bitflags! {
    #[derive(Default)]
    pub struct Status: u8 {
        const CARRY = 0x01;
        const ZERO_RESULT = 0x02;
        const INTERRUPT_DISABLE = 0x04;
        const DECIMAL_MODE = 0x08;
        const BREAK_COMMAND = 0x10;
        const UNUSED = 0x20;
        const OVERFLOW = 0x40;
        const NEGATIVE_RESULT = 0x80;
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = if self.contains(Self::NEGATIVE_RESULT) {
            'N'
        } else {
            'n'
        };
        let v = if self.contains(Self::OVERFLOW) {
            'V'
        } else {
            'v'
        };
        let u = if self.contains(Self::UNUSED) {
            'U'
        } else {
            'u'
        };
        let b = if self.contains(Self::BREAK_COMMAND) {
            'B'
        } else {
            'b'
        };
        let d = if self.contains(Self::DECIMAL_MODE) {
            'D'
        } else {
            'd'
        };
        let i = if self.contains(Self::INTERRUPT_DISABLE) {
            'I'
        } else {
            'i'
        };
        let z = if self.contains(Self::ZERO_RESULT) {
            'Z'
        } else {
            'z'
        };
        let c = if self.contains(Self::CARRY) { 'C' } else { 'c' };
        write!(f, "{}{}{}{}{}{}{}{}", n, v, u, b, d, i, z, c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Registers {
    /// Program counter
    pc: u16,
    /// Stack pointer
    sp: u8,
    // Processor status
    ps: Status,
    // Accumulator
    a: u8,
    // X index
    x: u8,
    // Y index
    y: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            pc: 0x0000,
            sp: 0xff,
            ps: Status::from_bits_truncate(0x20),
            a: 0x00,
            x: 0x00,
            y: 0x00,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cpu<B: Bus> {
    pub(crate) bus: B,
    registers: Registers,
    cycle: u64,
}

impl<B: Bus> Cpu<B> {
    const STACK_BASE: u16 = 0x0100;

    pub fn new(bus: B) -> Cpu<B> {
        Cpu {
            bus,
            registers: Default::default(),
            cycle: 0,
        }
    }

    pub fn reset(&mut self) {
        self.registers.pc = {
            let pcl = self.bus.read(0xfffc);
            let pch = self.bus.read(0xfffd);
            u16::from_be_bytes([pch, pcl])
        };
        self.cycle = 8;
    }

    fn get_negative_result_flag(&self) -> bool {
        self.registers.ps.contains(Status::NEGATIVE_RESULT)
    }

    fn set_negative_result_flag(&mut self, value: bool) {
        self.registers.ps.set(Status::NEGATIVE_RESULT, value);
    }

    fn set_negative_result_flag_for_value(&mut self, value: u8) {
        let is_negative = (value as i8).is_negative();
        self.registers.ps.set(Status::NEGATIVE_RESULT, is_negative);
    }

    fn get_zero_result_flag(&self) -> bool {
        self.registers.ps.contains(Status::ZERO_RESULT)
    }

    fn set_zero_result_flag(&mut self, value: bool) {
        self.registers.ps.set(Status::ZERO_RESULT, value);
    }

    fn set_zero_result_flag_for_value(&mut self, value: u8) {
        let is_zero = value == 0;
        self.registers.ps.set(Status::ZERO_RESULT, is_zero);
    }

    fn get_carry_flag(&self) -> bool {
        self.registers.ps.contains(Status::CARRY)
    }

    fn set_carry_flag(&mut self, value: bool) {
        self.registers.ps.set(Status::CARRY, value);
    }

    fn get_overflow_flag(&self) -> bool {
        self.registers.ps.contains(Status::OVERFLOW)
    }

    fn set_overflow_flag(&mut self, value: bool) {
        self.registers.ps.set(Status::OVERFLOW, value);
    }

    fn set_overflow_flag_for_result(&mut self, in_a: u8, in_b: u8, result: u8) {
        let did_overflow = (in_a ^ result) & (in_b ^ result) & 0x80 == 0x80;
        self.registers.ps.set(Status::OVERFLOW, did_overflow);
    }

    fn get_interrupt_disable_flag(&self) -> bool {
        self.registers.ps.contains(Status::INTERRUPT_DISABLE)
    }

    fn set_interrupt_disable_flag(&mut self, value: bool) {
        self.registers.ps.set(Status::INTERRUPT_DISABLE, value);
    }

    fn set_decimal_mode_flag(&mut self, value: bool) {
        self.registers.ps.set(Status::DECIMAL_MODE, value);
    }

    fn stack_address(&self) -> u16 {
        Self::STACK_BASE + self.registers.sp as u16
    }

    fn push(&mut self, data: u8) {
        let address = self.stack_address();
        self.registers.sp -= 1;
        self.write(address, data)
    }

    fn pull(&mut self) -> u8 {
        let address = self.stack_address();
        self.registers.sp += 1;
        self.read(address)
    }

    fn fetch(&mut self) -> u8 {
        let data = self.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        data
    }

    fn read(&mut self, address: u16) -> u8 {
        self.cycle += 1;
        self.bus.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.cycle += 1;
        self.bus.write(address, data)
    }

    fn decode(&mut self) -> (String, String) {
        let opcode = self.bus.read(self.registers.pc);
        let mnemonic = Instruction::for_opcode(opcode);
        let addressing_mode = AddressingMode::for_opcode(opcode);
        match addressing_mode {
            AddressingMode::Absolute => {
                let adl = self.bus.read(self.registers.pc.wrapping_add(1));
                let adh = self.bus.read(self.registers.pc.wrapping_add(2));
                let address = u16::from_be_bytes([adh, adl]);
                let byte_code = format!("{:02X} {:02X} {:02X}", opcode, adl, adh);
                let disassembly = format!("{} ${:04X}", mnemonic, address);
                (byte_code, disassembly)
            }
            AddressingMode::AbsoluteX => {
                let adl = self.bus.read(self.registers.pc.wrapping_add(1));
                let adh = self.bus.read(self.registers.pc.wrapping_add(2));
                let address = u16::from_be_bytes([adh, adl]);
                let byte_code = format!("{:02X} {:02X} {:02X}", opcode, adl, adh);
                let disassembly = format!("{} ${:04X},X", mnemonic, address);
                (byte_code, disassembly)
            }
            AddressingMode::AbsoluteY => {
                let adl = self.bus.read(self.registers.pc.wrapping_add(1));
                let adh = self.bus.read(self.registers.pc.wrapping_add(2));
                let address = u16::from_be_bytes([adh, adl]);
                let byte_code = format!("{:02X} {:02X} {:02X}", opcode, adl, adh);
                let disassembly = format!("{} ${:04X},Y", mnemonic, address);
                (byte_code, disassembly)
            }
            AddressingMode::Accumulator => {
                let byte_code = format!("{:02X}", opcode);
                let disassembly = format!("{} A", mnemonic);
                (byte_code, disassembly)
            }
            AddressingMode::Immediate => {
                let operand = self.bus.read(self.registers.pc.wrapping_add(1));
                let byte_code = format!("{:02X} {:02X}", opcode, operand);
                let disassembly = format!("{} #${:02X}", mnemonic, operand);
                (byte_code, disassembly)
            }
            AddressingMode::Implied => {
                let byte_code = format!("{:02X}", opcode);
                let disassembly = format!("{}", mnemonic);
                (byte_code, disassembly)
            }
            AddressingMode::IndirectAbsolute => {
                let idl = self.bus.read(self.registers.pc.wrapping_add(1));
                let idh = self.bus.read(self.registers.pc.wrapping_add(2));
                let address = u16::from_be_bytes([idh, idl]);
                let byte_code = format!("{:02X} {:02X} {:02X}", opcode, idl, idh);
                let disassembly = format!("{} (${:04X})", mnemonic, address);
                (byte_code, disassembly)
            }
            AddressingMode::IndirectZeroPageX => {
                let bal = self.bus.read(self.registers.pc.wrapping_add(1));
                let byte_code = format!("{:02X} {:02X}", opcode, bal);
                let disassembly = format!("{} (${:02X},X)", mnemonic, bal);
                (byte_code, disassembly)
            }
            AddressingMode::IndirectZeroPageY => {
                let bal = self.bus.read(self.registers.pc.wrapping_add(1));
                let byte_code = format!("{:02X} {:02X}", opcode, bal);
                let disassembly = format!("{} (${:02X}),Y", mnemonic, bal);
                (byte_code, disassembly)
            }
            AddressingMode::Relative => {
                let offset = self.bus.read(self.registers.pc.wrapping_add(1));
                let byte_code = format!("{:02X} {:02X}", opcode, offset);
                let disassembly = format!("{} *{:+}", mnemonic, offset);
                (byte_code, disassembly)
            }
            AddressingMode::ZeroPage => {
                let bal = self.bus.read(self.registers.pc.wrapping_add(1));
                let byte_code = format!("{:02X} {:02X}", opcode, bal);
                let disassembly = format!("{} ${:02X}", mnemonic, bal);
                (byte_code, disassembly)
            }
            AddressingMode::ZeroPageX => {
                let bal = self.bus.read(self.registers.pc.wrapping_add(1));
                let byte_code = format!("{:02X} {:02X}", opcode, bal);
                let disassembly = format!("{} ${:02X},X", mnemonic, bal);
                (byte_code, disassembly)
            }
            AddressingMode::ZeroPageY => {
                let bal = self.bus.read(self.registers.pc.wrapping_add(1));
                let byte_code = format!("{:02X} {:02X}", opcode, bal);
                let disassembly = format!("{} ${:02X},Y", mnemonic, bal);
                (byte_code, disassembly)
            }
            AddressingMode::Unimplemented => unimplemented!(
                "Encountered opcode {:02X} at {:04X}",
                opcode,
                self.registers.pc
            ),
        }
    }

    pub fn step(&mut self) {
        let (byte_code, disassembly) = self.decode();
        println!(
            "{:04X} {:8}   {:11}     A:{:02X} X:{:02X} Y:{:02X} S:{:02X} P:{} C:{} Stack: {:02X?}",
            self.registers.pc,
            byte_code,
            disassembly,
            self.registers.a,
            self.registers.x,
            self.registers.y,
            self.registers.sp,
            self.registers.ps,
            self.cycle,
            self.bus.read_range(self.stack_address() + 1..=0x01FF),
        );

        let opcode = self.fetch();
        let instruction = Self::INSTRUCTIONS[opcode as usize];
        instruction(self);
    }

    fn fetch_implied(&mut self) {
        self.read(self.registers.pc);
    }

    fn fetch_accumulator(&mut self) -> u8 {
        self.read(self.registers.pc);
        self.registers.a
    }

    fn fetch_immediate(&mut self) -> u8 {
        self.fetch()
    }

    fn fetch_absolute(&mut self) -> u16 {
        let adl = self.fetch();
        let adh = self.fetch();
        u16::from_be_bytes([adh, adl])
    }

    fn fetch_absolute_x_read(&mut self) -> u16 {
        let bal = self.fetch();
        let bah = self.fetch();
        let (bal_x, carry) = bal.overflowing_add(self.registers.x);
        let same_page_address = u16::from_be_bytes([bah, bal_x]);
        if carry {
            let bah_c = bah.wrapping_add(1);
            let next_page_address = u16::from_be_bytes([bah_c, bal_x]);
            self.read(same_page_address);
            next_page_address
        } else {
            same_page_address
        }
    }

    fn fetch_absolute_x_write(&mut self) -> u16 {
        let bal = self.fetch();
        let bah = self.fetch();
        let (bal_x, carry) = bal.overflowing_add(self.registers.x);
        let same_page_address = u16::from_be_bytes([bah, bal_x]);
        if carry {
            let bah_c = bah.wrapping_add(1);
            let next_page_address = u16::from_be_bytes([bah_c, bal_x]);
            self.read(same_page_address);
            next_page_address
        } else {
            self.read(same_page_address);
            same_page_address
        }
    }

    fn fetch_absolute_y_read(&mut self) -> u16 {
        let bal = self.fetch();
        let bah = self.fetch();
        let (bal_y, carry) = bal.overflowing_add(self.registers.y);
        let same_page_address = u16::from_be_bytes([bah, bal_y]);
        if carry {
            let bah_c = bah.wrapping_add(1);
            let next_page_address = u16::from_be_bytes([bah_c, bal_y]);
            self.read(same_page_address);
            next_page_address
        } else {
            same_page_address
        }
    }

    fn fetch_absolute_y_write(&mut self) -> u16 {
        let bal = self.fetch();
        let bah = self.fetch();
        let (bal_y, carry) = bal.overflowing_add(self.registers.y);
        let same_page_address = u16::from_be_bytes([bah, bal_y]);
        if carry {
            let bah_c = bah.wrapping_add(1);
            let next_page_address = u16::from_be_bytes([bah_c, bal_y]);
            self.read(same_page_address);
            next_page_address
        } else {
            self.read(same_page_address);
            same_page_address
        }
    }

    fn fetch_zero_page(&mut self) -> u16 {
        let adl = self.fetch();
        u16::from_be_bytes([0x00, adl])
    }

    fn fetch_zero_page_x(&mut self) -> u16 {
        let bal = self.fetch();
        self.read(u16::from_be_bytes([0x00, bal]));
        let bal_x = bal.wrapping_add(self.registers.x);
        u16::from_be_bytes([0x00, bal_x])
    }

    fn fetch_zero_page_y(&mut self) -> u16 {
        let bal = self.fetch();
        self.read(u16::from_be_bytes([0x00, bal]));
        let bal_y = bal.wrapping_add(self.registers.y);
        u16::from_be_bytes([0x00, bal_y])
    }

    fn fetch_indirect_x(&mut self) -> u16 {
        let bal = self.fetch();
        self.read(u16::from_be_bytes([0x00, bal]));
        let bal_x = bal.wrapping_add(self.registers.x);
        let bal_x_1 = bal_x.wrapping_add(1);
        let adl = self.read(u16::from_be_bytes([0x00, bal_x]));
        let adh = self.read(u16::from_be_bytes([0x00, bal_x_1]));
        u16::from_be_bytes([adh, adl])
    }

    fn fetch_indirect_y_read(&mut self) -> u16 {
        let ial = self.fetch();
        let ial_1 = ial.wrapping_add(1);
        let bal = self.read(u16::from_be_bytes([0x00, ial]));
        let bah = self.read(u16::from_be_bytes([0x00, ial_1]));
        let (bal_y, carry) = bal.overflowing_add(self.registers.y);
        let same_page_address = u16::from_be_bytes([bah, bal_y]);
        if carry {
            let bah_c = bah.wrapping_add(1);
            let next_page_address = u16::from_be_bytes([bah_c, bal_y]);
            self.read(same_page_address);
            next_page_address
        } else {
            same_page_address
        }
    }

    fn fetch_indirect_y_write(&mut self) -> u16 {
        let ial = self.fetch();
        let ial_1 = ial.wrapping_add(1);
        let bal = self.read(u16::from_be_bytes([0x00, ial]));
        let bah = self.read(u16::from_be_bytes([0x00, ial_1]));
        let (bal_y, carry) = bal.overflowing_add(self.registers.y);
        let same_page_address = u16::from_be_bytes([bah, bal_y]);
        if carry {
            let bah_c = bah.wrapping_add(1);
            let next_page_address = u16::from_be_bytes([bah_c, bal_y]);
            self.read(same_page_address);
            next_page_address
        } else {
            self.read(same_page_address);
            same_page_address
        }
    }

    fn nop_implied(&mut self) {
        self.fetch_implied();
    }

    fn clc_implied(&mut self) {
        self.fetch_implied();
        self.set_carry_flag(false);
    }

    fn cld_implied(&mut self) {
        self.fetch_implied();
        self.set_decimal_mode_flag(false);
    }

    fn cli_implied(&mut self) {
        self.fetch_implied();
        self.set_interrupt_disable_flag(false);
    }

    fn clv_implied(&mut self) {
        self.fetch_implied();
        self.set_overflow_flag(false);
    }

    fn sec_implied(&mut self) {
        self.fetch_implied();
        self.set_carry_flag(true);
    }

    fn sed_implied(&mut self) {
        self.fetch_implied();
        self.set_decimal_mode_flag(true);
    }

    fn sei_implied(&mut self) {
        self.fetch_implied();
        self.set_interrupt_disable_flag(true);
    }

    fn sev_implied(&mut self) {
        self.fetch_implied();
        self.set_overflow_flag(true);
    }

    fn inx_implied(&mut self) {
        let result = self.registers.x.wrapping_add(1);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.x = result;
    }

    fn iny_implied(&mut self) {
        let result = self.registers.y.wrapping_add(1);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.y = result;
    }

    fn dex_implied(&mut self) {
        let result = self.registers.x.wrapping_sub(1);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.x = result;
    }

    fn dey_implied(&mut self) {
        let result = self.registers.y.wrapping_sub(1);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.y = result;
    }

    fn tax_implied(&mut self) {
        self.fetch_implied();
        self.registers.x = self.registers.a;
    }

    fn tay_implied(&mut self) {
        self.fetch_implied();
        self.registers.y = self.registers.a;
    }

    fn txa_implied(&mut self) {
        self.fetch_implied();
        self.registers.a = self.registers.x;
    }

    fn tya_implied(&mut self) {
        self.fetch_implied();
        self.registers.a = self.registers.y;
    }

    fn tsx_implied(&mut self) {
        self.fetch_implied();
        self.registers.x = self.registers.sp;
    }

    fn txs_implied(&mut self) {
        self.fetch_implied();
        self.registers.sp = self.registers.x;
    }

    fn inc_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        self.inc(address);
    }

    fn inc_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        self.inc(address);
    }

    fn inc_absolute(&mut self) {
        let address = self.fetch_absolute();
        self.inc(address);
    }

    fn inc_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_write();
        self.inc(address);
    }

    fn inc(&mut self, address: u16) {
        let value = self.read(address);
        self.write(address, value);
        let result = value.wrapping_add(1);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.write(address, result);
    }

    fn dec_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        self.dec(address);
    }

    fn dec_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        self.dec(address);
    }

    fn dec_absolute(&mut self) {
        let address = self.fetch_absolute();
        self.dec(address);
    }

    fn dec_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_write();
        self.dec(address);
    }

    fn dec(&mut self, address: u16) {
        let value = self.read(address);
        self.write(address, value);
        let result = value.wrapping_sub(1);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.write(address, result);
    }

    fn asl_accumulator(&mut self) {
        let value = self.fetch_accumulator();
        let result = self.asl(value);
        self.registers.a = result;
    }

    fn asl_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        self.asl_mem(address);
    }

    fn asl_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        self.asl_mem(address);
    }

    fn asl_absolute(&mut self) {
        let address = self.fetch_absolute();
        self.asl_mem(address);
    }

    fn asl_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_write();
        self.asl_mem(address);
    }

    fn asl_mem(&mut self, address: u16) {
        let value = self.read(address);
        self.write(address, value);
        let result = self.asl(value);
        self.write(address, result);
    }

    fn asl(&mut self, value: u8) -> u8 {
        let carry_out = value & 0x80 == 0x80;
        let result = value << 1;
        self.set_carry_flag(carry_out);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        result
    }

    fn lsr_accumulator(&mut self) {
        let value = self.fetch_accumulator();
        let result = self.lsr(value);
        self.registers.a = result;
    }

    fn lsr_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        self.lsr_mem(address);
    }

    fn lsr_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        self.lsr_mem(address);
    }

    fn lsr_absolute(&mut self) {
        let address = self.fetch_absolute();
        self.lsr_mem(address);
    }

    fn lsr_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_write();
        self.lsr_mem(address);
    }

    fn lsr_mem(&mut self, address: u16) {
        let value = self.read(address);
        self.write(address, value);
        let result = self.lsr(value);
        self.write(address, result);
    }

    fn lsr(&mut self, value: u8) -> u8 {
        let carry_out = value & 0x80 == 0x80;
        let result = value >> 1;
        self.set_carry_flag(carry_out);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        result
    }

    fn rol_accumulator(&mut self) {
        let value = self.fetch_accumulator();
        let result = self.rol(value);
        self.registers.a = result;
    }

    fn rol_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        self.rol_mem(address);
    }

    fn rol_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        self.rol_mem(address);
    }

    fn rol_absolute(&mut self) {
        let address = self.fetch_absolute();
        self.rol_mem(address);
    }

    fn rol_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_write();
        self.rol_mem(address);
    }

    fn rol_mem(&mut self, address: u16) {
        let value = self.read(address);
        self.write(address, value);
        let result = self.rol(value);
        self.write(address, result);
    }

    fn rol(&mut self, value: u8) -> u8 {
        let carry_in = self.get_carry_flag();
        let carry_out = value & 0x80 == 0x80;
        let result = value << 1;
        let result = if carry_in { result | 0x01 } else { result };
        self.set_carry_flag(carry_out);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        result
    }

    fn ror_accumulator(&mut self) {
        let value = self.fetch_accumulator();
        let result = self.ror(value);
        self.registers.a = result;
    }

    fn ror_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        self.ror_mem(address);
    }

    fn ror_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        self.ror_mem(address);
    }

    fn ror_absolute(&mut self) {
        let address = self.fetch_absolute();
        self.ror_mem(address);
    }

    fn ror_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_write();
        self.ror_mem(address);
    }

    fn ror_mem(&mut self, address: u16) {
        let value = self.read(address);
        self.write(address, value);
        let result = self.ror(value);
        self.write(address, result);
    }

    fn ror(&mut self, value: u8) -> u8 {
        let carry_in = self.get_carry_flag();
        let carry_out = value & 0x01 == 0x01;
        let result = value >> 1;
        let result = if carry_in { result | 0x80 } else { result };
        self.set_carry_flag(carry_out);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        result
    }

    fn lda_immediate(&mut self) {
        let value = self.fetch_immediate();
        self.lda(value);
    }

    fn lda_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        self.lda(value);
    }

    fn lda_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        let value = self.read(address);
        self.lda(value);
    }

    fn lda_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        self.lda(value);
    }

    fn lda_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_read();
        let value = self.read(address);
        self.lda(value);
    }

    fn lda_absolute_y(&mut self) {
        let address = self.fetch_absolute_y_read();
        let value = self.read(address);
        self.lda(value);
    }

    fn lda_indirect_x(&mut self) {
        let address = self.fetch_indirect_x();
        let value = self.read(address);
        self.lda(value);
    }

    fn lda_indirect_y(&mut self) {
        let address = self.fetch_indirect_y_read();
        let value = self.read(address);
        self.lda(value);
    }

    fn lda(&mut self, value: u8) {
        self.set_zero_result_flag_for_value(value);
        self.set_negative_result_flag_for_value(value);
        self.registers.a = value;
    }

    fn ldx_immediate(&mut self) {
        let value = self.fetch_immediate();
        self.ldx(value);
    }

    fn ldx_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        self.ldx(value);
    }

    fn ldx_zero_page_y(&mut self) {
        let address = self.fetch_zero_page_y();
        let value = self.read(address);
        self.ldx(value);
    }

    fn ldx_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        self.ldx(value);
    }

    fn ldx_absolute_y(&mut self) {
        let address = self.fetch_absolute_y_read();
        let value = self.read(address);
        self.ldx(value);
    }

    fn ldx(&mut self, value: u8) {
        self.set_zero_result_flag_for_value(value);
        self.set_negative_result_flag_for_value(value);
        self.registers.x = value;
    }

    fn ldy_immediate(&mut self) {
        let value = self.fetch_immediate();
        self.ldy(value);
    }

    fn ldy_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        self.ldy(value);
    }

    fn ldy_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        let value = self.read(address);
        self.ldy(value);
    }

    fn ldy_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        self.ldy(value);
    }

    fn ldy_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_read();
        let value = self.read(address);
        self.ldy(value);
    }

    fn ldy(&mut self, value: u8) {
        self.set_zero_result_flag_for_value(value);
        self.set_negative_result_flag_for_value(value);
        self.registers.y = value;
    }

    fn adc_immediate(&mut self) {
        let value = self.fetch_immediate();
        self.adc(value);
    }

    fn adc_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        self.adc(value);
    }

    fn adc_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        let value = self.read(address);
        self.adc(value);
    }

    fn adc_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        self.adc(value);
    }

    fn adc_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_read();
        let value = self.read(address);
        self.adc(value);
    }

    fn adc_absolute_y(&mut self) {
        let address = self.fetch_absolute_y_read();
        let value = self.read(address);
        self.adc(value);
    }

    fn adc_indirect_x(&mut self) {
        let address = self.fetch_indirect_x();
        let value = self.read(address);
        self.adc(value);
    }

    fn adc_indirect_y(&mut self) {
        let address = self.fetch_indirect_y_read();
        let value = self.read(address);
        self.adc(value);
    }

    fn adc(&mut self, value: u8) {
        let carry_in = self.get_carry_flag();
        let accumulator = self.registers.a;
        let (result, carry_out_1) = value.overflowing_add(carry_in as u8);
        let (result, carry_out_2) = accumulator.overflowing_add(result);
        let carry_out = carry_out_1 | carry_out_2;
        self.set_overflow_flag_for_result(accumulator, value, result);
        self.set_carry_flag(carry_out);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.a = result;
    }

    fn sbc_immediate(&mut self) {
        let value = self.fetch_immediate();
        self.sbc(value);
    }

    fn sbc_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        self.sbc(value);
    }

    fn sbc_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        let value = self.read(address);
        self.sbc(value);
    }

    fn sbc_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        self.sbc(value);
    }

    fn sbc_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_read();
        let value = self.read(address);
        self.sbc(value);
    }

    fn sbc_absolute_y(&mut self) {
        let address = self.fetch_absolute_y_read();
        let value = self.read(address);
        self.sbc(value);
    }

    fn sbc_indirect_x(&mut self) {
        let address = self.fetch_indirect_x();
        let value = self.read(address);
        self.sbc(value);
    }

    fn sbc_indirect_y(&mut self) {
        let address = self.fetch_indirect_y_read();
        let value = self.read(address);
        self.sbc(value);
    }

    fn sbc(&mut self, value: u8) {
        let borrow_in = !self.get_carry_flag();
        let accumulator = self.registers.a;
        let (result, borrow_out_1) = value.overflowing_sub(borrow_in as u8);
        let (result, borrow_out_2) = accumulator.overflowing_sub(result);
        let carry_out = !(borrow_out_1 | borrow_out_2);
        self.set_overflow_flag_for_result(accumulator, value, result);
        self.set_carry_flag(carry_out);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.a = result;
    }

    fn and_immediate(&mut self) {
        let value = self.fetch_immediate();
        self.and(value);
    }

    fn and_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        self.and(value);
    }

    fn and_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        let value = self.read(address);
        self.and(value);
    }

    fn and_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        self.and(value);
    }

    fn and_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_read();
        let value = self.read(address);
        self.and(value);
    }

    fn and_absolute_y(&mut self) {
        let address = self.fetch_absolute_y_read();
        let value = self.read(address);
        self.and(value);
    }

    fn and_indirect_x(&mut self) {
        let address = self.fetch_indirect_x();
        let value = self.read(address);
        self.and(value);
    }

    fn and_indirect_y(&mut self) {
        let address = self.fetch_indirect_y_read();
        let value = self.read(address);
        self.and(value);
    }

    fn and(&mut self, value: u8) {
        let accumulator = self.registers.a;
        let result = accumulator & value;
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.a = result;
    }

    fn eor_immediate(&mut self) {
        let value = self.fetch_immediate();
        self.eor(value);
    }

    fn eor_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        self.eor(value);
    }

    fn eor_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        let value = self.read(address);
        self.eor(value);
    }

    fn eor_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        self.eor(value);
    }

    fn eor_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_read();
        let value = self.read(address);
        self.eor(value);
    }

    fn eor_absolute_y(&mut self) {
        let address = self.fetch_absolute_y_read();
        let value = self.read(address);
        self.eor(value);
    }

    fn eor_indirect_x(&mut self) {
        let address = self.fetch_indirect_x();
        let value = self.read(address);
        self.eor(value);
    }

    fn eor_indirect_y(&mut self) {
        let address = self.fetch_indirect_y_read();
        let value = self.read(address);
        self.eor(value);
    }

    fn eor(&mut self, value: u8) {
        let accumulator = self.registers.a;
        let result = accumulator ^ value;
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.a = result;
    }

    fn ora_immediate(&mut self) {
        let value = self.fetch_immediate();
        self.ora(value);
    }

    fn ora_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        self.ora(value);
    }

    fn ora_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        let value = self.read(address);
        self.ora(value);
    }

    fn ora_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        self.ora(value);
    }

    fn ora_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_read();
        let value = self.read(address);
        self.ora(value);
    }

    fn ora_absolute_y(&mut self) {
        let address = self.fetch_absolute_y_read();
        let value = self.read(address);
        self.ora(value);
    }

    fn ora_indirect_x(&mut self) {
        let address = self.fetch_indirect_x();
        let value = self.read(address);
        self.ora(value);
    }

    fn ora_indirect_y(&mut self) {
        let address = self.fetch_indirect_y_read();
        let value = self.read(address);
        self.ora(value);
    }

    fn ora(&mut self, value: u8) {
        let accumulator = self.registers.a;
        let result = accumulator | value;
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.a = result;
    }

    fn bit_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        self.bit(value);
    }

    fn bit_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        self.bit(value);
    }

    fn bit(&mut self, value: u8) {
        let accumulator = self.registers.a;
        let result = accumulator & value;
        let negative_out = value & 0x80 == 0x80;
        let overflow_out = value & 0x40 == 0x40;
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag(negative_out);
        self.set_overflow_flag(overflow_out);
    }

    fn cmp_immediate(&mut self) {
        let value = self.fetch_immediate();
        let register = self.registers.a;
        self.cmp(register, value);
    }

    fn cmp_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        let register = self.registers.a;
        self.cmp(register, value);
    }

    fn cmp_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        let value = self.read(address);
        let register = self.registers.a;
        self.cmp(register, value);
    }

    fn cmp_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        let register = self.registers.a;
        self.cmp(register, value);
    }

    fn cmp_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_read();
        let value = self.read(address);
        let register = self.registers.a;
        self.cmp(register, value);
    }

    fn cmp_absolute_y(&mut self) {
        let address = self.fetch_absolute_y_read();
        let value = self.read(address);
        let register = self.registers.a;
        self.cmp(register, value);
    }

    fn cmp_indirect_x(&mut self) {
        let address = self.fetch_indirect_x();
        let value = self.read(address);
        let register = self.registers.a;
        self.cmp(register, value);
    }

    fn cmp_indirect_y(&mut self) {
        let address = self.fetch_indirect_y_read();
        let value = self.read(address);
        let register = self.registers.a;
        self.cmp(register, value);
    }

    fn cpx_immediate(&mut self) {
        let value = self.fetch_immediate();
        let register = self.registers.x;
        self.cmp(register, value);
    }

    fn cpx_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        let register = self.registers.x;
        self.cmp(register, value);
    }

    fn cpx_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        let register = self.registers.x;
        self.cmp(register, value);
    }

    fn cpy_immediate(&mut self) {
        let value = self.fetch_immediate();
        let register = self.registers.y;
        self.cmp(register, value);
    }

    fn cpy_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        let value = self.read(address);
        let register = self.registers.y;
        self.cmp(register, value);
    }

    fn cpy_absolute(&mut self) {
        let address = self.fetch_absolute();
        let value = self.read(address);
        let register = self.registers.y;
        self.cmp(register, value);
    }

    fn cmp(&mut self, register: u8, value: u8) {
        let (result, carry_out) = register.overflowing_sub(value);
        self.set_carry_flag(carry_out);
        self.set_zero_result_flag_for_value(result);
        self.set_negative_result_flag_for_value(result);
        self.registers.a = result;
    }

    fn sta_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        self.sta(address);
    }

    fn sta_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        self.sta(address);
    }

    fn sta_absolute(&mut self) {
        let address = self.fetch_absolute();
        self.sta(address);
    }

    fn sta_absolute_x(&mut self) {
        let address = self.fetch_absolute_x_write();
        self.sta(address);
    }

    fn sta_absolute_y(&mut self) {
        let address = self.fetch_absolute_y_write();
        self.sta(address);
    }

    fn sta_indirect_x(&mut self) {
        let address = self.fetch_indirect_x();
        self.sta(address);
    }

    fn sta_indirect_y(&mut self) {
        let address = self.fetch_indirect_y_write();
        self.sta(address);
    }

    fn sta(&mut self, address: u16) {
        self.write(address, self.registers.a);
    }

    fn stx_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        self.sta(address);
    }

    fn stx_zero_page_y(&mut self) {
        let address = self.fetch_zero_page_y();
        self.sta(address);
    }

    fn stx_absolute(&mut self) {
        let address = self.fetch_absolute();
        self.sta(address);
    }

    fn stx(&mut self, address: u16) {
        self.write(address, self.registers.x);
    }

    fn sty_zero_page(&mut self) {
        let address = self.fetch_zero_page();
        self.sta(address);
    }

    fn sty_zero_page_x(&mut self) {
        let address = self.fetch_zero_page_x();
        self.sta(address);
    }

    fn sty_absolute(&mut self) {
        let address = self.fetch_absolute();
        self.sta(address);
    }

    fn sty(&mut self, address: u16) {
        self.write(address, self.registers.y);
    }

    fn pha_implied(&mut self) {
        self.fetch_implied();
        self.push(self.registers.a);
    }

    fn php_implied(&mut self) {
        self.fetch_implied();
        self.push(self.registers.ps.bits());
    }

    fn pla_implied(&mut self) {
        self.read(self.stack_address());
        let value = self.pull();
        self.set_zero_result_flag_for_value(value);
        self.set_negative_result_flag_for_value(value);
        self.registers.a = value;
    }

    fn plp_implied(&mut self) {
        self.read(self.stack_address());
        let value = self.pull();
        self.registers.ps = Status::from_bits_truncate(value);
    }

    fn bcc_relative(&mut self) {
        let condition = !self.get_carry_flag();
        self.branch(condition);
    }

    fn bcs_relative(&mut self) {
        let condition = self.get_carry_flag();
        self.branch(condition);
    }

    fn bne_relative(&mut self) {
        let condition = !self.get_zero_result_flag();
        self.branch(condition);
    }

    fn beq_relative(&mut self) {
        let condition = self.get_zero_result_flag();
        self.branch(condition);
    }

    fn bpl_relative(&mut self) {
        let condition = !self.get_negative_result_flag();
        self.branch(condition);
    }

    fn bmi_relative(&mut self) {
        let condition = self.get_negative_result_flag();
        self.branch(condition);
    }

    fn bvc_relative(&mut self) {
        let condition = !self.get_overflow_flag();
        self.branch(condition);
    }

    fn bvs_relative(&mut self) {
        let condition = self.get_overflow_flag();
        self.branch(condition);
    }

    fn branch(&mut self, condition: bool) {
        let offset = self.fetch();
        if !condition {
            return;
        }
        let [pch, pcl] = self.registers.pc.to_be_bytes();
        let (pcl_offset, carry) = (pcl as i16).overflowing_add(offset as i16);
        let pcl_offset = pcl_offset as u8;
        let same_page_address = u16::from_be_bytes([pch, pcl_offset]);
        self.read(same_page_address);
        self.registers.pc = if carry {
            let pch_c = pch.wrapping_add(1);
            let next_page_address = u16::from_be_bytes([pch_c, pcl_offset]);
            self.read(next_page_address);
            next_page_address
        } else {
            same_page_address
        }
    }

    fn brk_implied(&mut self) {
        self.fetch();
        let [pch, pcl] = self.registers.pc.to_be_bytes();
        let p = self.registers.ps.bits();
        self.push(pch);
        self.push(pcl);
        self.push(p);
        let adl = self.read(0xfffe);
        let adh = self.read(0xffff);
        self.registers.pc = u16::from_be_bytes([adh, adl]);
    }

    fn jsr_absolute(&mut self) {
        let adl = self.fetch();
        self.read(self.stack_address());
        let [pch, pcl] = self.registers.pc.to_be_bytes();
        self.push(pch);
        self.push(pcl);
        let adh = self.fetch();
        self.registers.pc = u16::from_be_bytes([adh, adl]);
    }

    fn rts_implied(&mut self) {
        self.fetch_immediate();
        self.read(self.stack_address());
        let pcl = self.pull();
        let pch = self.pull();
        self.read(u16::from_be_bytes([pch, pcl]));
        let pcl_1 = pcl.wrapping_add(1);
        self.registers.pc = u16::from_be_bytes([pch, pcl_1]);
    }

    fn rti_implied(&mut self) {
        self.fetch_immediate();
        self.read(self.stack_address());
        let p = self.pull();
        let pcl = self.pull();
        let pch = self.pull();
        self.registers.ps = Status::from_bits_truncate(p);
        self.registers.pc = u16::from_be_bytes([pch, pcl]);
    }

    fn jmp_absolute(&mut self) {
        let adl = self.fetch();
        let adh = self.fetch();
        self.registers.pc = u16::from_be_bytes([adh, adl]);
    }

    fn jmp_indirect(&mut self) {
        let ial = self.fetch();
        let iah = self.fetch();
        let adl = self.read(u16::from_be_bytes([iah, ial]));
        let ial_1 = ial.wrapping_add(1);
        let adh = self.read(u16::from_be_bytes([iah, ial_1]));
        self.registers.pc = u16::from_be_bytes([adh, adl]);
    }

    fn unimplemented(&mut self) {
        unimplemented!();
    }

    const INSTRUCTIONS: [fn(&mut Self); 256] = [
        Self::brk_implied,     // 00
        Self::ora_indirect_x,  // 01
        Self::unimplemented,   // 02
        Self::unimplemented,   // 03
        Self::unimplemented,   // 04
        Self::ora_zero_page,   // 05
        Self::asl_zero_page,   // 06
        Self::unimplemented,   // 07
        Self::php_implied,     // 08
        Self::ora_immediate,   // 09
        Self::asl_accumulator, // 0A
        Self::unimplemented,   // 0B
        Self::unimplemented,   // 0C
        Self::ora_absolute,    // 0D
        Self::asl_absolute,    // 0E
        Self::unimplemented,   // 0F
        Self::bpl_relative,    // 10
        Self::ora_indirect_y,  // 11
        Self::unimplemented,   // 12
        Self::unimplemented,   // 13
        Self::unimplemented,   // 14
        Self::ora_zero_page_x, // 15
        Self::asl_zero_page_x, // 16
        Self::unimplemented,   // 17
        Self::clc_implied,     // 18
        Self::ora_absolute_y,  // 19
        Self::unimplemented,   // 1A
        Self::unimplemented,   // 1B
        Self::unimplemented,   // 1C
        Self::ora_absolute_x,  // 1D
        Self::asl_absolute_x,  // 1E
        Self::unimplemented,   // 1F
        Self::jsr_absolute,    // 20
        Self::and_indirect_x,  // 21
        Self::unimplemented,   // 22
        Self::unimplemented,   // 23
        Self::bit_zero_page,   // 24
        Self::and_zero_page,   // 25
        Self::rol_zero_page,   // 26
        Self::unimplemented,   // 27
        Self::plp_implied,     // 28
        Self::and_immediate,   // 29
        Self::rol_accumulator, // 2A
        Self::unimplemented,   // 2B
        Self::bit_absolute,    // 2C
        Self::and_absolute,    // 2D
        Self::rol_absolute,    // 2E
        Self::unimplemented,   // 2F
        Self::bmi_relative,    // 30
        Self::and_indirect_y,  // 31
        Self::unimplemented,   // 32
        Self::unimplemented,   // 33
        Self::unimplemented,   // 34
        Self::and_zero_page_x, // 35
        Self::rol_zero_page_x, // 36
        Self::unimplemented,   // 37
        Self::sec_implied,     // 38
        Self::and_absolute_y,  // 39
        Self::unimplemented,   // 3A
        Self::unimplemented,   // 3B
        Self::unimplemented,   // 3C
        Self::and_absolute_x,  // 3D
        Self::rol_absolute_x,  // 3E
        Self::unimplemented,   // 3F
        Self::rti_implied,     // 40
        Self::eor_indirect_x,  // 41
        Self::unimplemented,   // 42
        Self::unimplemented,   // 43
        Self::unimplemented,   // 44
        Self::eor_zero_page,   // 45
        Self::lsr_zero_page,   // 46
        Self::unimplemented,   // 47
        Self::pha_implied,     // 48
        Self::eor_immediate,   // 49
        Self::lsr_accumulator, // 4A
        Self::unimplemented,   // 4B
        Self::jmp_absolute,    // 4C
        Self::eor_absolute,    // 4D
        Self::lsr_absolute,    // 4E
        Self::unimplemented,   // 4F
        Self::bvc_relative,    // 50
        Self::eor_indirect_y,  // 51
        Self::unimplemented,   // 52
        Self::unimplemented,   // 53
        Self::unimplemented,   // 54
        Self::eor_zero_page_x, // 55
        Self::lsr_zero_page_x, // 56
        Self::unimplemented,   // 57
        Self::cli_implied,     // 58
        Self::eor_absolute_y,  // 59
        Self::unimplemented,   // 5A
        Self::unimplemented,   // 5B
        Self::unimplemented,   // 5C
        Self::eor_absolute_x,  // 5D
        Self::lsr_absolute_x,  // 5E
        Self::unimplemented,   // 5F
        Self::rts_implied,     // 60
        Self::adc_indirect_x,  // 61
        Self::unimplemented,   // 62
        Self::unimplemented,   // 63
        Self::unimplemented,   // 64
        Self::adc_zero_page,   // 65
        Self::ror_zero_page,   // 66
        Self::unimplemented,   // 67
        Self::pla_implied,     // 68
        Self::adc_immediate,   // 69
        Self::ror_accumulator, // 6A
        Self::unimplemented,   // 6B
        Self::jmp_indirect,    // 6C
        Self::adc_absolute,    // 6D
        Self::ror_absolute,    // 6E
        Self::unimplemented,   // 6F
        Self::bvs_relative,    // 70
        Self::adc_indirect_y,  // 71
        Self::unimplemented,   // 72
        Self::unimplemented,   // 73
        Self::unimplemented,   // 74
        Self::adc_zero_page_x, // 75
        Self::ror_zero_page_x, // 76
        Self::unimplemented,   // 77
        Self::sei_implied,     // 78
        Self::adc_absolute_y,  // 79
        Self::unimplemented,   // 7A
        Self::unimplemented,   // 7B
        Self::unimplemented,   // 7C
        Self::adc_absolute_x,  // 7D
        Self::ror_absolute_x,  // 7E
        Self::unimplemented,   // 7F
        Self::unimplemented,   // 80
        Self::sta_indirect_x,  // 81
        Self::unimplemented,   // 82
        Self::unimplemented,   // 83
        Self::sty_zero_page,   // 84
        Self::sta_zero_page,   // 85
        Self::stx_zero_page,   // 86
        Self::unimplemented,   // 87
        Self::dey_implied,     // 88
        Self::unimplemented,   // 89
        Self::txa_implied,     // 8A
        Self::unimplemented,   // 8B
        Self::sty_absolute,    // 8C
        Self::sta_absolute,    // 8D
        Self::stx_absolute,    // 8E
        Self::unimplemented,   // 8F
        Self::bcc_relative,    // 90
        Self::sta_indirect_y,  // 91
        Self::unimplemented,   // 92
        Self::unimplemented,   // 93
        Self::sty_zero_page_x, // 94
        Self::sta_zero_page_x, // 95
        Self::stx_zero_page_y, // 96
        Self::unimplemented,   // 97
        Self::tya_implied,     // 98
        Self::unimplemented,   // 99
        Self::txs_implied,     // 9A
        Self::unimplemented,   // 9B
        Self::unimplemented,   // 9C
        Self::sta_absolute_x,  // 9D
        Self::unimplemented,   // 9E
        Self::unimplemented,   // 9F
        Self::ldy_immediate,   // A0
        Self::lda_indirect_x,  // A1
        Self::ldx_immediate,   // A2
        Self::unimplemented,   // A3
        Self::ldy_zero_page,   // A4
        Self::lda_zero_page,   // A5
        Self::ldx_zero_page,   // A6
        Self::unimplemented,   // A7
        Self::tay_implied,     // A8
        Self::lda_immediate,   // A9
        Self::tax_implied,     // AA
        Self::unimplemented,   // AB
        Self::ldy_absolute,    // AC
        Self::lda_absolute,    // AD
        Self::ldx_absolute,    // AE
        Self::unimplemented,   // AF
        Self::bcs_relative,    // B0
        Self::lda_indirect_y,  // B1
        Self::unimplemented,   // B2
        Self::unimplemented,   // B3
        Self::ldy_zero_page_x, // B4
        Self::lda_zero_page_x, // B5
        Self::ldx_zero_page_y, // B6
        Self::unimplemented,   // B7
        Self::clv_implied,     // B8
        Self::lda_absolute_y,  // B9
        Self::tsx_implied,     // BA
        Self::unimplemented,   // BB
        Self::ldy_absolute_x,  // BC
        Self::lda_absolute_x,  // BD
        Self::ldx_absolute_y,  // BE
        Self::unimplemented,   // BF
        Self::cpy_immediate,   // C0
        Self::cmp_indirect_x,  // C1
        Self::unimplemented,   // C2
        Self::unimplemented,   // C3
        Self::cpy_zero_page,   // C4
        Self::cmp_zero_page,   // C5
        Self::dec_zero_page,   // C6
        Self::unimplemented,   // C7
        Self::iny_implied,     // C8
        Self::cmp_immediate,   // C9
        Self::dex_implied,     // CA
        Self::unimplemented,   // CB
        Self::cpy_absolute,    // CC
        Self::cmp_absolute,    // CD
        Self::dec_absolute,    // CE
        Self::unimplemented,   // CF
        Self::bne_relative,    // D0
        Self::cmp_indirect_y,  // D1
        Self::unimplemented,   // D2
        Self::unimplemented,   // D3
        Self::unimplemented,   // D4
        Self::cmp_zero_page_x, // D5
        Self::dec_zero_page_x, // D6
        Self::unimplemented,   // D7
        Self::cld_implied,     // D8
        Self::cmp_absolute_y,  // D9
        Self::unimplemented,   // DA
        Self::unimplemented,   // DB
        Self::unimplemented,   // DC
        Self::cmp_absolute_x,  // DD
        Self::dec_absolute_x,  // DE
        Self::unimplemented,   // DF
        Self::cpx_immediate,   // E0
        Self::sbc_indirect_x,  // E1
        Self::unimplemented,   // E2
        Self::unimplemented,   // E3
        Self::cpx_zero_page,   // E4
        Self::sbc_zero_page,   // E5
        Self::inc_zero_page,   // E6
        Self::unimplemented,   // E7
        Self::inx_implied,     // E8
        Self::sbc_immediate,   // E9
        Self::nop_implied,     // EA
        Self::unimplemented,   // EB
        Self::cpx_absolute,    // EC
        Self::sbc_absolute,    // ED
        Self::inc_absolute,    // EE
        Self::unimplemented,   // EF
        Self::beq_relative,    // F0
        Self::sbc_indirect_y,  // F1
        Self::unimplemented,   // F2
        Self::unimplemented,   // F3
        Self::unimplemented,   // F4
        Self::sbc_zero_page_x, // F5
        Self::inc_zero_page_x, // F6
        Self::unimplemented,   // F7
        Self::sed_implied,     // F8
        Self::sbc_absolute_y,  // F9
        Self::unimplemented,   // FA
        Self::unimplemented,   // FB
        Self::unimplemented,   // FC
        Self::sbc_absolute_x,  // FD
        Self::inc_absolute_x,  // FE
        Self::unimplemented,   // FF
    ];
}

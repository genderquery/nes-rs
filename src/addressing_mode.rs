#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Implied,
    IndirectAbsolute,
    IndirectZeroPageX,
    IndirectZeroPageY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Unimplemented,
}

impl AddressingMode {
    /// The number of bytes an instruction with this addressing mode uses.
    ///
    /// Always at least one byte for the opcode and up to two more bytes for the
    /// operand, if any.
    pub fn len(&self) -> usize {
        match self {
            AddressingMode::Absolute => 3,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::Accumulator => 1,
            AddressingMode::Immediate => 2,
            AddressingMode::Implied => 1,
            AddressingMode::IndirectAbsolute => 3,
            AddressingMode::IndirectZeroPageX => 2,
            AddressingMode::IndirectZeroPageY => 2,
            AddressingMode::Relative => 2,
            AddressingMode::ZeroPage => 2,
            AddressingMode::ZeroPageX => 2,
            AddressingMode::ZeroPageY => 2,
            AddressingMode::Unimplemented => unimplemented!(),
        }
    }

    pub fn for_opcode(opcode: u8) -> AddressingMode {
        ADDRESSING_MODES[opcode as usize]
    }
}

const ADDRESSING_MODES: [AddressingMode; 256] = [
    // 00 BRK
    AddressingMode::Implied,
    // 01 ORA
    AddressingMode::IndirectZeroPageX,
    // 02 UNI
    AddressingMode::Unimplemented,
    // 03 UNI
    AddressingMode::Unimplemented,
    // 04 UNI
    AddressingMode::Unimplemented,
    // 05 ORA
    AddressingMode::ZeroPage,
    // 06 ASL
    AddressingMode::ZeroPage,
    // 07 UNI
    AddressingMode::Unimplemented,
    // 08 PHP
    AddressingMode::Implied,
    // 09 ORA
    AddressingMode::Immediate,
    // 0A ASL
    AddressingMode::Accumulator,
    // 0B UNI
    AddressingMode::Unimplemented,
    // 0C UNI
    AddressingMode::Unimplemented,
    // 0D ORA
    AddressingMode::Absolute,
    // 0E ASL
    AddressingMode::Absolute,
    // 0F UNI
    AddressingMode::Unimplemented,
    // 10 BPL
    AddressingMode::Relative,
    // 11 ORA
    AddressingMode::IndirectZeroPageY,
    // 12 UNI
    AddressingMode::Unimplemented,
    // 13 UNI
    AddressingMode::Unimplemented,
    // 14 UNI
    AddressingMode::Unimplemented,
    // 15 ORA
    AddressingMode::ZeroPageX,
    // 16 ASL
    AddressingMode::ZeroPageX,
    // 17 UNI
    AddressingMode::Unimplemented,
    // 18 CLC
    AddressingMode::Implied,
    // 19 ORA
    AddressingMode::AbsoluteY,
    // 1A UNI
    AddressingMode::Unimplemented,
    // 1B UNI
    AddressingMode::Unimplemented,
    // 1C UNI
    AddressingMode::Unimplemented,
    // 1D ORA
    AddressingMode::AbsoluteX,
    // 1E ASL
    AddressingMode::AbsoluteX,
    // 1F UNI
    AddressingMode::Unimplemented,
    // 20 JSR
    AddressingMode::Absolute,
    // 21 AND
    AddressingMode::IndirectZeroPageX,
    // 22 UNI
    AddressingMode::Unimplemented,
    // 23 UNI
    AddressingMode::Unimplemented,
    // 24 BIT
    AddressingMode::ZeroPage,
    // 25 AND
    AddressingMode::ZeroPage,
    // 26 ROL
    AddressingMode::ZeroPage,
    // 27 UNI
    AddressingMode::Unimplemented,
    // 28 PLP
    AddressingMode::Implied,
    // 29 AND
    AddressingMode::Immediate,
    // 2A ROL
    AddressingMode::Accumulator,
    // 2B UNI
    AddressingMode::Unimplemented,
    // 2C BIT
    AddressingMode::Absolute,
    // 2D AND
    AddressingMode::Absolute,
    // 2E ROL
    AddressingMode::Absolute,
    // 2F UNI
    AddressingMode::Unimplemented,
    // 30 BMI
    AddressingMode::Relative,
    // 31 AND
    AddressingMode::IndirectZeroPageY,
    // 32 UNI
    AddressingMode::Unimplemented,
    // 33 UNI
    AddressingMode::Unimplemented,
    // 34 UNI
    AddressingMode::Unimplemented,
    // 35 AND
    AddressingMode::ZeroPageX,
    // 36 ROL
    AddressingMode::ZeroPageX,
    // 37 UNI
    AddressingMode::Unimplemented,
    // 38 SEC
    AddressingMode::Implied,
    // 39 AND
    AddressingMode::AbsoluteY,
    // 3A UNI
    AddressingMode::Unimplemented,
    // 3B UNI
    AddressingMode::Unimplemented,
    // 3C UNI
    AddressingMode::Unimplemented,
    // 3D AND
    AddressingMode::AbsoluteX,
    // 3E ROL
    AddressingMode::AbsoluteX,
    // 3F UNI
    AddressingMode::Unimplemented,
    // 40 RTI
    AddressingMode::Implied,
    // 41 EOR
    AddressingMode::IndirectZeroPageX,
    // 42 UNI
    AddressingMode::Unimplemented,
    // 43 UNI
    AddressingMode::Unimplemented,
    // 44 UNI
    AddressingMode::Unimplemented,
    // 45 EOR
    AddressingMode::ZeroPage,
    // 46 LSR
    AddressingMode::ZeroPage,
    // 47 UNI
    AddressingMode::Unimplemented,
    // 48 PHA
    AddressingMode::Implied,
    // 49 EOR
    AddressingMode::Immediate,
    // 4A LSR
    AddressingMode::Accumulator,
    // 4B UNI
    AddressingMode::Unimplemented,
    // 4C JMP
    AddressingMode::Absolute,
    // 4D EOR
    AddressingMode::Absolute,
    // 4E LSR
    AddressingMode::Absolute,
    // 4F UNI
    AddressingMode::Unimplemented,
    // 50 BVC
    AddressingMode::Relative,
    // 51 EOR
    AddressingMode::IndirectZeroPageY,
    // 52 UNI
    AddressingMode::Unimplemented,
    // 53 UNI
    AddressingMode::Unimplemented,
    // 54 UNI
    AddressingMode::Unimplemented,
    // 55 EOR
    AddressingMode::ZeroPageX,
    // 56 LSR
    AddressingMode::ZeroPageX,
    // 57 UNI
    AddressingMode::Unimplemented,
    // 58 CLI
    AddressingMode::Implied,
    // 59 EOR
    AddressingMode::AbsoluteY,
    // 5A UNI
    AddressingMode::Unimplemented,
    // 5B UNI
    AddressingMode::Unimplemented,
    // 5C UNI
    AddressingMode::Unimplemented,
    // 5D EOR
    AddressingMode::AbsoluteX,
    // 5E LSR
    AddressingMode::AbsoluteX,
    // 5F UNI
    AddressingMode::Unimplemented,
    // 60 RTS
    AddressingMode::Implied,
    // 61 ADC
    AddressingMode::IndirectZeroPageX,
    // 62 UNI
    AddressingMode::Unimplemented,
    // 63 UNI
    AddressingMode::Unimplemented,
    // 64 UNI
    AddressingMode::Unimplemented,
    // 65 ADC
    AddressingMode::ZeroPage,
    // 66 ROR
    AddressingMode::ZeroPage,
    // 67 UNI
    AddressingMode::Unimplemented,
    // 68 PLA
    AddressingMode::Implied,
    // 69 ADC
    AddressingMode::Immediate,
    // 6A ROR
    AddressingMode::Accumulator,
    // 6B UNI
    AddressingMode::Unimplemented,
    // 6C JMP
    AddressingMode::IndirectAbsolute,
    // 6D ADC
    AddressingMode::Absolute,
    // 6E ROR
    AddressingMode::Absolute,
    // 6F UNI
    AddressingMode::Unimplemented,
    // 70 BVS
    AddressingMode::Relative,
    // 71 ADC
    AddressingMode::IndirectZeroPageY,
    // 72 UNI
    AddressingMode::Unimplemented,
    // 73 UNI
    AddressingMode::Unimplemented,
    // 74 UNI
    AddressingMode::Unimplemented,
    // 75 ADC
    AddressingMode::ZeroPageX,
    // 76 ROR
    AddressingMode::ZeroPageX,
    // 77 UNI
    AddressingMode::Unimplemented,
    // 78 SEI
    AddressingMode::Implied,
    // 79 ADC
    AddressingMode::AbsoluteY,
    // 7A UNI
    AddressingMode::Unimplemented,
    // 7B UNI
    AddressingMode::Unimplemented,
    // 7C UNI
    AddressingMode::Unimplemented,
    // 7D ADC
    AddressingMode::AbsoluteX,
    // 7E ROR
    AddressingMode::AbsoluteX,
    // 7F UNI
    AddressingMode::Unimplemented,
    // 80 UNI
    AddressingMode::Unimplemented,
    // 81 STA
    AddressingMode::IndirectZeroPageX,
    // 82 UNI
    AddressingMode::Unimplemented,
    // 83 UNI
    AddressingMode::Unimplemented,
    // 84 STY
    AddressingMode::ZeroPage,
    // 85 STA
    AddressingMode::ZeroPage,
    // 86 STX
    AddressingMode::ZeroPage,
    // 87 UNI
    AddressingMode::Unimplemented,
    // 88 DEY
    AddressingMode::Implied,
    // 89 UNI
    AddressingMode::Unimplemented,
    // 8A TXA
    AddressingMode::Implied,
    // 8B UNI
    AddressingMode::Unimplemented,
    // 8C STY
    AddressingMode::Absolute,
    // 8D STA
    AddressingMode::Absolute,
    // 8E STX
    AddressingMode::Absolute,
    // 8F UNI
    AddressingMode::Unimplemented,
    // 90 BCC
    AddressingMode::Relative,
    // 91 STA
    AddressingMode::IndirectZeroPageY,
    // 92 UNI
    AddressingMode::Unimplemented,
    // 93 UNI
    AddressingMode::Unimplemented,
    // 94 STY
    AddressingMode::ZeroPageX,
    // 95 STA
    AddressingMode::ZeroPageX,
    // 96 STX
    AddressingMode::ZeroPageY,
    // 97 UNI
    AddressingMode::Unimplemented,
    // 98 TYA
    AddressingMode::Implied,
    // 99 UNI
    AddressingMode::Unimplemented,
    // 9A TXS
    AddressingMode::Implied,
    // 9B UNI
    AddressingMode::Unimplemented,
    // 9C UNI
    AddressingMode::Unimplemented,
    // 9D STA
    AddressingMode::AbsoluteX,
    // 9E UNI
    AddressingMode::Unimplemented,
    // 9F UNI
    AddressingMode::Unimplemented,
    // A0 LDY
    AddressingMode::Immediate,
    // A1 LDA
    AddressingMode::IndirectZeroPageX,
    // A2 LDX
    AddressingMode::Immediate,
    // A3 UNI
    AddressingMode::Unimplemented,
    // A4 LDY
    AddressingMode::ZeroPage,
    // A5 LDA
    AddressingMode::ZeroPage,
    // A6 LDX
    AddressingMode::ZeroPage,
    // A7 UNI
    AddressingMode::Unimplemented,
    // A8 TAY
    AddressingMode::Implied,
    // A9 LDA
    AddressingMode::Immediate,
    // AA TAX
    AddressingMode::Implied,
    // AB UNI
    AddressingMode::Unimplemented,
    // AC LDY
    AddressingMode::Absolute,
    // AD LDA
    AddressingMode::Absolute,
    // AE LDX
    AddressingMode::Absolute,
    // AF UNI
    AddressingMode::Unimplemented,
    // B0 BCS
    AddressingMode::Relative,
    // B1 LDA
    AddressingMode::IndirectZeroPageY,
    // B2 UNI
    AddressingMode::Unimplemented,
    // B3 UNI
    AddressingMode::Unimplemented,
    // B4 LDY
    AddressingMode::ZeroPageX,
    // B5 LDA
    AddressingMode::ZeroPageX,
    // B6 LDX
    AddressingMode::ZeroPageY,
    // B7 UNI
    AddressingMode::Unimplemented,
    // B8 CLV
    AddressingMode::Implied,
    // B9 LDA
    AddressingMode::AbsoluteY,
    // BA TSX
    AddressingMode::Implied,
    // BB UNI
    AddressingMode::Unimplemented,
    // BC LDY
    AddressingMode::AbsoluteX,
    // BD LDA
    AddressingMode::AbsoluteX,
    // BE LDX
    AddressingMode::AbsoluteY,
    // BF UNI
    AddressingMode::Unimplemented,
    // C0 CPY
    AddressingMode::Immediate,
    // C1 CMP
    AddressingMode::IndirectZeroPageX,
    // C2 UNI
    AddressingMode::Unimplemented,
    // C3 UNI
    AddressingMode::Unimplemented,
    // C4 CPY
    AddressingMode::ZeroPage,
    // C5 CMP
    AddressingMode::ZeroPage,
    // C6 DEC
    AddressingMode::ZeroPage,
    // C7 UNI
    AddressingMode::Unimplemented,
    // C8 INY
    AddressingMode::Implied,
    // C9 CMP
    AddressingMode::Immediate,
    // CA DEX
    AddressingMode::Implied,
    // CB UNI
    AddressingMode::Unimplemented,
    // CC CPY
    AddressingMode::Absolute,
    // CD CMP
    AddressingMode::Absolute,
    // CE DEC
    AddressingMode::Absolute,
    // CF UNI
    AddressingMode::Unimplemented,
    // D0 BNE
    AddressingMode::Relative,
    // D1 CMP
    AddressingMode::IndirectZeroPageY,
    // D2 UNI
    AddressingMode::Unimplemented,
    // D3 UNI
    AddressingMode::Unimplemented,
    // D4 UNI
    AddressingMode::Unimplemented,
    // D5 CMP
    AddressingMode::ZeroPageX,
    // D6 DEC
    AddressingMode::ZeroPageX,
    // D7 UNI
    AddressingMode::Unimplemented,
    // D8 CLD
    AddressingMode::Implied,
    // D9 CMP
    AddressingMode::AbsoluteY,
    // DA UNI
    AddressingMode::Unimplemented,
    // DB UNI
    AddressingMode::Unimplemented,
    // DC UNI
    AddressingMode::Unimplemented,
    // DD CMP
    AddressingMode::AbsoluteX,
    // DE DEC
    AddressingMode::AbsoluteX,
    // DF UNI
    AddressingMode::Unimplemented,
    // E0 CPX
    AddressingMode::Immediate,
    // E1 SBC
    AddressingMode::IndirectZeroPageX,
    // E2 UNI
    AddressingMode::Unimplemented,
    // E3 UNI
    AddressingMode::Unimplemented,
    // E4 CPX
    AddressingMode::ZeroPage,
    // E5 SBC
    AddressingMode::ZeroPage,
    // E6 INC
    AddressingMode::ZeroPage,
    // E7 UNI
    AddressingMode::Unimplemented,
    // E8 INX
    AddressingMode::Implied,
    // E9 SBC
    AddressingMode::Immediate,
    // EA NOP
    AddressingMode::Implied,
    // EB UNI
    AddressingMode::Unimplemented,
    // EC CPX
    AddressingMode::Absolute,
    // ED SBC
    AddressingMode::Absolute,
    // EE INC
    AddressingMode::Absolute,
    // EF UNI
    AddressingMode::Unimplemented,
    // F0 BEQ
    AddressingMode::Relative,
    // F1 SBC
    AddressingMode::IndirectZeroPageY,
    // F2 UNI
    AddressingMode::Unimplemented,
    // F3 UNI
    AddressingMode::Unimplemented,
    // F4 UNI
    AddressingMode::Unimplemented,
    // F5 SBC
    AddressingMode::ZeroPageX,
    // F6 INC
    AddressingMode::ZeroPageX,
    // F7 UNI
    AddressingMode::Unimplemented,
    // F8 SED
    AddressingMode::Implied,
    // F9 SBC
    AddressingMode::AbsoluteY,
    // FA UNI
    AddressingMode::Unimplemented,
    // FB UNI
    AddressingMode::Unimplemented,
    // FC UNI
    AddressingMode::Unimplemented,
    // FD SBC
    AddressingMode::AbsoluteX,
    // FE INC
    AddressingMode::AbsoluteX,
    // FF UNI
    AddressingMode::Unimplemented,
];

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Instruction {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
    Unimplemented,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Instruction {
    pub fn for_opcode(opcode: u8) -> Instruction {
        INSTRUCTIONS[opcode as usize]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Instruction::Adc => "ADC",
            Instruction::And => "AND",
            Instruction::Asl => "ASL",
            Instruction::Bcc => "BCC",
            Instruction::Bcs => "BCS",
            Instruction::Beq => "BEQ",
            Instruction::Bit => "BIT",
            Instruction::Bmi => "BMI",
            Instruction::Bne => "BNE",
            Instruction::Bpl => "BPL",
            Instruction::Brk => "BRK",
            Instruction::Bvc => "BVC",
            Instruction::Bvs => "BVS",
            Instruction::Clc => "CLC",
            Instruction::Cld => "CLD",
            Instruction::Cli => "CLI",
            Instruction::Clv => "CLV",
            Instruction::Cmp => "CMP",
            Instruction::Cpx => "CPX",
            Instruction::Cpy => "CPY",
            Instruction::Dec => "DEC",
            Instruction::Dex => "DEX",
            Instruction::Dey => "DEY",
            Instruction::Eor => "EOR",
            Instruction::Inc => "INC",
            Instruction::Inx => "INX",
            Instruction::Iny => "INY",
            Instruction::Jmp => "JMP",
            Instruction::Jsr => "JSR",
            Instruction::Lda => "LDA",
            Instruction::Ldx => "LDX",
            Instruction::Ldy => "LDY",
            Instruction::Lsr => "LSR",
            Instruction::Nop => "NOP",
            Instruction::Ora => "ORA",
            Instruction::Pha => "PHA",
            Instruction::Php => "PHP",
            Instruction::Pla => "PLA",
            Instruction::Plp => "PLP",
            Instruction::Rol => "ROL",
            Instruction::Ror => "ROR",
            Instruction::Rti => "RTI",
            Instruction::Rts => "RTS",
            Instruction::Sbc => "SBC",
            Instruction::Sec => "SEC",
            Instruction::Sed => "SED",
            Instruction::Sei => "SEI",
            Instruction::Sta => "STA",
            Instruction::Stx => "STX",
            Instruction::Sty => "STY",
            Instruction::Tax => "TAX",
            Instruction::Tay => "TAY",
            Instruction::Tsx => "TSX",
            Instruction::Txa => "TXA",
            Instruction::Txs => "TXS",
            Instruction::Tya => "TYA",
            Instruction::Unimplemented => unimplemented!(),
        }
    }
}

const INSTRUCTIONS: [Instruction; 256] = [
    // 00 BRK Implied
    Instruction::Brk,
    // 01 ORA IndirectX
    Instruction::Ora,
    // 02
    Instruction::Unimplemented,
    // 03
    Instruction::Unimplemented,
    // 04
    Instruction::Unimplemented,
    // 05 ORA ZeroPage
    Instruction::Ora,
    // 06 ASL ZeroPage
    Instruction::Asl,
    // 07
    Instruction::Unimplemented,
    // 08 PHP Implied
    Instruction::Php,
    // 09 ORA Immediate
    Instruction::Ora,
    // 0A ASL Accumulator
    Instruction::Asl,
    // 0B
    Instruction::Unimplemented,
    // 0C
    Instruction::Unimplemented,
    // 0D ORA Absolute
    Instruction::Ora,
    // 0E ASL Absolute
    Instruction::Asl,
    // 0F
    Instruction::Unimplemented,
    // 10 BPL Relative
    Instruction::Bpl,
    // 11 ORA IndirectY
    Instruction::Ora,
    // 12
    Instruction::Unimplemented,
    // 13
    Instruction::Unimplemented,
    // 14
    Instruction::Unimplemented,
    // 15 ORA ZeroPageX
    Instruction::Ora,
    // 16 ASL ZeroPageX
    Instruction::Asl,
    // 17
    Instruction::Unimplemented,
    // 18 CLC Implied
    Instruction::Clc,
    // 19 ORA AbsoluteY
    Instruction::Ora,
    // 1A
    Instruction::Unimplemented,
    // 1B
    Instruction::Unimplemented,
    // 1C
    Instruction::Unimplemented,
    // 1D ORA AbsoluteX
    Instruction::Ora,
    // 1E ASL AbsoluteX
    Instruction::Asl,
    // 1F
    Instruction::Unimplemented,
    // 20 JSR Absolute
    Instruction::Jsr,
    // 21 AND IndirectX
    Instruction::And,
    // 22
    Instruction::Unimplemented,
    // 23
    Instruction::Unimplemented,
    // 24 BIT ZeroPage
    Instruction::Bit,
    // 25 AND ZeroPage
    Instruction::And,
    // 26 ROL ZeroPage
    Instruction::Rol,
    // 27
    Instruction::Unimplemented,
    // 28 PLP Implied
    Instruction::Plp,
    // 29 AND Immediate
    Instruction::And,
    // 2A ROL Accumulator
    Instruction::Rol,
    // 2B
    Instruction::Unimplemented,
    // 2C BIT Absolute
    Instruction::Bit,
    // 2D AND Absolute
    Instruction::And,
    // 2E ROL Absolute
    Instruction::Rol,
    // 2F
    Instruction::Unimplemented,
    // 30 BMI Relative
    Instruction::Bmi,
    // 31 AND IndirectY
    Instruction::And,
    // 32
    Instruction::Unimplemented,
    // 33
    Instruction::Unimplemented,
    // 34
    Instruction::Unimplemented,
    // 35 AND ZeroPageX
    Instruction::And,
    // 36 ROL ZeroPageX
    Instruction::Rol,
    // 37
    Instruction::Unimplemented,
    // 38 SEC Implied
    Instruction::Sec,
    // 39 AND AbsoluteY
    Instruction::And,
    // 3A
    Instruction::Unimplemented,
    // 3B
    Instruction::Unimplemented,
    // 3C
    Instruction::Unimplemented,
    // 3D AND AbsoluteX
    Instruction::And,
    // 3E ROL AbsoluteX
    Instruction::Rol,
    // 3F
    Instruction::Unimplemented,
    // 40 RTI Implied
    Instruction::Rti,
    // 41 EOR IndirectX
    Instruction::Eor,
    // 42
    Instruction::Unimplemented,
    // 43
    Instruction::Unimplemented,
    // 44
    Instruction::Unimplemented,
    // 45 EOR ZeroPage
    Instruction::Eor,
    // 46 LSR ZeroPage
    Instruction::Lsr,
    // 47
    Instruction::Unimplemented,
    // 48 PHA Implied
    Instruction::Pha,
    // 49 EOR Immediate
    Instruction::Eor,
    // 4A LSR Accumulator
    Instruction::Lsr,
    // 4B
    Instruction::Unimplemented,
    // 4C JMP Absolute
    Instruction::Jmp,
    // 4D EOR Absolute
    Instruction::Eor,
    // 4E LSR Absolute
    Instruction::Lsr,
    // 4F
    Instruction::Unimplemented,
    // 50 BVC Relative
    Instruction::Bvc,
    // 51 EOR IndirectY
    Instruction::Eor,
    // 52
    Instruction::Unimplemented,
    // 53
    Instruction::Unimplemented,
    // 54
    Instruction::Unimplemented,
    // 55 EOR ZeroPageX
    Instruction::Eor,
    // 56 LSR ZeroPageX
    Instruction::Lsr,
    // 57
    Instruction::Unimplemented,
    // 58 CLI Implied
    Instruction::Cli,
    // 59 EOR AbsoluteY
    Instruction::Eor,
    // 5A
    Instruction::Unimplemented,
    // 5B
    Instruction::Unimplemented,
    // 5C
    Instruction::Unimplemented,
    // 5D EOR AbsoluteX
    Instruction::Eor,
    // 5E LSR AbsoluteX
    Instruction::Lsr,
    // 5F
    Instruction::Unimplemented,
    // 60 RTS Implied
    Instruction::Rts,
    // 61 ADC IndirectX
    Instruction::Adc,
    // 62
    Instruction::Unimplemented,
    // 63
    Instruction::Unimplemented,
    // 64
    Instruction::Unimplemented,
    // 65 ADC ZeroPage
    Instruction::Adc,
    // 66 ROR ZeroPage
    Instruction::Ror,
    // 67
    Instruction::Unimplemented,
    // 68 PLA Implied
    Instruction::Pla,
    // 69 ADC Immediate
    Instruction::Adc,
    // 6A ROR Accumulator
    Instruction::Ror,
    // 6B
    Instruction::Unimplemented,
    // 6C JMP IndirectAbsolute
    Instruction::Jmp,
    // 6D ADC Absolute
    Instruction::Adc,
    // 6E ROR Absolute
    Instruction::Ror,
    // 6F
    Instruction::Unimplemented,
    // 70 BVS Relative
    Instruction::Bvs,
    // 71 ADC IndirectY
    Instruction::Adc,
    // 72
    Instruction::Unimplemented,
    // 73
    Instruction::Unimplemented,
    // 74
    Instruction::Unimplemented,
    // 75 ADC ZeroPageX
    Instruction::Adc,
    // 76 ROR ZeroPageX
    Instruction::Ror,
    // 77
    Instruction::Unimplemented,
    // 78 SEI Implied
    Instruction::Sei,
    // 79 ADC AbsoluteY
    Instruction::Adc,
    // 7A
    Instruction::Unimplemented,
    // 7B
    Instruction::Unimplemented,
    // 7C
    Instruction::Unimplemented,
    // 7D ADC AbsoluteX
    Instruction::Adc,
    // 7E ROR AbsoluteX
    Instruction::Ror,
    // 7F
    Instruction::Unimplemented,
    // 80
    Instruction::Unimplemented,
    // 81 STA IndirectX
    Instruction::Sta,
    // 82
    Instruction::Unimplemented,
    // 83
    Instruction::Unimplemented,
    // 84 STY ZeroPage
    Instruction::Sty,
    // 85 STA ZeroPage
    Instruction::Sta,
    // 86 STX ZeroPage
    Instruction::Stx,
    // 87
    Instruction::Unimplemented,
    // 88 DEY Implied
    Instruction::Dey,
    // 89
    Instruction::Unimplemented,
    // 8A TXA Implied
    Instruction::Txa,
    // 8B
    Instruction::Unimplemented,
    // 8C STY Absolute
    Instruction::Sty,
    // 8D STA Absolute
    Instruction::Sta,
    // 8E STX Absolute
    Instruction::Stx,
    // 8F
    Instruction::Unimplemented,
    // 90 BCC Relative
    Instruction::Bcc,
    // 91 STA IndirectY
    Instruction::Sta,
    // 92
    Instruction::Unimplemented,
    // 93
    Instruction::Unimplemented,
    // 94 STY ZeroPageX
    Instruction::Sty,
    // 95 STA ZeroPageX
    Instruction::Sta,
    // 96 STX ZeroPageY
    Instruction::Stx,
    // 97
    Instruction::Unimplemented,
    // 98 TYA Implied
    Instruction::Tya,
    // 99
    Instruction::Unimplemented,
    // 9A TXS Implied
    Instruction::Txs,
    // 9B
    Instruction::Unimplemented,
    // 9C
    Instruction::Unimplemented,
    // 9D STA AbsoluteX
    Instruction::Sta,
    // 9E
    Instruction::Unimplemented,
    // 9F
    Instruction::Unimplemented,
    // A0 LDY Immediate
    Instruction::Ldy,
    // A1 LDA IndirectX
    Instruction::Lda,
    // A2 LDX Immediate
    Instruction::Ldx,
    // A3
    Instruction::Unimplemented,
    // A4 LDY ZeroPage
    Instruction::Ldy,
    // A5 LDA ZeroPage
    Instruction::Lda,
    // A6 LDX ZeroPage
    Instruction::Ldx,
    // A7
    Instruction::Unimplemented,
    // A8 TAY Implied
    Instruction::Tay,
    // A9 LDA Immediate
    Instruction::Lda,
    // AA TAX Implied
    Instruction::Tax,
    // AB
    Instruction::Unimplemented,
    // AC LDY Absolute
    Instruction::Ldy,
    // AD LDA Absolute
    Instruction::Lda,
    // AE LDX Absolute
    Instruction::Ldx,
    // AF
    Instruction::Unimplemented,
    // B0 BCS Relative
    Instruction::Bcs,
    // B1 LDA IndirectY
    Instruction::Lda,
    // B2
    Instruction::Unimplemented,
    // B3
    Instruction::Unimplemented,
    // B4 LDY ZeroPageX
    Instruction::Ldy,
    // B5 LDA ZeroPageX
    Instruction::Lda,
    // B6 LDX ZeroPageY
    Instruction::Ldx,
    // B7
    Instruction::Unimplemented,
    // B8 CLV Implied
    Instruction::Clv,
    // B9 LDA AbsoluteY
    Instruction::Lda,
    // BA TSX Implied
    Instruction::Tsx,
    // BB
    Instruction::Unimplemented,
    // BC LDY AbsoluteX
    Instruction::Ldy,
    // BD LDA AbsoluteX
    Instruction::Lda,
    // BE LDX AbsoluteY
    Instruction::Ldx,
    // BF
    Instruction::Unimplemented,
    // C0 CPY Immediate
    Instruction::Cpy,
    // C1 CMP IndirectX
    Instruction::Cmp,
    // C2
    Instruction::Unimplemented,
    // C3
    Instruction::Unimplemented,
    // C4 CPY ZeroPage
    Instruction::Cpy,
    // C5 CMP ZeroPage
    Instruction::Cmp,
    // C6 DEC ZeroPage
    Instruction::Dec,
    // C7
    Instruction::Unimplemented,
    // C8 INY Implied
    Instruction::Iny,
    // C9 CMP Immediate
    Instruction::Cmp,
    // CA DEX Implied
    Instruction::Dex,
    // CB
    Instruction::Unimplemented,
    // CC CPY Absolute
    Instruction::Cpy,
    // CD CMP Absolute
    Instruction::Cmp,
    // CE DEC Absolute
    Instruction::Dec,
    // CF
    Instruction::Unimplemented,
    // D0 BNE Relative
    Instruction::Bne,
    // D1 CMP IndirectY
    Instruction::Cmp,
    // D2
    Instruction::Unimplemented,
    // D3
    Instruction::Unimplemented,
    // D4
    Instruction::Unimplemented,
    // D5 CMP ZeroPageX
    Instruction::Cmp,
    // D6 DEC ZeroPageX
    Instruction::Dec,
    // D7
    Instruction::Unimplemented,
    // D8 CLD Implied
    Instruction::Cld,
    // D9 CMP AbsoluteY
    Instruction::Cmp,
    // DA
    Instruction::Unimplemented,
    // DB
    Instruction::Unimplemented,
    // DC
    Instruction::Unimplemented,
    // DD CMP AbsoluteX
    Instruction::Cmp,
    // DE DEC AbsoluteX
    Instruction::Dec,
    // DF
    Instruction::Unimplemented,
    // E0 CPX Immediate
    Instruction::Cpx,
    // E1 SBC IndirectX
    Instruction::Sbc,
    // E2
    Instruction::Unimplemented,
    // E3
    Instruction::Unimplemented,
    // E4 CPX ZeroPage
    Instruction::Cpx,
    // E5 SBC ZeroPage
    Instruction::Sbc,
    // E6 INC ZeroPage
    Instruction::Inc,
    // E7
    Instruction::Unimplemented,
    // E8 INX Implied
    Instruction::Inx,
    // E9 SBC Immediate
    Instruction::Sbc,
    // EA NOP Implied
    Instruction::Nop,
    // EB
    Instruction::Unimplemented,
    // EC CPX Absolute
    Instruction::Cpx,
    // ED SBC Absolute
    Instruction::Sbc,
    // EE INC Absolute
    Instruction::Inc,
    // EF
    Instruction::Unimplemented,
    // F0 BEQ Relative
    Instruction::Beq,
    // F1 SBC IndirectY
    Instruction::Sbc,
    // F2
    Instruction::Unimplemented,
    // F3
    Instruction::Unimplemented,
    // F4
    Instruction::Unimplemented,
    // F5 SBC ZeroPageX
    Instruction::Sbc,
    // F6 INC ZeroPageX
    Instruction::Inc,
    // F7
    Instruction::Unimplemented,
    // F8 SED Implied
    Instruction::Sed,
    // F9 SBC AbsoluteY
    Instruction::Sbc,
    // FA
    Instruction::Unimplemented,
    // FB
    Instruction::Unimplemented,
    // FC
    Instruction::Unimplemented,
    // FD SBC AbsoluteX
    Instruction::Sbc,
    // FE INC AbsoluteX
    Instruction::Inc,
    // FF
    Instruction::Unimplemented,
];

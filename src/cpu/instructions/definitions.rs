// ====================================================
// ============== Instruction definitions =============
// ====================================================
//
// This part is very long.
// It was generated with help from the program
// found in /aux/lookup-to-rust.cpp
//
// Thanks to javidx9 (https://github.com/OneLoneCoder)
// for providing the lookup table!

#![allow(dead_code)]

use crate::cpu::instructions::Instruction;
use crate::cpu::Cpu;

pub const BRK_00: Instruction = Instruction {
    name: "BRK",
    opcode: 0x00,
    cycles: 7,
    addrmode: Cpu::imm,
    execute: Cpu::brk,
};

pub const ORA_01: Instruction = Instruction {
    name: "ORA",
    opcode: 0x01,
    cycles: 6,
    addrmode: Cpu::izx,
    execute: Cpu::ora,
};

pub const ORA_05: Instruction = Instruction {
    name: "ORA",
    opcode: 0x05,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::ora,
};

pub const ASL_06: Instruction = Instruction {
    name: "ASL",
    opcode: 0x06,
    cycles: 5,
    addrmode: Cpu::zp0,
    execute: Cpu::asl,
};

pub const PHP_08: Instruction = Instruction {
    name: "PHP",
    opcode: 0x08,
    cycles: 3,
    addrmode: Cpu::imp,
    execute: Cpu::php,
};

pub const ORA_09: Instruction = Instruction {
    name: "ORA",
    opcode: 0x09,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::ora,
};

pub const ASL_0A: Instruction = Instruction {
    name: "ASL",
    opcode: 0x0A,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::asl,
};

pub const ORA_0D: Instruction = Instruction {
    name: "ORA",
    opcode: 0x0D,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::ora,
};

pub const ASL_0E: Instruction = Instruction {
    name: "ASL",
    opcode: 0x0E,
    cycles: 6,
    addrmode: Cpu::abs,
    execute: Cpu::asl,
};

pub const BPL_10: Instruction = Instruction {
    name: "BPL",
    opcode: 0x10,
    cycles: 2,
    addrmode: Cpu::rel,
    execute: Cpu::bpl,
};

pub const ORA_11: Instruction = Instruction {
    name: "ORA",
    opcode: 0x11,
    cycles: 5,
    addrmode: Cpu::izy,
    execute: Cpu::ora,
};

pub const ORA_15: Instruction = Instruction {
    name: "ORA",
    opcode: 0x15,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::ora,
};

pub const ASL_16: Instruction = Instruction {
    name: "ASL",
    opcode: 0x16,
    cycles: 6,
    addrmode: Cpu::zpx,
    execute: Cpu::asl,
};

pub const CLC_18: Instruction = Instruction {
    name: "CLC",
    opcode: 0x18,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::clc,
};

pub const ORA_19: Instruction = Instruction {
    name: "ORA",
    opcode: 0x19,
    cycles: 4,
    addrmode: Cpu::aby,
    execute: Cpu::ora,
};

pub const ORA_1D: Instruction = Instruction {
    name: "ORA",
    opcode: 0x1D,
    cycles: 4,
    addrmode: Cpu::abx,
    execute: Cpu::ora,
};

pub const ASL_1E: Instruction = Instruction {
    name: "ASL",
    opcode: 0x1E,
    cycles: 7,
    addrmode: Cpu::abx,
    execute: Cpu::asl,
};

pub const JSR_20: Instruction = Instruction {
    name: "JSR",
    opcode: 0x20,
    cycles: 6,
    addrmode: Cpu::abs,
    execute: Cpu::jsr,
};

pub const AND_21: Instruction = Instruction {
    name: "AND",
    opcode: 0x21,
    cycles: 6,
    addrmode: Cpu::izx,
    execute: Cpu::and,
};

pub const BIT_24: Instruction = Instruction {
    name: "BIT",
    opcode: 0x24,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::bit,
};

pub const AND_25: Instruction = Instruction {
    name: "AND",
    opcode: 0x25,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::and,
};

pub const ROL_26: Instruction = Instruction {
    name: "ROL",
    opcode: 0x26,
    cycles: 5,
    addrmode: Cpu::zp0,
    execute: Cpu::rol,
};

pub const PLP_28: Instruction = Instruction {
    name: "PLP",
    opcode: 0x28,
    cycles: 4,
    addrmode: Cpu::imp,
    execute: Cpu::plp,
};

pub const AND_29: Instruction = Instruction {
    name: "AND",
    opcode: 0x29,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::and,
};

pub const ROL_2A: Instruction = Instruction {
    name: "ROL",
    opcode: 0x2A,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::rol,
};

pub const BIT_2C: Instruction = Instruction {
    name: "BIT",
    opcode: 0x2C,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::bit,
};

pub const AND_2D: Instruction = Instruction {
    name: "AND",
    opcode: 0x2D,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::and,
};

pub const ROL_2E: Instruction = Instruction {
    name: "ROL",
    opcode: 0x2E,
    cycles: 6,
    addrmode: Cpu::abs,
    execute: Cpu::rol,
};

pub const BMI_30: Instruction = Instruction {
    name: "BMI",
    opcode: 0x30,
    cycles: 2,
    addrmode: Cpu::rel,
    execute: Cpu::bmi,
};

pub const AND_31: Instruction = Instruction {
    name: "AND",
    opcode: 0x31,
    cycles: 5,
    addrmode: Cpu::izy,
    execute: Cpu::and,
};

pub const AND_35: Instruction = Instruction {
    name: "AND",
    opcode: 0x35,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::and,
};

pub const ROL_36: Instruction = Instruction {
    name: "ROL",
    opcode: 0x36,
    cycles: 6,
    addrmode: Cpu::zpx,
    execute: Cpu::rol,
};

pub const SEC_38: Instruction = Instruction {
    name: "SEC",
    opcode: 0x38,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::sec,
};

pub const AND_39: Instruction = Instruction {
    name: "AND",
    opcode: 0x39,
    cycles: 4,
    addrmode: Cpu::aby,
    execute: Cpu::and,
};

pub const AND_3D: Instruction = Instruction {
    name: "AND",
    opcode: 0x3D,
    cycles: 4,
    addrmode: Cpu::abx,
    execute: Cpu::and,
};

pub const ROL_3E: Instruction = Instruction {
    name: "ROL",
    opcode: 0x3E,
    cycles: 7,
    addrmode: Cpu::abx,
    execute: Cpu::rol,
};

pub const RTI_40: Instruction = Instruction {
    name: "RTI",
    opcode: 0x40,
    cycles: 6,
    addrmode: Cpu::imp,
    execute: Cpu::rti,
};

pub const EOR_41: Instruction = Instruction {
    name: "EOR",
    opcode: 0x41,
    cycles: 6,
    addrmode: Cpu::izx,
    execute: Cpu::eor,
};

pub const EOR_45: Instruction = Instruction {
    name: "EOR",
    opcode: 0x45,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::eor,
};

pub const LSR_46: Instruction = Instruction {
    name: "LSR",
    opcode: 0x46,
    cycles: 5,
    addrmode: Cpu::zp0,
    execute: Cpu::lsr,
};

pub const PHA_48: Instruction = Instruction {
    name: "PHA",
    opcode: 0x48,
    cycles: 3,
    addrmode: Cpu::imp,
    execute: Cpu::pha,
};

pub const EOR_49: Instruction = Instruction {
    name: "EOR",
    opcode: 0x49,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::eor,
};

pub const LSR_4A: Instruction = Instruction {
    name: "LSR",
    opcode: 0x4A,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::lsr,
};

pub const JMP_4C: Instruction = Instruction {
    name: "JMP",
    opcode: 0x4C,
    cycles: 3,
    addrmode: Cpu::abs,
    execute: Cpu::jmp,
};

pub const EOR_4D: Instruction = Instruction {
    name: "EOR",
    opcode: 0x4D,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::eor,
};

pub const LSR_4E: Instruction = Instruction {
    name: "LSR",
    opcode: 0x4E,
    cycles: 6,
    addrmode: Cpu::abs,
    execute: Cpu::lsr,
};

pub const BVC_50: Instruction = Instruction {
    name: "BVC",
    opcode: 0x50,
    cycles: 2,
    addrmode: Cpu::rel,
    execute: Cpu::bvc,
};

pub const EOR_51: Instruction = Instruction {
    name: "EOR",
    opcode: 0x51,
    cycles: 5,
    addrmode: Cpu::izy,
    execute: Cpu::eor,
};

pub const EOR_55: Instruction = Instruction {
    name: "EOR",
    opcode: 0x55,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::eor,
};

pub const LSR_56: Instruction = Instruction {
    name: "LSR",
    opcode: 0x56,
    cycles: 6,
    addrmode: Cpu::zpx,
    execute: Cpu::lsr,
};

pub const CLI_58: Instruction = Instruction {
    name: "CLI",
    opcode: 0x58,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::cli,
};

pub const EOR_59: Instruction = Instruction {
    name: "EOR",
    opcode: 0x59,
    cycles: 4,
    addrmode: Cpu::aby,
    execute: Cpu::eor,
};

pub const EOR_5D: Instruction = Instruction {
    name: "EOR",
    opcode: 0x5D,
    cycles: 4,
    addrmode: Cpu::abx,
    execute: Cpu::eor,
};

pub const LSR_5E: Instruction = Instruction {
    name: "LSR",
    opcode: 0x5E,
    cycles: 7,
    addrmode: Cpu::abx,
    execute: Cpu::lsr,
};

pub const RTS_60: Instruction = Instruction {
    name: "RTS",
    opcode: 0x60,
    cycles: 6,
    addrmode: Cpu::imp,
    execute: Cpu::rts,
};

pub const ADC_61: Instruction = Instruction {
    name: "ADC",
    opcode: 0x61,
    cycles: 6,
    addrmode: Cpu::izx,
    execute: Cpu::adc,
};

pub const ADC_65: Instruction = Instruction {
    name: "ADC",
    opcode: 0x65,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::adc,
};

pub const ROR_66: Instruction = Instruction {
    name: "ROR",
    opcode: 0x66,
    cycles: 5,
    addrmode: Cpu::zp0,
    execute: Cpu::ror,
};

pub const PLA_68: Instruction = Instruction {
    name: "PLA",
    opcode: 0x68,
    cycles: 4,
    addrmode: Cpu::imp,
    execute: Cpu::pla,
};

pub const ADC_69: Instruction = Instruction {
    name: "ADC",
    opcode: 0x69,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::adc,
};

pub const ROR_6A: Instruction = Instruction {
    name: "ROR",
    opcode: 0x6A,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::ror,
};

pub const JMP_6C: Instruction = Instruction {
    name: "JMP",
    opcode: 0x6C,
    cycles: 5,
    addrmode: Cpu::ind,
    execute: Cpu::jmp,
};

pub const ADC_6D: Instruction = Instruction {
    name: "ADC",
    opcode: 0x6D,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::adc,
};

pub const ROR_6E: Instruction = Instruction {
    name: "ROR",
    opcode: 0x6E,
    cycles: 6,
    addrmode: Cpu::abs,
    execute: Cpu::ror,
};

pub const BVS_70: Instruction = Instruction {
    name: "BVS",
    opcode: 0x70,
    cycles: 2,
    addrmode: Cpu::rel,
    execute: Cpu::bvs,
};

pub const ADC_71: Instruction = Instruction {
    name: "ADC",
    opcode: 0x71,
    cycles: 5,
    addrmode: Cpu::izy,
    execute: Cpu::adc,
};

pub const ADC_75: Instruction = Instruction {
    name: "ADC",
    opcode: 0x75,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::adc,
};

pub const ROR_76: Instruction = Instruction {
    name: "ROR",
    opcode: 0x76,
    cycles: 6,
    addrmode: Cpu::zpx,
    execute: Cpu::ror,
};

pub const SEI_78: Instruction = Instruction {
    name: "SEI",
    opcode: 0x78,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::sei,
};

pub const ADC_79: Instruction = Instruction {
    name: "ADC",
    opcode: 0x79,
    cycles: 4,
    addrmode: Cpu::aby,
    execute: Cpu::adc,
};

pub const ADC_7D: Instruction = Instruction {
    name: "ADC",
    opcode: 0x7D,
    cycles: 4,
    addrmode: Cpu::abx,
    execute: Cpu::adc,
};

pub const ROR_7E: Instruction = Instruction {
    name: "ROR",
    opcode: 0x7E,
    cycles: 7,
    addrmode: Cpu::abx,
    execute: Cpu::ror,
};

pub const STA_81: Instruction = Instruction {
    name: "STA",
    opcode: 0x81,
    cycles: 6,
    addrmode: Cpu::izx,
    execute: Cpu::sta,
};

pub const STY_84: Instruction = Instruction {
    name: "STY",
    opcode: 0x84,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::sty,
};

pub const STA_85: Instruction = Instruction {
    name: "STA",
    opcode: 0x85,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::sta,
};

pub const STX_86: Instruction = Instruction {
    name: "STX",
    opcode: 0x86,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::stx,
};

pub const DEY_88: Instruction = Instruction {
    name: "DEY",
    opcode: 0x88,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::dey,
};

pub const TXA_8A: Instruction = Instruction {
    name: "TXA",
    opcode: 0x8A,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::txa,
};

pub const STY_8C: Instruction = Instruction {
    name: "STY",
    opcode: 0x8C,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::sty,
};

pub const STA_8D: Instruction = Instruction {
    name: "STA",
    opcode: 0x8D,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::sta,
};

pub const STX_8E: Instruction = Instruction {
    name: "STX",
    opcode: 0x8E,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::stx,
};

pub const BCC_90: Instruction = Instruction {
    name: "BCC",
    opcode: 0x90,
    cycles: 2,
    addrmode: Cpu::rel,
    execute: Cpu::bcc,
};

pub const STA_91: Instruction = Instruction {
    name: "STA",
    opcode: 0x91,
    cycles: 6,
    addrmode: Cpu::izy,
    execute: Cpu::sta,
};

pub const STY_94: Instruction = Instruction {
    name: "STY",
    opcode: 0x94,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::sty,
};

pub const STA_95: Instruction = Instruction {
    name: "STA",
    opcode: 0x95,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::sta,
};

pub const STX_96: Instruction = Instruction {
    name: "STX",
    opcode: 0x96,
    cycles: 4,
    addrmode: Cpu::zpy,
    execute: Cpu::stx,
};

pub const TYA_98: Instruction = Instruction {
    name: "TYA",
    opcode: 0x98,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::tya,
};

pub const STA_99: Instruction = Instruction {
    name: "STA",
    opcode: 0x99,
    cycles: 5,
    addrmode: Cpu::aby,
    execute: Cpu::sta,
};

pub const TXS_9A: Instruction = Instruction {
    name: "TXS",
    opcode: 0x9A,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::txs,
};

pub const STA_9D: Instruction = Instruction {
    name: "STA",
    opcode: 0x9D,
    cycles: 5,
    addrmode: Cpu::abx,
    execute: Cpu::sta,
};

pub const LDY_A0: Instruction = Instruction {
    name: "LDY",
    opcode: 0xA0,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::ldy,
};

pub const LDA_A1: Instruction = Instruction {
    name: "LDA",
    opcode: 0xA1,
    cycles: 6,
    addrmode: Cpu::izx,
    execute: Cpu::lda,
};

pub const LDX_A2: Instruction = Instruction {
    name: "LDX",
    opcode: 0xA2,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::ldx,
};

pub const LDY_A4: Instruction = Instruction {
    name: "LDY",
    opcode: 0xA4,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::ldy,
};

pub const LDA_A5: Instruction = Instruction {
    name: "LDA",
    opcode: 0xA5,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::lda,
};

pub const LDX_A6: Instruction = Instruction {
    name: "LDX",
    opcode: 0xA6,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::ldx,
};

pub const TAY_A8: Instruction = Instruction {
    name: "TAY",
    opcode: 0xA8,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::tay,
};

pub const LDA_A9: Instruction = Instruction {
    name: "LDA",
    opcode: 0xA9,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::lda,
};

pub const TAX_AA: Instruction = Instruction {
    name: "TAX",
    opcode: 0xAA,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::tax,
};

pub const LDY_AC: Instruction = Instruction {
    name: "LDY",
    opcode: 0xAC,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::ldy,
};

pub const LDA_AD: Instruction = Instruction {
    name: "LDA",
    opcode: 0xAD,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::lda,
};

pub const LDX_AE: Instruction = Instruction {
    name: "LDX",
    opcode: 0xAE,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::ldx,
};

pub const BCS_B0: Instruction = Instruction {
    name: "BCS",
    opcode: 0xB0,
    cycles: 2,
    addrmode: Cpu::rel,
    execute: Cpu::bcs,
};

pub const LDA_B1: Instruction = Instruction {
    name: "LDA",
    opcode: 0xB1,
    cycles: 5,
    addrmode: Cpu::izy,
    execute: Cpu::lda,
};

pub const LDY_B4: Instruction = Instruction {
    name: "LDY",
    opcode: 0xB4,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::ldy,
};

pub const LDA_B5: Instruction = Instruction {
    name: "LDA",
    opcode: 0xB5,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::lda,
};

pub const LDX_B6: Instruction = Instruction {
    name: "LDX",
    opcode: 0xB6,
    cycles: 4,
    addrmode: Cpu::zpy,
    execute: Cpu::ldx,
};

pub const CLV_B8: Instruction = Instruction {
    name: "CLV",
    opcode: 0xB8,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::clv,
};

pub const LDA_B9: Instruction = Instruction {
    name: "LDA",
    opcode: 0xB9,
    cycles: 4,
    addrmode: Cpu::aby,
    execute: Cpu::lda,
};

pub const TSX_BA: Instruction = Instruction {
    name: "TSX",
    opcode: 0xBA,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::tsx,
};

pub const LDY_BC: Instruction = Instruction {
    name: "LDY",
    opcode: 0xBC,
    cycles: 4,
    addrmode: Cpu::abx,
    execute: Cpu::ldy,
};

pub const LDA_BD: Instruction = Instruction {
    name: "LDA",
    opcode: 0xBD,
    cycles: 4,
    addrmode: Cpu::abx,
    execute: Cpu::lda,
};

pub const LDX_BE: Instruction = Instruction {
    name: "LDX",
    opcode: 0xBE,
    cycles: 4,
    addrmode: Cpu::aby,
    execute: Cpu::ldx,
};

pub const CPY_C0: Instruction = Instruction {
    name: "CPY",
    opcode: 0xC0,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::cpy,
};

pub const CMP_C1: Instruction = Instruction {
    name: "CMP",
    opcode: 0xC1,
    cycles: 6,
    addrmode: Cpu::izx,
    execute: Cpu::cmp,
};

pub const CPY_C4: Instruction = Instruction {
    name: "CPY",
    opcode: 0xC4,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::cpy,
};

pub const CMP_C5: Instruction = Instruction {
    name: "CMP",
    opcode: 0xC5,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::cmp,
};

pub const DEC_C6: Instruction = Instruction {
    name: "DEC",
    opcode: 0xC6,
    cycles: 5,
    addrmode: Cpu::zp0,
    execute: Cpu::dec,
};

pub const INY_C8: Instruction = Instruction {
    name: "INY",
    opcode: 0xC8,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::iny,
};

pub const CMP_C9: Instruction = Instruction {
    name: "CMP",
    opcode: 0xC9,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::cmp,
};

pub const DEX_CA: Instruction = Instruction {
    name: "DEX",
    opcode: 0xCA,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::dex,
};

pub const CPY_CC: Instruction = Instruction {
    name: "CPY",
    opcode: 0xCC,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::cpy,
};

pub const CMP_CD: Instruction = Instruction {
    name: "CMP",
    opcode: 0xCD,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::cmp,
};

pub const DEC_CE: Instruction = Instruction {
    name: "DEC",
    opcode: 0xCE,
    cycles: 6,
    addrmode: Cpu::abs,
    execute: Cpu::dec,
};

pub const BNE_D0: Instruction = Instruction {
    name: "BNE",
    opcode: 0xD0,
    cycles: 2,
    addrmode: Cpu::rel,
    execute: Cpu::bne,
};

pub const CMP_D1: Instruction = Instruction {
    name: "CMP",
    opcode: 0xD1,
    cycles: 5,
    addrmode: Cpu::izy,
    execute: Cpu::cmp,
};

pub const CMP_D5: Instruction = Instruction {
    name: "CMP",
    opcode: 0xD5,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::cmp,
};

pub const DEC_D6: Instruction = Instruction {
    name: "DEC",
    opcode: 0xD6,
    cycles: 6,
    addrmode: Cpu::zpx,
    execute: Cpu::dec,
};

pub const CLD_D8: Instruction = Instruction {
    name: "CLD",
    opcode: 0xD8,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::cld,
};

pub const CMP_D9: Instruction = Instruction {
    name: "CMP",
    opcode: 0xD9,
    cycles: 4,
    addrmode: Cpu::aby,
    execute: Cpu::cmp,
};

pub const NOP_DA: Instruction = Instruction {
    name: "NOP",
    opcode: 0xDA,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::nop,
};

pub const CMP_DD: Instruction = Instruction {
    name: "CMP",
    opcode: 0xDD,
    cycles: 4,
    addrmode: Cpu::abx,
    execute: Cpu::cmp,
};

pub const DEC_DE: Instruction = Instruction {
    name: "DEC",
    opcode: 0xDE,
    cycles: 7,
    addrmode: Cpu::abx,
    execute: Cpu::dec,
};

pub const CPX_E0: Instruction = Instruction {
    name: "CPX",
    opcode: 0xE0,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::cpx,
};

pub const SBC_E1: Instruction = Instruction {
    name: "SBC",
    opcode: 0xE1,
    cycles: 6,
    addrmode: Cpu::izx,
    execute: Cpu::sbc,
};

pub const CPX_E4: Instruction = Instruction {
    name: "CPX",
    opcode: 0xE4,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::cpx,
};

pub const SBC_E5: Instruction = Instruction {
    name: "SBC",
    opcode: 0xE5,
    cycles: 3,
    addrmode: Cpu::zp0,
    execute: Cpu::sbc,
};

pub const INC_E6: Instruction = Instruction {
    name: "INC",
    opcode: 0xE6,
    cycles: 5,
    addrmode: Cpu::zp0,
    execute: Cpu::inc,
};

pub const INX_E8: Instruction = Instruction {
    name: "INX",
    opcode: 0xE8,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::inx,
};

pub const SBC_E9: Instruction = Instruction {
    name: "SBC",
    opcode: 0xE9,
    cycles: 2,
    addrmode: Cpu::imm,
    execute: Cpu::sbc,
};

pub const NOP_EA: Instruction = Instruction {
    name: "NOP",
    opcode: 0xEA,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::nop,
};

pub const CPX_EC: Instruction = Instruction {
    name: "CPX",
    opcode: 0xEC,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::cpx,
};

pub const SBC_ED: Instruction = Instruction {
    name: "SBC",
    opcode: 0xED,
    cycles: 4,
    addrmode: Cpu::abs,
    execute: Cpu::sbc,
};

pub const INC_EE: Instruction = Instruction {
    name: "INC",
    opcode: 0xEE,
    cycles: 6,
    addrmode: Cpu::abs,
    execute: Cpu::inc,
};

pub const BEQ_F0: Instruction = Instruction {
    name: "BEQ",
    opcode: 0xF0,
    cycles: 2,
    addrmode: Cpu::rel,
    execute: Cpu::beq,
};

pub const SBC_F1: Instruction = Instruction {
    name: "SBC",
    opcode: 0xF1,
    cycles: 5,
    addrmode: Cpu::izy,
    execute: Cpu::sbc,
};

pub const SBC_F5: Instruction = Instruction {
    name: "SBC",
    opcode: 0xF5,
    cycles: 4,
    addrmode: Cpu::zpx,
    execute: Cpu::sbc,
};

pub const INC_F6: Instruction = Instruction {
    name: "INC",
    opcode: 0xF6,
    cycles: 6,
    addrmode: Cpu::zpx,
    execute: Cpu::inc,
};

pub const SED_F8: Instruction = Instruction {
    name: "SED",
    opcode: 0xF8,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::sed,
};

pub const SBC_F9: Instruction = Instruction {
    name: "SBC",
    opcode: 0xF9,
    cycles: 4,
    addrmode: Cpu::aby,
    execute: Cpu::sbc,
};

pub const NOP_FA: Instruction = Instruction {
    name: "NOP",
    opcode: 0xFA,
    cycles: 2,
    addrmode: Cpu::imp,
    execute: Cpu::nop,
};

pub const SBC_FD: Instruction = Instruction {
    name: "SBC",
    opcode: 0xFD,
    cycles: 4,
    addrmode: Cpu::abx,
    execute: Cpu::sbc,
};

pub const INC_FE: Instruction = Instruction {
    name: "INC",
    opcode: 0xFE,
    cycles: 7,
    addrmode: Cpu::abx,
    execute: Cpu::inc,
};

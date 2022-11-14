//! Module that defines each CPU instruction and the instructions'
//! lookup table.
//! 
//! This module uses the `build_definitions`, `make_instructions`, and
//! `make_lookup_table` macros found in the source code to generate
//! a constant for each instruction and the lookup table.
//! 
//! Having a concrete constant for each instruction of type `Instruction`
//! allows for automatic documentation. To find information about a certain
//! instruction, just take a look at its constant's documentation.

// This part is a bit long, although it used to be longer :)
// It was generated with help from the program
// found at nes/etc/lookup-to-rust.cpp
//
// Thanks to javidx9 (https://github.com/OneLoneCoder)
// for providing the lookup table!

use crate::cpu::instructions::Instruction;
use crate::cpu::Cpu;

/// Builds a constant of type Instruction for each instruction and
/// defines the instructions' lookup table.
macro_rules! build_definitions {
    ($(($name:ident, $opcode:expr, $cycles:expr, $addrmode:expr, $execute:expr)),* $(,)?) => {
        make_instructions!($(($name, $opcode, $cycles, $addrmode, $execute)),*);
        make_lookup_table!($($name),*);
    }
}

/// Generates a constant of type `Instruction` for each valid
/// instruction of the NES.
///
/// Having a constant for each instruction allows for automatic
/// documentation and compile-time optimizations made by the
/// compiler.
///
/// For example, instruction definitions look like the following:
/// ```
/// pub const BRK_00: Instruction = Instruction {
///     name: "BRK",
///     opcode: 0x00,
///     cycles: 7,
///     addrmode: Cpu::imm,
///     execute: Cpu::brk,
/// };
/// ```
macro_rules! make_instructions {
    ($(($name:ident, $opcode:expr, $cycles:expr, $addrmode:expr, $execute:expr)),* $(,)?) => {
        $(
            #[doc = concat!(
                "```\n",
                "pub const ", stringify!($name), ": Instruction = Instruction {\n",
                "    name: \"", stringify!($name), "\",\n",
                "    opcode: ", stringify!($opcode), ",\n",
                "    cycles: ", stringify!($cycles), ",\n",
                "    addrmode: ", stringify!($addrmode), ",\n",
                "    execute: ", stringify!($execute), "\n",
                "}\n",
                "```\n"
            )]
            pub const $name: Instruction = Instruction {
                name: stringify!($name),
                opcode: $opcode,
                cycles: $cycles,
                addrmode: $addrmode,
                execute: $execute
            };
        )*
    };
}

/// Builds an array of size 256 that represents the 
/// CPU instructions' lookup table.
macro_rules! make_lookup_table {
    ($($name:ident),* $(,)?) => {
        /// Lookup table for the CPU instructions.
        /// 
        /// The instruction found at index `i` has opcode `i`.
        /// Some instructions are not defined by the CPU's standard.
        /// Their names contain the string XXX.
        pub const LOOKUP_TABLE: [&Instruction; 256] = [$(&$name),*];
    }
}

build_definitions![
    (X00_BRK, 0x00, 7, Cpu::imm, Cpu::brk),
    (X01_ORA, 0x01, 6, Cpu::izx, Cpu::ora),
    (X02_XXX, 0x02, 2, Cpu::imp, Cpu::nop),
    (X03_XXX, 0x03, 8, Cpu::imp, Cpu::nop),
    (X04_XXX, 0x04, 3, Cpu::imp, Cpu::nop),
    (X05_ORA, 0x05, 3, Cpu::zp0, Cpu::ora),
    (X06_ASL, 0x06, 5, Cpu::zp0, Cpu::asl),
    (X07_XXX, 0x07, 5, Cpu::imp, Cpu::nop),
    (X08_PHP, 0x08, 3, Cpu::imp, Cpu::php),
    (X09_ORA, 0x09, 2, Cpu::imm, Cpu::ora),
    (X0A_ASL, 0x0A, 2, Cpu::imp, Cpu::asl),
    (X0B_XXX, 0x0B, 2, Cpu::imp, Cpu::nop),
    (X0C_XXX, 0x0C, 4, Cpu::imp, Cpu::nop),
    (X0D_ORA, 0x0D, 4, Cpu::abs, Cpu::ora),
    (X0E_ASL, 0x0E, 6, Cpu::abs, Cpu::asl),
    (X0F_XXX, 0x0F, 6, Cpu::imp, Cpu::nop),
    (X10_BPL, 0x10, 2, Cpu::rel, Cpu::bpl),
    (X11_ORA, 0x11, 5, Cpu::izy, Cpu::ora),
    (X12_XXX, 0x12, 2, Cpu::imp, Cpu::nop),
    (X13_XXX, 0x13, 8, Cpu::imp, Cpu::nop),
    (X14_XXX, 0x14, 4, Cpu::imp, Cpu::nop),
    (X15_ORA, 0x15, 4, Cpu::zpx, Cpu::ora),
    (X16_ASL, 0x16, 6, Cpu::zpx, Cpu::asl),
    (X17_XXX, 0x17, 6, Cpu::imp, Cpu::nop),
    (X18_CLC, 0x18, 2, Cpu::imp, Cpu::clc),
    (X19_ORA, 0x19, 4, Cpu::aby, Cpu::ora),
    (X1A_XXX, 0x1A, 2, Cpu::imp, Cpu::nop),
    (X1B_XXX, 0x1B, 7, Cpu::imp, Cpu::nop),
    (X1C_XXX, 0x1C, 4, Cpu::imp, Cpu::nop),
    (X1D_ORA, 0x1D, 4, Cpu::abx, Cpu::ora),
    (X1E_ASL, 0x1E, 7, Cpu::abx, Cpu::asl),
    (X1F_XXX, 0x1F, 7, Cpu::imp, Cpu::nop),
    (X20_JSR, 0x20, 6, Cpu::abs, Cpu::jsr),
    (X21_AND, 0x21, 6, Cpu::izx, Cpu::and),
    (X22_XXX, 0x22, 2, Cpu::imp, Cpu::nop),
    (X23_XXX, 0x23, 8, Cpu::imp, Cpu::nop),
    (X24_BIT, 0x24, 3, Cpu::zp0, Cpu::bit),
    (X25_AND, 0x25, 3, Cpu::zp0, Cpu::and),
    (X26_ROL, 0x26, 5, Cpu::zp0, Cpu::rol),
    (X27_XXX, 0x27, 5, Cpu::imp, Cpu::nop),
    (X28_PLP, 0x28, 4, Cpu::imp, Cpu::plp),
    (X29_AND, 0x29, 2, Cpu::imm, Cpu::and),
    (X2A_ROL, 0x2A, 2, Cpu::imp, Cpu::rol),
    (X2B_XXX, 0x2B, 2, Cpu::imp, Cpu::nop),
    (X2C_BIT, 0x2C, 4, Cpu::abs, Cpu::bit),
    (X2D_AND, 0x2D, 4, Cpu::abs, Cpu::and),
    (X2E_ROL, 0x2E, 6, Cpu::abs, Cpu::rol),
    (X2F_XXX, 0x2F, 6, Cpu::imp, Cpu::nop),
    (X30_BMI, 0x30, 2, Cpu::rel, Cpu::bmi),
    (X31_AND, 0x31, 5, Cpu::izy, Cpu::and),
    (X32_XXX, 0x32, 2, Cpu::imp, Cpu::nop),
    (X33_XXX, 0x33, 8, Cpu::imp, Cpu::nop),
    (X34_XXX, 0x34, 4, Cpu::imp, Cpu::nop),
    (X35_AND, 0x35, 4, Cpu::zpx, Cpu::and),
    (X36_ROL, 0x36, 6, Cpu::zpx, Cpu::rol),
    (X37_XXX, 0x37, 6, Cpu::imp, Cpu::nop),
    (X38_SEC, 0x38, 2, Cpu::imp, Cpu::sec),
    (X39_AND, 0x39, 4, Cpu::aby, Cpu::and),
    (X3A_XXX, 0x3A, 2, Cpu::imp, Cpu::nop),
    (X3B_XXX, 0x3B, 7, Cpu::imp, Cpu::nop),
    (X3C_XXX, 0x3C, 4, Cpu::imp, Cpu::nop),
    (X3D_AND, 0x3D, 4, Cpu::abx, Cpu::and),
    (X3E_ROL, 0x3E, 7, Cpu::abx, Cpu::rol),
    (X3F_XXX, 0x3F, 7, Cpu::imp, Cpu::nop),
    (X40_RTI, 0x40, 6, Cpu::imp, Cpu::rti),
    (X41_EOR, 0x41, 6, Cpu::izx, Cpu::eor),
    (X42_XXX, 0x42, 2, Cpu::imp, Cpu::nop),
    (X43_XXX, 0x43, 8, Cpu::imp, Cpu::nop),
    (X44_XXX, 0x44, 3, Cpu::imp, Cpu::nop),
    (X45_EOR, 0x45, 3, Cpu::zp0, Cpu::eor),
    (X46_LSR, 0x46, 5, Cpu::zp0, Cpu::lsr),
    (X47_XXX, 0x47, 5, Cpu::imp, Cpu::nop),
    (X48_PHA, 0x48, 3, Cpu::imp, Cpu::pha),
    (X49_EOR, 0x49, 2, Cpu::imm, Cpu::eor),
    (X4A_LSR, 0x4A, 2, Cpu::imp, Cpu::lsr),
    (X4B_XXX, 0x4B, 2, Cpu::imp, Cpu::nop),
    (X4C_JMP, 0x4C, 3, Cpu::abs, Cpu::jmp),
    (X4D_EOR, 0x4D, 4, Cpu::abs, Cpu::eor),
    (X4E_LSR, 0x4E, 6, Cpu::abs, Cpu::lsr),
    (X4F_XXX, 0x4F, 6, Cpu::imp, Cpu::nop),
    (X50_BVC, 0x50, 2, Cpu::rel, Cpu::bvc),
    (X51_EOR, 0x51, 5, Cpu::izy, Cpu::eor),
    (X52_XXX, 0x52, 2, Cpu::imp, Cpu::nop),
    (X53_XXX, 0x53, 8, Cpu::imp, Cpu::nop),
    (X54_XXX, 0x54, 4, Cpu::imp, Cpu::nop),
    (X55_EOR, 0x55, 4, Cpu::zpx, Cpu::eor),
    (X56_LSR, 0x56, 6, Cpu::zpx, Cpu::lsr),
    (X57_XXX, 0x57, 6, Cpu::imp, Cpu::nop),
    (X58_CLI, 0x58, 2, Cpu::imp, Cpu::cli),
    (X59_EOR, 0x59, 4, Cpu::aby, Cpu::eor),
    (X5A_XXX, 0x5A, 2, Cpu::imp, Cpu::nop),
    (X5B_XXX, 0x5B, 7, Cpu::imp, Cpu::nop),
    (X5C_XXX, 0x5C, 4, Cpu::imp, Cpu::nop),
    (X5D_EOR, 0x5D, 4, Cpu::abx, Cpu::eor),
    (X5E_LSR, 0x5E, 7, Cpu::abx, Cpu::lsr),
    (X5F_XXX, 0x5F, 7, Cpu::imp, Cpu::nop),
    (X60_RTS, 0x60, 6, Cpu::imp, Cpu::rts),
    (X61_ADC, 0x61, 6, Cpu::izx, Cpu::adc),
    (X62_XXX, 0x62, 2, Cpu::imp, Cpu::nop),
    (X63_XXX, 0x63, 8, Cpu::imp, Cpu::nop),
    (X64_XXX, 0x64, 3, Cpu::imp, Cpu::nop),
    (X65_ADC, 0x65, 3, Cpu::zp0, Cpu::adc),
    (X66_ROR, 0x66, 5, Cpu::zp0, Cpu::ror),
    (X67_XXX, 0x67, 5, Cpu::imp, Cpu::nop),
    (X68_PLA, 0x68, 4, Cpu::imp, Cpu::pla),
    (X69_ADC, 0x69, 2, Cpu::imm, Cpu::adc),
    (X6A_ROR, 0x6A, 2, Cpu::imp, Cpu::ror),
    (X6B_XXX, 0x6B, 2, Cpu::imp, Cpu::nop),
    (X6C_JMP, 0x6C, 5, Cpu::ind, Cpu::jmp),
    (X6D_ADC, 0x6D, 4, Cpu::abs, Cpu::adc),
    (X6E_ROR, 0x6E, 6, Cpu::abs, Cpu::ror),
    (X6F_XXX, 0x6F, 6, Cpu::imp, Cpu::nop),
    (X70_BVS, 0x70, 2, Cpu::rel, Cpu::bvs),
    (X71_ADC, 0x71, 5, Cpu::izy, Cpu::adc),
    (X72_XXX, 0x72, 2, Cpu::imp, Cpu::nop),
    (X73_XXX, 0x73, 8, Cpu::imp, Cpu::nop),
    (X74_XXX, 0x74, 4, Cpu::imp, Cpu::nop),
    (X75_ADC, 0x75, 4, Cpu::zpx, Cpu::adc),
    (X76_ROR, 0x76, 6, Cpu::zpx, Cpu::ror),
    (X77_XXX, 0x77, 6, Cpu::imp, Cpu::nop),
    (X78_SEI, 0x78, 2, Cpu::imp, Cpu::sei),
    (X79_ADC, 0x79, 4, Cpu::aby, Cpu::adc),
    (X7A_XXX, 0x7A, 2, Cpu::imp, Cpu::nop),
    (X7B_XXX, 0x7B, 7, Cpu::imp, Cpu::nop),
    (X7C_XXX, 0x7C, 4, Cpu::imp, Cpu::nop),
    (X7D_ADC, 0x7D, 4, Cpu::abx, Cpu::adc),
    (X7E_ROR, 0x7E, 7, Cpu::abx, Cpu::ror),
    (X7F_XXX, 0x7F, 7, Cpu::imp, Cpu::nop),
    (X80_XXX, 0x80, 2, Cpu::imp, Cpu::nop),
    (X81_STA, 0x81, 6, Cpu::izx, Cpu::sta),
    (X82_XXX, 0x82, 2, Cpu::imp, Cpu::nop),
    (X83_XXX, 0x83, 6, Cpu::imp, Cpu::nop),
    (X84_STY, 0x84, 3, Cpu::zp0, Cpu::sty),
    (X85_STA, 0x85, 3, Cpu::zp0, Cpu::sta),
    (X86_STX, 0x86, 3, Cpu::zp0, Cpu::stx),
    (X87_XXX, 0x87, 3, Cpu::imp, Cpu::nop),
    (X88_DEY, 0x88, 2, Cpu::imp, Cpu::dey),
    (X89_XXX, 0x89, 2, Cpu::imp, Cpu::nop),
    (X8A_TXA, 0x8A, 2, Cpu::imp, Cpu::txa),
    (X8B_XXX, 0x8B, 2, Cpu::imp, Cpu::nop),
    (X8C_STY, 0x8C, 4, Cpu::abs, Cpu::sty),
    (X8D_STA, 0x8D, 4, Cpu::abs, Cpu::sta),
    (X8E_STX, 0x8E, 4, Cpu::abs, Cpu::stx),
    (X8F_XXX, 0x8F, 4, Cpu::imp, Cpu::nop),
    (X90_BCC, 0x90, 2, Cpu::rel, Cpu::bcc),
    (X91_STA, 0x91, 6, Cpu::izy, Cpu::sta),
    (X92_XXX, 0x92, 2, Cpu::imp, Cpu::nop),
    (X93_XXX, 0x93, 6, Cpu::imp, Cpu::nop),
    (X94_STY, 0x94, 4, Cpu::zpx, Cpu::sty),
    (X95_STA, 0x95, 4, Cpu::zpx, Cpu::sta),
    (X96_STX, 0x96, 4, Cpu::zpy, Cpu::stx),
    (X97_XXX, 0x97, 4, Cpu::imp, Cpu::nop),
    (X98_TYA, 0x98, 2, Cpu::imp, Cpu::tya),
    (X99_STA, 0x99, 5, Cpu::aby, Cpu::sta),
    (X9A_TXS, 0x9A, 2, Cpu::imp, Cpu::txs),
    (X9B_XXX, 0x9B, 5, Cpu::imp, Cpu::nop),
    (X9C_XXX, 0x9C, 5, Cpu::imp, Cpu::nop),
    (X9D_STA, 0x9D, 5, Cpu::abx, Cpu::sta),
    (X9E_XXX, 0x9E, 5, Cpu::imp, Cpu::nop),
    (X9F_XXX, 0x9F, 5, Cpu::imp, Cpu::nop),
    (XA0_LDY, 0xA0, 2, Cpu::imm, Cpu::ldy),
    (XA1_LDA, 0xA1, 6, Cpu::izx, Cpu::lda),
    (XA2_LDX, 0xA2, 2, Cpu::imm, Cpu::ldx),
    (XA3_XXX, 0xA3, 6, Cpu::imp, Cpu::nop),
    (XA4_LDY, 0xA4, 3, Cpu::zp0, Cpu::ldy),
    (XA5_LDA, 0xA5, 3, Cpu::zp0, Cpu::lda),
    (XA6_LDX, 0xA6, 3, Cpu::zp0, Cpu::ldx),
    (XA7_XXX, 0xA7, 3, Cpu::imp, Cpu::nop),
    (XA8_TAY, 0xA8, 2, Cpu::imp, Cpu::tay),
    (XA9_LDA, 0xA9, 2, Cpu::imm, Cpu::lda),
    (XAA_TAX, 0xAA, 2, Cpu::imp, Cpu::tax),
    (XAB_XXX, 0xAB, 2, Cpu::imp, Cpu::nop),
    (XAC_LDY, 0xAC, 4, Cpu::abs, Cpu::ldy),
    (XAD_LDA, 0xAD, 4, Cpu::abs, Cpu::lda),
    (XAE_LDX, 0xAE, 4, Cpu::abs, Cpu::ldx),
    (XAF_XXX, 0xAF, 4, Cpu::imp, Cpu::nop),
    (XB0_BCS, 0xB0, 2, Cpu::rel, Cpu::bcs),
    (XB1_LDA, 0xB1, 5, Cpu::izy, Cpu::lda),
    (XB2_XXX, 0xB2, 2, Cpu::imp, Cpu::nop),
    (XB3_XXX, 0xB3, 5, Cpu::imp, Cpu::nop),
    (XB4_LDY, 0xB4, 4, Cpu::zpx, Cpu::ldy),
    (XB5_LDA, 0xB5, 4, Cpu::zpx, Cpu::lda),
    (XB6_LDX, 0xB6, 4, Cpu::zpy, Cpu::ldx),
    (XB7_XXX, 0xB7, 4, Cpu::imp, Cpu::nop),
    (XB8_CLV, 0xB8, 2, Cpu::imp, Cpu::clv),
    (XB9_LDA, 0xB9, 4, Cpu::aby, Cpu::lda),
    (XBA_TSX, 0xBA, 2, Cpu::imp, Cpu::tsx),
    (XBB_XXX, 0xBB, 4, Cpu::imp, Cpu::nop),
    (XBC_LDY, 0xBC, 4, Cpu::abx, Cpu::ldy),
    (XBD_LDA, 0xBD, 4, Cpu::abx, Cpu::lda),
    (XBE_LDX, 0xBE, 4, Cpu::aby, Cpu::ldx),
    (XBF_XXX, 0xBF, 4, Cpu::imp, Cpu::nop),
    (XC0_CPY, 0xC0, 2, Cpu::imm, Cpu::cpy),
    (XC1_CMP, 0xC1, 6, Cpu::izx, Cpu::cmp),
    (XC2_XXX, 0xC2, 2, Cpu::imp, Cpu::nop),
    (XC3_XXX, 0xC3, 8, Cpu::imp, Cpu::nop),
    (XC4_CPY, 0xC4, 3, Cpu::zp0, Cpu::cpy),
    (XC5_CMP, 0xC5, 3, Cpu::zp0, Cpu::cmp),
    (XC6_DEC, 0xC6, 5, Cpu::zp0, Cpu::dec),
    (XC7_XXX, 0xC7, 5, Cpu::imp, Cpu::nop),
    (XC8_INY, 0xC8, 2, Cpu::imp, Cpu::iny),
    (XC9_CMP, 0xC9, 2, Cpu::imm, Cpu::cmp),
    (XCA_DEX, 0xCA, 2, Cpu::imp, Cpu::dex),
    (XCB_XXX, 0xCB, 2, Cpu::imp, Cpu::nop),
    (XCC_CPY, 0xCC, 4, Cpu::abs, Cpu::cpy),
    (XCD_CMP, 0xCD, 4, Cpu::abs, Cpu::cmp),
    (XCE_DEC, 0xCE, 6, Cpu::abs, Cpu::dec),
    (XCF_XXX, 0xCF, 6, Cpu::imp, Cpu::nop),
    (XD0_BNE, 0xD0, 2, Cpu::rel, Cpu::bne),
    (XD1_CMP, 0xD1, 5, Cpu::izy, Cpu::cmp),
    (XD2_XXX, 0xD2, 2, Cpu::imp, Cpu::nop),
    (XD3_XXX, 0xD3, 8, Cpu::imp, Cpu::nop),
    (XD4_XXX, 0xD4, 4, Cpu::imp, Cpu::nop),
    (XD5_CMP, 0xD5, 4, Cpu::zpx, Cpu::cmp),
    (XD6_DEC, 0xD6, 6, Cpu::zpx, Cpu::dec),
    (XD7_XXX, 0xD7, 6, Cpu::imp, Cpu::nop),
    (XD8_CLD, 0xD8, 2, Cpu::imp, Cpu::cld),
    (XD9_CMP, 0xD9, 4, Cpu::aby, Cpu::cmp),
    (XDA_NOP, 0xDA, 2, Cpu::imp, Cpu::nop),
    (XDB_XXX, 0xDB, 7, Cpu::imp, Cpu::nop),
    (XDC_XXX, 0xDC, 4, Cpu::imp, Cpu::nop),
    (XDD_CMP, 0xDD, 4, Cpu::abx, Cpu::cmp),
    (XDE_DEC, 0xDE, 7, Cpu::abx, Cpu::dec),
    (XDF_XXX, 0xDF, 7, Cpu::imp, Cpu::nop),
    (XE0_CPX, 0xE0, 2, Cpu::imm, Cpu::cpx),
    (XE1_SBC, 0xE1, 6, Cpu::izx, Cpu::sbc),
    (XE2_XXX, 0xE2, 2, Cpu::imp, Cpu::nop),
    (XE3_XXX, 0xE3, 8, Cpu::imp, Cpu::nop),
    (XE4_CPX, 0xE4, 3, Cpu::zp0, Cpu::cpx),
    (XE5_SBC, 0xE5, 3, Cpu::zp0, Cpu::sbc),
    (XE6_INC, 0xE6, 5, Cpu::zp0, Cpu::inc),
    (XE7_XXX, 0xE7, 5, Cpu::imp, Cpu::nop),
    (XE8_INX, 0xE8, 2, Cpu::imp, Cpu::inx),
    (XE9_SBC, 0xE9, 2, Cpu::imm, Cpu::sbc),
    (XEA_NOP, 0xEA, 2, Cpu::imp, Cpu::nop),
    (XEB_XXX, 0xEB, 2, Cpu::imp, Cpu::sbc),
    (XEC_CPX, 0xEC, 4, Cpu::abs, Cpu::cpx),
    (XED_SBC, 0xED, 4, Cpu::abs, Cpu::sbc),
    (XEE_INC, 0xEE, 6, Cpu::abs, Cpu::inc),
    (XEF_XXX, 0xEF, 6, Cpu::imp, Cpu::nop),
    (XF0_BEQ, 0xF0, 2, Cpu::rel, Cpu::beq),
    (XF1_SBC, 0xF1, 5, Cpu::izy, Cpu::sbc),
    (XF2_XXX, 0xF2, 2, Cpu::imp, Cpu::nop),
    (XF3_XXX, 0xF3, 8, Cpu::imp, Cpu::nop),
    (XF4_XXX, 0xF4, 4, Cpu::imp, Cpu::nop),
    (XF5_SBC, 0xF5, 4, Cpu::zpx, Cpu::sbc),
    (XF6_INC, 0xF6, 6, Cpu::zpx, Cpu::inc),
    (XF7_XXX, 0xF7, 6, Cpu::imp, Cpu::nop),
    (XF8_SED, 0xF8, 2, Cpu::imp, Cpu::sed),
    (XF9_SBC, 0xF9, 4, Cpu::aby, Cpu::sbc),
    (XFA_NOP, 0xFA, 2, Cpu::imp, Cpu::nop),
    (XFB_XXX, 0xFB, 7, Cpu::imp, Cpu::nop),
    (XFC_XXX, 0xFC, 4, Cpu::imp, Cpu::nop),
    (XFD_SBC, 0xFD, 4, Cpu::abx, Cpu::sbc),
    (XFE_INC, 0xFE, 7, Cpu::abx, Cpu::inc),
    (XFF_XXX, 0xFF, 7, Cpu::imp, Cpu::nop),
];

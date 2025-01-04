//! Addressing modes for the 6502 CPU.

use super::Cpu;

#[derive(Debug, Clone, Copy)]
pub struct AddrMode {
    pub typ: AddrType,
    pub run: fn(cpu: &mut Cpu) -> u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddrType {
    Imp,
    Imm,
    Zp0,
    Zpx,
    Zpy,
    Abs,
    Abx,
    Aby,
    Ind,
    Izx,
    Izy,
    Rel,
}

impl Cpu {
    /// Implied addressing
    ///
    /// Either no data is part of the instruction
    /// or the data used is on the A register
    pub const IMP: AddrMode = AddrMode {
        typ: AddrType::Imp,
        run: |cpu| {
            cpu.data.fetched = cpu.a;
            0
        },
    };

    /// Immediate addressing
    ///
    /// The data is supplied by the next
    /// byte of an instruction.
    /// The program counter will be increased.
    pub const IMM: AddrMode = AddrMode {
        typ: AddrType::Imm,
        run: |cpu| {
            cpu.data.addr_abs = cpu.pc;
            cpu.pc += 1;
            0
        },
    };

    /// Zero-page addressing
    ///
    /// To our CPU, in a 16-bit memory address,
    /// the higher 8 bits are called a "page" and
    /// the lower 8 bits are called an "offset".
    /// This way, we can represent a 16-bit memory
    /// as containing 256 pages, each with an offset
    /// of 256 bytes.
    ///
    /// Zero-page addressing means the data we are
    /// looking for is on the first page of the memory,
    /// that is, page 0x00.
    pub const ZP0: AddrMode = AddrMode {
        typ: AddrType::Zp0,
        run: |cpu| {
            // Doing this, the higher 8 bits can only be 0x00, which is
            // page 0x00
            cpu.data.addr_abs = cpu.read_inc_pc() as u16;
            cpu.data.addr_abs &= 0x00FF; // just to be sure
            0
        },
    };

    /// Zero-page addressing with X offset
    ///
    /// Same as the above, but with an offset to the desired address
    /// as set by the X register
    pub const ZPX: AddrMode = AddrMode {
        typ: AddrType::Zpx,
        run: |cpu| {
            cpu.data.addr_abs = cpu.read_inc_pc() as u16 + cpu.x as u16;
            cpu.data.addr_abs &= 0x00FF; // prevents changing page
            0
        },
    };

    /// Zero-page addressing with Y offset
    ///
    /// Same as the above, but the offset is set by the Y register
    pub const ZPY: AddrMode = AddrMode {
        typ: AddrType::Zpy,
        run: |cpu| {
            cpu.data.addr_abs = cpu.read_inc_pc() as u16 + cpu.y as u16;
            cpu.data.addr_abs &= 0x00FF; // prevents changing page
            0
        },
    };

    /// Absolute addressing
    ///
    /// The full address is set by the two following bytes to the
    /// instruction
    pub const ABS: AddrMode = AddrMode {
        typ: AddrType::Abs,
        run: |cpu| {
            let low = cpu.read_inc_pc() as u16;
            let high = cpu.read_inc_pc() as u16;

            cpu.data.addr_abs = (high << 8) | low;
            0
        },
    };

    /// Absolute addressing with X register offset
    ///
    /// Same as the above, but an offset will be set by the
    /// X register. This instruction may need an additional
    /// clock cycle.
    pub const ABX: AddrMode = AddrMode {
        typ: AddrType::Abx,
        run: |cpu| {
            let low = cpu.read_inc_pc() as u16;
            let high = cpu.read_inc_pc() as u16;

            cpu.data.addr_abs = (high << 8) | low;
            cpu.data.addr_abs += cpu.x as u16;

            // If the page is changed by the addition, an additional
            // clock cycle may be necessary
            u8::from(cpu.data.addr_abs & 0xFF00 != high << 8)
        },
    };

    /// Absolute addressing with X register offset
    ///
    /// Same as the above, but an offset will be set by the
    /// Y register. This instruction may need an additional
    /// clock cycle.
    pub const ABY: AddrMode = AddrMode {
        typ: AddrType::Aby,
        run: |cpu| {
            let low = cpu.read_inc_pc() as u16;
            let high = cpu.read_inc_pc() as u16;

            cpu.data.addr_abs = (high << 8) | low;
            cpu.data.addr_abs = cpu.data.addr_abs.wrapping_add(cpu.y as u16);

            // If the page is changed by the addition, an additional
            // clock cycle may be necessary
            u8::from(cpu.data.addr_abs & 0xFF00 != high << 8)
        },
    };

    /// Indirect addressing
    ///
    /// The absolute address given by the instruction acts as a pointer.
    /// The cpu.addr_abs will be set as the absolute address found
    /// at the location of memory pointed by the pointer given by the
    /// instruction.
    pub const IND: AddrMode = AddrMode {
        typ: AddrType::Ind,
        run: |cpu| {
            let pointer_low = cpu.read_inc_pc() as u16;
            let pointer_high = cpu.read_inc_pc() as u16;

            let pointer = (pointer_high << 8) | pointer_low;

            if pointer_low == 0x00FF {
                // There is a bug on this addressing mode, which creates
                // functionality that some 6502 programs use. Because of
                // this, we will implement the bug.
                let low = cpu.read(pointer) as u16;
                let high = cpu.read(pointer & 0xFF00) as u16;
                cpu.data.addr_abs = (high << 8) | low;
            } else {
                // Normal behaviour
                let low = cpu.read(pointer) as u16;
                let high = cpu.read(pointer + 1) as u16;
                cpu.data.addr_abs = (high << 8) | low;
            }
            0
        },
    };

    /// Zero-page indirect addresing with X register offset
    ///
    /// The value of cpu.addr_abs will be set to the value pointer
    /// by the pointer found at the zero-page offset given by the instruction
    /// plus the X register.
    pub const IZX: AddrMode = AddrMode {
        typ: AddrType::Izx,
        run: |cpu| {
            // offset address on the 0 page
            let zero_addr = cpu.read_inc_pc() as u16 + cpu.x as u16;

            // & 0x00FF prevents changing page
            let low = cpu.read(zero_addr & 0x00FF) as u16;
            let high = cpu.read((zero_addr + 1) & 0x00FF) as u16;

            cpu.data.addr_abs = (high << 8) | low;
            0
        },
    };

    /// Zero-page indirect addresing with Y register offset
    ///
    /// This behaves differently than IZX. The value of cpu.addr_abs
    /// will be set by the value of the pointer found at the zero-page
    /// offset given by the instruction. Then, cpu.addr_abs will be
    /// added by the value in the Y register. This instruction may need
    /// an additional cycle.
    pub const IZY: AddrMode = AddrMode {
        typ: AddrType::Izy,
        run: |cpu| {
            let zero_addr = cpu.read_inc_pc() as u16;

            // & 0x00FF prevents changing page
            let low = cpu.read(zero_addr & 0x00FF) as u16;
            let high = cpu.read((zero_addr + 1) & 0x00FF) as u16;

            cpu.data.addr_abs = (high << 8) | low;
            cpu.data.addr_abs = cpu.data.addr_abs.wrapping_add(cpu.y as u16);

            // If the page is changed by the addition, an additional
            // clock cycle may be necessary
            u8::from(cpu.data.addr_abs & 0xFF00 != high << 8)
        },
    };

    /// Relative addressing
    ///
    /// This addressing mode is used by branching instructions.
    /// Branching instructions can only jump to a location further
    /// than 127 bytes from its location.
    pub const REL: AddrMode = AddrMode {
        typ: AddrType::Rel,
        run: |cpu| {
            cpu.data.addr_rel = cpu.read_inc_pc() as u16;

            if cpu.data.addr_rel & 0x0080 != 0 {
                // the 8 bit value read by the CPU is negative
                // if this is the case, we set the higher 8 bits
                // of cpu.addr_rel to 0xFF
                cpu.data.addr_rel |= 0xFF00;
            }
            0
        },
    };
}

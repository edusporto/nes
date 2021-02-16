// ====================================================
// ============ Instruction implementations ===========
// ====================================================

use crate::cpu::flags::CpuFlags;
use crate::cpu::Cpu;

/// The 6502 has a hardcoded base location for the stack pointer
const STACK_BASE: u16 = 0x0100;

impl Cpu {
    /// Helper function. Sets the Z flag if the accumulator
    /// is 0.
    fn set_zero(&mut self) {
        self.status.set(CpuFlags::Z, self.a == 0);
    }

    /// Helper function. Sets the N flag if the accumulator
    /// is negative.
    fn set_negative(&mut self) {
        self.status.set(CpuFlags::N, self.a & 0x80 != 0);
    }

    /// Helper function. Branches the Program Counter and adds
    /// possible additional cycles
    ///
    /// All branch instructions need from 1 to 2 additional cycles.
    fn branch(&mut self) {
        // Needs an additional clock cycle.
        self.cycles += 1;

        self.addr_abs = self.pc + self.addr_rel;

        if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
            // If the page changes, another additional cycle
            // will be needed.
            self.cycles += 1;
        }

        self.pc = self.addr_abs;
    }

    /// Add memory to Accumulator with Carry
    ///
    /// Adds the accumulator to the value in memory.
    /// Also adds the carry bit.
    ///
    /// The operation executed can be represented as `A := A + M + C`.
    ///
    /// May change the value of the flags N, C, Z and V.
    ///
    /// Can work with both signed and unsigned values.
    /// To work out the overflow, we will use the following
    /// truth table:
    ///
    /// | Accumulator positive | Memory positive | Result positive | Overflow |
    /// |----------------------|-----------------|-----------------|----------|
    /// | 0                    | 0               | 0               | 0        |
    /// | 0                    | 0               | 1               | 1        |
    /// | 0                    | 1               | 0               | 0        |
    /// | 0                    | 1               | 1               | 0        |
    /// | 1                    | 0               | 0               | 0        |
    /// | 1                    | 0               | 1               | 0        |
    /// | 1                    | 1               | 0               | 1        |
    /// | 1                    | 1               | 1               | 0        |
    ///
    /// The logical formula we will be using to find out if an overflow happens
    /// is the following:
    ///
    /// * If the accumulator and the memory are positive and the result is
    /// negative, an overflow happened.
    /// * If the accumulator and the memory are negative and the result is
    /// positive, an overflow happened.
    /// * Otherwise, no overflow happened.
    pub fn adc(&mut self) -> u8 {
        self.fetch();

        // 0 or 1
        let c = u8::from(self.status.contains(CpuFlags::C));

        let addition = self.a as u16 + self.fetched as u16 + c as u16;

        // If the result is over 0xFF, a carry bit is needed
        self.status.set(CpuFlags::C, addition > 0xFF);

        // If the 8 bit addition results in 0x00, the Z flag is set to 1
        // We need to remove the higher 8 bits of the 16-bit addition
        self.status.set(CpuFlags::Z, addition & 0x00FF == 0);

        // If the most significant bit of the 8-bit addition is 1,
        // the result may be negative if it is treated like so. The N
        // flag will be set to 1
        self.status.set(CpuFlags::N, addition & 0x0080 != 0);

        // * If the accumulator and the memory are positive and the result is
        // negative, an overflow happened.
        // * If the accumulator and the memory are negative and the result is
        // positive, an overflow happened.
        // * Otherwise, no overflow happened.
        let acc_pos = self.a & 0x80 == 0;
        let mem_pos = self.fetched & 0x80 == 0;
        let res_pos = addition & 0x80 == 0;
        let overflow = (acc_pos && mem_pos && !res_pos) || (!acc_pos && !mem_pos && res_pos);
        self.status.set(CpuFlags::V, overflow);

        // Set the accumulator to the 8-bit result of the addition
        self.a = (addition & 0x00FF) as u8;
        1
    }

    /// Bitwise AND
    ///
    /// Performs a bitwise AND between the fetched
    /// data and the Accumulator register, and sets
    /// the Accumulator as the result of the operation.
    ///
    /// Sets the Z and N flags.
    ///
    /// May need an additional clock cycle.
    pub fn and(&mut self) -> u8 {
        self.fetch();

        self.a &= self.fetched;

        self.set_zero();
        self.set_negative();

        1
    }

    pub fn asl(&mut self) -> u8 {
        todo!()
    }

    /// Branch on Carry Clear
    ///
    /// Branches if the C flag is 0.
    pub fn bcc(&mut self) -> u8 {
        if !self.status.contains(CpuFlags::C) {
            self.branch();
        }
        0
    }

    /// Branch on Carry Set
    ///
    /// Branches if the C flag is 1.
    pub fn bcs(&mut self) -> u8 {
        if self.status.contains(CpuFlags::C) {
            self.branch();
        }
        0
    }

    /// Branch on Result Zero
    ///
    /// Branches if the Z flag is 1.
    pub fn beq(&mut self) -> u8 {
        if self.status.contains(CpuFlags::Z) {
            self.branch();
        }
        0
    }

    pub fn bit(&mut self) -> u8 {
        todo!()
    }

    /// Branch on Result Minus
    ///
    /// Branches if the N flag is 1.
    pub fn bmi(&mut self) -> u8 {
        if self.status.contains(CpuFlags::N) {
            self.branch();
        }
        0
    }

    /// Branch on Result not Zero
    ///
    /// Branches if the Z flag is 0.
    pub fn bne(&mut self) -> u8 {
        if !self.status.contains(CpuFlags::Z) {
            self.branch();
        }
        0
    }

    /// Branch on Result Plus
    ///
    /// Branches if the N flag is 0.
    pub fn bpl(&mut self) -> u8 {
        if !self.status.contains(CpuFlags::N) {
            self.branch();
        }
        0
    }

    pub fn brk(&mut self) -> u8 {
        todo!()
    }

    /// Branch on Overflow clear
    ///
    /// Branches if the V flag is 0.
    pub fn bvc(&mut self) -> u8 {
        if !self.status.contains(CpuFlags::V) {
            self.branch();
        }
        0
    }

    /// Branch on Overflow set
    ///
    /// Branches if the V flag is 1.
    pub fn bvs(&mut self) -> u8 {
        if self.status.contains(CpuFlags::V) {
            self.branch();
        }
        0
    }

    /// Clear Carry Flag
    ///
    /// Sets C to 0.
    pub fn clc(&mut self) -> u8 {
        self.status.set(CpuFlags::C, false);
        0
    }

    /// Clear Decimal Mode Flag
    ///
    /// Sets D to 0.
    pub fn cld(&mut self) -> u8 {
        self.status.set(CpuFlags::D, false);
        0
    }

    /// Clear Interrupt Disable Bit
    ///
    /// Sets I to 0.
    pub fn cli(&mut self) -> u8 {
        self.status.set(CpuFlags::I, false);
        0
    }

    /// Clear Overflow Flag
    ///
    /// Sets V to 0.
    pub fn clv(&mut self) -> u8 {
        self.status.set(CpuFlags::V, false);
        0
    }

    pub fn cmp(&mut self) -> u8 {
        todo!()
    }

    pub fn cpx(&mut self) -> u8 {
        todo!()
    }
    pub fn cpy(&mut self) -> u8 {
        todo!()
    }
    pub fn dec(&mut self) -> u8 {
        todo!()
    }
    pub fn dex(&mut self) -> u8 {
        todo!()
    }
    pub fn dey(&mut self) -> u8 {
        todo!()
    }
    pub fn eor(&mut self) -> u8 {
        todo!()
    }
    pub fn inc(&mut self) -> u8 {
        todo!()
    }
    pub fn inx(&mut self) -> u8 {
        todo!()
    }
    pub fn iny(&mut self) -> u8 {
        todo!()
    }
    pub fn jmp(&mut self) -> u8 {
        todo!()
    }
    pub fn jsr(&mut self) -> u8 {
        todo!()
    }
    pub fn lda(&mut self) -> u8 {
        todo!()
    }
    pub fn ldx(&mut self) -> u8 {
        todo!()
    }
    pub fn ldy(&mut self) -> u8 {
        todo!()
    }
    pub fn lsr(&mut self) -> u8 {
        todo!()
    }
    pub fn nop(&mut self) -> u8 {
        todo!()
    }
    pub fn ora(&mut self) -> u8 {
        todo!()
    }

    /// Push Accumulator on Stack
    ///
    /// Pushes the value on the Accumulator register to
    /// the stack.
    pub fn pha(&mut self) -> u8 {
        self.write(STACK_BASE + self.stkp as u16, self.a);
        self.stkp -= 1;
        0
    }

    /// Push Processor Status on Stack
    ///
    /// Pushes the Status register, which contains the processor flags,
    /// to the stack.
    ///
    /// The B and U flags are set to 1 on the pushed value, but
    /// are set to 0 on the register itself.
    pub fn php(&mut self) -> u8 {
        self.write(
            STACK_BASE + self.stkp as u16,
            (self.status | CpuFlags::B | CpuFlags::U).bits(),
        );

        self.status.set(CpuFlags::B, false);
        self.status.set(CpuFlags::U, false);

        self.stkp -= 1;
        0
    }

    /// Pull Accumulator from Stack
    ///
    /// Pulls the top of the stack to the Accumulator.
    ///
    /// May change the Z and N flags.
    pub fn pla(&mut self) -> u8 {
        self.a = self.read(STACK_BASE + self.stkp as u16);
        self.stkp += 1;
        self.set_zero();
        self.set_negative();
        0
    }

    pub fn plp(&mut self) -> u8 {
        todo!()
    }
    pub fn rol(&mut self) -> u8 {
        todo!()
    }
    pub fn ror(&mut self) -> u8 {
        todo!()
    }
    pub fn rti(&mut self) -> u8 {
        todo!()
    }
    pub fn rts(&mut self) -> u8 {
        todo!()
    }

    /// Subtract Memory from Accumulator with Borrow
    ///
    /// Subtracts the Accumulator by the value from memory.
    /// Will also subtract the opposite of the carry bit, called
    /// the "borrow".
    ///
    /// The operation executed can be represented as `A := A - M - (1-C)`.
    ///
    /// May change the value of the flags N, C, Z and V.
    pub fn sbc(&mut self) -> u8 {
        // A := A - M - (1-C)
        // A := A + (-M) + 1 + C
        // (since -X = (~X) + 1) =>
        // A := A + (~M) + C
        // (this way, the implementation is similar to Cpu::adc)

        self.fetch();

        // Changes the value of self.fetched to reflect the discussion above
        self.fetched = !self.fetched;

        // 0 or 1
        let c = u8::from(self.status.contains(CpuFlags::C));

        let addition = self.a as u16 + self.fetched as u16 + c as u16;

        // If the result is over 0xFF, a carry bit is needed
        self.status.set(CpuFlags::C, addition > 0xFF);

        // If the 8 bit addition results in 0x00, the Z flag is set to 1
        // We need to remove the higher 8 bits of the 16-bit addition
        self.status.set(CpuFlags::Z, addition & 0x00FF == 0);

        // If the most significant bit of the 8-bit addition is 1,
        // the result may be negative if it is treated like so. The N
        // flag will be set to 1
        self.status.set(CpuFlags::N, addition & 0x0080 != 0);

        // * If the accumulator and the memory are positive and the result is
        // negative, an overflow happened.
        // * If the accumulator and the memory are negative and the result is
        // positive, an overflow happened.
        // * Otherwise, no overflow happened.
        let acc_pos = self.a & 0x80 == 0;
        let mem_pos = self.fetched & 0x80 == 0;
        let res_pos = addition & 0x80 == 0;
        let overflow = (acc_pos && mem_pos && !res_pos) || (!acc_pos && !mem_pos && res_pos);
        self.status.set(CpuFlags::V, overflow);

        // Set the accumulator to the 8-bit result of the addition
        self.a = (addition & 0x00FF) as u8;

        1
    }

    pub fn sec(&mut self) -> u8 {
        todo!()
    }
    pub fn sed(&mut self) -> u8 {
        todo!()
    }
    pub fn sei(&mut self) -> u8 {
        todo!()
    }
    pub fn sta(&mut self) -> u8 {
        todo!()
    }
    pub fn stx(&mut self) -> u8 {
        todo!()
    }
    pub fn sty(&mut self) -> u8 {
        todo!()
    }
    pub fn tax(&mut self) -> u8 {
        todo!()
    }
    pub fn tay(&mut self) -> u8 {
        todo!()
    }
    pub fn tsx(&mut self) -> u8 {
        todo!()
    }
    pub fn txa(&mut self) -> u8 {
        todo!()
    }
    pub fn txs(&mut self) -> u8 {
        todo!()
    }
    pub fn tya(&mut self) -> u8 {
        todo!()
    }
}

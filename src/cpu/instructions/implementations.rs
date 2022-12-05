///! Instruction implementations.
use crate::cpu::flags::CpuFlags;
use crate::cpu::{Cpu, STACK_BASE};

use super::Instruction;

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

        // Impossible for addition below to overflow on our machine
        // since the result is `u16`, we will later check if the addition
        // overflowed considering only the lower 8 bits
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

    /// Shift Left One Bit (Memory or Accumulator)
    ///
    /// Shifts left the value either in memory or in the Accumulator
    /// by 1
    ///
    /// May change the flags N, Z, C
    pub fn asl(&mut self) -> u8 {
        self.fetch();

        let result = (self.fetched as u16) << 1;

        // if the 16-bit result is over 255, a carry bit is needed
        self.status.set(CpuFlags::C, result > 255);
        // cant use functions self.set_negative() and self.set_zero()
        self.status.set(CpuFlags::N, result & 0x80 != 0);
        self.status.set(CpuFlags::Z, result & 0xFF == 0);

        // if the addresing mode is implied, write to the Accumulator
        // otherwise, write to the memory
        let addrmode = Instruction::lookup(self.opcode).addrmode;
        if addrmode as usize == Cpu::imp as usize {
            self.a = (result & 0xFF) as u8;
        } else {
            self.write(self.addr_abs, (result & 0xFF) as u8);
        }

        0
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

    /// Tests Bits in Memory with Accumulator
    ///
    /// Z := A & M == 0
    /// N := M & 0b10000000
    /// V := V & 0b01000000
    pub fn bit(&mut self) -> u8 {
        self.fetch();

        let result = self.a & self.fetched;
        // 0xFF to keep consistency
        self.status.set(CpuFlags::Z, result & 0xFF == 0);
        self.status.set(CpuFlags::N, self.fetched & (1 << 7) != 0);
        self.status.set(CpuFlags::V, self.fetched & (1 << 6) != 0);

        0
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

    /// Force Break
    pub fn brk(&mut self) -> u8 {
        // Performs something similar to an IRQ.
        self.pc += 1;

        self.status.set(CpuFlags::I, true);
        self.write(
            STACK_BASE + self.stkp as u16,
            ((self.pc >> 8) & 0x00FF) as u8,
        );
        self.stkp -= 1;
        self.write(STACK_BASE + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        // differs from IRQ here
        self.status.set(CpuFlags::B, true);
        self.write(STACK_BASE + self.stkp as u16, self.status.bits());
        self.stkp -= 1;
        self.status.set(CpuFlags::B, false);

        // grabs the new program counter from the addresses 0xFFFE and 0xFFFF
        let low = self.read(0xFFFE) as u16;
        let high = self.read(0xFFFF) as u16;
        self.pc = low | (high << 8);
        0
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

    /// Compare Memory with Accumulator
    ///
    /// Compares the value in memory to the value in the Accumulator.
    ///
    /// C := A >= M,
    /// Z := (A - M) == 0
    ///
    /// May change the C, Z, N flags.
    ///
    /// May need an additional cycle.
    pub fn cmp(&mut self) -> u8 {
        self.fetch();

        // TODO: may be a source of bug
        let result = (self.a as u16).wrapping_sub(self.fetched as u16);

        self.status.set(CpuFlags::C, self.a >= self.fetched);
        self.status.set(CpuFlags::Z, result & 0xFF == 0);
        self.status.set(CpuFlags::N, result & 0x80 != 0);

        1
    }

    /// Compare Memory and Index X
    ///
    /// Compares the value in memory to the value in the X register.
    ///
    /// C := X >= M,
    /// Z := (X - M) == 0
    ///
    /// May change the C, Z, N flags.
    pub fn cpx(&mut self) -> u8 {
        self.fetch();

        let result = (self.x as u16).wrapping_sub(self.fetched as u16);

        self.status.set(CpuFlags::C, self.x >= self.fetched);
        self.status.set(CpuFlags::Z, result & 0xFF == 0);
        self.status.set(CpuFlags::N, result & 0x80 != 0);

        0
    }

    /// Compare Memory and Index Y
    ///
    /// Compares the value in memory to the value in the X register.
    ///
    /// C := Y >= M,
    /// Z := (Y - M) == 0
    ///
    /// May change the C, Z, N flags.
    pub fn cpy(&mut self) -> u8 {
        self.fetch();

        let result = (self.y as u16).wrapping_sub(self.fetched as u16);

        self.status.set(CpuFlags::C, self.y >= self.fetched);
        self.status.set(CpuFlags::Z, result & 0xFF == 0);
        self.status.set(CpuFlags::N, result & 0x80 != 0);

        0
    }

    /// Decrement Memory by One
    ///
    /// Decreases the value in memory by 1.
    ///
    /// May change the N, Z flags.
    pub fn dec(&mut self) -> u8 {
        self.fetch();

        let new = self.fetched.wrapping_sub(1);

        self.write(self.addr_abs, new);
        self.status.set(CpuFlags::N, new & 0x80 != 0);
        self.status.set(CpuFlags::Z, new & 0xFF == 0);

        0
    }

    /// Decrement Index X by One
    ///
    /// Decreases the value in the X register by 1.
    ///
    /// May change the N, Z flags.
    pub fn dex(&mut self) -> u8 {
        self.x = self.x.wrapping_sub(1);
        self.status.set(CpuFlags::N, self.x & 0x80 != 0);
        self.status.set(CpuFlags::Z, self.x & 0xFF == 0);
        0
    }

    /// Decrement Index Y by One
    ///
    /// Decreases the value in the Y register by 1.
    ///
    /// May change the N, Z flags.
    pub fn dey(&mut self) -> u8 {
        self.y = self.y.wrapping_sub(1);
        self.status.set(CpuFlags::N, self.y & 0x80 != 0);
        self.status.set(CpuFlags::Z, self.y & 0xFF == 0);
        0
    }

    /// Exclusivse-OR Memory with Accumulator
    ///
    /// A := A XOR M
    ///
    /// May change the N, Z flags.
    ///
    /// May need an additional clock cycle.
    pub fn eor(&mut self) -> u8 {
        self.fetch();

        let new = self.a ^ self.fetched;

        self.a = new;
        self.set_negative();
        self.set_zero();

        1
    }

    /// Increment Memory by One
    ///
    /// Increases the value in memory by 1.
    ///
    /// May change the N, Z flags.
    pub fn inc(&mut self) -> u8 {
        self.fetch();

        let new = self.fetched.wrapping_add(1);

        self.write(self.addr_abs, new);
        self.status.set(CpuFlags::N, new & 0x80 != 0);
        self.status.set(CpuFlags::Z, new & 0xFF == 0);

        0
    }

    /// Increment Index X by One
    ///
    /// Increases the value of the X register by 1.
    ///
    /// May change the N, Z flags.
    pub fn inx(&mut self) -> u8 {
        self.x = self.x.wrapping_add(1);
        self.status.set(CpuFlags::N, self.x & 0x80 != 0);
        self.status.set(CpuFlags::Z, self.x & 0xFF == 0);
        0
    }

    /// Increment Index X by One
    ///
    /// Increases the value of the X register by 1.
    ///
    /// May change the N, Z flags.
    pub fn iny(&mut self) -> u8 {
        self.y = self.y.wrapping_add(1);
        self.status.set(CpuFlags::N, self.y & 0x80 != 0);
        self.status.set(CpuFlags::Z, self.y & 0xFF == 0);
        0
    }

    /// Jump to New Location
    ///
    /// Changes the PC to the value in memory.
    pub fn jmp(&mut self) -> u8 {
        self.pc = self.addr_abs;
        0
    }

    /// Jump to New Location Saving Return Address
    ///
    /// Changes the PC to the value in memory but pushes
    /// the PC value to the stack.
    ///
    /// Meant to be used with [`Cpu::rts`]
    pub fn jsr(&mut self) -> u8 {
        self.pc -= 1; // pushes the PC at the instruction to memory

        self.write(STACK_BASE + self.stkp as u16, ((self.pc >> 8) & 0xFF) as u8);
        self.stkp -= 1;
        self.write(STACK_BASE + self.stkp as u16, (self.pc & 0xFF) as u8);
        self.stkp -= 1;

        self.pc = self.addr_abs;
        0
    }

    /// Load Accumulator with Memory
    ///
    /// A := M
    ///
    /// May set the N, Z flags.
    ///
    /// May need an additional clock cycle.
    pub fn lda(&mut self) -> u8 {
        self.fetch();

        self.a = self.fetched;
        self.set_negative();
        self.set_zero();

        1
    }

    /// Load Index X with Memory
    ///
    /// X := M
    ///
    /// May need an additional clock cycle.
    pub fn ldx(&mut self) -> u8 {
        self.fetch();

        self.x = self.fetched;
        self.status.set(CpuFlags::N, self.x & 0x80 != 0);
        self.status.set(CpuFlags::Z, self.x == 0);

        1
    }

    /// Load Index Y with Memory
    ///
    /// Y := M
    ///
    /// May need an additional clock cycle.
    pub fn ldy(&mut self) -> u8 {
        self.fetch();

        self.y = self.fetched;
        self.status.set(CpuFlags::N, self.y & 0x80 != 0);
        self.status.set(CpuFlags::Z, self.y == 0);

        1
    }

    /// Shift One Bit Right (Memory or Accumulator)
    ///
    /// Shifts right the value either in memory or in the Accumulator
    /// by 1
    ///
    /// May change the flags N, Z, C
    pub fn lsr(&mut self) -> u8 {
        self.fetch();

        let result = (self.fetched as u16) >> 1;

        // if the lowest bit was lost, set the Carry flag
        self.status.set(CpuFlags::C, self.fetched & 0x01 != 0);
        // cant use functions self.set_negative() and self.set_zero()
        self.status.set(CpuFlags::N, result & 0x80 != 0);
        self.status.set(CpuFlags::Z, result & 0xFF == 0);

        // if the addresing mode is implied, write to the Accumulator
        // otherwise, write to the memory
        let addrmode = Instruction::lookup(self.opcode).addrmode;
        if addrmode as usize == Cpu::imp as usize {
            self.a = (result & 0xFF) as u8;
        } else {
            self.write(self.addr_abs, (result & 0xFF) as u8);
        }

        0
    }

    /// No Operation
    ///
    /// No operation is executed.
    ///
    /// The following link presents the CPU illegal opcodes sometimes
    /// used by NES games:
    /// <https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes>
    ///
    /// Some of the illegal opcodes represent NOP instructions. They
    /// may require additional cycles.
    pub fn nop(&mut self) -> u8 {
        match self.opcode {
            // may need additional cycles
            0x1C => 1,
            0x3C => 1,
            0x5C => 1,
            0x7C => 1,
            0xDC => 1,
            0xFC => 1,
            // do not need additional cycles
            _ => 0,
        }
    }

    /// OR Memory with Accumulator
    ///
    /// A := A | M
    ///
    /// May change the N, Z flags.
    ///
    /// May need an additional clock cycle.
    pub fn ora(&mut self) -> u8 {
        self.fetch();

        self.a |= self.fetched;
        self.set_negative();
        self.set_zero();

        1
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
        self.stkp += 1;
        self.a = self.read(STACK_BASE + self.stkp as u16);
        self.set_zero();
        self.set_negative();
        0
    }

    /// Pull Processor Status from Stack
    ///
    /// Sets the status flags as the top value of the Stack.
    ///
    /// Sets the U flag to 1.
    pub fn plp(&mut self) -> u8 {
        self.stkp += 1;
        let status = self.read(STACK_BASE + self.stkp as u16);
        self.status = CpuFlags::from_bits_truncate(status);
        self.status.set(CpuFlags::U, true);
        0
    }

    /// Rotate One Bit Left (Memory or Accumulator)
    ///
    /// Shifts the value to the left. The carry flag is shifted into
    /// bit 0 and the original bit 7 is shifted into the Carry flag.
    ///
    /// May change the N, Z, C flags.
    pub fn rol(&mut self) -> u8 {
        self.fetch();

        let carry = u16::from(self.status.contains(CpuFlags::C));
        let result = ((self.fetched as u16) << 1) | carry;

        self.status.set(CpuFlags::C, result > 0xFF);
        self.status.set(CpuFlags::N, result & 0x80 != 0);
        self.status.set(CpuFlags::Z, result & 0xFF == 0);

        let addrmode = Instruction::lookup(self.opcode).addrmode;
        if addrmode as usize == Cpu::imp as usize {
            self.a = (result & 0xFF) as u8;
        } else {
            self.write(self.addr_abs, (result & 0xFF) as u8);
        }

        0
    }

    /// Rotate One Bit Right (Memory or Accumulator)
    ///
    /// Shifts the value to the right. The carry flag is shifted into
    /// bit 7 and the original bit 0 is shifted into the Carry flag.
    ///
    /// May change the N, Z, C flags.
    pub fn ror(&mut self) -> u8 {
        self.fetch();

        // if C is set, carry = 0b10000000
        let carry = (u16::from(self.status.contains(CpuFlags::C))) << 7;
        let result = carry | ((self.fetched as u16) >> 1);

        // bit 0 of the original value is 1
        self.status.set(CpuFlags::C, self.fetched & 0x01 != 0);
        self.status.set(CpuFlags::N, result & 0x80 != 0);
        self.status.set(CpuFlags::Z, result & 0xFF == 0);

        let addrmode = Instruction::lookup(self.opcode).addrmode;
        if addrmode as usize == Cpu::imp as usize {
            self.a = (result & 0xFF) as u8;
        } else {
            self.write(self.addr_abs, (result & 0xFF) as u8);
        }

        0
    }

    /// Return from interrupt
    ///
    /// Affects all flags.
    pub fn rti(&mut self) -> u8 {
        self.stkp += 1;
        let status = self.read(STACK_BASE + self.stkp as u16);
        self.status = CpuFlags::from_bits_truncate(status);
        self.status.set(CpuFlags::B, false); // TODO: is this it?
        self.status.set(CpuFlags::U, false);

        self.stkp += 1;
        self.pc = self.read(STACK_BASE + self.stkp as u16) as u16;
        self.stkp += 1;
        self.pc |= (self.read(STACK_BASE + self.stkp as u16) as u16) << 8;

        0
    }

    /// Return from Subroutine
    ///
    /// Pulls the address at the top of the stack and sets
    /// the PC to it plus 1.
    ///
    /// Meant to be used with [`Cpu::jsr`]
    pub fn rts(&mut self) -> u8 {
        self.stkp += 1;
        self.pc = self.read(STACK_BASE + self.stkp as u16) as u16;
        self.stkp += 1;
        self.pc |= (self.read(STACK_BASE + self.stkp as u16) as u16) << 8;
        self.pc += 1;

        0
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
        let fetched_not = !self.fetched;

        // 0 or 1
        let c = u8::from(self.status.contains(CpuFlags::C));

        // Can't overflow due to the same reasons found in `Cpu::adc`
        let addition = self.a as u16 + fetched_not as u16 + c as u16;

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
        let mem_pos = fetched_not & 0x80 == 0;
        let res_pos = addition & 0x80 == 0;
        let overflow = (acc_pos && mem_pos && !res_pos) || (!acc_pos && !mem_pos && res_pos);
        self.status.set(CpuFlags::V, overflow);

        // Set the accumulator to the 8-bit result of the addition
        self.a = (addition & 0x00FF) as u8;

        1
    }

    /// Set Carry Flag
    ///
    /// Sets the Carry flag to 1.
    pub fn sec(&mut self) -> u8 {
        self.status.set(CpuFlags::C, true);
        0
    }

    /// Set Decimal Flag
    ///
    /// Sets the Decimal Mode flag to 1.
    pub fn sed(&mut self) -> u8 {
        self.status.set(CpuFlags::D, true);
        0
    }

    /// Set Interrupt Disable Status
    ///
    /// Sets the Interrupt flag to 1.
    pub fn sei(&mut self) -> u8 {
        self.status.set(CpuFlags::I, true);
        0
    }

    /// Store Accumulator in Memory
    ///
    /// M := A
    pub fn sta(&mut self) -> u8 {
        self.write(self.addr_abs, self.a);
        0
    }

    /// Store Index X in Memory
    ///
    /// M := X
    pub fn stx(&mut self) -> u8 {
        self.write(self.addr_abs, self.x);
        0
    }

    /// Store Index Y in Memory
    ///
    /// M := Y
    pub fn sty(&mut self) -> u8 {
        self.write(self.addr_abs, self.y);
        0
    }

    /// Transfer Accumulator to Index X
    ///
    /// X := A
    ///
    /// May change the N, Z flags.
    pub fn tax(&mut self) -> u8 {
        self.x = self.a;

        self.status.set(CpuFlags::N, self.x & 0x80 != 0);
        self.status.set(CpuFlags::Z, self.x & 0xFF == 0);

        0
    }

    /// Transfer Accumulator to Index Y
    ///
    /// Y := A
    ///
    /// May change the N, Z flags.
    pub fn tay(&mut self) -> u8 {
        self.y = self.a;

        self.status.set(CpuFlags::N, self.y & 0x80 != 0);
        self.status.set(CpuFlags::Z, self.y & 0xFF == 0);

        0
    }

    /// Transfer Stack Pointer to Index X
    ///
    /// X := STKP
    ///
    /// May change the N, Z flags.
    pub fn tsx(&mut self) -> u8 {
        self.x = self.stkp;

        self.status.set(CpuFlags::N, self.x & 0x80 != 0);
        self.status.set(CpuFlags::Z, self.x & 0xFF == 0);

        0
    }

    /// Transfer Index X to Accumulator
    ///
    /// A := X
    ///
    /// May change the N, Z flags.
    pub fn txa(&mut self) -> u8 {
        self.a = self.x;

        self.set_negative();
        self.set_zero();

        0
    }

    /// Transfer Index X to Stack Register
    ///
    /// STKP := X
    pub fn txs(&mut self) -> u8 {
        self.stkp = self.x;
        0
    }

    /// Transfer Index Y to Accumulator
    /// A := Y
    /// May change the N, Z flags.
    pub fn tya(&mut self) -> u8 {
        self.a = self.y;

        self.set_negative();
        self.set_zero();

        0
    }

    /// Captures illegal opcodes
    pub fn xxx(&mut self) -> u8 {
        0
    }
}

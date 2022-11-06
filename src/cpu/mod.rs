//! Module representing a 6502 CPU.
//!
//! The CPU has the following external connections:
//! * Address (16-bit)
//! * Read/Write Data (8-bit)
//! * Read/Write flag
//! * Clock
//!
//! The CPU will be connected to a Bus by the address lines
//! and data lines. More devices will be connected to the Bus,
//! such as the Picture Processing Unit.
//!
//! For now, the only other device connected to the CPU will
//! be the RAM.
//!
//! Our CPU has three registers:
//! * A: Accumulator (8-bit)
//! * X (8-bit)
//! * Y (8-bit)
//! * STKP: Stack pointer (8-bit)
//! * PC: Program counter (16-bit)
//! * STATUS: Status flags (8-bit)
//!
//! The instructions performed by the CPU can have different
//! sizes: they can be 1 byte, 2 bytes or 3 bytes. This means
//! that some instructions will be executed in several clocks.
//!
//! With each instruction, we will have to deal with
//! * Function
//! * Address mode
//! * Cycles

pub mod instructions;
mod addressing;
mod flags;

use crate::ram::{Ram, RAM_MIRROR};
use flags::CpuFlags;
use instructions::Instruction;

/// The 6502 has a hardcoded base location for the stack pointer
pub const STACK_BASE: u16 = 0x0100;

/// Defines a CPU and its registers
pub struct Cpu {
    /// Representd the Bus which the CPU is connected to.
    /// The CPU has to connect to the Bus after being created.
    // pub bus: Option<Bus>,

    /// Random Access Memory, 2 kb size with mirrorring up to 8 kb
    pub ram: Ram,

    /// Accumulator register
    pub a: u8,
    /// X register
    pub x: u8,
    /// Y register
    pub y: u8,
    /// Stack pointer
    pub stkp: u8,
    /// Program counter
    pub pc: u16,
    /// STATUS register
    pub status: CpuFlags,

    /// Cycles left on current instruction
    ///
    /// Contains the amount of cycles
    /// remaining by the current function.
    /// When it reaches 0, the next
    /// instruction will execute.
    cycles: u8,

    /// Represents the opcode currently being
    /// executed.
    opcode: u8,

    /// Fetched data to an instruction.
    /// Represents the input to the ALU.
    ///
    /// Its value will be set on the addressing
    /// mode functions and it will be used on
    /// the instruction execute functions.
    fetched: u8,

    /// Represents used memory addresses.
    addr_abs: u16,

    /// The relative memory address is used by
    /// branching instructions.
    addr_rel: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            // bus: None,
            ram: Ram::default(),

            a: 0,
            x: 0,
            y: 0,
            stkp: 0,
            pc: 0,
            status: CpuFlags::empty(),

            cycles: 0,
            opcode: 0,
            fetched: 0,
            addr_abs: 0,
            addr_rel: 0,
        }
    }

    /// Connect a Bus to the CPU
    // pub fn connect_bus(&mut self, bus: Bus) {
    //     self.bus = Some(bus);
    // }

    /// Write `data` to memory at address `addr`
    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram.write_mirrored(addr, data, RAM_MIRROR)
        // match &mut self.bus {
        //     Some(bus) => bus.write(addr, data),
        //     None => panic!(
        //         "called `write` on unconnected CPU. \
        //         consider calling Bus::connect_cpu"
        //     ),
        // }
    }

    /// Read value from memory at address `addr`
    pub fn read(&self, addr: u16) -> u8 {
        self.ram.read_mirrored(addr, RAM_MIRROR)
        // match &self.bus {
        //     Some(bus) => bus.read(addr),
        //     None => panic!(
        //         "called `read` on unconnected CPU. \
        //         consider calling Bus::connect_cpu"
        //     ),
        // }
    }

    /// Reads from the address at the Program Counter
    pub fn read_from_pc(&self) -> u8 {
        self.read(self.pc)
    }

    /// Reads from the address at the Program Counter
    /// and increments the program counter.
    pub fn read_inc_pc(&mut self) -> u8 {
        let result = self.read(self.pc);
        self.pc += 1;
        result
    }

    /// **Resets the CPU into a known state**
    ///
    /// Takes 8 CPU cycles.
    ///
    /// A = 0,
    /// X = 0,
    /// Y = 0,
    /// STKP = 0xFD,
    /// STATUS = 0x00 | CpuFlags::U
    ///
    ///
    /// The PC will be set to the value pointed by the
    /// 16-bit pointer found at 0xFFFC
    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = CpuFlags::empty() | CpuFlags::U;

        self.addr_abs = 0xFFFC;
        let low = self.read(self.addr_abs) as u16;
        let high = self.read(self.addr_abs + 1) as u16;
        self.pc = high << 8 | low;

        self.addr_abs = 0;
        self.addr_rel = 0;
        self.fetched = 0;

        self.cycles = 8;
    }

    /// **Interrupt request**
    ///
    /// Only executes if the I flag is 0.
    ///
    /// Takes 7 cycles.
    ///
    /// Writes the current PC to the Stack,
    /// Sets the following flags:
    /// B := 0
    /// U := 1
    /// I := 1
    /// Writes the STATUS register to the Stack.
    ///
    /// The PC will be set to the value pointed by the
    /// 16-bit pointer found at 0xFFFE
    pub fn irq(&mut self) {
        if self.status.contains(CpuFlags::I) {
            return;
        }

        self.write(
            STACK_BASE + self.stkp as u16,
            ((self.pc >> 8) & 0x00FF) as u8,
        );
        self.stkp -= 1;
        self.write(STACK_BASE + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        self.status.set(CpuFlags::B, false);
        self.status.set(CpuFlags::U, true);
        self.status.set(CpuFlags::I, true);
        self.write(STACK_BASE + self.stkp as u16, self.status.bits());
        self.stkp -= 1;

        self.addr_abs = 0xFFFE;
        let low = self.read(self.addr_abs) as u16;
        let high = self.read(self.addr_abs) as u16;
        self.pc = (high << 8) | low;

        self.cycles = 7;
    }

    /// **Non-maskable interrupt**
    ///
    /// Takes 8 cycles.
    ///
    /// Same as the above, but it doesn't check the I flag
    /// before executing.
    ///
    /// The PC will be set to the value pointed by the
    /// 16-bit pointer found at 0xFFFA
    pub fn nmi(&mut self) {
        self.write(
            STACK_BASE + self.stkp as u16,
            ((self.pc >> 8) & 0x00FF) as u8,
        );
        self.stkp -= 1;
        self.write(STACK_BASE + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        self.status.set(CpuFlags::B, false);
        self.status.set(CpuFlags::U, true);
        self.status.set(CpuFlags::I, true);
        self.write(STACK_BASE + self.stkp as u16, self.status.bits());
        self.stkp -= 1;

        self.addr_abs = 0xFFFA;
        let low = self.read(self.addr_abs) as u16;
        let high = self.read(self.addr_abs) as u16;
        self.pc = (high << 8) | low;

        self.cycles = 8;
    }

    /// **Executes a clock cycle**
    ///
    /// If an instruction has clock cycles pending, does nothing.
    /// Otherwise, it reads the current instruction from the PC
    /// and executes it.
    pub fn clock(&mut self) {
        if self.cycles != 0 {
            self.cycles -= 1;
            return;
        }

        // Read opcode from address at the Program Counter
        let opcode = self.read_inc_pc();

        let ins = Instruction::lookup(opcode);

        // Each instructions needs a different amount of
        // clock cycles.
        let mut cycles = ins.cycles;

        // Call instruction
        let add_cycle1 = (ins.addrmode)(self);
        let add_cycle2 = (ins.execute)(self);

        // `addrmode` and `execute` return either 0 or 1.
        // If both return 0, an additional cycle is needed.
        cycles += add_cycle1 & add_cycle2;

        self.cycles = cycles;

        self.cycles -= 1;
    }

    /// Fetches the data required by the current instruction.
    ///
    /// Not used by the implied address mode.
    fn fetch(&mut self) -> u8 {
        let addrmode = Instruction::lookup(self.opcode).addrmode;
        // Compare function pointers
        if addrmode as usize != Cpu::imp as usize {
            self.fetched = self.read(self.addr_abs);
        }
        self.fetched
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

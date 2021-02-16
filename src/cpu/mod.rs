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

mod addressing;
mod flags;
mod instructions;

use crate::bus::Bus;
use flags::CpuFlags;
use instructions::Instruction;

/// Defines a CPU and its registers
pub struct Cpu {
    /// Representd the Bus which the CPU is connected to.
    /// The CPU has to connect to the Bus after being created.
    pub bus: Option<Bus>,

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
            bus: None,

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

    pub fn connect_bus(&mut self, bus: Bus) {
        self.bus = Some(bus);
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match &mut self.bus {
            Some(bus) => bus.write(addr, data),
            None => panic!(
                "called `write` on unconnected CPU. \
                consider calling Bus::connect_cpu"
            ),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match &self.bus {
            Some(bus) => bus.read(addr),
            None => panic!(
                "called `read` on unconnected CPU. \
                consider calling Bus::connect_cpu"
            ),
        }
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

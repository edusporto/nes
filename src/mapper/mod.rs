//! Module for the Mappers.
//!
//! Mappers are circuits contained within NES cartridges that allow
//! games to expand the NES' capabilities and bypass its limitations.

pub mod mappers;

use std::fmt::Debug;

/// Mapper trait.
///
/// Allows the creation of generic Mappers.
pub trait Mapper: Debug {
    /// Map reads from the CPU.
    fn cpu_map_read(&self, addr: u16) -> (bool, u32);
    /// Map writes from the CPU.
    fn cpu_map_write(&self, addr: u16) -> (bool, u32);
    /// Map reads from the PPU.
    fn ppu_map_read(&self, addr: u16) -> (bool, u32);
    /// Map writes from the PPU.
    fn ppu_map_write(&self, addr: u16) -> (bool, u32);
}

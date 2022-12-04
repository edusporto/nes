//! Module for the game cartridge.

use std::fs;
use std::io::{self, Read, Seek, SeekFrom};

use binread::{BinRead, BinReaderExt};

use crate::mapper::mappers;
use crate::mapper::Mapper;

#[derive(Debug)]
pub struct Cartridge {
    pub mirror: CartridgeMirror,

    program_memory: Vec<u8>,
    character_memory: Vec<u8>,

    mapper_id: u8,
    program_banks: u8,
    character_banks: u8,

    mapper: Box<dyn Mapper>,
}

#[derive(Default, Clone, Debug)]
pub enum CartridgeMirror {
    #[default]
    Horizontal,
    Vertical,
    OneScreenLow,
    OneScreenHigh,
}

/// Format header for iNES
#[derive(BinRead, Debug, Clone, Copy)]
struct CartridgeHeader {
    name: [u8; 4],
    program_rom_chunks: u8,
    character_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    program_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [u8; 5],
}

/// Cartridge Error
///
/// - FileError: Could not read cartridge file
/// - HeaderError: Could not read the file's header
/// - FileTypeError: Unknown file type for cartridge
#[derive(Debug)]
pub enum Error {
    FileError(io::Error),
    HeaderError(binread::Error),
    FileTypeError(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileError(_) => write!(f, "could not read ROM file"),
            Error::HeaderError(_) => write!(f, "invalid header for ROM file"),
            Error::FileTypeError(_) => write!(f, "unknown ROM file type"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::FileError(err)
    }
}

impl From<binread::Error> for Error {
    fn from(err: binread::Error) -> Self {
        Self::HeaderError(err)
    }
}

impl Cartridge {
    pub fn from_file(file_name: &str) -> Result<Cartridge, Error> {
        Cartridge::from_bytes(&fs::read(file_name)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Cartridge, Error> {
        let mut reader = binread::io::Cursor::new(bytes);

        let header: CartridgeHeader = reader.read_be()?;

        if header.mapper1 & 0x04 != 0 {
            // skip 512 bytes
            reader.seek(SeekFrom::Current(512))?;
        }

        let mut program_memory: Vec<u8> = Vec::new();
        let mut character_memory: Vec<u8> = Vec::new();

        let mapper_id: u8 = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
        let program_banks: u8;
        let character_banks: u8;

        let mirror = if header.mapper1 & 0x01 != 0 {
            CartridgeMirror::Vertical
        } else {
            CartridgeMirror::Horizontal
        };

        let file_type: u8 = 1;

        match file_type {
            0 => todo!(),
            1 => {
                program_banks = header.program_rom_chunks;
                program_memory.resize(program_banks as usize * 16384, 0);
                reader.read_exact(&mut program_memory)?;

                character_banks = header.character_rom_chunks;
                character_memory.resize(character_banks as usize * 8192, 0);
                reader.read_exact(&mut character_memory)?;
            }
            2 => todo!(),
            _ => return Err(Error::FileTypeError(file_type)),
        };

        let mapper = match mapper_id {
            0 => Box::new(mappers::Mapper0::new(program_banks, character_banks)),
            _ => todo!(),
        };

        Ok(Cartridge {
            program_memory,
            character_memory,

            mapper_id,
            program_banks,
            character_banks,

            mirror,
            mapper,
        })
    }

    pub fn cpu_map_read(&self, addr: u16, data: u8) -> (bool, u8) {
        let (mapped, mapped_addr) = self.mapper.cpu_map_read(addr);
        match mapped {
            true => (true, self.program_memory[mapped_addr as usize]),
            false => (false, data),
        }
    }

    pub fn cpu_map_write(&self, addr: u16, data: u8) -> (bool, u8) {
        let (mapped, mapped_addr) = self.mapper.cpu_map_write(addr);
        match mapped {
            true => (true, self.program_memory[mapped_addr as usize]),
            false => (false, data),
        }
    }

    pub fn ppu_map_read(&self, addr: u16, data: u8) -> (bool, u8) {
        let (mapped, mapped_addr) = self.mapper.ppu_map_read(addr);
        match mapped {
            true => (true, self.character_memory[mapped_addr as usize]),
            false => (false, data),
        }
    }

    pub fn ppu_map_write(&self, addr: u16, data: u8) -> (bool, u8) {
        let (mapped, mapped_addr) = self.mapper.ppu_map_write(addr);
        match mapped {
            true => (true, self.character_memory[mapped_addr as usize]),
            false => (false, data),
        }
    }
}

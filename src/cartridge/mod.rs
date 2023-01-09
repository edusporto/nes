//! Module for the game cartridge.

use std::fs;
use std::io::{self, Read, Seek, SeekFrom};

use binread::{BinRead, BinReaderExt};
use thiserror::Error;

use crate::mapper::mappers;
use crate::mapper::Mapper;

#[derive(Debug)]
pub struct Cartridge {
    pub(crate) mirror: CartridgeMirror,
    pub(crate) header: CartridgeHeader,

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
pub struct CartridgeHeader {
    name: [u8; 4],
    program_rom_chunks: u8,
    character_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    program_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    _unused: [u8; 5],
}

/// Cartridge Error
///
/// - FileError: Could not read cartridge file
/// - HeaderError: Could not read the file's header
/// - FileTypeError: Unknown file type for cartridge
/// - UnimplementedError: Functionality not yet implemented
#[derive(Error, Debug)]
pub enum CartridgeError {
    #[error("could not read ROM file: {0}")]
    FileError(#[from] io::Error),
    #[error("invalid header for ROM file")]
    HeaderError(#[from] binread::Error),
    #[error("unknown ROM file type: {0}")]
    FileTypeError(u8),
    #[error("unimplemented: {0}")]
    UnimplementedError(String),
}

impl Cartridge {
    pub fn from_file(file_name: &str) -> Result<Cartridge, CartridgeError> {
        Cartridge::from_bytes(&fs::read(file_name)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Cartridge, CartridgeError> {
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
            0 => return Err(CartridgeError::UnimplementedError("file type 0".into())),
            1 => {
                program_banks = header.program_rom_chunks;
                program_memory.resize(program_banks as usize * 16384, 0);
                reader.read_exact(&mut program_memory)?;

                character_banks = header.character_rom_chunks;
                character_memory.resize(character_banks as usize * 8192, 0);
                reader.read_exact(&mut character_memory)?;
            }
            2 => return Err(CartridgeError::UnimplementedError("file type 2".into())),
            _ => return Err(CartridgeError::FileTypeError(file_type)),
        };

        let mapper = match mapper_id {
            0 => Box::new(mappers::Mapper0::new(program_banks, character_banks)),
            _ => {
                return Err(CartridgeError::UnimplementedError(format!(
                    "mapper with id {mapper_id}"
                )))
            }
        };

        Ok(Cartridge {
            program_memory,
            character_memory,

            mapper_id,
            program_banks,
            character_banks,

            mirror,
            header,
            mapper,
        })
    }

    pub fn cpu_map_read(&self, addr: u16) -> Option<u8> {
        self.mapper
            .cpu_map_read(addr)
            .map(|mapped_addr| self.program_memory[mapped_addr as usize])
    }

    pub fn cpu_map_write(&mut self, addr: u16, data: u8) -> Option<u8> {
        self.mapper.cpu_map_write(addr, data).map(|mapped_addr| {
            self.program_memory[mapped_addr as usize] = data;
            self.program_memory[mapped_addr as usize]
        })
    }

    pub fn ppu_map_read(&self, addr: u16) -> Option<u8> {
        self.mapper
            .ppu_map_read(addr)
            .map(|mapped_addr| self.character_memory[mapped_addr as usize])
    }

    pub fn ppu_map_write(&mut self, addr: u16, data: u8) -> Option<u8> {
        self.mapper.ppu_map_write(addr).map(|mapped_addr| {
            self.character_memory[mapped_addr as usize] = data;
            self.character_memory[mapped_addr as usize]
        })
    }
}

impl std::fmt::Display for Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.header.name))
    }
}

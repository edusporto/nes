//! Module for the game cartridge.

use std::fs;
use std::io::{self, Read, Seek, SeekFrom};

use binread::{BinRead, BinReaderExt};

#[derive(Clone, Debug)]
pub struct Cartridge {
    program_memory: Vec<u8>,
    character_memory: Vec<u8>,

    mapper_id: u8,
    program_banks: u8,
    character_banks: u8,
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
#[derive(Debug)]
pub enum Error {
    FileError(io::Error),
    HeaderError(binread::Error),
    FileTypeError(u8),
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
    fn from_file(file_name: &str) -> Result<Cartridge, Error> {
        Cartridge::read(&fs::read(file_name)?)
    }

    fn read(bytes: &[u8]) -> Result<Cartridge, Error> {
        let mut reader = binread::io::Cursor::new(bytes);

        let header: CartridgeHeader = reader.read_be()?;

        if header.mapper1 & 0x04 != 0 {
            // skip 512 bytes
            reader.seek(SeekFrom::Current(512))?;
        }

        let mut program_memory: Vec<u8> = Vec::new();
        let mut character_memory: Vec<u8> = Vec::new();

        let mapper_id: u8 = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
        let mut program_banks: u8 = 0;
        let mut character_banks: u8 = 0;

        let file_type: u8 = 1;

        match file_type {
            0 => todo!(),
            1 => {
                program_banks = header.program_rom_chunks;
                program_memory.resize(program_banks as usize * 16384, 0);
                reader.read(&mut program_memory)?;

                character_banks = header.character_rom_chunks;
                character_memory.resize(character_banks as usize * 8192, 0);
                reader.read(&mut character_memory)?;
            },
            2 => todo!(),
            _ => return Err(Error::FileTypeError(file_type)),
        };

        Ok(Cartridge {
            program_memory,
            character_memory,
            
            mapper_id,
            program_banks,
            character_banks,
        })
    }
}

//! Module for the Object Attribute Memory

/// Object Address Memory. Stores sprites.
#[derive(Clone, Copy, Debug)]
pub struct Oam {
    mem: [[u8; 4]; 64],
}

impl Oam {
    // these functions are all implemented to avoid transmuting
    // the `mem` array into a slice of bytes and back

    pub fn get_byte(&self, index: u8) -> u8 {
        self.mem[index as usize / 4][index as usize % 4]
    }

    pub fn set_byte(&mut self, index: u8, data: u8) {
        self.mem[index as usize / 4][index as usize % 4] = data;
    }

    pub fn get_entry(&self, index: u8) -> OamEntry {
        // mirror with 63 to avoid overflow
        OamEntry::from(self.mem[index as usize & 63])
    }

    pub fn set_entry(&mut self, index: u8, entry: OamEntry) {
        // mirror with 63 to avoid overflow
        self.mem[index as usize & 63] = entry.into();
    }
}

impl Default for Oam {
    fn default() -> Self {
        Self { mem: [[0; 4]; 64] }
    }
}

/// Single entry in the Object Attribute Memory (OAM).
/// Represents sprites rendered in the foreground.
#[derive(Copy, Clone, Debug, Default)]
pub struct OamEntry {
    // Note: this is the order of data expected by the NES
    /// Y position of sprite.
    pub y: u8,
    /// Tile ID in pattern memory.
    pub tile_id: u8,
    /// Defines how sprite should be rendered.
    pub attribute: u8,
    /// X position of sprite.
    pub x: u8,
}

impl From<[u8; 4]> for OamEntry {
    fn from(arr: [u8; 4]) -> Self {
        OamEntry {
            y: arr[0],
            tile_id: arr[1],
            attribute: arr[2],
            x: arr[3],
        }
    }
}

impl From<OamEntry> for [u8; 4] {
    fn from(entry: OamEntry) -> Self {
        [entry.y, entry.tile_id, entry.attribute, entry.x]
    }
}

#[test]
fn test_get_byte() {
    use itertools::Itertools;
    use std::convert::TryInto;

    let oam = Oam {
        mem: (0..=255)
            .chunks(4)
            .into_iter()
            .map(|chunk| chunk.collect::<Vec<_>>().try_into().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    };

    for i in 0..=255_u8 {
        assert_eq!(oam.get_byte(i), i);
    }
}

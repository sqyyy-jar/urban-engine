use std::io::{Cursor, Error, ErrorKind, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use clap::error::Result;

pub const MAGIC: [u8; 4] = *b"\0urb";
pub const EXECUTABLE: u32 = 0x1;

pub struct Header {
    pub magic: [u8; 4],
    pub flags: u32,
    pub entrypoint: u64,
}

impl Header {
    pub fn has_valid_magic(&self) -> bool {
        self.magic == MAGIC
    }

    pub fn is_executable(&self) -> bool {
        self.flags & EXECUTABLE != 0
    }

    pub fn read(bytes: &[u8]) -> Result<Self, Error> {
        let mut result = Self {
            magic: [0; 4],
            flags: 0,
            entrypoint: 0,
        };
        if bytes.len() < 4 {
            return Err(Error::new(ErrorKind::Other, "Invalid binary header"));
        }
        let mut cursor = Cursor::new(bytes);
        let mut magic = [0; 4];
        let Ok(_) = cursor.read_exact(&mut magic) else {
            return Err(Error::new(ErrorKind::Other, "Invalid binary header"));
        };
        result.magic = magic;
        let Ok(flags) = cursor.read_u32::<LittleEndian>() else {
            return Err(Error::new(ErrorKind::Other, "Invalid binary header"));
        };
        result.flags = flags;
        if result.is_executable() {
            let Ok(entrypoint) = cursor.read_u64::<LittleEndian>() else {
                return Err(Error::new(ErrorKind::Other, "Invalid binary header"));
            };
            result.entrypoint = entrypoint;
        }
        Ok(result)
    }
}

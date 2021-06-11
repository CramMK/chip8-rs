use std::fs::File;
use std::io::prelude::*;

const BUFFER_SIZE: usize = 3584;

pub struct Cartridge {
    pub rom: [u8; BUFFER_SIZE],
    pub size: usize,
}

impl Cartridge {
    pub fn new(filename: &str) -> Self {
        let mut file = File::open(filename).expect("Error while opening file!");
        let mut buffer = [0u8; BUFFER_SIZE];

        // either read a byte, or noting (0)
        let bytes = if let Ok(bytes) = file.read(&mut buffer) {
            bytes
        } else {
            0
        };

        Cartridge {
            rom: buffer,
            size: bytes,
        }
    }
}

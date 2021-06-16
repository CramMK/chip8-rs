use clap::{App, Arg};

mod cartridge;
mod display;
mod fontset;
mod input;
mod processor;

use crate::cartridge::Cartridge;
use crate::processor::Processor;

const MEMORY_SIZE: usize = 4096;
const GAME_ENTRY: usize = 0x200; // most games load into 0x200
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SCALE: usize = 20;

#[derive(Debug)]
enum ChipError {
    CartridgeNotFound,
}

fn main() -> Result<(), ChipError> {
    let app = App::new("chip8-rs")
        .version("0.1.0")
        .author("Marco Thomas <mail@marco-thomas.net>")
        .about("A small chip8 emulator, written in Rust")
        .arg(
            Arg::with_name("cartridge")
                .short("c")
                .takes_value(true)
                .help("A cartridge binary file"),
        )
        .get_matches();

    let game_file = app
        .value_of("cartridge")
        .ok_or(ChipError::CartridgeNotFound)?;

    let mut processor = Processor::new();

    let cartridge = Cartridge::new(&game_file);

    processor.start(&cartridge.rom);
    Ok(())
}

extern crate rand;
extern crate sdl2;

use std::env;

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

fn main() {
    let game_file = env::args().nth(1);
    match game_file {
        Some(_) => println!("Found a cartridge file! Trying to load..."),
        None => {
            println!("No cartridge file found! Exiting!");
            return;
        }
    };

    let mut processor = Processor::new();

    // load cartridge file
    let cartridge = Cartridge::new(&game_file.unwrap());

    processor.start(&cartridge.rom);
}

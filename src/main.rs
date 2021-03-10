extern crate rand;
extern crate sdl2;

use std::env;

mod processor;
mod fontset;

use crate::processor::Processor;

fn main() {
    let cartridge = env::args().nth(1);
    match cartridge {
        Some(_) => println!("Found a cartridge file! Trying to load..."),
        None => {
            println!("No cartridge file found! Exiting!");
            return;
        }
    };

    let mut processor = Processor::new();

    // load cartridge file

    processor.start();

}

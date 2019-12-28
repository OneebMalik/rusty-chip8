use std::env;
//use std::fs::File;
//use std::path::Path;
//use std::io::Read;

mod chip8;
mod cpu;

use chip8::Chip8;

fn main() {
    
    if env::args().nth(1) == None {
        panic!("No ROM filepath provided");
    }

    let rom_file_path = env::args().nth(1).unwrap();
    let mut chip8 = Chip8::new();

    chip8.load_rom(rom_file_path);
    chip8.run();
}

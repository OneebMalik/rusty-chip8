mod chip8;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn main() {
    let rom_file_path = env::args().nth(1).unwrap();

    let rom = read_bin(rom_file_path);

    println!("{:X?}", rom);
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();

    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}

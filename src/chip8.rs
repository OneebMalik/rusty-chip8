use super::cpu::Cpu;
use std::fs::File;
use std::path::Path;
use std::io::Read;

#[derive(Debug)]
pub struct Chip8 {
    ram: Vec<u8>,
    cpu: Cpu,
    pub rom: Box<[u8]>
}

impl Chip8 {
    pub fn new() -> Chip8 {
            Chip8 {
                ram: vec![0; 4096],
                cpu: Cpu::default(),
                rom: Box::default()
            }
    }

    pub fn run(&self) {
        // Iterate over every two bytes and run 16 bit instruction.
        //for x in (0..(self.rom.len()/2)).step_by(2) {

        for i in 0..50 {
            let instruction = (self.rom[i] as u16) << 8 | self.rom[i + 1] as u16;

            self.cpu.execute(instruction);
        }
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) {
        let mut file = File::open(path).unwrap();
        let mut file_buf = Vec::new();

        file.read_to_end(&mut file_buf).unwrap();
        self.rom = file_buf.into_boxed_slice()
    }
}

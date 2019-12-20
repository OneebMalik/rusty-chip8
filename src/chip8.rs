use super::cpu::Cpu;

#[derive(Debug)]
pub struct Chip8 {
    ram: Vec<u8>,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
            Chip8 {
                ram: vec![0; 4096],
                cpu: Cpu::default()
            }
    }
}


use super::cpu::Cpu;

pub struct Chip8 {
    ram: [u8; 4096],
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
            Chip8 {
                ram: [0; 4096],
                cpu: Cpu::default()
            }
    }
}

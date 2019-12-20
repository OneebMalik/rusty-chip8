use super::cpu::Cpu;

struct Chip8 {
    ram: [u8; 4096],
    cpu: Cpu
}

impl Chip8 {
    fn new() -> Chip8 {
            Chip8 {
                ram: [0; 4096],
                cpu: Cpu {}
            }
    }
}

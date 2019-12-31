use super::cpu::Cpu;
use std::fs::File;
use std::path::Path;
use std::io::Read;

const CHAR_SPRITES: [[u8; 5]; 16] = [
                                    /*0*/ [0xF0, 0x90, 0x90, 0x90, 0xF0],
                                    /*1*/ [0x20, 0x60, 0x20, 0x20, 0x70],
                                    /*2*/ [0xF0, 0x10, 0xF0, 0x80, 0xF0],
                                    /*3*/ [0xF0, 0x10, 0xF0, 0x10, 0xF0],
                                    /*4*/ [0x90, 0x90, 0xF0, 0x10, 0x10],
                                    /*5*/ [0xF0, 0x80, 0xF0, 0x10, 0xF0],
                                    /*6*/ [0xF0, 0x80, 0xF0, 0x90, 0xF0],
                                    /*7*/ [0xF0, 0x10, 0x20, 0x40, 0x40],
                                    /*8*/ [0xF0, 0x90, 0xF0, 0x90, 0xF0],
                                    /*9*/ [0xF0, 0x90, 0xF0, 0x10, 0xF0],
                                    /*A*/ [0xF0, 0x90, 0xF0, 0x90, 0x90],
                                    /*B*/ [0xE0, 0x90, 0xE0, 0x90, 0xE0],
                                    /*C*/ [0xF0, 0x80, 0x80, 0x80, 0xF0],
                                    /*D*/ [0xE0, 0x90, 0x90, 0x90, 0xE0],
                                    /*E*/ [0xF0, 0x80, 0xF0, 0x80, 0xF0],
                                    /*F*/ [0xF0, 0x80, 0xF0, 0x80, 0x80]
                                    ];

#[derive(Debug)]
pub struct Chip8 {
    ram: Vec<u8>,
    cpu: Cpu,
    rom_size: usize
}

impl Chip8 {
    pub fn new() -> Chip8 {
            Chip8 {
                ram: vec![0; 4096],
                cpu: Cpu::default(),
                rom_size: 0
            }
    }

    pub fn run(&self) {
        println!("{:#X?}", CHAR_SPRITES);
        for i in (0x200..self.rom_size).step_by(2) {
            let instruction = (self.ram[i] as u16) << 8 | self.ram[i + 1] as u16;
            self.cpu.execute(instruction);
        }
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) {
        let mut file = File::open(path).unwrap();
        let mut file_buf = Vec::new();

        file.read_to_end(&mut file_buf).unwrap();

        self.rom_size = file_buf.len();

        //println!("ROM SIZE: {}", self.rom_size);

        for (index, _) in file_buf.iter().enumerate() {
            self.ram[0x200+index] = file_buf[index];
        }

        //println!("RAM: {:X?}, {:X?}, {:X?}, {:X?}", self.ram[0x702], self.ram[0x703], self.ram[0x704], self.ram[0x203]);

        //for i in 0x200..0x703 {
        //    print!("{:X?}: {:X?}\t", i, self.ram[i]);
        //}

        //println!("\n");
    }
}

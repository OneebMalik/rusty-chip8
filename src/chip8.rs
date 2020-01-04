use super::cpu::Cpu;
use std::fs::File;
use std::path::Path;
use std::io::Read;

const CHAR_SPRITES: [[u8; 5]; 16] = [
    /*0*/ [0xf0, 0x90, 0x90, 0x90, 0xf0],
    /*1*/ [0x20, 0x60, 0x20, 0x20, 0x70],
    /*2*/ [0xf0, 0x10, 0xf0, 0x80, 0xf0],
    /*3*/ [0xf0, 0x10, 0xf0, 0x10, 0xf0],
    /*4*/ [0x90, 0x90, 0xf0, 0x10, 0x10],
    /*5*/ [0xf0, 0x80, 0xf0, 0x10, 0xf0],
    /*6*/ [0xf0, 0x80, 0xf0, 0x90, 0xf0],
    /*7*/ [0xf0, 0x10, 0x20, 0x40, 0x40],
    /*8*/ [0xf0, 0x90, 0xf0, 0x90, 0xf0],
    /*9*/ [0xf0, 0x90, 0xf0, 0x10, 0xf0],
    /*a*/ [0xf0, 0x90, 0xf0, 0x90, 0x90],
    /*b*/ [0xe0, 0x90, 0xe0, 0x90, 0xe0],
    /*c*/ [0xf0, 0x80, 0x80, 0x80, 0xf0],
    /*d*/ [0xe0, 0x90, 0x90, 0x90, 0xe0],
    /*e*/ [0xf0, 0x80, 0xf0, 0x80, 0xf0],
    /*f*/ [0xf0, 0x80, 0xf0, 0x80, 0x80]
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

    pub fn start(&mut self) {
        for addr in (0x200..self.rom_size).step_by(2) {
            //let instruction = (self.ram[i] as u16) << 8 | self.ram[i + 1] as u16;
            self.cpu.pc = addr as u16;
            self.cpu.execute(&self.ram);
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

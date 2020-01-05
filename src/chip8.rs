use super::cpu;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use super::display;

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
    cpu: cpu::Cpu,
    rom_size: usize,
    display: display::Display
}

impl Chip8 {
    pub fn new() -> Chip8 {
            Chip8 {
                ram: vec![0; 4096],
                cpu: cpu::Cpu::default(),
                rom_size: 0,
                display: display::Display::draw()
            }
    }

    pub fn start(&mut self) {
       self.display.event_loop();
       self.display.draw_sprite(CHAR_SPRITES[0].to_vec(), 5, 5);
        loop {
            self.cpu.execute(&self.ram);
            self.cpu.pc += 2;
        }
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) {
        let mut file = File::open(path).unwrap();
        let mut file_buf = Vec::new();

        file.read_to_end(&mut file_buf).unwrap();

        self.rom_size = file_buf.len();

        for (index, _) in file_buf.iter().enumerate() {
            self.ram[0x200+index] = file_buf[index];
        }

        //for i in 0x200..0x703 {
        //    print!("{:X?}: {:X?}\t\t", i, self.ram[i]);
        //}
    }
}

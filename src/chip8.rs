use std::fs::File;
use std::path::Path;
use std::io::Read;
// use std::fmt;

use std::process;

use super::cpu;
use super::display;

use sdl2::pixels::Color;

// use std::collections::VecDeque;

use std::{thread, time};

const PROGRAM_START_ADDR: u16 = 0x200;

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

        self.cpu.ld_reg = -1;

        // TODO: move to cpu.rs
        self.cpu.pc = PROGRAM_START_ADDR;

        'main: loop {
            if self.display.event_poll() == "quit" {
                break 'main;
            }

            if self.cpu.sprite_queued {
               self.display.draw_sprite(self.cpu.sprite_buffer.pop_front().unwrap());
               self.cpu.sprite_queued = false;
            }

            self.cpu.execute(&mut self.ram);

            self.cpu.pc += 2;

            if self.cpu.dt > 0 {
                self.cpu.dt -= 1;
            };

            // TODO: Add to flags struct
            if self.cpu.cls {
                self.display.canvas.set_draw_color(Color::RGB(0, 0, 0));
                self.display.canvas.clear();
                self.display.canvas.present();

                self.cpu.cls = false;
            }

            if self.display.collision {
                self.cpu.vx[15] = 1;
            } else {
                self.cpu.vx[15] = 0;
            }

            if self.cpu.ld_reg != -1 {
                for index in 0..self.cpu.ld_reg {
                    self.cpu.vx[index as usize] = self.ram[(self.cpu.i + index as u16) as usize];
                }

                self.cpu.i = self.cpu.i + self.cpu.ld_reg as u16 + 1;

                for i in 0x25A..0x300 {
           print!("{:X?}: {:X?}\t\t", i, self.ram[i]);
        }

            println!("{:X?}", self.cpu);

            // process::exit(0);

                self.cpu.ld_reg = -1;
            }

            let frame_delay = time::Duration::from_micros(5000);

            thread::sleep(frame_delay);
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

        // for i in 0x200..0x703 {
        //    print!("{:X?}: {:X?}\t\t", i, self.ram[i]);
        // }
    }
}

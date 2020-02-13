// BIG TODO CHANGE KEYPRESS TO KEYDOWN AND KEYUP. CHECK COWGOD.

use rand;
use rand::Rng;

use super::sprite;

use std::collections::VecDeque;

const START_ADDR: u16 = 0x200;

#[derive(Default, Debug)]
pub struct Cpu {
    // V0-VF registers. VF register reserved for instruction flags.
    pub vx: [u8; 16],
    // index pointer
    pub i: u16,
    // program counter
    pub pc: u16,
    // stack pointer
    sp: usize,
    // delay timer
    pub dt: u8,
    // sound timer
    st: u8,
    // program counter
    stack: [u16; 16],
    //sprite queued
    pub sprite_queued: bool,
    // sprite buffer
    pub sprite_buffer: VecDeque<sprite::Sprite>,
    //clear screen flag
    pub cls: bool,
    // LD Vx, [I]
    pub ld_reg: i8,
    // LD [I], Vx
    pub ld_i: i8,
    // Flag for setting index register to associated font
    pub load_font: i8,
    pub key_pressed: i8
}

impl Cpu {

    pub fn execute(&mut self, ram: &mut Vec<u8>) {

        let instruction = (ram[self.pc as usize] as u16) << 8 | ram[(self.pc + 1)  as usize] as u16;

        println!("\ninstruction(0x{:X}): 0x{:X}", self.pc, instruction);

        let nnn = instruction & 0x0FFF;
        let n = (instruction & 0x000F) as u8;
        let x = (instruction >> 8 & 0x000F) as u8;
        let y = (instruction >> 4 & 0x000F) as u8;
        let kk = (instruction & 0x00FF) as u8;

        let opcode = instruction >> 12 & 0x000F;

        println!("nnn: {:X}, n: {:X}, x: {:X}, y: {:X}, kk: {:X}", nnn, n, x, y, kk);

        match opcode {
            0x0 => {
                match instruction {
                    // CLS
                    0x00E0 => {
                        self.cls = true;
                        println!("CLS");
                    },
                    // RET
                    0x00EE => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp];
                        println!("RET");
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0x1 => {
                // JP addr
                self.pc = nnn as u16;

                self.pc -= 2;

                println!("JP addr");
            },
            0x2 => {
                //CALL
                self.stack[self.sp] = self.pc;
                self.sp += 1;

                self.pc = nnn as u16;

                self.pc -= 2;
                println!("CALL addr");
            },
            0x3 => {
                // SE Vx, byte
                println!("SE Vx, byte");
                if self.vx[x as usize] == kk {
                    self.pc += 2;
                }
            },
            0x4 => {
                // SNE Vx, byte
                if self.vx[x as usize] != kk {
                    self.pc += 2;
                }
                println!("SNE Vx, byte");
            },
            0x5 => {
                match n {
                    // SE Vx, Vy
                    0x0 => {
                        if self.vx[x as usize] == self.vx[y as usize] {
                            self.pc += 2;
                        }
                        println!("SE Vx, Vy");
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0x6 => {
                // LD Vx, byte
                self.vx[x as usize] = kk;
                println!("LD Vx, byte");
            },
            0x7 => {
                // ADD Vx, byte
                self.vx[x as usize] = self.vx[x as usize].wrapping_add(kk);
                println!("ADD Vx, byte");
            },
            0x8 => {
                // OR Vx, Vy
                match n {
                    // LD Vx, Vy
                    0x0 => {
                        self.vx[x as usize] = self.vx[y as usize];
                        println!("LD Vx, Vy")
                    },
                    // OR Vx, Vy
                    0x1 => {
                        self.vx[x as usize] = self.vx[x as usize] | self.vx[y as usize];
                        println!("OR Vx, Vy");
                    },
                    // AND Vx, Vy
                    0x2 => {
                        self.vx[x as usize] = self.vx[x as usize] & self.vx[y as usize];
                        println!("AND Vx, Vy")
                    },
                    // XOR Vx, Vy
                    0x3 => {
                        self.vx[x as usize] = self.vx[x as usize] ^ self.vx[y as usize];
                        println!("XOR Vx, Vy");
                    },
                    // ADD Vx, Vy
                    0x4 => {
                        let add_tuple = self.vx[x as usize].overflowing_add(self.vx[y as usize]);
                        self.vx[x as usize] = add_tuple.0;
                        if add_tuple.1 {
                            self.vx[0xF] = 1;
                        } else {
                            self.vx[0xF] = 0;
                        }
                        println!("ADD Vx, Vy");
                    },
                    // SUB Vx, Vy
                    0x5 => {
                        let sub_tuple = self.vx[x as usize].overflowing_sub(self.vx[y as usize]);
                        self.vx[x as usize] = sub_tuple.0;
                        if sub_tuple.1 {
                            self.vx[0xF] = 0;
                        } else {
                            self.vx[0xF] = 1;
                        }
                        // self.vx[x as usize] = self.vx[x as usize].wrapping_sub(self.vx[y as usize]);
                        println!("SUB Vx, Vy");
                    },
                    // SHR Vx {, Vy}
                    0x6 => {
                        if self.vx[y as usize] & 0x1 == 1 {
                            self.vx[0xF] = 1;
                        } else {
                            self.vx[0xF] = 0; 
                        }
                        self.vx[y as usize] = self.vx[y as usize] >> 1;
                        // self.vx[x as usize] = self.vx[y as usize] >> 1;
                        self.vx[x as usize] = self.vx[y as usize];
                        println!("SHR Vx {{, Vy}}");
                    },
                    // SUBN Vx, Vy
                    0x7 => {
                        panic!("SUBN Vx, Vy")
                    },
                    // SHL Vx {, Vy}
                    0xE => {
                        if self.vx[y as usize] >> 7 & 0x1 == 1 {
                            self.vx[0xF] = 1;
                        } else {
                            self.vx[0xF] = 0; 
                        }
                        self.vx[y as usize] = self.vx[y as usize] << 1;
                        // self.vx[x as usize] = self.vx[y as usize] << 1;
                        self.vx[x as usize] = self.vx[y as usize];
                        println!("SHL Vx {{, Vy}}");
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0x9 => {
                match n {
                    // SNE Vx, Vy
                    0x0 => {
                        if self.vx[x as usize] != self.vx[y as usize] {
                            self.pc += 2;
                        }
                        println!("SNE Vx, Vy");
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0xa => {
                // LD I, addr
                println!("LD I, addr");
                self.i = nnn;
            },
            0xb => {
                // JP V0, addr
                self.pc = nnn + self.vx[0] as u16;
                self.pc -= 2;
                println!("JP V0, addr");
            },
            0xc => {
                // RND Vx, byte
                let mut rng = rand::thread_rng();
                self.vx[x as usize] = rng.gen::<u8>() & kk;
                println!("RND Vx, byte");
            },
            0xd => {
                //DRW Vx, Vy, nibble
                println!("DRW Vx, Vy, nibble");

                let sprite_size = n;

                let mut sprite: Vec<u8> = vec![0; n as usize];

                // Read 'n' bytes from memory starting from address in I register.
                for index in 0..sprite_size {
                    sprite[index as usize] = ram[(self.i + index as u16) as usize];
                }

                self.sprite_buffer.push_back(sprite::Sprite {
                    data: sprite,
                    x: self.vx[x as usize] as i32,
                    y: self.vx[y as usize] as i32
                });
                
                self.sprite_queued = true;
            },
            0xe => {
                match kk {
                    // SKP Vx
                    0x9E => {
                        if self.key_pressed == self.vx[x as usize] as i8 {
                            self.pc += 2;
                            self.key_pressed = -1;
                        }

                        println!("SKP Vx");
                    },
                    // SKNP Vx
                    0xA1 => {
                        if self.key_pressed != self.vx[x as usize] as i8 {
                            self.pc += 2;
                        }
                        
                        println!("SKNP Vx");
                    },
                    _ => panic!("Invalid instruction (0x{:X}).", instruction)
                }
            },
            0xf => {
                match kk {
                    // LD Vx, DT 
                    0x07 => {
                        self.vx[x as usize] = self.dt;
                        println!("LD Vx, DT");
                    },
                    // LD Vx, K
                    // TODO event poll until key press. dont move forward in execution
                    0x0A => {
                        if self.key_pressed == -1 {
                            self.pc -= 2;
                        } else {
                            self.vx[x as usize] = self.key_pressed as u8;
                            self.key_pressed = -1;
                        }
                        panic!("LD Vx, K");
                    },
                    // LD DT, Vx
                    0x15 => {
                        self.dt = self.vx[x as usize];
                        println!("LD DT, Vx");
                    },
                    // LD ST, Vx
                    0x18 => {
                        self.st = self.vx[x as usize];
                        println!("LD ST, Vx")
                    },
                    // ADD I, Vx
                    0x1E => {
                        self.i = self.i.wrapping_add(self.vx[x as usize] as u16);
                        println!("ADD I, Vx");
                    },
                    // LD F, Vx
                    0x29 => {
                        self.load_font = self.vx[x as usize] as i8;
                        println!("LD F, Vx");
                    },
                    // LD B, Vx
                    0x33 => {
                        // let self.font = self.vx[x as usize];
                        // TODO THE WHOLE THING
                        panic!("LD B, Vx");
                    },
                    // LD [I], Vx
                    0x55 => {
                        self.ld_i = x as i8;
                        println!("LD [I], Vx");
                    },
                    // LD Vx, [I]
                    0x65 => {
                        self.ld_reg = x as i8;
                        println!("LD Vx, [I]");
                    },
                    _ => panic!("Invalid instruction (0x{:X}).", instruction)
                }
            },
            _ => panic!("Invalid instruction (0x{:X}).", instruction)
        }

        println!("I: {:X}, VX: {:X?} \n", self.i, self.vx);
    }
}

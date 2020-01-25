use super::sprite;

use std::collections::VecDeque;

const START_ADDR: u16 = 0x200;

#[derive(Default, Debug)]
pub struct Cpu {
    // V0-VF registers. VF register reserved for instruction flags.
    vx: [u8; 16],
    // index pointer
    i: u16,
    // program counter
    pub pc: u16,
    // stack pointer
    sp: usize,
    // delay timer
    dt: u8,
    // sound timer
    st: u8,
    // program counter
    stack: [u16; 16],
    //sprite queued
    pub sprite_queued: bool,
    // sprite buffer
    pub sprite_buffer: VecDeque<sprite::Sprite>
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
                        panic!("CLS")
                    },
                    // RET
                    0x00EE => {
                        panic!("RET")
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0x1 => {
                // JP addr
                // println!("JP addr");
                // self.stack[self.sp] = self.pc;
                // self.sp += 1;
                self.pc = nnn as u16;

                // self.pc -= 2;

                println!("JP addr");
                // self.execute(ram);
            },
            0x2 => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                println!("CALL addr");
            },
            0x3 => {
                // SE Vx, byte
                println!("SE Vx, byte");
                if (self.vx[x as usize] == kk) {
                    self.pc += 2;
                }
            },
            0x4 => {
                // SNE Vx, byte
                panic!("SNE Vx, byte")
            },
            0x5 => {
                match n {
                    // SE Vx, Vy
                    0x0 => {
                        panic!("SE Vx, Vy")
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0x6 => {
                // LD Vx, byte
                self.vx[x as usize] = kk;
                println!("LD Vx, byte {:x?}", self.vx);
            },
            0x7 => {
                // ADD Vx, byte
                self.vx[x as usize] = self.vx[x as usize] + kk;
                println!("ADD Vx, byte");
            },
            0x8 => {
                // OR Vx, Vy
                match n {
                    // LD Vx, Vy
                    0x0 => {
                        panic!("LD Vx, Vy")
                    },
                    // OR Vx, Vy
                    0x1 => {
                        panic!("OR Vx, Vy")
                    },
                    // AND Vx, Vy
                    0x2 => {
                        panic!("ADD Vx, Vy")
                    },
                    // XOR Vx, Vy
                    0x3 => {
                        panic!("XOR Vx, Vy")
                    },
                    // ADD Vx, Vy
                    0x4 => {
                        panic!("ADD Vx, Vy")
                    },
                    // SUB Vx, Vy
                    0x5 => {
                        panic!("SUB Vx, Vy")
                    },
                    // SHR Vx {, Vy}
                    0x6 => {
                        panic!("SHR Vx {{, Vy}}")
                    },
                    // SUBN Vx, Vy
                    0x7 => {
                        panic!("SUBN Vx, Vy")
                    },
                    // SHL Vx {, Vy}
                    0xE => {
                        panic!("SHL Vx {{, Vy}}")
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0x9 => {
                match n {
                    // SNE Vx, Vy
                    0x0 => {
                        panic!("SNE Vx, Vy")
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
                panic!("JP V0, addr");
            },
            0xc => {
                // RND Vx, byte
                panic!("RND Vx, byte");
            },
            0xd => {
                //DRW Vx, Vy, nibble
                println!("DRW Vx, Vy, nibble");

                let sprite_size = n;

                let mut sprite: Vec<u8> = vec![0; n as usize];

                // Read 'n' bytes from memory starting from address in I register.
                for index in 0..sprite_size-1 {
                    sprite[index as usize] = ram[(self.i + index as u16) as usize];
                }

                self.sprite_buffer.push_back(sprite::Sprite {
                    data: sprite,
                    x: x as i32,
                    y: y as i32
                });
                
                self.sprite_queued = true;
            },
            0xe => {
                match kk {
                    // SKP Vx
                    0x9E => {
                        panic!("SKP Vx")
                    },
                    // SKNP Vx
                    0xA1 => {
                        panic!("SKNP Vx")
                    },
                    _ => panic!("Invalid instruction (0x{:X}).", instruction)       
                }
            },
            0xf => {
                match kk {
                    // LD Vx, DT 
                    0x07 => {
                        panic!("LD Vx, DT")
                    },
                    // LD Vx, K
                    0x0A => {
                        panic!("LD Vx, K")
                    },
                    // LD DT, Vx
                    0x15 => {
                        panic!("LD DT, Vx")
                    },
                    // LD ST, Vx
                    0x18 => {
                        panic!("LD ST, Vx")
                    },
                    // ADD I, Vx
                    0x1E => {
                        println!("ADD I, Vx");
                        self.i = self.i + self.vx[x as usize] as u16;
                        println!("Vx: {:X?}", self.vx);
                    },
                    // LD F, Vx
                    0x29 => {
                        panic!("LD F, Vx")
                    },
                    // LD B, Vx
                    0x33 => {
                        panic!("LD B, Vx")
                    },
                    // LD [I], Vx
                    0x55 => {
                        panic!("LD [I], Vx")
                    },
                    // LD Vx, [I]
                    0x65 => {
                        panic!("LD Vx, [I]")
                    },
                    _ => panic!("Invalid instruction (0x{:X}).", instruction)
                }
            },
            _ => panic!("Invalid instruction (0x{:X}).", instruction)
        }

        //self.pc += 2;
        println!("\n");
    }
}

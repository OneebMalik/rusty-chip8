#[derive(Default, Debug)]
pub struct Cpu {
    // V0-VF registers. VF register reserved for instruction flags.
    vx: [u8; 16],
    // index pointer
    i: u16,
    // program counter
    pc: u16,
    // stack pointer
    sp: u8,
    // delay timer
    dt: u8,
    // sound timer
    st: u8,
    // program counter
    stack: [u16; 16]
}

impl Cpu {
    pub fn execute(&self, instruction: u16) {
        println!("instruction: 0x{:X}", instruction);

        let nnn = instruction & 0x0FFF;
        let n = instruction & 0x000F;
        let x = instruction >> 8 & 0x000F;
        let y = instruction >> 4 & 0x000F;
        let kk = instruction & 0x00FF;

        let opcode = instruction >> 12 & 0x000F;

        println!("nnn: {:X}, n: {:X}, x: {:X}, y: {:X}, kk: {:X}", nnn, n, x, y, kk);

        match opcode {
            0x0 => {
                match instruction {
                    // CLS
                    0x00E0 => {
                        println!("CLS")
                    },
                    // RET
                    0x00EE => {
                        println!("RET")
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0x1 => {
                // JP addr
                println!("JP addr")
            },
            0x2 => {
                // CALL addr
                println!("CALL addr")
            },
            0x3 => {
                // SE Vx, byte
                println!("SE Vx, byte")
            },
            0x4 => {
                // SNE Vx, byte
                println!("SNE Vx, byte")
            },
            0x5 => {
                match n {
                    // SE Vx, Vy
                    0x0 => {
                        println!("SE Vx, Vy")
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0x6 => {
                // LD Vx, byte
                println!("LD Vx, byte");
            },
            0x7 => {
                // ADD Vx, byte
                println!("ADD Vx, byte");
            },
            0x8 => {
                // OR Vx, Vy
                match n {
                    // LD Vx, Vy
                    0x0 => {
                        println!("LD Vx, Vy")
                    },
                    // OR Vx, Vy
                    0x1 => {
                        println!("OR Vx, Vy")
                    },
                    // AND Vx, Vy
                    0x2 => {
                        println!("ADD Vx, Vy")
                    },
                    // XOR Vx, Vy
                    0x3 => {
                        println!("XOR Vx, Vy")
                    },
                    // ADD Vx, Vy
                    0x4 => {
                        println!("ADD Vx, Vy")
                    },
                    // SUB Vx, Vy
                    0x5 => {
                        println!("SUB Vx, Vy")
                    },
                    // SHR Vx {, Vy}
                    0x6 => {
                        println!("SHR Vx {{, Vy}}")
                    },
                    // SUBN Vx, Vy
                    0x7 => {
                        println!("SUBN Vx, Vy")
                    },
                    // SHL Vx {, Vy}
                    0xE => {
                        println!("SHL Vx {{, Vy}}")
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0x9 => {
                match n {
                    // SNE Vx, Vy
                    0x0 => {
                        println!("SNE Vx, Vy")
                    },
                    _ => panic!("Invalid instruction (0x{:X})", instruction)
                }
            },
            0xa => {
                // LD I, addr
                println!("LD I, addr");
            },
            0xb => {
                // JP V0, addr
                println!("JP V0, addr");
            },
            0xc => {
                // RND Vx, byte
                println!("RND Vx, byte");
            },
            0xd => {
                // DRW Vx, Vy, nibble
                println!("DRW Vx, Vy, nibble");
            },
            0xe => {
                match kk {
                    // SKP Vx
                    0x9E => {
                        println!("SKP Vx")
                    },
                    // SKNP Vx
                    0xA1 => {
                        println!("SKNP Vx")
                    },
                    _ => panic!("Invalid instruction (0x{:X}).", instruction)       
                }
            },
            0xf => {
                match kk {
                    // LD Vx, DT 
                    0x07 => {
                        println!("LD Vx, DT")
                    },
                    // LD Vx, K
                    0x0A => {
                        println!("LD Vx, K")
                    },
                    // LD DT, Vx
                    0x15 => {
                        println!("LD DT, Vx")
                    },
                    // LD ST, Vx
                    0x18 => {
                        println!("LD ST, Vx")
                    },
                    // ADD I, Vx
                    0x1E => {
                        println!("ADD I, Vx")
                    },
                    // LD F, Vx
                    0x29 => {
                        println!("LD F, Vx")
                    },
                    // LD B, Vx
                    0x33 => {
                        println!("LD B, Vx")
                    },
                    // LD [I], Vx
                    0x55 => {
                        println!("LD [I], Vx")
                    },
                    // LD Vx, [I]
                    0x65 => {
                        println!("LD Vx, [I]")
                    },
                    _ => panic!("Invalid instruction (0x{:X}).", instruction)
                }
            },
            _ => panic!("Invalid instruction (0x{:X}).", instruction)
        }

        println!("\n");
    }
}

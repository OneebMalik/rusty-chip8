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
                println!("opcode is: 9"); 
            },
            0xa => {
                println!("opcode is: A");
            },
            0xb => {
                println!("opcode is: B"); 
            },
            0xc => {
                println!("opcode is: C"); 
            },
            0xd => {
                println!("opcode is: D"); 
            },
            0xe => {
                println!("opcode is: E"); 
            },
            0xf => {
                println!("opcode is: F"); 
            },
            _ => panic!("Invalid instruction (0x{:X}).", instruction)
        }

        println!("\n");
    }
}

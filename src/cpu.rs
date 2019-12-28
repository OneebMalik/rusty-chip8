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
                println!("opcode is: 0"); 
            },
            0x1 => {
                println!("opcode is: 1"); 
            },
            0x2 => {
                println!("opcode is: 2");
            },
            0x3 => {
                println!("opcode is: 3");
            },
            0x4 => {
                println!("opcode is: 4"); 
            },
            0x5 => {
                println!("opcode is: 5"); 
            },
            0x6 => {
                println!("opcode is: 6"); 
            },
            0x7 => {
                println!("opcode is: 7"); 
            },
            0x8 => {
                println!("opcode is: 8"); 
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

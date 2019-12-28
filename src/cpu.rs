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

        println!("nnn: {:X}, n: {:X}, x: {:X}, y: {:X}, kk: {:X}", nnn, n, x, y, kk);

    }
}

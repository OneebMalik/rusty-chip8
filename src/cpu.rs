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

//fn read_byte(u8) {
    
//}

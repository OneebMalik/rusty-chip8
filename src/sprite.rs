use std::fmt;

pub struct Sprite {
    pub data: Vec<u8>,
    pub x: i32,
    pub y: i32
}

impl fmt::Debug for Sprite {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ data: {:X?}, x: {}, y: {} }}", self.data, self.x, self.y)
    }
}
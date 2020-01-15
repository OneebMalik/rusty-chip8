use std::fmt;

pub struct Sprite {
    data: Vec<u8>,
    x: i32,
    y: i32
}

impl fmt::Debug for Sprite {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sprite")
    }
}
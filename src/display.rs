extern crate sdl2;

use std::fmt;

// use std::collections::VecDeque;

use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::EventPump;

use super::sprite;

//use std::{thread, time};

const WINDOW_WIDTH: u32 = 64;
const WINDOW_HEIGHT: u32 = 32;

const SCALE_FACTOR: u32 = 20;

pub struct Display {
    pub canvas: Canvas<Window>,
    event_pump: EventPump,
    sdl_context: Sdl
}

impl Display {
    pub fn draw() -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
     
        let window = video_subsystem.window("Rusty CHIP-8", WINDOW_WIDTH * SCALE_FACTOR, WINDOW_HEIGHT * SCALE_FACTOR)
            .position_centered()
            .build()
            .unwrap();
     
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
     
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        Display {
            canvas,
            event_pump,
            sdl_context
        }
    }

    pub fn event_poll(&mut self) -> &str {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return "quit"
                },
                _ => return ""
            };
        }

        ""
    }

    pub fn draw_sprite(&mut self, sprite: sprite::Sprite) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        println!("SPRITE DATA: {:X?}", sprite.data);

        // TODO: Wrap around with mod and XOR and collisions.

        println!("\n\n\n {:X?} \n\n\n", self.canvas.surface.raw());

        for (index, val) in sprite.data.iter().enumerate() {
            for bit in 0..8 {
                println!("x, y: {}, {:X}", bit as i32 + sprite.x, index as i32 + sprite.y);
                if (val >> (7 - bit) ) as u8 & 0x01 == 1u8 {
                    
                    let bit_x = ( (bit as i32 + sprite.x) % WINDOW_WIDTH as i32 );
                    let bit_y = ( (index as i32 + sprite.y) % WINDOW_HEIGHT as i32 );

                    self.canvas.fill_rect(Rect::new(bit_x * SCALE_FACTOR as i32,
                                                    bit_y * SCALE_FACTOR as i32, 
                                                    SCALE_FACTOR as u32, 
                                                    SCALE_FACTOR as u32)
                    );
                }
            }
        }

        self.canvas.present();
    }

    pub fn test(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        self.canvas.fill_rect(Rect::new(32*SCALE_FACTOR as i32, 16*SCALE_FACTOR as i32, SCALE_FACTOR as u32, SCALE_FACTOR as u32));
        self.canvas.present();
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Display")
    }
}

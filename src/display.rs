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
    sdl_context: Sdl,
    vram: Box<[u8]>,
    pub collision: bool
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

        let event_pump = sdl_context.event_pump().unwrap();

        Display {
            canvas,
            event_pump,
            sdl_context,
            vram: vec![0u8; WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize].into_boxed_slice(),
            collision: false
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

        // println!("\n\n\n {:X?} \n\n\n", self.vram);
        println!("\n\n\n {:X?} \n\n\n", sprite);
        for (index, val) in sprite.data.iter().enumerate() {
            for bit in 0..8 {
                self.draw_pixel(bit, index, &sprite, val);
            }
        }

        self.canvas.present();
    }

    pub fn draw_pixel(&mut self, pos_x: usize, pos_y: usize, sprite: &sprite::Sprite, val: &u8) {
        let canvas_x = ( (pos_x as i32 + sprite.x) % WINDOW_WIDTH as i32 );
        let canvas_y = ( (pos_y as i32 + sprite.y) % WINDOW_HEIGHT as i32 );

        if (val >> (7 - pos_x) ) as u8 & 0x01 == 1u8 {
            if self.vram[((WINDOW_WIDTH * canvas_y as u32) + canvas_x as u32) as usize] == 1 {
                self.collision = true;

                self.canvas.set_draw_color(Color::RGB(0, 0, 0));

                self.canvas.fill_rect(Rect::new(canvas_x * SCALE_FACTOR as i32,
                    canvas_y * SCALE_FACTOR as i32, 
                    SCALE_FACTOR as u32, 
                    SCALE_FACTOR as u32)
                );

                self.vram[((WINDOW_WIDTH * canvas_y as u32) + canvas_x as u32) as usize] = 0;
                return;
            } else {
                // self.collision = false;

                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
    
                self.canvas.fill_rect(Rect::new(canvas_x * SCALE_FACTOR as i32,
                                                canvas_y * SCALE_FACTOR as i32, 
                                                SCALE_FACTOR as u32, 
                                                SCALE_FACTOR as u32)
                );
    
                self.vram[((WINDOW_WIDTH * canvas_y as u32) + canvas_x as u32) as usize] = 1;
            }
        }
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Display")
    }
}

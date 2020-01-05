use std::fmt;

use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

const WINDOW_WIDTH: u32 = 64;
const WINDOW_HEIGHT: u32 = 32;

const SCALE_FACTOR: u32 = 20;

pub struct Display {
    canvas: Canvas<Window>,
    context: Sdl
}

impl Display {
    pub fn draw() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
     
        let window = video_subsystem.window("Rusty CHIP-8", WINDOW_WIDTH * SCALE_FACTOR, WINDOW_HEIGHT * SCALE_FACTOR)
            .position_centered()
            .build()
            .unwrap();
     
        let mut canv = window.into_canvas().build().unwrap();
     
        canv.set_draw_color(Color::RGB(0, 0, 0));
        canv.clear();

        Display {
            canvas: canv,
            context: sdl_context
        }
    }

    pub fn event_loop(&self) { 
        let mut event_pump = self.context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn draw_sprite(&mut self, sprite: Vec<u8>, x: i32, y: i32) {
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

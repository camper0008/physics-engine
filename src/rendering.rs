extern crate sdl2;

// use sdl2::*;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use std::time::Duration;

use crate::{Rectangle, RigidBody};

pub struct RenderingContext {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl RenderingContext {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();
        RenderingContext { canvas, event_pump }
    }
    pub fn set_draw_color(&mut self, color: sdl2::pixels::Color) {
        self.canvas.set_draw_color(color);
    }
    pub fn fill_rect(&mut self, rect: &Rectangle) {
        self.canvas
            .fill_rect(sdl2::rect::Rect::new(
                (rect.pos().x - rect.width * 0.5) as i64 as i32,
                (600.0 - (rect.pos().y - rect.height * 0.5)) as i64 as i32,
                (rect.width) as u64 as u32,
                (rect.height) as u64 as u32,
            ))
            .unwrap();
    }
    pub fn clear(&mut self) {
        self.canvas.clear();
    }
    pub fn events(&mut self) -> sdl2::event::EventPollIterator {
        self.event_pump.poll_iter()
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }
}

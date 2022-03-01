mod rigid_body;
mod scene;
mod vector2;

use crate::rigid_body::{Rectangle, RigidBody};
use crate::scene::Scene;
use crate::vector2::Vector2;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

pub fn main() {
    let scene = Scene {
        gravity: Vector2::new(0.0, 9.82),
        pressure: 101325.0,  // 1atm in pascal
        temperature: 293.15, // 20c in kelvin
    };
    let mut rect = Rectangle {
        m_drag_coefficient: 2.1, // magic number
        m_mass: 10.0,
        m_pos: Vector2::new(50.0, 550.0),
        m_vel: Vector2::new(10.0, 0.0),
        m_owner: scene,
        m_time_fallen: 0.0,
        width: 10.0,
        height: 10.0,
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(000, 000, 255));
        canvas
            .fill_rect(Rect::new(
                (rect.pos().x - rect.width * 0.5) as i64 as i32,
                (600.0 - (rect.pos().y - rect.height * 0.5)) as i64 as i32,
                (rect.width) as u64 as u32,
                (rect.height) as u64 as u32,
            ))
            .unwrap();

        rect.tick(1.0 / 60.0);

        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // fancy way of saying 1 / 60 seconds
    }
}

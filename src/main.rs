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

const CANVAS_WIDTH: f64 = 1000.0;
const CANVAS_HEIGHT: f64 = 1000.0;
const PIXEL_PER_METER: f64 = 1.0;

pub fn main() {
    let mut paused = false;

    let scene = Scene {
        gravity: Vector2::new(0.0, -9.82),
        pressure: 101325.0,  // 1atm in pascal
        temperature: 293.15, // 20c in kelvin
    };
    let mut rect = Rectangle {
        m_drag_coefficient: 1.4, // magic number
        m_mass: 1.0,
        m_pos: Vector2::new(
            CANVAS_WIDTH * 0.5 / PIXEL_PER_METER,
            CANVAS_HEIGHT * 0.9 / PIXEL_PER_METER,
        ),
        m_vel: Vector2::new(1.0, 00.0),
        m_owner: scene,
        m_time_fallen: 0.0,
        width: 100.0,
        height: 100.0,
    };

    /*
    let mut rect2 = Rectangle {
        m_drag_coefficient: 2.1, // magic number
        m_mass: 0.1,
        m_pos: Vector2::new(5.0, CANVAS_HEIGHT * 0.9 / PIXEL_PER_METER),
        m_vel: Vector2::new(1.0, 0.0),
        m_owner: scene,
        m_time_fallen: 0.0,
        width: 100.0,
        height: 100.0,
    };
    */

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32)
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
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => paused = !paused,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(000, 000, 255));
        canvas
            .fill_rect(Rect::new(
                ((rect.pos().x - rect.width * 0.5) * PIXEL_PER_METER) as i64 as i32,
                (CANVAS_HEIGHT - ((rect.pos().y - rect.height * 0.5) * PIXEL_PER_METER)) as i64
                    as i32,
                (rect.width * PIXEL_PER_METER) as u64 as u32,
                (rect.height * PIXEL_PER_METER) as u64 as u32,
            ))
            .unwrap();
        /*
        canvas.set_draw_color(Color::RGB(255, 000, 000));
        canvas
            .fill_rect(Rect::new(
                ((rect2.pos().x - rect2.width * 0.5) * PIXEL_PER_METER) as i64 as i32,
                (CANVAS_HEIGHT - ((rect2.pos().y - rect2.height * 0.5) * PIXEL_PER_METER)) as i64
                    as i32,
                (rect2.width * PIXEL_PER_METER) as u64 as u32,
                (rect2.height * PIXEL_PER_METER) as u64 as u32,
            ))
            .unwrap();
        */

        if !paused {
            rect.tick(1.0 / 60.0);
            //rect2.tick(1.0 / 60.0);
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // 1 second in nanoseconds over 60; 60fps
    }
}

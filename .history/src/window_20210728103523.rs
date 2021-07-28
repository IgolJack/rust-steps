extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use std::time::Duration;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
pub fn canvas() {
let sdl_context = sdl2::init().unwrap();
let video_subsystem = sdl_context.video().unwrap();

let window = video_subsystem.window("Example", 800, 600).opengl().build().unwrap();
let mut canvas = window.into_canvas().build().unwrap();
canvas.clear();
canvas.present();

let mut event_pump = sdl_context.event_pump().unwrap();

let mut lastx = 0;
let mut lasty = 0;

'running: loop {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => break 'running,

            Event::MouseButtonDown { x, y, .. } => {
                let color = Color::RGB(x as u8, y as u8, 255);

                let _ = canvas.Rect(lastx, lasty, x as i16, y as i16, color);
                lastx = x as i16;
                lasty = y as i16;
                println!("mouse btn down at ({},{})", x, y);
                canvas.present();
            }

            _ => {}
        }
    }

    canvas.clear();
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

    }
}

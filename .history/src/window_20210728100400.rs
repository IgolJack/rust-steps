extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use std::time::Duration;
use sdl2::keyboard::Keycode;

pub fn canvas() {
let sdl_context = sdl2::init().unwrap();
let video_subsystem = sdl_context.video().unwrap();

let window = video_subsystem.window("Example", 800, 600).build().unwrap();
let mut canvas = window.into_canvas().build().unwrap();

canvas.set_draw_color(Color::RGB(255, 210, 0));
canvas.fill_rect(Rect::new(10, 10, 780, 580)).unwrap();
canvas.present();
let mut event_pump = sdl_context.event_pump().unwrap();

'running: loop {
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

    canvas.clear();
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

    }
}

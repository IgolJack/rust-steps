extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
pub fn Canvas() {
let sdl_context = sdl2::init().unwrap();
let video_subsystem = sdl_context.video().unwrap();

let window = video_subsystem.window("Example", 800, 600).build().unwrap();
let mut canvas = window.into_canvas().build().unwrap();

canvas.set_draw_color(Color::RGB(255, 210, 0));
canvas.fill_rect(Rect::new(10, 10, 780, 580)).unwrap();
canvas.present();

}

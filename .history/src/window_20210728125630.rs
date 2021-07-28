extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::*;
use std::mem;

pub struct Screen {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    sdl_context: sdl2::Sdl,
    pub event_p: sdl2::EventPump,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        let inicializate = init(width, height);
        Screen {
            canvas: inicializate.0,
            sdl_context: inicializate.1,
            event_p: inicializate.2,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, color: [u8; 3]) {
        self.canvas.set_draw_color(Color::RGB(color[0], color[1], color[2]));
        self.canvas.draw_point(Point::new(x, y)).unwrap();
        self.canvas.present();
    }
    
    pub fn wait_for_esc(&mut self) {
    
        let mut running = true;
    
        while running {
            for event in self.event_p.poll_iter() {
                use sdl2::event::Event;
    
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        running = false
                    },
                    _ => {}
                }
            }
        }
    }

    fn put_pixel(&mut self, x: i32, y: i32, color: [u8; 3]) {
        self.canvas.draw_point(Point::new(x, y)).unwrap()
    }

    pub fn line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: [u8; 3]) {
        let mut sleep: bool = false;
        if (x0 as i32 - x1 as i32).abs() < (y0 as i32 - y1 as i32).abs() {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
            sleep = true
        }
        else if x0 > x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }
    
        let dx: f32 = x1 as f32 - x0 as f32;
        let dy: f32 = y1 as f32 - y0 as f32;
        // let derror: f32 = dy/dx;
        let derror2: f32 = dy.abs() * 2.0;
        let mut error2: f32 = 0.0;
        let mut y = y0;
        for x in x0..x1 {
            if sleep {
                self.put_pixel(y, x, color);
            } else {
                self.put_pixel(x, y, color);
            }
            error2 += derror2;
            if error2 > dx {
                if y1 > y0 {
                    y += 1;
                } else {
                    y -= 1;
                }
                error2 -= dx * 2.0;
            }
        }
    }
}

fn init(width: u32, height: u32) -> (sdl2::render::Canvas<sdl2::video::Window>, sdl2::Sdl, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Example", width, height).opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    canvas.clear();
    canvas.present();
    (canvas, sdl_context, event_pump)
}
// canvas.clear();
// canvas.present();
// ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))




// 'running: loop {
//     for event in win.event_p.poll_iter() {
//         match event {
//             Event::Quit { .. }
//             | Event::KeyDown {
//                 keycode: Some(Keycode::Escape),
//                 ..
//             } => break 'running,

//             Event::MouseButtonDown { x, y, .. } => {
//                 let color = Color::RGB(x as u8, y as u8, 255);
//                 let _ =win.canvas.line(lastx, lasty, x as i16, y as i16, color);
//                 lastx = x as i16;
//                 lasty = y as i16;
//                 println!("mouse btn down at ({},{})", x, y);
//                 canvas.present();
//             }

//             _ => {}
//         }
//     }
// }
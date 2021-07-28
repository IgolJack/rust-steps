extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
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
        self.canvas.fill_rect(Rect::new(10, 10, 780, 580));
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
                    Event::MouseButtonDown { x, y, .. } => {
                                        let color = Color::RGB(x as u8, y as u8, 255);
                                        let _ = self.canvas.line(lastx, lasty, x as i16, y as i16, color);
                                        lastx = x as i16;
                                        lasty = y as i16;
                                        println!("mouse btn down at ({},{})", x, y);
                                        canvas.present();
                                    }
                    _ => {}
                }
            }
        }
    }
}

fn init(width: u32, height: u32) -> (sdl2::render::Canvas<sdl2::video::Window>, sdl2::Sdl, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Example", width, height).opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context.event_pump().unwrap();
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
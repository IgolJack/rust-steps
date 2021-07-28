extern crate sdl2;

pub struct Screen {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    sdl_context: sdl2::Sdl,
    event_p: sdl2::EventPump,
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
}

fn init(width: u32, height: u32) -> (sdl2::render::Canvas<sdl2::video::Window>, sdl2::Sdl, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Example", width, height).opengl().build().unwrap();
    let canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, sdl_context, event_pump)
}




    // canvas.clear();
    // canvas.present();
    // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

    

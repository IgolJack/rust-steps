mod draw;
mod parce;
mod graph;
use draw::Drawer;
use parce::Object;

extern crate glfw;
use glfw::{Action, Context, Key};


const WHITE: [u8; 3] = [255, 255, 255];
// const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
// const GREEN: [u8; 3] = [0, 255, 0];


fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }


    let mut pop = Drawer::new(600, 600);
   
    let mut hop = Object::new("./src/obj/african_head.obj");
    hop.read(); 

    let mut x: u16 = 0;

    for face in &hop.facets {
        for i in 0..3 {
            let v0 = &hop.vert[face[i] as usize];
            let v1 = &hop.vert[face[(i + 1)%3] as usize];
            // let x0 = cmp::min(((v0.x+1.)*499 as f32/2.) as u32, 499);
            // let y0 = cmp::min(((v0.y+1.)*499 as f32/2.) as u32, 499);
            // let x1 = cmp::min(((v1.x+1.)*499 as f32/2.) as u32, 499);
            // let y1 = cmp::min(((v1.y+1.)*499 as f32/2.) as u32, 499);

            let x0 = ((v0.x+1.)*599 as f32/2.) as u32;
            let y0 = ((v0.y+1.)*599 as f32/2.) as u32;
            let x1 = ((v1.x+1.)*599 as f32/2.) as u32;
            let y1 = ((v1.y+1.)*599 as f32/2.) as u32;
            x += 1;

            
            pop.line(x0, y0, x1, y1, WHITE);
            
            println!("{} - {} {} {} {}", x, x0, y0, x1, y1);
            
        }
    }
    pop.save();
}


fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
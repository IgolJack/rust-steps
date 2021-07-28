mod draw;
mod parce;
mod window;
mod graph;

use draw::Drawer;
use parce::Object;
use window::Screen;
const WHITE: [u8; 3] = [255, 255, 255];
// const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
// const GREEN: [u8; 3] = [0, 255, 0];


fn main() {
    let win = Screen::new(600, 400);

    'running: loop {
        for event in win.event_p.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
    
                Event::MouseButtonDown { x, y, .. } => {
                    let color = Color::RGB(x as u8, y as u8, 255);
                    let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    canvas.present();
                }
    
                _ => {}
            }
        }

    let mut pop = Drawer::new(600, 600);
    let mut hop = Object::new("./src/obj/african_head.obj");
    hop.read(); 


    for face in &hop.facets {
        for i in 0..3 {
            let v0 = &hop.vert[face[i] as usize];
            let v1 = &hop.vert[face[(i + 1)%3] as usize];

            let x0 = ((v0.x+1.)*599 as f32/2.) as u32;
            let y0 = ((v0.y+1.)*599 as f32/2.) as u32;
            let x1 = ((v1.x+1.)*599 as f32/2.) as u32;
            let y1 = ((v1.y+1.)*599 as f32/2.) as u32;
            pop.line(x0, y0, x1, y1, WHITE);
            //println!("{} - {} {} {} {}", x, x0, y0, x1, y1);
        }
    }
    pop.save();
}

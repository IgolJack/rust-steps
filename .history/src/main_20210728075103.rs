mod draw;
mod parce;
mod graph;
use draw::Drawer;
use parce::Object;


extern crate sdl2;

use sdl2::event::{Event};



mod window;


const WHITE: [u8; 3] = [255, 255, 255];
// const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
// const GREEN: [u8; 3] = [0, 255, 0];


fn main() {
      // start sdl2 with everything
      let ctx = sdl2::init().unwrap();
      let video_ctx = ctx.video().unwrap();
  
      // Create a window and show it
      let mut window  = match video_ctx.window("eg02", 640, 480).position_centered().opengl().build() {
          Ok(window) => window,
          Err(err)   => panic!("failed to create window: {}", err)
      };
  
      {
          window.show();
      }
  
      let mut events = ctx.event_pump().unwrap();
  
      // loop until we receive a QuitEvent
      'event : loop {
          // poll_event returns the most recent event or NoEvent if nothing has happened
          for event in events.poll_iter() {
              match event {
                  Event::Quit{..} => break 'event,
                  _               => continue
              }
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





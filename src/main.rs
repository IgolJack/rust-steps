mod draw;
mod parce;
mod graph;
use draw::Drawer;
use parce::Object;

const WHITE: [u8; 3] = [255, 255, 255];
// const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
// const GREEN: [u8; 3] = [0, 255, 0];

const HEIGHT: u32 = 500;
const WIDTH: u32 = HEIGHT;

fn main() {
    let mut pop = Drawer::new(WIDTH, HEIGHT);
   
    let mut hop = Object::new("./src/obj/african_head.obj");
    hop.read(); 


    
    let mut x: i32 = 0;
    for face in hop.facets {
        x += 1;
        if x < 1258 {
            println!("{} {} {}", face[0], face[1], face[2]);
            for i in 0..3 {
                println!("{}", i);
                let v0 = &hop.vert[face[i] as usize];
                let v1 = &hop.vert[face[(i + 1)%3] as usize];
                println!("v0: {} v1: {}", v0, v1);
                let x0 = ((v0.x+1.)*WIDTH as f32/2.) as u32;
                let y0 = ((v0.y+1.)*HEIGHT as f32/2.) as u32;
                let x1 = ((v1.x+1.)*WIDTH as f32/2.) as u32;
                let y1 = ((v1.y+1.)*HEIGHT as f32/2.) as u32;
                pop.line(x0, y0, x1, y1, WHITE);
            }
        }
      
    }


    pop.save();
}

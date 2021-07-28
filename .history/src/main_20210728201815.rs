mod parce;
mod window;
mod graph;

use parce::Object;
use window::Screen;
const WHITE: [u8; 3] = [255, 255, 255];
// const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
// const GREEN: [u8; 3] = [0, 255, 0];

const WIDTH: u32 = 600; 
const HEIGHT: u32 = 600;

struct Trian {
    x: [i32; 2],
    y: [i32; 2],
   }

fn main() {
    let mut win = Screen::new(WIDTH, HEIGHT);
    win.canvas.clear();

    let mut hop = Object::new("./src/obj/african_head.obj");
    hop.read(); 
   
    let a = Trian {x:[20, 20], y:[40, 10]};
    let b = Trian {x: [40, 10], y: [60, 40]};
    let c = Trian {x: [60, 40], y: [20, 20]};

    win.line(a.x[0], a.x[1], a.y[0], a.y[1], WHITE);
    win.line(b.x[0], b.x[1], b.y[0], b.y[1], WHITE);
    win.line(c.x[0], c.x[1], c.y[0], c.y[1], WHITE);


    // let mut lo: Vec<[i32;4]> = Vec::new();
    // for face in &hop.facets {
    //     for i in 0..3 {
    //         let v0 = &hop.vert[face[i] as usize];
    //         let v1 = &hop.vert[face[(i + 1)%3] as usize];

    //         let x0 = ((v0.x+1.)*599 as f32/2.) as i32;
    //         let y0 = ((v0.y+1.)*599 as f32/2.) as i32;
    //         let x1 = ((v1.x+1.)*599 as f32/2.) as i32;
    //         let y1 = ((v1.y+1.)*599 as f32/2.) as i32;
            
    //         lo.push([x0, y0, x1, y1]);
    //         //println!("{} {} {} {}", x0, y0, x1, y1);
    //     }
    // }
   
    // for line in lo {
    //     win.line(line[0], line[1], line[2], line[3], WHITE);
    // }
     win.canvas.present();
    
    win.wait_for_esc();
}




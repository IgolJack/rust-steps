mod parce;
mod window;
mod graph;
use std::mem::swap;

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
   
    let mut a = Trian {x:[200, 200], y:[400, 100]};
    let mut b = Trian {x: [400, 100], y: [600, 400]};
    let mut c = Trian {x: [600, 400], y: [200, 200]};

    if a.y > b.y {swap(&mut a, &mut b);}
    if a.y > c.y {swap(&mut a, &mut c);}
    if b.y > c.y {swap(&mut b, &mut c);}

    win.line(a.x, a.y, WHITE);
    win.line(b.x, b.y, WHITE);
    win.line(c.x, c.y, WHITE);


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




mod parce;
mod window;
mod graph;
use std::mem::swap;

use parce::Object;
use window::Screen;


// const WHITE: [u8; 3] = [255, 255, 255];
const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
const GREEN: [u8; 3] = [0, 255, 0];

const WIDTH: u32 = 600; 
const HEIGHT: u32 = 600;

struct Line {
    f_coord: [i32; 2],
    s_coord: [i32; 2],
   }

fn main() {
    let mut win = Screen::new(WIDTH, HEIGHT);
    win.canvas.clear();

    let mut hop = Object::new("./src/obj/african_head.obj");
    hop.read(); 
   
    let mut a = Line {f_coord:[200, 200], s_coord:[400, 100]};
    let mut b = Line {f_coord: [400, 100], s_coord: [600, 400]};
    let mut c = Line {f_coord: [600, 400], s_coord: [200, 200]};

    if a.s_coord > b.s_coord {swap(&mut a, &mut b);}
    if a.s_coord > c.s_coord {swap(&mut a, &mut c);}
    if b.s_coord > c.s_coord {swap(&mut b, &mut c);}

    win.line(a.f_coord, a.s_coord, GREEN);
    win.line(b.f_coord, b.s_coord, GREEN);
    win.line(c.f_coord, c.s_coord, RED);


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




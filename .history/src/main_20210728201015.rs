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


fn main() {
    let mut win = Screen::new(WIDTH, HEIGHT);
    win.canvas.clear();

    

    // let mut pop = Drawer::new(600, 600);

    let mut hop = Object::new("./src/obj/african_head.obj");
    hop.read(); 
    let mut lo: Vec<[i32;4]> = Vec::new();

    for face in &hop.facets {
        for i in 0..3 {
            let v0 = &hop.vert[face[i] as usize];
            let v1 = &hop.vert[face[(i + 1)%3] as usize];

            let x0 = ((v0.x+1.)*600 as f32/2.) as i32;
            let y0 = ((v0.y+1.)*600 as f32/2.) as i32;
            let x1 = ((v1.x+1.)*600 as f32/2.) as i32;
            let y1 = ((v1.y+1.)*600 as f32/2.) as i32;
            
            lo.push([x0, y0, x1, y1]);
            //println!("{} {} {} {}", x0, y0, x1, y1);
        }
    }
    // println!("Count of lines: {}", lo.len());
    //let mut x: i32 = 0;
    for line in lo {
        //x+=1;
        //println!("Line: {}", x);
        win.line(line[0], line[1], line[2], line[3], WHITE);
        
    }
    win.canvas.present();
    
    // pop.save();
    win.wait_for_esc();
}


// struct Trian {
//     x: [i32; 2],
//     y: [i32; 2],
//    }

// fn triangle() {
//     let a = Trian {x:[20, 20], y:[40, 10]};
//     let b = Trian {x: [40, 10], y: [60, 40]};
//     let c = Trian {x: [60, 40], y: [20, 20]};

// }



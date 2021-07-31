mod parce;
mod window;
mod graph;
use std::mem::swap;
use graph::*;
use parce::Object;
use window::Screen;


// const WHITE: [u8; 3] = [255, 255, 255];
//const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
// const GREEN: [u8; 3] = [0, 255, 0];

const WIDTH: u32 = 600; 
const HEIGHT: u32 = 600;

fn main() {
    let mut win = Screen::new(WIDTH, HEIGHT);
    win.canvas.clear();

    let mut hop = Object::new("./src/obj/african_head.obj");
    hop.read(); 
   
    // let mut a = Vector2D {x: 300.0, y: 200.0};
    // let mut b = Vector2D {x: 400.0, y: 100.0};
    // let mut c = Vector2D {x: 500.0, y: 400.0};

    // if a.y > b.y {swap(&mut a, &mut b);}
    // if a.y > c.y {swap(&mut a, &mut c);}
    // if b.y > c.y {swap(&mut b, &mut c);}

    // let segment_height = b.y - a.y + 1.0;
    // let total_height = c.y - a.y;

    // for i in a.y as i32..b.y as i32 + 1 {
    //     let alfa: f32 = (i as f32 - a.y) / total_height as f32;
    //     let beta: f32 = (i as f32 - a.y) / segment_height as f32;
    //     let a_vec: Vector2D = a + (c - a) * alfa;
    //     let b_vec: Vector2D = a + (b - a) * beta;
    //     println!("alfa - {} beta - {} avec - {} bvec - {}", alfa, beta, a_vec.x, b_vec);
    //     win.put_pixel(a_vec.x as i32, a_vec.y as i32, RED);
    //     win.put_pixel(b_vec.x as i32, b_vec.y as i32, BLUE);
    //     let mut child = Command::new("sleep").arg("1").spawn().unwrap();
    //     let _result = child.wait().unwrap();
    //     win.line([b_vec.x as i32, b_vec.y as i32],[a_vec.x as i32, b_vec.y as i32], GREEN);
    //     win.canvas.present();
    // }

    // for i in b.y as i32..c.y as i32 {
    //     let segment_height = c.y - b.y + 1.0;
    //     let alfa: f32 = (i as f32 - a.y) / total_height as f32;
    //     let beta: f32 = (i as f32 - b.y) / segment_height as f32;
        
    //     let mut a_vec: Vector2D = a + (c - a) * alfa;
    //     let mut b_vec: Vector2D = b + (c - b) * beta;

    //     if a_vec.x > b_vec.x {swap(&mut a_vec, &mut b_vec)}
    //     win.line([b_vec.x as i32, b_vec.y as i32],[a_vec.x as i32, a_vec.y as i32], GREEN);
    // }

    let light_dir: Vector3D = Vector3D{x: 0., y: 0., z: -1.};
    for face in &hop.facets {
        
        let mut screen_coords = Vec::new(); 
        let mut world_coords = Vec::new(); 
        for i in 0..3 {
            let v0  = &hop.vert[face[i] as usize];
            let one = (v0.x + 1.) * WIDTH as f32 /2.;
            let two = (v0.y + 1.) * HEIGHT as f32 /2.;
            world_coords.push(Vector3D{x: v0.x, y: v0.y, z: v0.z});
            screen_coords.push(Vector2D{x: one , y: two});
        }
        let n: Vector3D = (world_coords[2] - world_coords[0])^(world_coords[1] - world_coords[0]);
        let n = n.normalize();
        let intensivity = n.z * light_dir.z;

        let color: [u8; 3] = [(255. * intensivity) as u8, (255. * intensivity) as u8, (255. * intensivity) as u8];
        println!("{}", intensivity);
        if intensivity > 0. {
            triangle(screen_coords[0], screen_coords[1], screen_coords[2], &mut win, color);
        }
    }

    win.canvas.present();
    win.wait_for_esc();
}


fn triangle(mut a:  Vector2D, mut b: Vector2D, mut c: Vector2D,  win: &mut window::Screen, color: [u8; 3]) {
   

    if a.y == b.y && a.y == c.y {return};
    if a.y > b.y {swap(&mut a, &mut b);}
    if a.y > c.y {swap(&mut a, &mut c);}
    if b.y > c.y {swap(&mut b, &mut c);}

    let total_height = c.y - a.y;

    win.line([a.x as  i32, a.y as i32], [b.x as i32, b.y as i32], color);
    win.line([b.x as  i32, b.y as i32], [c.x as i32, c.y as i32], color);
    win.line([c.x as  i32, c.y as i32], [a.x as i32, a.y as i32], color);

    for i in 0 .. total_height as i32 {
        let is_half: bool = (i as f32 > b.y - a.y) || b.y == a.y;
        let segment_height = if is_half {c.y - b.y} else {b.y - a.y};
        let alfa: f32 = i as f32 / total_height;
        let beta: f32 = (i as f32 - (if is_half{b.y - a.y} else {0.0})) / segment_height as f32;
        let mut a_vec: Vector2D = a + ((c - a) * alfa); 
        let mut b_vec: Vector2D = if is_half {b + (c - b) * beta} else {a + (b - a) * beta};
        if a_vec.x > b_vec.x {swap(&mut a_vec, &mut b_vec)};
        for j in a_vec.x as i32 .. b_vec.x as i32 + 1 {
            win.put_pixel(j, a.y as i32 + i, color)
        } 
    }
}
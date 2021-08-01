mod parce;
mod window;
mod graph;
use std::mem::swap;
use graph::*;
use parce::Object;
use window::Screen;


// const WHITE: [u8; 3] = [255, 255, 255];
// const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
// const GREEN: [u8; 3] = [0, 255, 0];
            
const SCREEN_SIZE: [u32; 2] = [600, 600];

fn main() {
    let mut win = Screen::new(SCREEN_SIZE[0], SCREEN_SIZE[1]);
    win.canvas.clear();

    // triangle(Vector2D{x: 50., y: 20.}, Vector2D{x: 100., y: 200.}, Vector2D{x: 20., y: 300.}, &mut win, WHITE);
    // triangle(Vector2D{x: 100., y: 200.}, Vector2D{x: 20., y: 300.}, Vector2D{x: 500., y: 500.}, &mut win, RED);

    let mut face = Object::new("./src/obj/african_head.obj");
    face.read(); 
   
    let light_dir = Vector3D{x: 0., y: 0., z: -1.};
    
    for facet in &face.facets {
        let mut screen_coords = Vec::new(); 
        let mut world_coords = Vec::new(); 

        for i in 0..3 {
            let v0  = &face.vert[facet[i] as usize];
            let one = ((v0.x + 1.) * SCREEN_SIZE[0] as f32 /2.) as i32;
            let two = ((v0.y + 1.) * SCREEN_SIZE[1] as f32 /2.) as i32;
            world_coords.push(Vector3D{x: v0.x, y: v0.y, z: v0.z});
            screen_coords.push(Vector2D{x: one as f32 , y: two as f32});
        }
        let mut n: Vector3D = (world_coords[2] - world_coords[0])^(world_coords[1] - world_coords[0]);
        let n = n.normalize();
        let intensivity = n.x * light_dir.x + n.y * light_dir.y + n.z * light_dir.z;

        let color: [u8; 3] = [(255. * intensivity) as u8, (255. * intensivity) as u8, (255. * intensivity) as u8];
        println!("{}", intensivity);
        if intensivity > 0.0 {
            triangle(screen_coords[0], screen_coords[1], screen_coords[2], &mut win, color);
        }
    }

    win.canvas.present();
    win.wait_for_esc();
}


fn triangle(mut a:  Vector2D, mut b: Vector2D, mut c: Vector2D,  win: &mut window::Screen, color: [u8; 3]) {
    if a.y > b.y {swap(&mut a, &mut b);}
    if a.y > c.y {swap(&mut a, &mut c);}
    if b.y > c.y {swap(&mut b, &mut c);}

    let total_height = (c.y - a.y) as i32;

    // win.line([a.x as  i32, a.y as i32], [b.x as i32, b.y as i32], color);
    // win.line([b.x as  i32, b.y as i32], [c.x as i32, c.y as i32], color);
    // win.line([c.x as  i32, c.y as i32], [a.x as i32, a.y as i32], color);

    for i in 0 .. total_height as i32 {
        let is_half: bool = (i as f32 > b.y - a.y) || b.y == a.y;
        let segment_height = (if is_half {c.y - b.y} else {b.y - a.y}) as i32 ;
        let alfa: f32 = i as f32 / total_height as f32;
        let beta: f32 = (i as f32 - (if is_half {b.y - a.y} else {0.0})) / segment_height as f32;
        let mut a_vec: Vector2D = a + ((c - a) * alfa); 
        let mut b_vec: Vector2D = if is_half {b + (c - b) * beta} else {a + (b - a) * beta};
        if a_vec.x > b_vec.x {swap(&mut a_vec, &mut b_vec)};
        for j in a_vec.x as i32 - 1 .. b_vec.x as i32 + 1{
            win.put_pixel(j, a.y as i32 + i, color)
        } 
        
    }
}
mod parce;
mod window;
mod graph;
use std::mem::swap;
use graph::*;
use parce::Object;
use window::Screen;

const WHITE: [u8; 3] = [255, 255, 255];
const RED: [u8; 3] = [255, 0, 0];
const BLUE: [u8; 3] = [0, 0, 255];
const GREEN: [u8; 3] = [0, 255, 0];
            
const SCREEN_SIZE: [u32; 2] = [800, 800];
const DEPTH: u32 = 255;

fn main() {
    let mut win = Screen::new(SCREEN_SIZE[0], SCREEN_SIZE[1]);
    win.canvas.clear();

    let mut face = Object::new("./src/obj/african_head.obj");
    face.read(); 
   
    let light_dir = Vector3D{x: 0., y: 0., z: -1.};
    
    for facet in &face.facets {
        let mut screen_coords = Vec::new(); 
        let mut world_coords = vec![Vector3D { x: 0., y: 0., z: 0. }; 3];

        for i in 0..3 {
            let v0  = &face.vert[facet[i] as usize];
            world_coords[i] = *v0;
            screen_coords.push(Vector3D{x: (v0.x + 1.) * win.width as f32 /2. , y: (v0.y + 1.) * win.height as f32 / 2., z: (v0.z + 1.)/ DEPTH as f32 / 2.});
        }

        let mut n: Vector3D = (world_coords[2] - world_coords[0])^(world_coords[1] - world_coords[0]);
        let n = n.normalize();
        let intensivity = n.x * light_dir.x + n.y * light_dir.y + n.z * light_dir.z;

        let color: [u8; 3] = [(255. * intensivity) as u8, (255. * intensivity) as u8, (255. * intensivity) as u8];
        if intensivity > 0.0 {
            triangle(screen_coords[0], screen_coords[1], screen_coords[2], &mut win, color);
        }
    }
    win.canvas.present();
    win.wait_for_esc();
}

fn triangle(mut a:  Vector3D, mut b: Vector3D, mut c: Vector3D,  win: &mut window::Screen, color: [u8; 3]) {
    if a.y == b.y && a.y == c.y {return};
   
    if a.y > b.y {swap(&mut a, &mut b);}
    if a.y > c.y {swap(&mut a, &mut c);}
    if b.y > c.y {swap(&mut b, &mut c);}

    let total_height = (c.y - a.y) as i32;

    for i in 0 .. total_height as i32 {
        let is_half: bool = (i as f32 > b.y - a.y) || b.y == a.y;
        let segment_height = (if is_half {c.y - b.y} else {b.y - a.y}) as i32;

        let alfa: f32 = i as f32 / total_height as f32;
        let beta: f32 = (i as f32 - (if is_half {b.y - a.y} else {0.0})) / segment_height as f32;

        let mut a_vec: Vector3D = a + ((c - a) * alfa); 
        let mut b_vec: Vector3D = if is_half {b + (c - b) * beta} else {a + (b - a) * beta};

        if a_vec.x > b_vec.x {swap(&mut a_vec, &mut b_vec)};

        for j in a_vec.x as i32 .. b_vec.x as i32 + 1 {
            let phi: f32 = if b_vec.x == a_vec.x {1.} else {(j as f32 - a_vec.x)/(b_vec.x - a_vec.x)};
            let p_vec: Vector3D = a_vec + (b_vec - a_vec) * phi;

            let idx: u32 = (p_vec.x as u32 + p_vec.y as u32 * win.width) as u32;
            
            if win.zbufer[idx as usize] < p_vec.z {
                win.zbufer[idx as usize] = p_vec.z;
                win.put_pixel(p_vec.x as i32, p_vec.y as i32, color)
            }
        }   
    }
}
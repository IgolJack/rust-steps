mod draw;
mod parce;
mod graph;
use draw::Drawer;
use parce::Object;

const WHITE: [u8; 3] = [255, 255, 255];
// const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
// const GREEN: [u8; 3] = [0, 255, 0];


fn main() {
    let mut pop = Drawer::new(500, 500);
   
    let mut hop = Object::new("./src/obj/african_head.obj");
    hop.read(); 

    for face in &hop.facets {
        for i in 0..3 {
            let v0 = &hop.vert[face[i] as usize];
            let v1 = &hop.vert[face[(i + 1)%3] as usize];
            let x0 = ((v0.x+1.)*499 as f32/2.) as u32;
            let y0 = ((v0.y+1.)*499 as f32/2.) as u32;
            let x1 = ((v1.x+1.)*499 as f32/2.) as u32;
            let y1 = ((v1.y+1.)*499 as f32/2.) as u32;
            println!("{} {} {} {}", x0, y0, x1, y1);
            pop.line(x0, y0, x1, y1, WHITE);
        }
    }
    pop.save();
}

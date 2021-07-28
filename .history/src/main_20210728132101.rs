mod draw;
mod parce;
mod window;
mod graph;


// use draw::Drawer;
use parce::Object;
use window::Screen;
const WHITE: [u8; 3] = [255, 255, 255];
const RED: [u8; 3] = [255, 0, 0];
// const BLUE: [u8; 3] = [0, 0, 255];
// const GREEN: [u8; 3] = [0, 255, 0];


fn main() {
    let mut win = Screen::new(600, 600);
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
            println!("{} {} {} {}", x0, y0, x1, y1);
        }
    }
    let mut x = 0;
    for line in lo {
        x += 1;
        if x > 4000 {
            win.line(line[0], line[1], line[2], line[3], RED);
        } else {
            win.line(line[0], line[1], line[2], line[3], WHITE);
        }
        
    }
    win.canvas.present();
    
    // pop.save();
    win.wait_for_esc();
}


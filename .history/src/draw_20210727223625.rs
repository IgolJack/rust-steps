use std::mem;
use image::{ImageBuffer, RgbImage};

pub struct Drawer {
    pub image: RgbImage,
    pub width: u32,
    pub height: u32,
}

impl Drawer {
    pub fn new(width: u32, height: u32) -> Drawer {
        Drawer {
            image: ImageBuffer::new(width, height),
            width,
            height 
        }
    }

    pub fn save(&self) {
        self.image.save("output.png").unwrap();
    }

 
    pub fn line(&mut self, mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, color: [u8; 3]) {
        let mut sleep: bool = false;
        if (x0 as i32 - x1 as i32).abs() < (y0 as i32 - y1 as i32).abs() {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
            sleep = true
        }
        else if x0 > x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }
    
        let dx: f32 = x1 as f32 - x0 as f32;
        let dy: f32 = y1 as f32 - y0 as f32;
        // let derror: f32 = dy/dx;
        let derror2: f32 = dy.abs() * 2.0;
        let mut error2: f32 = 0.0;
        let mut y = y0;
        for x in x0..x1 {
            if sleep {
                self.put_pixel(&y, &x, color);
            } else {
                self.put_pixel(&x, &y, color);
            }
            error2 += derror2;
            if error2 > dx {
                if y1 > y0 {
                    y += 1;
                } else {
                    y -= 1;
                }
                error2 -= dx * 2.0;
            }
        }
    }

    fn put_pixel(&mut self, x: &u32, y: &u32, color: [u8; 3]) {
        let x =  *x;
        let y =  *y;
        let g: image::Rgb<u8> =  image::Rgb(color);
        *self.image.get_pixel_mut(x, y) = g;
    }
}
use std::fmt;
use core::ops;

#[derive(
    Debug,
    Clone, Copy,
)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

pub struct Point {
    x: i32,
    y: i32,
}

pub struct Vector3D {
    pub x: f32,
    pub y: f32, 
    pub z: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D {
            x,
            y,
            z,
        }
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}


impl ops::Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl ops::Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl ops::Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {x: self.x  * other , y: self.y  * other}
    }
}



impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D {
            x,
            y,
        }
    }

    pub fn new_from_point(p: Point) -> Vector2D {
        Vector2D {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
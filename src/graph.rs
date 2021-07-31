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
#[derive(
    Debug,
    Clone, Copy,
)]
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
impl ops::BitXor for Vector3D {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self {x: self.y * other.z - self.z * other.y, y: self.z * other.x - self.x * other.z, z: self.x * other.y - self.y * other.x}
    }
}

impl ops::Sub for Vector3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl ops::Mul for Vector3D {
    type Output = Self; 

    fn mul(self, other: Self) -> Self {
        Self{x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }

}

impl ops::Mul<f32> for Vector3D {
    type Output = Self; 

    fn mul(self, other: f32) -> Self {
        Self{x: self.x * other, y: self.y * other, z: self.z * other}
    }

}

impl Vector3D {
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self{
        let ko = 1. / self.norm();
        Self{x: self.x * ko, y: self.y * ko, z: self.z * ko} 
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
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
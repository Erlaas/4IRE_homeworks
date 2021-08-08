use std::fmt::{Display, Formatter, Result};

pub struct Sphere {
    radius: f64,
}

pub struct Parallelepiped {
    x: f64,
    y: f64,
    h: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        Sphere { radius }
    }
    pub fn volume(&self) -> f64 {
        if self.radius >= 0f64 {
            (4.0 / 3.0) * std::f64::consts::PI * self.radius.powi(3)
        } else {
            f64::NAN
        }
    }
}

impl Parallelepiped {
    pub fn new(x: f64, y: f64, h: f64) -> Parallelepiped {
        Parallelepiped { x, y, h }
    }
    pub fn volume(&self) -> f64 {
        if self.x >= 0f64 && self.y >= 0f64 && self.h >= 0f64 {
            self.x * self.y * self.h
        } else {
            f64::NAN
        }
    }
}

impl Display for Sphere {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Sphere(radius:{})", self.radius)
    }
}

impl Display for Parallelepiped {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Parallelepiped(x:{}, y:{}, h:{})",
            self.x, self.y, self.h
        )
    }
}

use std::fmt::{Display, Formatter, Result};

pub struct Circle {
    radius: f64,
}

pub struct Rectangle {
    x: f64,
    y: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    pub fn area(&self) -> f64 {
        if self.radius >= 0f64 {
            self.radius * self.radius * std::f64::consts::PI
        } else {
            f64::NAN
        }
    }
}

impl Rectangle {
    pub fn new(x: f64, y: f64) -> Rectangle {
        Rectangle { x, y }
    }
    pub fn area(&self) -> f64 {
        if self.x >= 0f64 && self.y >= 0f64 {
            self.x * self.y
        } else {
            f64::NAN
        }
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Circle(radius:{})", self.radius)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Rectangle(x:{}, y:{})", self.x, self.y)
    }
}

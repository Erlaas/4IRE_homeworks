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
		self.radius * self.radius * std::f64::consts::PI
	}
}

impl Rectangle {
	pub fn new(x: f64, y: f64) -> Rectangle {
		Rectangle { x, y }
	}
	pub fn area(&self) -> f64 {
		self.x * self.y
	}
}

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
		(4.0 / 3.0) * std::f64::consts::PI * self.radius * self.radius * self.radius
	}
}

impl Parallelepiped {
	pub fn new(x: f64, y: f64, h: f64) -> Parallelepiped {
		Parallelepiped { x, y, h }
	}
	pub fn volume(&self) -> f64 {
		self.x * self.y * self.h
	}
}

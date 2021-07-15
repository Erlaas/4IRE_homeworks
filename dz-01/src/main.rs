use area_volume::area;
use area_volume::volume;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		println!("Usage: Type_of_figure n1 n2 n3");
		println!("\tCircle radius");
		println!("\tRectangle x y");
		println!("\tSphere radius");
		println!("\tParallelepiped x y h");
		println!("Example:");
		println!("\tParallelepiped 5 4 2");
		return;
	}
	if args[1].to_lowercase().eq("circle") {
		let radius: f64 = args[2].parse().unwrap();
		let c = area::Circle::new(radius);
		println!("Circle area = {}", c.area());
	} else if args[1].to_lowercase().eq("rectangle") {
		let x = args[2].parse().unwrap();
		let y = args[3].parse().unwrap();
		let r = area::Rectangle::new(x, y);
		println!("Rectangle area = {}", r.area());
	} else if args[1].to_lowercase().eq("sphere") {
		let radius = args[2].parse().unwrap();
		let c = volume::Sphere::new(radius);
		println!("Sphere volume = {}", c.volume());
	} else if args[1].to_lowercase().eq("parallelepiped") {
		let x = args[2].parse().unwrap();
		let y = args[3].parse().unwrap();
		let h = args[4].parse().unwrap();
		let p = volume::Parallelepiped::new(x, y, h);
		println!("Parallelepiped volume = {}", p.volume());
	}
}

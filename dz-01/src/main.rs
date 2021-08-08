use area_volume::area;
use area_volume::volume;
use std::env;

const ERROR_MESSAGE: &str = "Usage: Type_of_figure n1 n2 n3\n
\tCircle radius\n
\tRectangle x y\n
\tSphere radius\n
\tParallelepiped x y h\n
Example:\n
\tParallelepiped 5 4 2 ";

fn main() {
    let mut iter = env::args().skip(1);
    let figure = iter
        .next()
        .unwrap_or_else(|| {
            eprintln!("{}", ERROR_MESSAGE);
            std::process::exit(1)
        })
        .to_lowercase();

    let nums: Result<Vec<f64>, core::num::ParseFloatError> =
        iter.map(|str| str.parse::<f64>()).collect();
    let nums = match nums {
        Ok(vec) => vec,
        Err(err) => {
            eprintln!("{}\n{}", err, ERROR_MESSAGE);
            std::process::exit(1)
        }
    };

    match (figure.as_str(), nums.as_slice()) {
        ("circle", [r]) => {
            let c = area::Circle::new(*r);
            println!("The area of {} is {}", c, c.area());
        }
        ("rectangle", [x, y]) => {
            let r = area::Rectangle::new(*x, *y);
            println!("The area of {} is {}", r, r.area());
        }
        ("sphere", [r]) => {
            let s = volume::Sphere::new(*r);
            println!("The volume of {} is {}", s, s.volume());
        }
        ("parallelepiped", [x, y, h]) => {
            let p = volume::Parallelepiped::new(*x, *y, *h);
            println!("The volume of {} is {}", p, p.volume());
        }
        _ => {
            eprintln!("{}", ERROR_MESSAGE);
            std::process::exit(1)
        }
    }
}

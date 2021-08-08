pub mod area;
pub mod volume;

#[cfg(test)]
mod tests {
    use crate::{
        area::{Circle, Rectangle},
        volume::{Parallelepiped, Sphere},
    };

    #[test]
    fn circle_edge_test() {
        let c = Circle::new(0f64);
        assert_eq!(c.area(), 0f64);

        let c = Circle::new(-1f64);
        assert!(c.area().is_nan());

        let c = Circle::new(-123456789.123456789f64);
        assert!(c.area().is_nan());

        let c = Circle::new(f64::MAX);
        assert_eq!(c.area(), f64::INFINITY);

        let c = Circle::new(f64::NAN);
        assert!(c.area().is_nan());

        let c = Circle::new(f64::INFINITY);
        assert_eq!(c.area(), f64::INFINITY);
    }

    #[test]
    fn circle_test() {
        let c = Circle::new(5f64);
        assert_eq!(c.area(), 25f64 * std::f64::consts::PI);

        let c = Circle::new(741743938.9284752);
        assert_eq!(c.area(), 5.501840709371295e+17 * std::f64::consts::PI);

        let c = Circle::new(7f64);
        assert_eq!(c.area(), 49f64 * std::f64::consts::PI);

        let c = Circle::new(5.501840709371295);
        assert_eq!(c.area(), 30.27025119129523 * std::f64::consts::PI);
    }

    #[test]
    fn rectangle_edge_test() {
        let r = Rectangle::new(0f64, 0f64);
        assert_eq!(r.area(), 0f64);

        let r = Rectangle::new(0f64, 25f64);
        assert_eq!(r.area(), 0f64);

        let r = Rectangle::new(666_f64, 0f64);
        assert_eq!(r.area(), 0f64);

        let r = Rectangle::new(-1456f64, -123456f64);
        assert!(r.area().is_nan());

        let r = Rectangle::new(-1654f64, 4654f64);
        assert!(r.area().is_nan());

        let r = Rectangle::new(1654f64, -4654f64);
        assert!(r.area().is_nan());

        let r = Rectangle::new(f64::MAX, 420f64);
        assert_eq!(r.area(), f64::INFINITY);

        let r = Rectangle::new(f64::NAN, 156487f64);
        assert!(r.area().is_nan());
    }

    #[test]
    fn rectangle_test() {
        let r = Rectangle::new(5f64, 25f64);
        assert_eq!(r.area(), 125f64);

        let r = Rectangle::new(25f64, 5f64);
        assert_eq!(r.area(), 125f64);

        let r = Rectangle::new(1654564894f64, 1f64);
        assert_eq!(r.area(), 1654564894f64);

        let r = Rectangle::new(1654564894f64, 1654564894f64);
        assert_eq!(r.area(), 1654564894f64 * 1654564894f64);
    }

    #[test]
    fn sphere_edge_test() {
        let c = Sphere::new(0f64);
        assert_eq!(c.volume(), 0f64);

        let c = Sphere::new(-25f64);
        assert!(c.volume().is_nan());

        let c = Sphere::new(-987654321.987321465f64);
        assert!(c.volume().is_nan());

        let c = Sphere::new(f64::MAX);
        assert_eq!(c.volume(), f64::INFINITY);

        let c = Sphere::new(f64::NAN);
        assert!(c.volume().is_nan());

        let c = Sphere::new(f64::INFINITY);
        assert_eq!(c.volume(), f64::INFINITY);
    }

    #[test]
    fn sphere_test() {
        let s = Sphere::new(5.2f64);
        assert_eq!(s.volume().round(), 589f64);

        let s = Sphere::new(465f64);
        assert_eq!(s.volume().round(), 421_160_340_f64);
    }

    #[test]
    fn parallelepiped_edge_test() {
        let p = Parallelepiped::new(0f64, 50f64, 20f64);
        assert_eq!(p.volume(), 0f64);

        let p = Parallelepiped::new(420f64, 0f64, 1235f64);
        assert_eq!(p.volume(), 0f64);

        let p = Parallelepiped::new(4654f64, 1f64, 0f64);
        assert_eq!(p.volume(), 0f64);

        let p = Parallelepiped::new(0f64, 0f64, 0f64);
        assert_eq!(p.volume(), 0f64);

        let p = Parallelepiped::new(-20f64, 50f64, 20f64);
        assert!(p.volume().is_nan());

        let p = Parallelepiped::new(420f64, -60f64, 1235f64);
        assert!(p.volume().is_nan());

        let p = Parallelepiped::new(4654f64, 1f64, -1f64);
        assert!(p.volume().is_nan());

        let p = Parallelepiped::new(-5f64, -1f64, -156456897.123f64);
        assert!(p.volume().is_nan());

        let p = Parallelepiped::new(123f64, f64::MAX, 5798f64);
        assert_eq!(p.volume(), f64::INFINITY);

        let p = Parallelepiped::new(f64::INFINITY, 2f64, 5798f64);
        assert_eq!(p.volume(), f64::INFINITY);

        let p = Parallelepiped::new(123f64, f64::MAX, f64::NAN);
        assert!(p.volume().is_nan());
    }
}

use std::cell::Cell;

struct Point {
    x: f64,
    y: f64,
    visited: bool,
}

fn cross_product(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    p1.x * p2.y + p3.x * p1.y + p2.x * p3.y - p3.x * p2.y - p2.x * p1.y - p1.x * p3.y
}

mod test {
    use super::*;

    #[test]
    fn test_cross_product() {
        let p1 = Point {
            x: 0.0,
            y: 0.0,
            visited: false,
        };
        let p2 = Point {
            x: 1.0,
            y: 0.0,
            visited: false,
        };
        let p3 = Point {
            x: 0.5,
            y: 0.0,
            visited: false,
        };
        assert_eq!(0.0, cross_product(p1, p2, p3))
    }
}

fn main() {}

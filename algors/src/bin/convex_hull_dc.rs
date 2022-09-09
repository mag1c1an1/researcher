use std::borrow::Borrow;
use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;

struct Point {
    x: f64,
    y: f64,
    visited: bool,
}

fn cross_product(p1: &Rc<RefCell<Point>>, p2: &Rc<RefCell<Point>>, p3: &Rc<RefCell<Point>>) -> f64 {
    let p1 = p1.borrow_mut();
    let p2 = p2.borrow_mut();
    let p3 = p3.borrow_mut();
    p1.x * p2.y + p3.x * p1.y + p2.x * p3.y - p3.x * p2.y - p2.x * p1.y - p1.x * p3.y
}

fn dc(points: &mut Vec<Rc<RefCell<Point>>>, ver: i32) {
    if points.len() <= 2 {
        for p in points.iter() {
            p.borrow_mut().visited = true
        }
    }
    let pi = points.first().unwrap();
    let pj = points.last().unwrap();
    let (mut mx, mut mn) = (f64::MIN, f64::MAX);
    let (mut imax, mut imin) = (0_usize, 0_usize);
    for i in 1..points.len() - 1 {
        let cp = cross_product(
            pi, pj, points.get(i).unwrap(),
        );
        if cp > mx && cp >= 0.0 {
            mx = cp;
            imax = i;
        }
        if cp < mn && cp <= 0.0 {
            mn = cp;
            imin = i;
        }
    }
    let mut s1: Vec<Rc<RefCell<Point>>> = Vec::new();
    let mut s2: Vec<Rc<RefCell<Point>>> = Vec::new();
    let mut s3: Vec<Rc<RefCell<Point>>> = Vec::new();
    let mut s4: Vec<Rc<RefCell<Point>>> = Vec::new();
    for i in 0..points.len() - 1 {
        let v1 = cross_product(
            pi,
            points.get(imax).unwrap(),
            points.get(i).unwrap(),
        );
        let v2 = cross_product(
            pi,
            points.get(imin).unwrap(),
            points.get(i).unwrap(),
        );
        if v1 >= 0.0 {
            s1.push(Rc::clone(points.get(i).unwrap()));
        }
        if v2 <= 0.0 {
            s3.push(Rc::clone(points.get(i).unwrap()));
        }
    }for i in 1..points.len() {
        let v1 = cross_product(
            points.get(imax).unwrap(),
            pj,
            points.get(i).unwrap(),
        );
        let v2 = cross_product(
            points.get(imin).unwrap(),
            pj,
            points.get(i).unwrap(),
        );
        if v1 >= 0.0 {
            s2.push(Rc::clone(points.get(i).unwrap()));
        }
        if v2 <= 0.0 {
            s4.push(Rc::clone(points.get(i).unwrap()));
        }
        match ver {
            1 => {
                dc(&mut s1,1);
                dc(&mut s2,1);
            }
            2 => {
                dc(&mut s3,2);
                dc(&mut s4,2);
            }
            3 => {
                dc(&mut s1,1);
                dc(&mut s2,1);
                dc(&mut s3,2);
                dc(&mut s4,2);
            }
            _ => {println!("Something wrong!")}
        }
    }
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
        let p1 = Rc::new(RefCell::new(p1));
        let p2 = Rc::new(RefCell::new(p2));
        let p3 = Rc::new(RefCell::new(p3));
        let mut vc = vec![p1, p2, p3];
        dc(&mut vc,3);
    }
}

fn main() {}

use std::ops::{Add, Mul, Sub};
fn main() {
    println!("fft now");
}

const PI: f64 = std::f64::consts::PI;

#[derive(Default, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub image: f64,
}

impl Complex {
    fn new(real: f64, image: f64) -> Self {
        Self { real, image }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.real + rhs.real, self.image + rhs.image)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.real * rhs.real - self.image * rhs.image,
            self.real * rhs.image + self.image * rhs.real,
        )
    }
}
impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.real - rhs.real, self.image - rhs.image)
    }
}

#[allow(dead_code)]
fn fft(a: &mut [Complex], lim: usize) {
    if lim == 1 {
        return;
    }
    let mut a0 = vec![Complex::default(); lim >> 1];
    let mut a1 = vec![Complex::default(); lim >> 1];
    for i in (0..lim).step_by(2) {
        a0[i >> 1] = a[i];
        a1[i >> 1] = a[i + 1];
    }
    fft(&mut a0, lim >> 1);
    fft(&mut a1, lim >> 1);
    let val = 2.0 * PI / lim as f64;
    let wn = Complex::new(val.cos(), val.sin());
    let mut w = Complex::new(1.0, 0.0);
    for k in 0..(lim >> 1) {
        a[k] = a0[k] + w * a1[k];
        a[k + (lim >> 1)] = a0[k] - w * a1[k];
        w = w * wn;
    }
}

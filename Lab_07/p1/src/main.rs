use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl<T: Mul<Output = T> + Neg<Output = T> + Copy> Complex {
    fn new(real: T, imag: T) -> Self {
        Complex { real, imag }
    }

    fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}

impl<T: Add<Output = T>> Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy> Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Complex {
            real: self.real * other.real - (self.imag * other.imag),
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl<T: fmt::Display + PartialEq + PartialOrd + Default + Copy> fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag < T::default() {
            write!(f, "{}{}i", self.real, self.imag)
        } else if self.imag == T::default() {
            write!(f, "{}", self.real)
        } else {
            write!(f, "{}+{}i", self.real, self.imag)
        }
    }
}

impl<
        T: From<i32>
            + From<f64>
            + Div<Output = T>
            + Default
            + Neg<Output = T>
            + Mul<Output = T>
            + Copy,
    > From<i32> for Complex
{
    fn from(real: i32) -> Self {
        Complex::new(T::from(real), T::default())
    }
}

impl<
        T: From<i32>
            + From<f64>
            + Div<Output = T>
            + Default
            + Neg<Output = T>
            + Mul<Output = T>
            + Copy,
    > From<f64> for Complex
{
    fn from(real: f64) -> Self {
        Complex::new(from(real), default())
    }
}

impl<T: Neg<Output = T>> Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}

macro_rules! assert_eq_rel {
    ($x:expr, $y:expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3.0);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5.into()) * 2.0.into();
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}

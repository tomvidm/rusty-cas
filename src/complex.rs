use std::ops::{Add, Sub, Mul, Div};
use std::f64::consts::PI;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Complex {
    real: f64,
    imag: f64
}

impl Complex {
    fn from(real: f64, imag: f64) -> Complex {
        Complex{
            real: real,
            imag: imag
        }
    }

    fn zero() -> Complex {
        Complex::from(0., 0.)
    }

    fn conj(self) -> Complex {
        Complex::from(self.real, -self.imag)
    }

    fn real(self) -> Complex {
        Complex::from(self.real, 0.)
    }

    fn imag(self) -> Complex {
        Complex::from(0., self.imag)
    }

    fn norm_squared(self) -> Complex {
        Complex::from(self.real * self.real + self.imag * self.imag, 0.)
    }

    fn norm(self) -> Complex {
        Complex::from(self.norm_squared().real.sqrt(), 0.)
    }

    fn exp(self) -> Complex {
        let real_part = self.real.exp();
        Complex::from(real_part * self.imag.cos(),
                      real_part * self.imag.sin())
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::from(self.real + other.real,
                     self.imag + other.imag)
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex::from(self.real - other.real,
                     self.imag - other.imag)
    }
}

#[cfg(test)]
#[test]
fn test_complex() {
    let c = Complex::from(PI, 0.);
    let a = Complex::from(1., 1.);
    let b = Complex::from(-1., -1.);
    assert_eq!(a + b, Complex::from(0., 0.));
    assert_eq!(a - b, Complex::from(2., 2.));
    assert_eq!(a + b.conj(), Complex::from(0., 2.));
    assert_eq!(a.real() + a.imag(), a);
    assert_eq!(Complex::zero().exp(), Complex::from(1., 0.));
}


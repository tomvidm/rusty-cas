#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Div, Neg};

pub type ComplexType = f64;
pub type RealType = f64;
pub type IntegerType = i64;

// Numeric type
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Numeric {
    Real(RealType),
    Complex(ComplexType),
    Integer(IntegerType)
}

impl Numeric {
    pub fn one() -> Numeric {
        Numeric::Integer(1)
    }

    pub fn zero() -> Numeric {
        Numeric::Integer(0)
    }

    pub fn from_real(real: RealType) -> Numeric {
        Numeric::Real(real)
    }

    pub fn from_complex(complex: ComplexType) -> Numeric {
        Numeric::Complex(complex)
    }

    pub fn from_integer(integer: IntegerType) -> Numeric {
        Numeric::Integer(integer)
    }

    pub fn to_real(self) -> RealType {
        match self {
            Numeric::Real(real) => return real,
            Numeric::Complex(complex) => return complex as RealType,
            Numeric::Integer(integer) => return integer as RealType 
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Numeric::Real(real) => return *real == 0.,
            Numeric::Complex(complex) => return *complex == 0.,
            Numeric::Integer(integer) => return *integer == 0
        }
    }

    pub fn is_unity(&self) -> bool {
        match self {
            Numeric::Real(real) => return *real == 1.,
            Numeric::Complex(complex) => return *complex == 1.,
            Numeric::Integer(integer) => return *integer == 1
        }
    }

    pub fn pow(&self, pow: IntegerType) -> Numeric {
        match self {
            Numeric::Real(real) => return Numeric::from_real(real.powi(pow as i32)),
            Numeric::Complex(complex) => return Numeric::from_complex(complex.powi(pow as i32)),
            Numeric::Integer(integer) => return Numeric::from_integer(integer.pow(pow as u32))
        }
    }
}

impl Neg for Numeric {
    type Output = Numeric;

    fn neg(self) -> Numeric {
        match self {
            Numeric::Real(real) => return Numeric::from_real(-real),
            Numeric::Complex(complex) => return Numeric::from_complex(-complex),
            Numeric::Integer(integer) => return Numeric::from_integer(-integer)
        }
    }
}

impl Add for Numeric {
    type Output = Numeric;

    fn add(self, other: Numeric) -> Numeric {
        match self {
            Numeric::Real(lhs_real) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real,
                    Numeric::Complex(rhs_complex) => rhs_complex as RealType,
                    Numeric::Integer(rhs_integer) => rhs_integer as RealType
                };
                return Numeric::from_real(lhs_real + rhs)
            },
            Numeric::Complex(lhs_complex) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real as ComplexType,
                    Numeric::Complex(rhs_complex) => rhs_complex,
                    Numeric::Integer(rhs_integer) => rhs_integer as ComplexType
                };
                return Numeric::from_complex(lhs_complex + rhs)
            },
            Numeric::Integer(lhs_integer) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real as IntegerType,
                    Numeric::Complex(rhs_complex) => rhs_complex as IntegerType,
                    Numeric::Integer(rhs_integer) => rhs_integer
                };
                return Numeric::from_integer(lhs_integer + rhs)
            }
        }
    }
}

impl Sub for Numeric {
    type Output = Numeric;

    fn sub(self, other: Numeric) -> Numeric {
        match self {
            Numeric::Real(lhs_real) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real,
                    Numeric::Complex(rhs_complex) => rhs_complex as RealType,
                    Numeric::Integer(rhs_integer) => rhs_integer as RealType
                };
                return Numeric::from_real(lhs_real - rhs)
            },
            Numeric::Complex(lhs_complex) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real as ComplexType,
                    Numeric::Complex(rhs_complex) => rhs_complex,
                    Numeric::Integer(rhs_integer) => rhs_integer as ComplexType
                };
                return Numeric::from_complex(lhs_complex - rhs)
            },
            Numeric::Integer(lhs_integer) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real as IntegerType,
                    Numeric::Complex(rhs_complex) => rhs_complex as IntegerType,
                    Numeric::Integer(rhs_integer) => rhs_integer
                };
                return Numeric::from_integer(lhs_integer - rhs)
            }
        }
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, other: Numeric) -> Numeric {
        match self {
            Numeric::Real(lhs_real) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real,
                    Numeric::Complex(rhs_complex) => rhs_complex as RealType,
                    Numeric::Integer(rhs_integer) => rhs_integer as RealType
                };
                return Numeric::from_real(lhs_real * rhs)
            },
            Numeric::Complex(lhs_complex) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real as ComplexType,
                    Numeric::Complex(rhs_complex) => rhs_complex,
                    Numeric::Integer(rhs_integer) => rhs_integer as ComplexType
                };
                return Numeric::from_complex(lhs_complex * rhs)
            },
            Numeric::Integer(lhs_integer) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real as IntegerType,
                    Numeric::Complex(rhs_complex) => rhs_complex as IntegerType,
                    Numeric::Integer(rhs_integer) => rhs_integer
                };
                return Numeric::from_integer(lhs_integer * rhs)
            }
        }
    }
}

impl Div for Numeric {
    type Output = Numeric;

    fn div(self, other: Numeric) -> Numeric {
        match self {
            Numeric::Real(lhs_real) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real,
                    Numeric::Complex(rhs_complex) => rhs_complex as RealType,
                    Numeric::Integer(rhs_integer) => rhs_integer as RealType
                };
                return Numeric::from_real(lhs_real / rhs)
            },
            Numeric::Complex(lhs_complex) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real as ComplexType,
                    Numeric::Complex(rhs_complex) => rhs_complex,
                    Numeric::Integer(rhs_integer) => rhs_integer as ComplexType
                };
                return Numeric::from_complex(lhs_complex / rhs)
            },
            Numeric::Integer(lhs_integer) => {
                let rhs = match other {
                    Numeric::Real(rhs_real) => rhs_real as IntegerType,
                    Numeric::Complex(rhs_complex) => rhs_complex as IntegerType,
                    Numeric::Integer(rhs_integer) => rhs_integer
                };
                return Numeric::from_integer(lhs_integer / rhs)
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_basic_operations() {
    let a = Numeric::from_real(1.);
    let b = Numeric::from_integer(1);

    assert_eq!(a + b, Numeric::from_real(2.));
}
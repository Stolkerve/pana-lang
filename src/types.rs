use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Numeric {
    Int(i128), // dependiendo del build, tendras: 32 o 64 bits de numero entero
    Float(f64),
}

impl Neg for Numeric {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Numeric::Int(int) => Numeric::Int(-int),
            Numeric::Float(float) => Numeric::Float(-float),
        }
    }
}

impl Add for Numeric {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Numeric::Int(a), Numeric::Int(b)) => Numeric::Int(a + b),
            (Numeric::Int(a), Numeric::Float(b)) => Numeric::Float(a as f64 + b),
            (Numeric::Float(a), Numeric::Int(b)) => Numeric::Float(a + b as f64),
            (Numeric::Float(a), Numeric::Float(b)) => Numeric::Float(a + b),
        }
    }
}

impl Sub for Numeric {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Numeric::Int(a), Numeric::Int(b)) => Numeric::Int(a - b),
            (Numeric::Int(a), Numeric::Float(b)) => Numeric::Float(a as f64 - b),
            (Numeric::Float(a), Numeric::Int(b)) => Numeric::Float(a - b as f64),
            (Numeric::Float(a), Numeric::Float(b)) => Numeric::Float(a - b),
        }
    }
}

impl Mul for Numeric {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Numeric::Int(a), Numeric::Int(b)) => Numeric::Int(a * b),
            (Numeric::Int(a), Numeric::Float(b)) => Numeric::Float(a as f64 * b),
            (Numeric::Float(a), Numeric::Int(b)) => Numeric::Float(a * b as f64),
            (Numeric::Float(a), Numeric::Float(b)) => Numeric::Float(a * b),
        }
    }
}

impl Div for Numeric {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Numeric::Int(a), Numeric::Int(b)) => Numeric::Int(a / b),
            (Numeric::Int(a), Numeric::Float(b)) => Numeric::Float(a as f64 / b),
            (Numeric::Float(a), Numeric::Int(b)) => Numeric::Float(a / b as f64),
            (Numeric::Float(a), Numeric::Float(b)) => Numeric::Float(a / b),
        }
    }
}

impl Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Numeric::Int(int) => write!(f, "{}", int),
            Numeric::Float(float) => write!(f, "{}", float),
        }
    }
}

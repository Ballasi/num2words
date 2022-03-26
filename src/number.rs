use std::fmt;
use std::ops::{Add, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Number {
    pub fn as_i64(&self) -> i64 {
        match *self {
            Number::Int(val) => val,
            Number::Float(val) => val as i64,
        }
    }

    pub fn as_f64(&self) -> f64 {
        match *self {
            Number::Int(val) => val as f64,
            Number::Float(val) => val,
        }
    }
}

impl Into<Number> for i64 {
    fn into(self) -> Number {
        Number::Int(self)
    }
}

impl Into<Number> for f64 {
    fn into(self) -> Number {
        Number::Float(self)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(val) => write!(f, "{}", val),
            Number::Float(val) => write!(f, "{}", val),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Number::Int(i), Number::Int(j)) => Number::Int(i + j),
            (Number::Float(i), Number::Int(j)) => Number::Float(i + j as f64),
            (Number::Int(i), Number::Float(j)) => Number::Float(i as f64 + j),
            (Number::Float(i), Number::Float(j)) => Number::Float(i + j),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Number::Int(i), Number::Int(j)) => Number::Int(i - j),
            (Number::Float(i), Number::Int(j)) => Number::Float(i - j as f64),
            (Number::Int(i), Number::Float(j)) => Number::Float(i as f64 - j),
            (Number::Float(i), Number::Float(j)) => Number::Float(i - j),
        }
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Number::Int(val) => Number::Int(-val),
            Number::Float(val) => Number::Float(-val),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Number;

    #[test]
    fn add() {
        let i1 = Number::Int(1);
        let i2 = Number::Int(2);
        let f1 = Number::Float(0.5);
        let f2 = Number::Float(1.5);

        assert_eq!(i1 + i2, Number::Int(3));
        assert_eq!(f1 + i2, Number::Float(2.5));
        assert_eq!(i2 + f2, Number::Float(3.5));
        assert_eq!(f1 + f2, Number::Float(2.));
    }

    #[test]
    fn sub() {
        let i1 = Number::Int(1);
        let i2 = Number::Int(2);
        let f1 = Number::Float(0.5);
        let f2 = Number::Float(1.5);

        assert_eq!(i2 - i1, Number::Int(1));
        assert_eq!(i2 - f1, Number::Float(1.5));
        assert_eq!(i2 - f2, Number::Float(0.5));
        assert_eq!(f2 - f1, Number::Float(1.));
    }

    #[test]
    fn neg() {
        let i1 = Number::Int(1);
        let f1 = Number::Float(0.5);

        assert_eq!(-i1, Number::Int(-1));
        assert_eq!(-f1, Number::Float(-0.5));
    }
}

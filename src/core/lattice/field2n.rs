
/*
  Represents and defines power-of-2 fields
 */

 use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg};
use num_complex::Complex64;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field2n {
    data: Vec<Complex64>,
    format: Format,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Coefficient,
    Evaluation,
}

impl Field2n {
    pub fn new(size: usize, format: Format, initialize_element_to_zero: bool) -> Self {
        let initial_value = if initialize_element_to_zero {
            Complex64::new(0.0, 0.0)
        } else {
            Complex64::new(f64::MIN, 0.0)
        };
        Field2n {
            data: vec![initial_value; size],
            format,
        }
    }

    pub fn get_format(&self) -> Format {
        self.format
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    // Other methods like Inverse, Plus, Minus, Times, etc. would be implemented here.
}

impl Add for Field2n {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Implementation for addition
        unimplemented!()
    }
}

impl AddAssign for Field2n {
    fn add_assign(&mut self, rhs: Self) {
        // Implementation for addition assignment
        unimplemented!()
    }
}

impl Sub for Field2n {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // Implementation for subtraction
        unimplemented!()
    }
}

impl SubAssign for Field2n {
    fn sub_assign(&mut self, rhs: Self) {
        // Implementation for subtraction assignment
        unimplemented!()
    }
}

impl Mul for Field2n {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // Implementation for multiplication
        unimplemented!()
    }
}

impl Neg for Field2n {
    type Output = Self;

    fn neg(self) -> Self::Output {
        // Implementation for negation
        unimplemented!()
    }
}

impl fmt::Display for Field2n {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        for val in &self.data {
            write!(f, "{} ", val)?;
        }
        write!(f, "]")
    }
}



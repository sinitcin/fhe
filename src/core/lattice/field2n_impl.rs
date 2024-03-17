/*
  implementation of the power-of-2 fields
 */

 use std::complex;
use std::vec::Vec;

pub struct Field2n {
    format: Format,
    elements: Vec<complex::Complex<f64>>,
}

impl Field2n {
    pub fn new(element: Poly) -> Self {
        if element.get_format() != Format::COEFFICIENT {
            panic!("Poly not in Format::COEFFICIENT representation");
        }
        let size = element.get_length();
        let mut elements = Vec::with_capacity(size);
        let negative_threshold = element.get_modulus() / Poly::Integer(2);
        for i in 0..size {
            if element[i] > negative_threshold {
                elements.push(complex::Complex::new(
                    -1.0 * (element.get_modulus() - element[i]).convert_to_int() as f64,
                    0.0,
                ));
            } else {
                elements.push(complex::Complex::new(
                    element[i].convert_to_int() as f64,
                    0.0,
                ));
            }
        }
        Field2n {
            format: Format::COEFFICIENT,
            elements,
        }
    }

    pub fn inverse(&self) -> Self {
        if self.format == Format::COEFFICIENT {
            panic!("Polynomial not in Format::EVALUATION representation");
        }
        let mut inverse = self.clone();
        for i in 0..inverse.elements.len() {
            let real = inverse.elements[i].re;
            let imag = inverse.elements[i].im;
            let quotient = real * real + imag * imag;
            inverse.elements[i] = complex::Complex::new(real / quotient, -imag / quotient);
        }
        inverse
    }

    pub fn plus(&self, rhs: &Field2n) -> Self {
        if self.format != rhs.get_format() {
            panic!("Operands are not in the same format");
        }
        let mut sum = self.clone();
        for i in 0..rhs.elements.len() {
            sum.elements[i] += rhs.elements[i];
        }
        sum
    }

    pub fn plus_scalar(&self, scalar: f64) -> Self {
        if self.format != Format::COEFFICIENT {
            panic!("Field2n scalar addition is currently supported only for Format::COEFFICIENT representation");
        }
        let mut sum = self.clone();
        sum.elements[0] += scalar;
        sum
    }

    pub fn minus(&self, rhs: &Field2n) -> Self {
        if self.format != rhs.get_format() {
            panic!("Operands are not in the same format");
        }
        let mut difference = self.clone();
        for i in 0..rhs.elements.len() {
            difference.elements[i] -= rhs.elements[i];
        }
        difference
    }

    pub fn times(&self, rhs: &Field2n) -> Self {
        if self.format != Format::EVALUATION && rhs.get_format() != Format::EVALUATION {
            panic!("At least one of the polynomials is not in Format::EVALUATION representation");
        }
        let mut result = self.clone();
        for i in 0..rhs.elements.len() {
            result.elements[i] *= rhs.elements[i];
        }
        result
    }

    pub fn shift_right(&self) -> Self {
        if self.format != Format::COEFFICIENT {
            panic!("Polynomial not in Format::COEFFICIENT representation");
        }
        let mut result = self.clone();
        let size = result.elements.len();
        let tmp = complex::Complex::new(-1.0, 0.0) * result.elements[size - 1];
        for i in (1..size).rev() {
            result.elements[i] = result.elements[i - 1];
        }
        result.elements[0] = tmp;
        result
    }

    pub fn automorphism_transform(&self, i: usize) -> Self {
        if self.format != Format::EVALUATION {
            panic!("Field2n Automorphism is only implemented for Format::EVALUATION format");
        }
        if i % 2 == 0 {
            panic!("automorphism index should be odd");
        }
        let mut result = self.clone();
        let m = result.elements.len() * 2;
        for j in (1..m).step_by(2) {
            let idx = (j * i) % m;
            result.elements[(idx + 1) / 2 - 1] = self.elements[(j + 1) / 2 - 1];
        }
        result
    }

    pub fn transpose(&self) -> Self {
        let size = self.elements.len();
        if self.format != Format::COEFFICIENT {
            return self.automorphism_transform(size * 2 - 1);
        }
        let negone = complex::Complex::new(-1.0, 0.0);
        let mut transpose = Field2n {
            format: Format::COEFFICIENT,
            elements: Vec::with_capacity(size),
        };
        transpose.elements.push(self.elements[0]);
        for i in 1..size {
            transpose.elements.push(negone * self.elements[size - i]);
        }
        transpose
    }

    pub fn extract_odd(&self) -> Self {
        if self.format != Format::COEFFICIENT {
            panic!("Polynomial not in Format::COEFFICIENT representation");
        }
        let size = self.elements.len();
        let mut odds = Field2n {
            format: Format::COEFFICIENT,
            elements: Vec::with_capacity(size / 2),
        };
        for i in 0..odds.elements.len() {
            odds.elements[i] = self.elements[1 + 2 * i];
        }
        odds
    }

    pub fn extract_even(&self) -> Self {
        if self.format != Format::COEFFICIENT {
            panic!("Polynomial not in Format::COEFFICIENT representation");
        }
        let size = self.elements.len();
        let mut evens = Field2n {
            format: Format::COEFFICIENT,
            elements: Vec::with_capacity(size / 2),
        };
        for i in 0..evens.elements.len() {
            evens.elements[i] = self.elements[0 + 2 * i];
        }
        evens
    }

    pub fn permute(&self) -> Self {
        if self.format != Format::COEFFICIENT {
            panic!("Polynomial not in Format::COEFFICIENT representation");
        }
        let size = self.elements.len();
        let mut permuted = Field2n {
            format: Format::COEFFICIENT,
            elements: Vec::with_capacity(size),
        };
        let mut even_ptr = 0;
        let mut odd_ptr = size / 2;
        for i in 0..size {
            permuted.elements[even_ptr] = self.elements[i];
            even_ptr += 1;
            permuted.elements[odd_ptr] = self.elements[i];
            odd_ptr += 1;
        }
        permuted
    }

    pub fn inverse_permute(&self) -> Self {
        if self.format != Format::COEFFICIENT {
            panic!("Polynomial not in Format::COEFFICIENT representation");
        }
        let size = self.elements.len();
        let mut invpermuted = Field2n {
            format: Format::COEFFICIENT,
            elements: Vec::with_capacity(size),
        };
        let mut even_ptr = 0;
        let mut odd_ptr = size / 2;
        for i in 0..size {
            invpermuted.elements[i] = self.elements[even_ptr];
            even_ptr += 1;
            invpermuted.elements[i] = self.elements[odd_ptr];
            odd_ptr += 1;
        }
        invpermuted
    }

    pub fn scalar_mult(&self, d: f64) -> Self {
        let size = self.elements.len();
        let mut scaled = Field2n {
            format: self.format,
            elements: Vec::with_capacity(size),
        };
        for i in 0..size {
            scaled.elements[i] = d * self.elements[i];
        }
        scaled
    }

    pub fn switch_format(&mut self) {
        let r = if self.format == Format::COEFFICIENT {
            DiscreteFourierTransform::forward_transform(self)
        } else {
            DiscreteFourierTransform::inverse_transform(self)
        };
        self.format = if self.format == Format::COEFFICIENT {
            Format::EVALUATION
        } else {
            Format::COEFFICIENT
        };
        for i in 0..r.elements.len() {
            self.elements[i] = r.elements[i];
        }
    }
}

#[derive(Clone, Copy)]
pub enum Format {
    COEFFICIENT,
    EVALUATION,
}

pub struct Poly {
    // implementation details
}

impl Poly {
    pub fn get_format(&self) -> Format {
        // implementation details
    }

    pub fn get_length(&self) -> usize {
        // implementation details
    }

    pub fn get_modulus(&self) -> Poly::Integer {
        // implementation details
    }

    pub fn convert_to_int(&self) -> i64 {
        // implementation details
    }
}

pub struct DiscreteFourierTransform {
    // implementation details
}

impl DiscreteFourierTransform {
    pub fn forward_transform(field: &Field2n) -> Field2n {
        // implementation details
    }

    pub fn inverse_transform(field: &Field2n) -> Field2n {
        // implementation details
    }
}




/*
  Provides the utility for sampling trapdoor lattices as described in https://eprint.iacr.org/2017/844.pdf
  https://eprint.iacr.org/2018/946, and "Implementing Token-Based Obfuscation under (Ring) LWE" as described in
  https://eprint.iacr.org/2018/1222.pdf.
 */

 use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(rows * cols, data.len());
        Matrix { data, rows, cols }
    }

    fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }
}

impl<T: Default + Clone> Matrix<T> {
    fn new_default(rows: usize, cols: usize) -> Self {
        let data = vec![T::default(); rows * cols];
        Matrix { data, rows, cols }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{} ", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.get(i, j) != other.get(i, j) {
                    return false;
                }
            }
        }
        true
    }
}

impl<T: Add<Output = T> + Clone> Add for Matrix<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut result = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                *result.get_mut(i, j) = self.get(i, j).clone() + other.get(i, j).clone();
            }
        }
        result
    }
}

impl<T: Sub<Output = T> + Clone> Sub for Matrix<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut result = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                *result.get_mut(i, j) = self.get(i, j).clone() - other.get(i, j).clone();
            }
        }
        result
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Clone> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::new_default(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    *result.get_mut(i, j) =
                        result.get(i, j).clone() + self.get(i, k).clone() * other.get(k, j).clone();
                }
            }
        }
        result
    }
}

#[derive(Clone, Debug)]
struct Element {
    value: i64,
}

impl Element {
    fn new(value: i64) -> Self {
        Element { value }
    }
}

impl Default for Element {
    fn default() -> Self {
        Element { value: 0 }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Add for Element {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Element::new(self.value + other.value)
    }
}

impl Sub for Element {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Element::new(self.value - other.value)
    }
}

impl Mul for Element {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Element::new(self.value * other.value)
    }
}

#[derive(Clone, Debug)]
struct RLWETrapdoorPair<Element> {
    m_r: Matrix<Element>,
    m_e: Matrix<Element>,
}

impl<Element> RLWETrapdoorPair<Element> {
    fn new(r: Matrix<Element>, e: Matrix<Element>) -> Self {
        RLWETrapdoorPair { m_r: r, m_e: e }
    }
}

#[derive(Clone, Debug)]
struct RLWETrapdoorUtility<Element> {
    params: Element,
    stddev: f64,
    base: i64,
    bal: bool,
}

impl<Element> RLWETrapdoorUtility<Element> {
    fn new(params: Element, stddev: f64, base: i64, bal: bool) -> Self {
        RLWETrapdoorUtility {
            params,
            stddev,
            base,
            bal,
        }
    }

    fn trapdoor_gen(
        &self,
        params: Element,
        stddev: f64,
        base: i64,
        bal: bool,
    ) -> (Matrix<Element>, RLWETrapdoorPair<Element>) {
        unimplemented!()
    }

    fn trapdoor_gen_square_mat(
        &self,
        params: Element,
        stddev: f64,
        dimension: usize,
        base: i64,
        bal: bool,
    ) -> (Matrix<Element>, RLWETrapdoorPair<Element>) {
        unimplemented!()
    }

    fn gauss_samp(
        &self,
        n: usize,
        k: usize,
        a: Matrix<Element>,
        t: RLWETrapdoorPair<Element>,
        u: Element,
        dgg: DggType,
        dgg_large_sigma: DggType,
        base: i64,
    ) -> Matrix<Element> {
        unimplemented!()
    }

    fn gauss_samp_square_mat(
        &self,
        n: usize,
        k: usize,
        a: Matrix<Element>,
        t: RLWETrapdoorPair<Element>,
        u: Matrix<Element>,
        dgg: DggType,
        dgg_large_sigma: DggType,
        base: i64,
    ) -> Matrix<Element> {
        unimplemented!()
    }

    fn gauss_samp_online(
        &self,
        n: usize,
        k: usize,
        a: Matrix<Element>,
        t: RLWETrapdoorPair<Element>,
        u: Element,
        dgg: DggType,
        perturbation_vector: Matrix<Element>,
        base: i64,
    ) -> Matrix<Element> {
        unimplemented!()
    }

    fn gauss_samp_offline(
        &self,
        n: usize,
        k: usize,
        t: RLWETrapdoorPair<Element>,
        dgg: DggType,
        dgg_large_sigma: DggType,
        base: i64,
    ) -> Matrix<Element> {
        unimplemented!()
    }

    fn z_sample_sigma_p(
        &self,
        n: usize,
        s: f64,
        sigma: f64,
        t_prime: RLWETrapdoorPair<Element>,
        dgg: DggType,
        dgg_large_sigma: DggType,
        perturbation_vector: Matrix<Element>,
    ) {
        unimplemented!()
    }

    fn sample_pert_square_mat(
        &self,
        n: usize,
        s: f64,
        sigma: f64,
        t_prime: RLWETrapdoorPair<Element>,
        dgg: DggType,
        dgg_large_sigma: DggType,
        perturbation_vector: Matrix<Element>,
    ) {
        unimplemented!()
    }
}
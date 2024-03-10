/*
  Provides the utility for sampling trapdoor lattices as described in https://eprint.iacr.org/2017/844.pdf,
  https://eprint.iacr.org/2018/946, and "Implementing Token-Based Obfuscation under (Ring) LWE" as described in
  https://eprint.iacr.org/2018/1222.pdf.
 */

 use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    fn new(data: Vec<T>, rows: usize, cols: usize) -> Self {
        Matrix {
            data,
            rows,
            cols,
        }
    }
}

impl<T> Matrix<T>
where
    T: Clone + Default,
{
    fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }
}

impl<T> Matrix<T>
where
    T: Clone + Default + Add<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    fn mult(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.cols, other.rows);

        let mut result = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.data[i * result.cols + j] +=
                        self.data[i * self.cols + k].clone() * other.data[k * other.cols + j].clone();
                }
            }
        }

        result
    }
}

#[derive(Clone, Debug)]
struct Element {
    modulus: i64,
}

impl Element {
    fn new(modulus: i64) -> Self {
        Element { modulus }
    }
}

#[derive(Clone, Debug)]
struct RLWETrapdoorPair<Element> {
    m_e: Matrix<Element>,
    m_r: Matrix<Element>,
}

impl<Element> RLWETrapdoorPair<Element> {
    fn new(m_e: Matrix<Element>, m_r: Matrix<Element>) -> Self {
        RLWETrapdoorPair { m_e, m_r }
    }
}

#[derive(Clone, Debug)]
struct DggType;

impl DggType {
    fn generate_integer_karney(&self, _min: i64, _max: i64) -> i64 {
        // implementation omitted
        0
    }
}

#[derive(Clone, Debug)]
struct ParmType;

impl ParmType {
    fn get_modulus(&self) -> i64 {
        // implementation omitted
        0
    }
}

#[derive(Clone, Debug)]
struct DCRTPoly {
    params: Rc<ParmType>,
}

impl DCRTPoly {
    type DggType = DggType;

    fn new(params: Rc<ParmType>) -> Self {
        DCRTPoly { params }
    }

    fn get_params(&self) -> Rc<ParmType> {
        Rc::clone(&self.params)
    }
}

#[derive(Clone, Debug)]
struct NativePoly {
    format: Format,
}

impl NativePoly {
    fn new(format: Format) -> Self {
        NativePoly { format }
    }
}

#[derive(Clone, Debug)]
enum Format {
    Evaluation,
    Coefficient,
}

#[derive(Clone, Debug)]
struct Field2n;

impl Field2n {
    fn scalar_mult(&self, _scalar: f64) -> Self {
        // implementation omitted
        Field2n
    }
}

#[derive(Clone, Debug)]
struct LatticeGaussSampUtility<Element> {
    _element: PhantomData<Element>,
}

impl<Element> LatticeGaussSampUtility<Element> {
    fn gauss_samp_gq_arb_base(
        _perturbed_syndrome: Element,
        _c: f64,
        _k: usize,
        _modulus: i64,
        _base: i64,
        _dgg: &DggType,
        _z_hat_bbi: &mut Matrix<i64>,
    ) {
        // implementation omitted
    }
}

#[derive(Clone, Debug)]
struct RLWETrapdoorUtility<Element> {
    _element: PhantomData<Element>,
}

impl<Element> RLWETrapdoorUtility<Element> {
    fn gauss_samp_online(
        _n: usize,
        _k: usize,
        _a: &Matrix<Element>,
        _t: &RLWETrapdoorPair<Element>,
        _u: &Element,
        _dgg: &DggType,
        _p_hat: &Matrix<Element>,
        _base: i64,
    ) -> Matrix<Element> {
        // implementation omitted
        Matrix::zeros(0, 0)
    }

    fn gauss_samp_offline(
        _n: usize,
        _k: usize,
        _t: &RLWETrapdoorPair<Element>,
        _dgg: &DggType,
        _dgg_large_sigma: &DggType,
        _base: i64,
    ) -> Rc<Matrix<Element>> {
        // implementation omitted
        Rc::new(Matrix::zeros(0, 0))
    }
}

impl RLWETrapdoorUtility<DCRTPoly> {
    fn z_sample_sigma_p(
        _n: usize,
        _s: f64,
        _sigma: f64,
        _t_prime: &RLWETrapdoorPair<DCRTPoly>,
        _dgg: &DCRTPoly::DggType,
        _dgg_large_sigma: &DCRTPoly::DggType,
        _perturbation_vector: &mut Rc<Matrix<DCRTPoly>>,
    ) {
        // implementation omitted
    }
}

fn split_int64_into_elements<T>(matrix: Matrix<i64>, _n: usize, _params: Rc<ParmType>) -> Matrix<T>
where
    T: Clone + Default,
{
    // implementation omitted
    Matrix::zeros(0, 0)
}

/*
  Provides detailed algorithms for G-sampling and perturbation sampling as described in https://eprint.iacr.org/2017/844.pdf,
  https://eprint.iacr.org/2018/946, and "Implementing Token-Based Obfuscation under (Ring) LWE" as described in
  https://eprint.iacr.org/2018/1222.pdf
 */

 use std::cmp::Ordering;
use std::f64::consts::PI;
use std::sync::Arc;
use std::vec::Vec;

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Matrix<T> {
    pub fn new(init: impl Fn() -> T, rows: usize, cols: usize) -> Self {
        let data = vec![init(); rows * cols];
        Self { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if let Some(cell) = self.get_mut(row, col) {
            *cell = value;
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

pub struct Field2n {
    data: Vec<f64>,
    size: usize,
}

impl Field2n {
    pub fn new(size: usize) -> Self {
        let data = vec![0.0; size];
        Self { data, size }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, index: usize) -> Option<&f64> {
        if index < self.size {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut f64> {
        if index < self.size {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize, value: f64) {
        if let Some(cell) = self.get_mut(index) {
            *cell = value;
        }
    }

    pub fn extract_even(&self) -> Self {
        let mut even = Self::new(self.size / 2);
        for i in (0..self.size).step_by(2) {
            even.set(i / 2, self.data[i]);
        }
        even
    }

    pub fn extract_odd(&self) -> Self {
        let mut odd = Self::new(self.size / 2);
        for i in (1..self.size).step_by(2) {
            odd.set(i / 2, self.data[i]);
        }
        odd
    }

    pub fn switch_format(&mut self) {
        for i in 0..self.size {
            self.data[i] = self.data[i].round();
        }
    }

    pub fn set_format(&mut self, format: Format) {
        match format {
            Format::Coefficient => self.switch_format(),
            Format::Evaluation => {}
        }
    }

    pub fn inverse(&self) -> Self {
        let mut inverse = Self::new(self.size);
        for i in 0..self.size {
            inverse.set(i, 1.0 / self.data[i]);
        }
        inverse
    }

    pub fn transpose(&self) -> Self {
        let mut transpose = Self::new(self.size);
        for i in 0..self.size {
            transpose.set(i, self.data[i]);
        }
        transpose
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 1.0;
        for i in 0..self.size {
            det *= self.data[i];
        }
        det
    }

    pub fn cofactor_matrix(&self) -> Self {
        let mut cofactor = Self::new(self.size);
        for i in 0..self.size {
            let mut product = 1.0;
            for j in 0..self.size {
                if i != j {
                    product *= self.data[j];
                }
            }
            cofactor.set(i, product);
        }
        cofactor
    }
}

pub enum Format {
    Coefficient,
    Evaluation,
}

pub struct DggType {
    // implementation details
}

impl DggType {
    pub fn generate_integer_karney(&self, mean: f64, stddev: f64) -> i64 {
        // implementation details
        0
    }
}

pub struct LatticeGaussSampUtility<Element> {
    // implementation details
}

impl<Element> LatticeGaussSampUtility<Element> {
    pub fn gauss_samp_gq(
        syndrome: &Element,
        stddev: f64,
        k: usize,
        q: &Element::Integer,
        base: i64,
        dgg: &mut Element::DggType,
        z: &mut Matrix<i64>,
    ) {
        let u = syndrome.crt_interpolate();
        let modulus = u.get_params().get_modulus();
        let sigma = stddev / (base + 1) as f64;
        let m_digits = get_digits(&modulus, base, k).unwrap().clone();

        let mut l = vec![0.0; k];
        let mut h = vec![0.0; k];
        let mut c = Matrix::new(|| 0.0, k, 1);

        l[0] = ((base as f64) * (1.0 + 1.0 / k as f64) + 1.0).sqrt();
        for i in 1..k {
            l[i] = ((base as f64) * (1.0 + 1.0 / (k - i) as f64)).sqrt();
        }
        h[0] = 0.0;
        for i in 1..k {
            h[i] = ((base as f64) * (1.0 - 1.0 / (k - (i - 1)) as f64)).sqrt();
        }

        c.set(0, 0, m_digits[0] as f64 / base as f64);
        for i in 1..k {
            c.set(i, 0, (c.get(i - 1, 0).unwrap() + m_digits[i] as f64) / base as f64);
        }

        for j in 0..u.get_length() {
            let v = u.at(j);
            let p = perturb(sigma, k, u.get_length(), &l, &h, base, dgg);
            let mut a = Matrix::new(|| 0.0, k, 1);
            let v_digits = get_digits(&v, base, k).unwrap().clone();

            a.set(0, 0, ((v_digits[0] as i64) - p[0]) as f64 / base as f64);
            for t in 1..k {
                a.set(
                    t,
                    0,
                    (a.get(t - 1, 0).unwrap() + (v_digits[t] as i64) - p[t]) as f64 / base as f64,
                );
            }

            let zj = sample_c(&c, k, u.get_length(), sigma, dgg, &a);
            z.set(0, j, base * zj[0] + (m_digits[0] as i64) * zj[k - 1] + (v_digits[0] as i64));
            for t in 1..k - 1 {
                z.set(
                    t,
                    j,
                    base * zj[t] - zj[t - 1] + (m_digits[t] as i64) * zj[k - 1] + (v_digits[t] as i64),
                );
            }
            z.set(
                k - 1,
                j,
                (m_digits[k - 1] as i64) * zj[k - 1] - zj[k - 2] + (v_digits[k - 1] as i64),
            );
        }
    }

    pub fn gauss_samp_gq_arb_base(
        syndrome: &Element,
        stddev: f64,
        k: usize,
        q: &Element::Integer,
        base: i64,
        dgg: &mut Element::DggType,
        z: &mut Matrix<i64>,
    ) {
        let u = syndrome.crt_interpolate();
        let modulus = u.get_params().get_modulus();
        let sigma = stddev / (base + 1) as f64;
        let m_digits = get_digits(&modulus, base, k).unwrap().clone();

        let mut l = vec![0.0; k];
        let mut h = vec![0.0; k];
        let mut c = Matrix::new(|| 0.0, k, 1);

        l[0] = ((base as f64) * (1.0 + 1.0 / k as f64) + 1.0).sqrt();
        for i in 1..k {
            l[i] = ((base as f64) * (1.0 + 1.0 / (k - i) as f64)).sqrt();
        }
        h[0] = 0.0;
        for i in 1..k {
            h[i] = ((base as f64) * (1.0 - 1.0 / (k - (i - 1)) as f64)).sqrt();
        }

        c.set(0, 0, (m_digits[0] as i64) as f64 / base as f64);
        for i in 1..k {
            c.set(
                i,
                0,
                ((c.get(i - 1, 0).unwrap() + (m_digits[i] as i64)) as f64) / base as f64,
            );
        }

        for j in 0..u.get_length() {
            let v = u.at(j);
            let v_digits = get_digits(&v, base, k).unwrap().clone();
            let p = perturb_float(sigma, k, u.get_length(), &l, &h, base, dgg);
            let mut a = Matrix::new(|| 0.0, k, 1);

            a.set(0, 0, ((v_digits[0] as i64) - p[0]) as f64 / base as f64);
            for t in 1..k {
                a.set(
                    t,
                    0,
                    (a.get(t - 1, 0).unwrap() + (v_digits[t] as i64) - p[t]) as f64 / base as f64,
                );
            }

            let zj = sample_c(&c, k, u.get_length(), sigma, dgg, &a);
            z.set(0, j, base * zj[0] + (m_digits[0] as i64) * zj[k - 1] + (v_digits[0] as i64));
            for t in 1..k - 1 {
                z.set(
                    t,
                    j,
                    base * zj[t] - zj[t - 1] + (m_digits[t] as i64) * zj[k - 1] + (v_digits[t] as i64),
                );
            }
            z.set(
                k - 1,
                j,
                (m_digits[k - 1] as i64) * zj[k - 1] - zj[k - 2] + (v_digits[k - 1] as i64),
            );
        }
    }

    pub fn perturb(
        sigma: f64,
        k: usize,
        n: usize,
        l: &[f64],
        h: &[f64],
        base: i64,
        dgg: &mut Element::DggType,
    ) -> Vec<i64> {
        let mut z = vec![0; k];
        let mut d = 0.0;
        for i in 0..k {
            z[i] = dgg.generate_integer_karney(d / l[i], sigma / l[i]);
            d = -z[i] * h[i];
        }
        let mut p = vec![0; k];
        p[0] = (2 * base + 1) * z[0] + base * z[1];
        for i in 1..k - 1 {
            p[i] = base * (z[i - 1] + 2 * z[i] + z[i + 1]);
        }
        p[k - 1] = base * (z[k - 2] + 2 * z[k - 1]);
        p
    }

    pub fn perturb_float(
        sigma: f64,
        k: usize,
        n: usize,
        l: &[f64],
        h: &[f64],
        base: i64,
        dgg: &mut Element::DggType,
    ) -> Vec<f64> {
        let mut z = vec![0.0; k];
        for i in 0..k {
            z[i] = dgg.generate_integer_karney(0.0, sigma);
        }
        let mut p = vec![0.0; k];
        for i in 0..k - 1 {
            p[i] = l[i] * z[i] + h[i + 1] * z[i + 1];
        }
        p[k - 1] = h[k - 1] * z[k - 1];
        p
    }

    pub fn sample_c(
        c: &Matrix<f64>,
        k: usize,
        n: usize,
        sigma: f64,
        dgg: &mut Element::DggType,
        a: &Matrix<f64>,
    ) -> Vec<i64> {
        let mut z = vec![0; k];
        z[k - 1] = dgg.generate_integer_karney(-a.get(k - 1, 0).unwrap() / c.get(k - 1, 0).unwrap(), sigma / c.get(k - 1, 0).unwrap());
        let mut a = a.clone();
        a.set(k - 1, 0, a.get(k - 1, 0).unwrap() + (z[k - 1] as f64) * c.get(k - 1, 0).unwrap());
        for i in 0..k - 1 {
            z[i] = dgg.generate_integer_karney(-a.get(i, 0).unwrap(), sigma);
        }
        z
    }

    pub fn z_sample_sigma2x2(
        a: &Field2n,
        b: &Field2n,
        d: &Field2n,
        c: &Matrix<Field2n>,
        dgg: &Element::DggType,
        q: &mut Matrix<i64>,
    ) {
        let n = a.size();
        let d_coeff = d.clone();
        d_coeff.set_format(Format::Coefficient);
        let q2_int = z_sample_f(&d_coeff, &c.get(1, 0).unwrap(), dgg, n);
        let mut q2 = Field2n::new(n);
        let q2_minusc2 = &q2 - &c.get(1, 0).unwrap();
        q2_minusc2.switch_format();
        let product = &b * &d.inverse() * &q2_minusc2;
        let c1 = &c.get(0, 0).unwrap() + &product;
        let f = &a - &b * &d.inverse() * &b.transpose();
        let q1_int = z_sample_f(&f, &c1, dgg, n);
        for i in 0..q1_int.rows() {
            q.set(i, 0, q1_int.get(i, 0).unwrap().clone());
        }
        for i in 0..q2_int.rows() {
            q.set(i + q1_int.rows(), 0, q2_int.get(i, 0).unwrap().clone());
        }
    }

    pub fn sample_mat(
        a: &Matrix<Field2n>,
        b: &Matrix<Field2n>,
        d: &Matrix<Field2n>,
        c: &Matrix<Field2n>,
        dgg: &Element::DggType,
        p: &mut Matrix<i64>,
    ) {
        let d = c.rows();
        if d == 2 {
            z_sample_sigma2x2(
                &a.get(0, 0).unwrap(),
                &b.get(0, 0).unwrap(),
                &d.get(0, 0).unwrap(),
                &c,
                dgg,
                p,
            );
            return;
        }
        let n = d.get(0, 0).unwrap().size();
        let dim_a = a.rows();
        let dim_d = d.rows();
        let mut q1_int = Matrix::new(|| 0, n * dim_d, 1);
        let mut c0 = Matrix::new(|| Field2n::new(n), dim_a, 1);
        let mut c1 = Matrix::new(|| Field2n::new(n), dim_d, 1);
        let mut q_f1 = Matrix::new(|| Field2n::new(n), dim_d, 1);
        let mut d_inverse = Matrix::new(|| Field2n::new(n), dim_d, dim_d);
        if dim_d == 1 {
            let d_eval = d.get(0, 0).unwrap();
            d_eval.set_format(Format::Coefficient);
            c1.set(0, 0, c.get(d - 1, 0).unwrap().clone());
            c0 = c.extract_rows(0, d - 2);
            q1_int = z_sample_f(&d_eval, &c1.get(0, 0).unwrap(), dgg, d_eval.size());
            d_inverse.set(0, 0, d.get(0, 0).unwrap().inverse());
            q_f1.set(0, 0, Field2n::new_from_matrix(&q1_int));
        } else if dim_d == 2 {
            c1 = c.extract_rows(dim_a, d - 1);
            c0 = c.extract_rows(0, dim_a - 1);
            z_sample_sigma2x2(
                &d.get(0, 0).unwrap(),
                &d.get(0, 1).unwrap(),
                &d.get(1, 1).unwrap(),
                &c1,
                dgg,
                &mut q1_int,
            );
            for i in 0..dim_d {
                q_f1.set(i, 0, Field2n::new_from_matrix(&q1_int.extract_rows(i * n, i * n + n - 1)));
            }
            let det = &d.get(0, 0).unwrap() * &d.get(1, 1).unwrap() - &d.get(0, 1).unwrap() * &d.get(1, 0).unwrap();
            let det_inverse = det.inverse();
            d_inverse.set(0, 0, &d.get(1, 1).unwrap() * &det_inverse);
            d_inverse.set(0, 1, -&d.get(0, 1).unwrap() * &det_inverse);
            d_inverse.set(1, 0, -&d.get(1, 0).unwrap() * &det_inverse);
            d_inverse.set(1, 1, &d.get(0, 0).unwrap() * &det_inverse);
        } else {
            c1 = c.extract_rows(dim_a, d - 1);
            c0 = c.extract_rows(0, dim_a - 1);
            let new_dim_a = (dim_d as f64 / 2.0).ceil() as usize;
            let new_dim_d = (dim_d as f64 / 2.0).floor() as usize;
            let mut new_a = Matrix::new(|| Field2n::new(n), new_dim_a, new_dim_a);
            let mut new_b = Matrix::new(|| Field2n::new(n), new_dim_a, new_dim_d);
            let mut new_d = Matrix::new(|| Field2n::new(n), new_dim_d, new_dim_d);
            for i in 0..new_dim_a {
                for j in 0..new_dim_a {
                    new_a.set(i, j, d.get(i, j).unwrap().clone());
                }
            }
            for i in 0..new_dim_a {
                for j in 0..new_dim_d {
                    new_b.set(i, j, d.get(i, j + new_dim_a).unwrap().clone());
                }
            }
            for i in 0..new_dim_d {
                for j in 0..new_dim_d {
                    new_d.set(i, j, d.get(i + new_dim_a, j + new_dim_a).unwrap().clone());
                }
            }
            sample_mat(&new_a, &new_b, &new_d, &c1, dgg, &mut q1_int);
            for i in 0..dim_d {
                q_f1.set(i, 0, Field2n::new_from_matrix(&q1_int.extract_rows(i * n, i * n + n - 1)));
            }
            let mut det = Field2n::new(n);
            d.get(0, 0).unwrap().determinant(&mut det);
            let mut det_inverse = det.inverse();
            let cofactor = d.get(0, 0).unwrap().cofactor_matrix();
            d_inverse = cofactor.transpose() * &det_inverse;
        }
        let sigma = &a - &b * &d_inverse * &q_f1;
        let diff = &q_f1 - &c1;
        let mut c_new = &c0 + &b * &d_inverse * &diff;
        let new_dim_a = (dim_a as f64 / 2.0).ceil() as usize;
        let new_dim_d = (dim_a as f64 / 2.0).floor() as usize;
        let mut new_a = Matrix::new(|| Field2n::new(n), new_dim_a, new_dim_a);
        let mut new_b = Matrix::new(|| Field2n::new(n), new_dim_a, new_dim_d);
        let mut new_d = Matrix::new(|| Field2n::new(n), new_dim_d, new_dim_d);
        for i in 0..new_dim_a {
            for j in 0..new_dim_a {
                new_a.set(i, j, sigma.get(i, j).unwrap().clone());
            }
        }
        for i in 0..new_dim_a {
            for j in 0..new_dim_d {
                new_b.set(i, j, sigma.get(i, j + new_dim_a).unwrap().clone());
            }
        }
        for i in 0..new_dim_d {
            for j in 0..new_dim_d {
                new_d.set(i, j, sigma.get(i + new_dim_a, j + new_dim_a).unwrap().clone());
            }
        }
        let mut q0 = Matrix::new(|| 0, n * dim_a, 1);
        sample_mat(&new_a, &new_b, &new_d, &c_new, dgg, &mut q0);
        *p = q0;
        p.v_stack(&q1_int);
    }

    pub fn z_sample_f(
        f: &Field2n,
        c: &Field2n,
        dgg: &Element::DggType,
        n: usize,
    ) -> Matrix<i64> {
        if f.size() == 1 {
            let mut p = Matrix::new(|| 0, 1, 1);
            p.set(0, 0, dgg.generate_integer_karney(c.get(0).unwrap().clone(), (f.get(0).unwrap().clone()).sqrt()));
            return p;
        }
        let f0 = f.extract_even();
        let f1 = f.extract_odd();
        let mut q_z_vector = Matrix::new(|| 0, f0.size() * 2, 1);
        let mut c_permuted = Matrix::new(|| Field2n::new(n), 2, 1);
        c_permuted.set(0, 0, c.extract_even());
        c_permuted.set(1, 0, c.extract_odd());
        z_sample_sigma2x2(&f0, &f1, &f0, &c_permuted, dgg, &mut q_z_vector);
        inverse_permute(&mut q_z_vector);
        q_z_vector
    }

    pub fn permute(p: &mut Matrix<i32>) -> Matrix<i32> {
        let mut permuted = Matrix::new(|| 0, p.rows(), 1);
        let mut even_ptr = 0;
        let mut odd_ptr = p.rows() / 2;
        for i in 0..p.rows() {
            if i % 2 == 0 {
                permuted.set(even_ptr, 0, p.get(i, 0).unwrap().clone());
                even_ptr += 1;
            } else {
                permuted.set(odd_ptr, 0, p.get(i, 0).unwrap().clone());
                odd_ptr += 1;
            }
        }
        permuted
    }

    pub fn inverse_permute(p: &mut Matrix<i64>) {
        let mut vector_permuted = vec![0; p.rows()];
        let mut even_ptr = 0;
        let mut odd_ptr = vector_permuted.len() / 2;
        for i in 0..vector_permuted.len() {
            vector_permuted[i] = p.get(i, 0).unwrap().clone();
        }
        for i in 0..vector_permuted.len() {
            if i % 2 == 0 {
                p.set(even_ptr, 0, vector_permuted[i]);
                even_ptr += 1;
            } else {
                p.set(odd_ptr, 0, vector_permuted[i]);
                odd_ptr += 1;
            }
        }
    }
}

pub trait Element {
    type Integer;
    type DggType: Dgg;
    fn crt_interpolate(&self) -> Self;
    fn get_length(&self) -> usize;
    fn at(&self, index: usize) -> Self::Integer;
    fn get_params(&self) -> Arc<Self>;
}

pub trait Dgg {
    fn generate_integer_karney(&self, mean: f64, stddev: f64) -> i64;
}

fn get_digits(q: &Field2n, base: i64, k: usize) -> Option<Vec<i64>> {
    let mut digits = Vec::new();
    let mut q = q.clone();
    for _ in 0..k {
        let digit = (q.get(0).unwrap() % base as f64) as i64;
        digits.push(digit);
        q.set(0, (q.get(0).unwrap() / base as f64).floor());
    }
    if q.get(0).unwrap() != 0.0 {
        None
    } else {
        digits.reverse();
        Some(digits)
    }
}

impl Element for Field2n {
    type Integer = Field2n;
    type DggType = DggType;

    fn crt_interpolate(&self) -> Self {
        // implementation details
        self.clone()
    }

    fn get_length(&self) -> usize {
        self.size()
    }

    fn at(&self, index: usize) -> Self::Integer {
        // implementation details
        self.clone()
    }

    fn get_params(&self) -> Arc<Self> {
        // implementation details
        Arc::new(self.clone())
    }
}

impl Dgg for DggType {
    fn generate_integer_karney(&self, mean: f64, stddev: f64) -> i64 {
        // implementation details
        0
    }
}
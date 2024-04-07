/*
  This code provides generation of gaussian distributions of discrete values. Discrete uniform generator relies on
  the built-in C++ generator for 32-bit unsigned integers defined in <random>
*/
use rand::Rng;
use std::cmp::Ordering;
use std::f64::consts::E;
use std::f64::consts::PI;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub struct DiscreteGaussianGeneratorImpl {
    m_std: f64,
    m_vals: Vec<f64>,
    m_a: f64,
}

impl DiscreteGaussianGeneratorImpl {
    pub fn new(std: f64) -> Self {
        Self {
            m_std: std,
            m_vals: Vec::new(),
            m_a: 0.0,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.m_std > 1.000000001
    }

    pub fn set_std(&mut self, std: f64) {
        if (self.m_std.log2() > 59.0) {
            panic!("Standard deviation cannot exceed 59 bits");
        }
        if (self.m_std = std) < KARNEY_THRESHOLD {
            self.initialize();
        }
    }

    pub fn get_std(&self) -> f64 {
        self.m_std
    }

    pub fn initialize(&mut self) {
        const ACC: f64 = 5e-32;
        const M: f64 = (-2.0 * ACC.log(E)).sqrt();
        let fin = (self.m_std * M).ceil() as i32;
        self.m_vals.clear();
        self.m_vals.reserve(fin as usize);
        let variance = 2.0 * self.m_std * self.m_std;
        let mut cusum = 0.0;
        for x in 1..=fin {
            cusum += (-((x * x) as f64) / variance).exp();
            self.m_vals.push(cusum);
        }
        self.m_a = 1.0 / (2.0 * cusum + 1.0);
        for x in 0..fin {
            self.m_vals[x as usize] *= self.m_a;
        }
    }

    pub fn generate_int(&self) -> i32 {
        let seed = rand::thread_rng().gen_range(0.0, 1.0) - 0.5;
        let tmp = seed.abs() - self.m_a / 2.0;
        if tmp <= 0.0 {
            return 0;
        }
        let index = self.find_in_vector(tmp);
        let val = index as i32 * if seed > 0.0 { 1 } else { -1 };
        val
    }

    pub fn generate_int_vector(&self, size: u32) -> Vec<i64> {
        let mut ans = Vec::with_capacity(size as usize);
        if !peikert {
            for _ in 0..size {
                ans.push(generate_integer_karney(0, self.m_std));
            }
            return ans;
        }
        for _ in 0..size {
            let seed = rand::thread_rng().gen_range(0.0, 1.0) - 0.5;
            let tmp = seed.abs() - self.m_a / 2.0;
            let val = if tmp > 0.0 {
                index = self.find_in_vector(tmp);
                index as i64 * if seed > 0.0 { 1 } else { -1 }
            } else {
                0
            };
            ans.push(val);
        }
        ans
    }

    pub fn find_in_vector(&self, s: f64, search: f64) -> u32 {
        let lower = self
            .m_vals
            .binary_search_by(|x| x.partial_cmp(&search).unwrap_or(Ordering::Equal));
        match lower {
            Ok(index) => index as u32 + 1,
            Err(_) => panic!(
                "DGG Inversion Sampling. FindInVector value not found: {}",
                search
            ),
        }
    }

    pub fn generate_integer(&self, modulus: i64) -> i64 {
        let seed = rand::thread_rng().gen_range(0.0, 1.0) - 0.5;
        let tmp = seed.abs() - self.m_a / 2.0;
        if tmp <= 0.0 {
            return 0;
        }
        let index = self.find_in_vector(tmp);
        let val = index as i32 * if seed > 0.0 { 1 } else { -1 };
        if val < 0 {
            modulus - (-val)
        } else {
            val
        }
    }

    pub fn generate_vector(&self, size: u32, modulus: i64) -> Vec<i64> {
        let result = self.generate_int_vector(size);
        let mut ans = Vec::with_capacity(size as usize);
        for i in 0..size {
            let v = result[i as usize];
            let val = if v < 0 { modulus - (-v) } else { v };
            ans.push(val);
        }
        ans
    }

    pub fn generate_integer(mean: f64, stddev: f64, n: usize, modulus: i64) -> i64 {
        let t = (n as f64).log2() * stddev;
        let uniform_int = rand::distributions::Uniform::from(mean - t..=mean + t);
        let uniform_real = rand::distributions::Uniform::from(0.0..=1.0);
        let mut x;
        loop {
            x = uniform_int.sample(&mut rand::thread_rng());
            if uniform_real.sample(&mut rand::thread_rng())
                > unnormalized_gaussian_pdf(mean, stddev, x)
            {
                break;
            }
        }
        if x < 0 {
            modulus - (-x)
        } else {
            x
        }
    }

    pub fn generate_integer(mean: f64, stddev: f64, n: usize) -> i32 {
        if mean.is_infinite() {
            panic!("DiscreteGaussianGeneratorImpl called with mean == +-inf");
        }
        if stddev.is_infinite() {
            panic!("DiscreteGaussianGeneratorImpl called with stddev == +-inf");
        }
        let t = (n as f64).log2() * stddev;
        let uniform_int = rand::distributions::Uniform::from(mean - t..=mean + t);
        let uniform_real = rand::distributions::Uniform::from(0.0..=1.0);
        let sigma_factor = 1.0 / (-2.0 * stddev * stddev);
        let mut count = 0;
        const LIMIT: u32 = 10000;
        let mut x;
        let mut flag_success = false;
        while !flag_success {
            x = uniform_int.sample(&mut rand::thread_rng());
            let dice = uniform_real.sample(&mut rand::thread_rng());
            flag_success = dice <= unnormalized_gaussian_pdf_optimized(mean, sigma_factor, x);
            count += 1;
            if count > LIMIT {
                panic!("GenerateInteger could not find success after repeated attempts");
            }
        }
        x
    }

    pub fn generate_integer_karney(mean: f64, stddev: f64) -> i64 {
        let mut result;
        let uniform_sign = rand::distributions::Uniform::from(0..=1);
        let uniform_j = rand::distributions::Uniform::from(0..=(stddev.ceil() as i64 - 1));
        let mut g = rand::thread_rng();
        let mut flag_success = false;
        let mut k;
        while !flag_success {
            k = algorithm_g(&mut g);
            if !algorithm_p(&mut g, k * (k - 1)) {
                continue;
            }
            let s = uniform_sign.sample(&mut g);
            let s = if s == 0 { -1 } else { 1 };
            let di0 = stddev * k as f64 + s as f64 * mean;
            let i0 = di0.ceil() as i64;
            let x0 = (i0 as f64 - di0) / stddev;
            let j = uniform_j.sample(&mut g);
            let x = x0 + j as f64 / stddev;
            if !(x < 1.0) || (x == 0.0 && s < 0 && k == 0) {
                continue;
            }
            let mut h = k + 1;
            while h > 0 && algorithm_b(&mut g, k, x) {
                h -= 1;
            }
            if h < 0 {
                result = s * (i0 + j);
                flag_success = true;
            }
        }
        result
    }

    pub fn algorithm_p(&mut self, g: &mut rand::rngs::ThreadRng, n: i32) -> bool {
        let mut n = n;
        while n > 0 && algorithm_h(g) {
            n -= 1;
        }
        n < 0
    }

    pub fn algorithm_g(&mut self, g: &mut rand::rngs::ThreadRng) -> i32 {
        let mut n = 0;
        while algorithm_h(g) {
            n += 1;
        }
        n
    }

    pub fn algorithm_h(&mut self, g: &mut rand::rngs::ThreadRng) -> bool {
        let dist = rand::distributions::Uniform::from(0.0..=1.0);
        let h_a = dist.sample(g);
        if h_a > 0.5 {
            return true;
        }
        if h_a < 0.5 {
            loop {
                let h_b = dist.sample(g);
                if h_b > h_a {
                    return false;
                } else if h_b < h_a {
                    h_a = dist.sample(g);
                } else {
                    return algorithm_h_double(g);
                }
                if h_a > h_b {
                    return true;
                } else if h_a == h_b {
                    return algorithm_h_double(g);
                }
            }
        } else {
            return algorithm_h_double(g);
        }
    }

    pub fn algorithm_h_double(&mut self, g: &mut rand::rngs::ThreadRng) -> bool {
        let dist = rand::distributions::Uniform::from(0.0..=1.0);
        let h_a = dist.sample(g);
        if !(h_a < 0.5) {
            return true;
        }
        loop {
            let h_b = dist.sample(g);
            if !(h_b < h_a) {
                return false;
            } else {
                h_a = dist.sample(g);
            }
            if !(h_a < h_b) {
                return true;
            }
        }
    }

    pub fn algorithm_b(&mut self, g: &mut rand::rngs::ThreadRng, k: i32, x: f64) -> bool {
        let dist = rand::distributions::Uniform::from(0.0..=1.0);
        let y = x;
        let mut n = 0;
        let m = 2 * k + 2;
        let mut z;
        let mut r;
        let r_temp;
        loop {
            z = dist.sample(g);
            if z > y {
                break;
            } else if z < y {
                r = dist.sample(g);
                r_temp = (2 * k + x) / m;
                if r > r_temp {
                    break;
                } else if r < r_temp {
                    y = z;
                } else {
                    return algorithm_b_double(g, k, x);
                }
            } else {
                return algorithm_b_double(g, k, x);
            }
        }
        n % 2 == 0
    }

    pub fn algorithm_b_double(&mut self, g: &mut rand::rngs::ThreadRng, k: i32, x: f64) -> bool {
        let dist = rand::distributions::Uniform::from(0.0..=1.0);
        let y = x;
        let mut n = 0;
        let m = 2 * k + 2;
        let mut z;
        let mut r;
        loop {
            z = dist.sample(g);
            if !(z < y) {
                break;
            }
            r = dist.sample(g);
            if !(r < (2 * k + x) / m) {
                break;
            }
            y = z;
        }
        n % 2 == 0
    }

    fn unnormalized_gaussian_pdf(mean: f64, stddev: f64, x: i64) -> f64 {
        let variance = stddev * stddev;
        let exponent = -((x as f64 - mean) * (x as f64 - mean)) / (2.0 * variance);
        (1.0 / (stddev * (2.0 * PI).sqrt())) * E.powf(exponent)
    }

    fn unnormalized_gaussian_pdf_optimized(mean: f64, sigma_factor: f64, x: i64) -> f64 {
        let exponent = -((x as f64 - mean) * (x as f64 - mean)) * sigma_factor;
        (1.0 / (stddev * (2.0 * PI).sqrt())) * E.powf(exponent)
    }
}

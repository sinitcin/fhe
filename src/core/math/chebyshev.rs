/*
 * This code provides Chebyshev approximation utilities.

Чебышев - необходим для аппроксимации многочленов
 Многочлены Чебышева первого рода

*/
use std::f64;
use std::vec::Vec;

pub fn eval_chebyshev_coefficients(func: fn(f64) -> f64, a: f64, b: f64, degree: u32) -> Vec<f64> {
    let mut coefficients: Vec<f64> = Vec::new();
    for i in 0..=degree {
        let x = ((b - a) / 2.0)
            * f64::cos(((2 * i + 1) as f64 / (2 * degree + 2) as f64) * f64::consts::PI)
            + ((a + b) / 2.0);
        let y = func(x);
        coefficients.push(y);
    }
    coefficients
}

// Матрицы не делаем
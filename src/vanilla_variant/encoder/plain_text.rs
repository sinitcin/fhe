use ndarray::{Array, Array1};
use ndarray_linalg::types::c64;
use std::ops::*;

/// Эта структура представляет основу для манипуляции с текстом на уровне
/// высокоуровневой абстракции в контексте схемы CKKS.Структура
/// сконцентрирована на представлении и манипуляции с полиномами. Или,
/// в другом контексте, могут представлять вектор коэффициентов для полинома,
/// где коэффициенты являются комплексными числами (c64).
#[derive(Debug, Clone)]
pub struct PlainText(pub Array1<c64>);

impl PlainText {
    pub fn new(vec: Array1<c64>) -> PlainText {
        PlainText(vec)
    }

    /// Вычисляет значение полинома в заданном коэффициенте
    #[allow(dead_code)]
    pub fn eval(&self, root: c64) -> c64 {
        let mut sum = c64::new(0f64, 0f64);
        for i in 0..self.0.len() {
            sum = sum + root.powu(i as u32) * self.0[i];
        }
        sum
    }

    /// Возвращает "размер" полинома, вычисленный как корень из суммы
    /// квадратов модулей коэффициентов. Размер функции:
    ///|h| = (a_0^2 + a_1^2 + ... + a_n^2 )^1/2
    #[allow(dead_code)]
    pub fn size(&self) -> c64 {
        self.0
            .iter()
            .fold(c64::new(0f64, 0f64), |sum, a| sum + a.powi(2))
            .powf(0.5)
    }
}

impl Add for PlainText {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        PlainText(self.0 + rhs.0)
    }
}

impl Mul for PlainText {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let d = self.0.len() + rhs.0.len() - 1;
        let mut poly = Array::zeros(d);

        for k in 0..=d {
            for i in 0..=k {
                if self.0.len() <= i || rhs.0.len() <= k - i {
                    continue;
                }
                poly[k] = poly[k] + self.0[i] * rhs.0[k - i];
            }
        }

        PlainText(poly)
    }
}

impl Div<usize> for PlainText {
    type Output = Self;
    fn div(self, rhs: usize) -> Self {
        PlainText::new(self.0.mapv(|x| x / rhs as f64))
    }
}

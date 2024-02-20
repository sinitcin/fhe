use super::plain_text::PlainText;
use core::f64;
use error::LinalgError;
use std::f64::consts::PI;

// Крейт `ndarray` предоставляет *n*-мерный контейнер для общих элементов.
// и для числовых значений.
use ndarray::{s, Array, Array1, Array2};

// Крейт `ndarray-linalg` предоставляет функции линейной алгебры для `ArrayBase`,
// структуры данных n-мерного массива, предоставляемой
// [`ndarray`](https://github.com/rust-ndarray/ndarray).
use ndarray_linalg::types::c64;
use ndarray_linalg::*;

/// !Класс, отвечающий за кодирование и декодирование данных в контексте CKKS.
pub struct CKKSEncoder {
    /// Количество элементов для которых создается матрица Вандермонда.
    m: usize,
    /// Единичный элемент
    unity: c64,
    /// Базис для преобразований
    basis: Array2<c64>,
    /// Коэффициент для масштабирования
    scale: usize,
}

impl CKKSEncoder {
    pub fn new(m: usize, scale: usize) -> CKKSEncoder {
        // В результате получается комплексное число, представляющее собой
        // (e^{\frac{2\pi i}{m}}), то есть корень (m)-й степени из единицы.
        // Этот корень используется для генерации элементов матрицы Вандермонда
        // в последующем коде.
        let unity = (2f64 * c64::i() /* Возвращает мнимую единицу */ 
            / c64::new(
                // Вещественная часть комплексного числа
                m as f64, 
                // Мнимая часть комплексного числа
                0f64)
            * PI)
            .exp();

        let basis = CKKSEncoder::vandermonde(m, unity).t().to_owned();
        CKKSEncoder {
            m,
            unity,
            basis,
            scale,
        }
    }

    pub fn unity(&self) -> c64 {
        self.unity
    }

    pub fn basis(&self) -> Array2<c64> {
        self.basis.clone()
    }
}

impl CKKSEncoder {
    /// !Кодирование, где z - одномерный массив
    pub fn encode(&self, z: Array1<c64>) -> Result<PlainText, LinalgError> {
        // C^n/2 -> H
        let pi_z = self.pi_inverse(&z);

        // Поддерживается точность. Перемножаем все элементы pi_z на scale.
        let scaled = pi_z.mapv(|x| self.scale as f64 * x);

        // H -> sigma(R) s.t. sigma(R) \in H
        let rounded = self.into_integer_basis(scaled);

        /* sigma(R) -> R . Это инверсия сигмы */
        let mut plain_text = self.sigma_inverse(rounded)?;

        /* содержит значения c64. Необходимо преобразовать их в целые числа. */
        plain_text.0.mapv_inplace(|x| c64::new(x.re.round(), x.im));

        Ok(plain_text)
    }

    /// !Декодирование :: R ->  sigma(R) -> C^n/2
    pub fn decode(&self, p: PlainText) -> Result<Array1<c64>, LinalgError> {
        /*  деление коэффициентов p на масштаб .*/
        let rescaled: PlainText = p / self.scale;

        /*  R -> sigma(R) .*/
        let rounded = self.sigma(rescaled)?;

        /* sigma(R) -> C^n/2 */
        Ok(self.pi(&rounded))
    }

    /// !Декодирование: \mathcal{R}->\sigma(\mathcal{R})$
    /// Вычисление x .
    /// x = A z
    ///
    /// Для вычисления x присвойте корень в A, вычислите его и возьмите произведение с z.
    /// Где: A - матрица Вандермонда. $z \в \mathcal{R},x \в \mathcal{\sigma{R}}$
    pub fn sigma(&self, poly: PlainText) -> Result<Array1<c64>, LinalgError> {
        let n = self.m / 2;
        let mut z = vec![];

        for i in 0..n {
            let root = self.unity.powu(2 * i as u32 + 1);
            let res = poly.eval(root);
            z.push(res);
        }

        // TODO: позволяет нам обнаружить isize::MAX < res.len().
        Ok(Array::from(z))
    }

    /// !Кодирование: $\sigma(\mathbf{R})->\mathcal{R}$
    /// Решает следующее выражение:
    /// $ z = A^-1 x $
    ///
    /// Где: A - матрица Вандермонда.
    pub fn sigma_inverse(&self, z: Array1<c64>) -> Result<PlainText, LinalgError> {
        let vandermonde = CKKSEncoder::vandermonde(self.m, self.unity);
        // Решает систему линейных уравнений `A * x = b`, где `A` - это `self`, `b` - аргумент, а `x` - успешный результат.
        let coefficients = vandermonde.solve_into(z)?;
        Ok(PlainText::new(coefficients))
    }

    pub fn pi(&self, z: &Array1<c64>) -> Array1<c64> {
        let n = self.m / 4;

        /* H->C^N/2 уменьшить вектор вдвое*/
        z.slice(s![..n]).to_owned()
    }

    pub fn pi_inverse(&self, z: &Array1<c64>) -> Array1<c64> {
        //1. Инвертирует z в z'.
        let zd = z.slice(s![..;-1]).to_owned();
        //2. Преобразование элементов 'z' в сопряженные
        let zd_conjugate = zd.mapv(|x| x.conj());

        let mut res = vec![];
        for i in z.into_iter().chain(zd_conjugate.into_iter()) {
            res.push(*i)
        }

        // TODO: позволяет нам обнаружить isize::MAX < res.len().
        Array::from(res)
    }

    pub fn into_integer_basis(&self, z: Array1<c64>) -> Array1<c64> {
        let real_coordinates = CKKSEncoder::compute_basis_coordinate(&self.basis, z);
        let rounded_coordinates = CKKSEncoder::coordinates_wise_random_rounding(real_coordinates);
        self.basis.t().dot(&rounded_coordinates)
    }
}

/* Вспомогательные методы */
impl CKKSEncoder {
    /// Выполняются следующие вычисления:
    ///
    /// $<z,bi>/|b_i|^2$
    ///
    fn compute_basis_coordinate(basis: &Array2<c64>, z: Array1<c64>) -> Array1<c64> {
        let mut tmp = vec![];
        for i in 0..basis.nrows() {
            let bi = basis.row(i).to_owned();
            let bi_conj = bi.mapv(|x| x.conj());
            let mut zi = z.dot(&bi_conj) / bi.dot(&bi_conj);
            zi.im = 0.;
            tmp.push(zi);
        }
        Array1::from(tmp)
    }

    fn coordinates_wise_random_rounding(coordinates: Array1<c64>) -> Array1<c64> {
        use rand::distributions::weighted::WeightedIndex;
        use rand::prelude::*;
        let decimals = &coordinates - &coordinates.mapv(|x| c64::new(x.re.floor(), x.im));
        // Чем ближе целое число, тем выше вероятность округления.
        // Например, если c=4,6, то вероятность округления до 4 равна 0,6, а вероятность округления до 5 - 0,4.
        let subtract_decimals = decimals.mapv(|c| {
            let choices = [c, c - 1.];
            let weights = [1. - c.re, c.re];
            let mut rng = rand::thread_rng();
            let dist = WeightedIndex::new(&weights).unwrap();
            choices[dist.sample(&mut rng)]
        });
        // Координаты, округленные до ближайшего целого числа.
        // Например, координата = [1.2,1.7,2.6] и предположим, что subtract_decimals = [-0.8,0.7,-0.4].
        // Вы можете выполнить вычитание по координатам.
        // координата - subtract_decimals = [2.0,1.0,3.0].
        coordinates - subtract_decimals
    }

    /// Создаёт матрицу Вандермонда.
    /// --
    /// В данном контексте, метод vandermonde использует единичный элемент unity
    /// (комплексное число, обозначающее корень из единицы) для генерации набора
    /// значений (\alpha_i). Матрица формируется для (m/2) элементов, где каждый
    /// элемент — это степень единицы, возводимая в соответствующую степень для
    /// каждой строки матрицы. Это делается для подготовки основания для преобразования
    /// данных в процессе кодирования и декодирования в CKKSEncoder.
    fn vandermonde(m: usize, unity: c64) -> Array2<c64> {
        let n: usize = m / 2;
        let mut mat = vec![];

        for i in 0..n {
            let root = unity.powu(2 * i as u32 + 1);
            for j in 0..n {
                mat.push(root.powu(j as u32));
            }
        }

        let res = Array::from_shape_vec((n, n), mat).unwrap();
        res
    }
}

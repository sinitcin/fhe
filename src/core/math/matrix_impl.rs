/*
 This code provide a templated matrix implementation
*/
use std::vec::Vec;

pub struct Matrix<Element> {
    data: Vec<Vec<Element>>,
    rows: usize,
    cols: usize,
    allocZero: alloc_func,
}

impl<Element> Matrix<Element> {
    pub fn new(allocZero: alloc_func, rows: usize, cols: usize, allocGen: alloc_func) -> Self {
        let mut data = Vec::new();
        data.resize(rows, Vec::new());
        for row in data.iter_mut() {
            for _ in 0..cols {
                row.push(allocGen());
            }
        }
        Matrix {
            data,
            rows,
            cols,
            allocZero,
        }
    }

    pub fn fill(&mut self, val: Element) -> &mut Self {
        for row in self.data.iter_mut() {
            for col in row.iter_mut() {
                *col = val;
            }
        }
        self
    }

    pub fn mult(&self, other: &Matrix<Element>) -> Result<Matrix<Element>, String> {
        if self.cols != other.rows {
            return Err(String::from("incompatible matrix multiplication"));
        }
        let mut result = Matrix::new(self.allocZero, self.rows, other.cols, self.allocZero);
        if self.rows == 1 {
            for col in 0..result.cols {
                for i in 0..self.cols {
                    result.data[0][col] += self.data[0][i] * other.data[i][col];
                }
            }
        } else {
            for row in 0..result.rows {
                for i in 0..self.cols {
                    for col in 0..result.cols {
                        result.data[row][col] += self.data[row][i] * other.data[i][col];
                    }
                }
            }
        }
        Ok(result)
    }

    pub fn add_assign(&mut self, other: &Matrix<Element>) -> Result<&mut Self, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(String::from(
                "Addition operands have incompatible dimensions",
            ));
        }
        for j in 0..self.cols {
            for i in 0..self.rows {
                self.data[i][j] += other.data[i][j];
            }
        }
        Ok(self)
    }

    pub fn sub_assign(&mut self, other: &Matrix<Element>) -> Result<&mut Self, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(String::from(
                "Subtraction operands have incompatible dimensions",
            ));
        }
        for j in 0..self.cols {
            for i in 0..self.rows {
                self.data[i][j] -= other.data[i][j];
            }
        }
        Ok(self)
    }

    pub fn transpose(&self) -> Matrix<Element> {
        let mut result = Matrix::new(self.allocZero, self.cols, self.rows, self.allocZero);
        for row in 0..self.rows {
            for col in 0..self.cols {
                result.data[col][row] = self.data[row][col];
            }
        }
        result
    }
}

// YSP The signature of this method needs to be changed in the future
// Laplace's formula is used to find the determinant
// Complexity is O(d!), where d is the dimension
// The determinant of a matrix is expressed in terms of its minors
// recursive implementation
// There are O(d^3) decomposition algorithms that can be implemented to support
// larger dimensions. Examples include the LU decomposition, the QR
// decomposition or the Cholesky decomposition(for positive definite matrices).
impl<Element> Matrix<Element> {
    fn determinant(&self) -> Element
    where
        Element: std::ops::Sub<Output = Element>
            + std::ops::Mul<Output = Element>
            + std::ops::Add<Output = Element>
            + std::ops::Neg<Output = Element>
            + Copy,
    {
        if self.rows != self.cols {
            panic!("Supported only for square matrix");
        }

        if self.rows < 1 {
            panic!("Dimension should be at least one");
        }

        if self.rows == 1 {
            return self.data[0][0];
        } else if self.rows == 2 {
            return self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1];
        } else {
            let n = self.rows;
            let mut determinant = Element::default();
            let mut result = Matrix::new(self.rows - 1, self.cols - 1);

            for j1 in 0..n {
                for i in 1..n {
                    let mut j2 = 0;

                    for j in 0..n {
                        if j == j1 {
                            continue;
                        }

                        result.data[i - 1][j2] = self.data[i][j];
                        j2 += 1;
                    }
                }

                let temp_determinant = result.determinant();
                if j1 % 2 == 0 {
                    determinant = determinant + self.data[0][j1] * temp_determinant;
                } else {
                    determinant = determinant - self.data[0][j1] * temp_determinant;
                }
            }

            determinant
        }
    }
}

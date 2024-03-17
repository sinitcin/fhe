
/*
  matrix class implementations and type specific implementations
 */

 use std::sync::Arc;
use ndarray::Array2;

struct Element {
    values: Vec<i32>,
    modulus: i32,
    format: Format,
}

impl Element {
    fn get_length(&self) -> usize {
        self.values.len()
    }

    fn get_modulus(&self) -> i32 {
        self.modulus
    }

    fn get_values(&self) -> &Vec<i32> {
        &self.values
    }

    fn switch_format(&mut self) {
        // Implementation for switching format
    }
}

enum Format {
    Coefficient,
    Other, // Placeholder for other formats
}

fn rotate(in_mat: Array2<Element>) -> Array2<i32> {
    let n = in_mat[[0, 0]].get_length() as usize;
    let modulus = in_mat[[0, 0]].get_modulus();
    let rows = in_mat.nrows() * n;
    let cols = in_mat.ncols() * n;
    let mut result = Array2::<i32>::zeros((rows, cols));
    for (row, col, rot_row, rot_col) in iproduct!(0..in_mat.nrows(), 0..in_mat.ncols(), 0..n, 0..n) {
        let value = in_mat[[row, col]].get_values()[(rot_row + n - rot_col) % n];
        result[[row * n + rot_row, col * n + rot_col]] = value;
        if rot_row < rot_col {
            result[[row * n + rot_row, col * n + rot_col]] = (modulus - value).rem_euclid(modulus);
        }
    }
    result
}

fn rotate_vec_result(in_mat: Array2<Element>) -> Array2<Vec<i32>> {
    let n = in_mat[[0, 0]].get_length() as usize;
    let modulus = in_mat[[0, 0]].get_modulus();
    let rows = in_mat.nrows() * n;
    let cols = in_mat.ncols() * n;
    let mut result = Array2::from_elem((rows, cols), vec![0; 1]);
    for (row, col, rot_row, rot_col) in iproduct!(0..in_mat.nrows(), 0..in_mat.ncols(), 0..n, 0..n) {
        let value = in_mat[[row, col]].get_values()[(rot_row + n - rot_col) % n];
        result[[row * n + rot_row, col * n + rot_col]][0] = value;
        if rot_row < rot_col {
            result[[row * n + rot_row, col * n + rot_col]][0] = (modulus - value).rem_euclid(modulus);
        }
    }
    result
}

fn convert_to_int32<T: Into<i32> + Copy>(input: Array2<T>, modulus: T) -> Array2<i32> {
    let negative_threshold = modulus.into() / 2;
    input.mapv(|elem| {
        let elem = elem.into();
        if elem > negative_threshold {
            -1 * ((modulus.into() - elem) as i32)
        } else {
            elem as i32
        }
    })
}

fn set_format(in_mat: &mut Array2<Element>, format: Format) {
    for elem in in_mat.iter_mut() {
        if elem.format != format {
            elem.switch_format();
        }
    }
}



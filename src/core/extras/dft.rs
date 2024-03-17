/*
  Example of Discrete Fourier Transform
 */

 extern crate num_complex;
extern crate num_traits;

use num_complex::Complex;
use std::time::{Duration, Instant};

fn main() {
    let mut dft_vec: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); 64];
    for i in 0..64 {
        dft_vec[i] = Complex::new(match i {
            0 | 8 | 16 | 24 | 32 | 40 | 48 | 56 => 4.0,
            1 | 9 | 17 | 25 | 33 | 41 | 49 | 57 => 5.0,
            2 | 10 | 18 | 26 | 34 | 42 | 50 | 58 => 5.0,
            3 | 11 | 19 | 27 | 35 | 43 | 51 | 59 => 4.2,
            4 | 12 | 20 | 28 | 36 | 44 | 52 | 60 => 5.0,
            5 | 13 | 21 | 29 | 37 | 45 | 53 | 61 => 7.1,
            6 | 14 | 22 | 30 | 38 | 46 | 54 | 62 => 6.0,
            7 | 15 | 23 | 31 | 39 | 47 | 55 | 63 => 3.0,
            _ => 0.0,
        }, 0.0);
    }

    // Assuming DiscreteFourierTransform::PreComputeTable and DiscreteFourierTransform::ForwardTransform
    // are implemented elsewhere in Rust, and currentDateTime() is replaced with Instant::now()
    // DiscreteFourierTransform::PreComputeTable(128);

    let start = Instant::now();
    // let dft_vec2 = DiscreteFourierTransform::ForwardTransform(&dft_vec);
    let duration = start.elapsed();
    println!("Without table: {:?} ms", duration.as_millis());

    let start = Instant::now();
    // let dft_vec3 = DiscreteFourierTransform::ForwardTransform(&dft_vec);
    let duration = start.elapsed();
    println!("With table: {:?} ms", duration.as_millis());
}



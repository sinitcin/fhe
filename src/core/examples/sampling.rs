// Rust does not have a direct equivalent library for OpenFHE as of my last update.
// The following Rust code is a conceptual translation and will not compile without a suitable cryptographic library.

use std::time::{Duration, Instant};
use std::f64::consts::LOG2_E;

fn main() {
    let std_base = 34.0;
    let std = (1 << 22) as f64;
    let center_count = 1024;

    // Rust equivalent cryptographic constructs would be needed here.
    // let bg = BitGenerator::new();
    // let dgg = DiscreteGaussianGenerator::new(4);
    // let dgg_rejection = DiscreteGaussianGenerator::new(4);
    // let dgg4 = DiscreteGaussianGenerator::new(std_base);

    let count = 1000;
    let smoothing_parameter = 6.0;
    println!("Distribution parameter = {}", std);

    // In Rust, we typically use Vec for dynamic arrays.
    let mut peikert_samplers = Vec::new();
    let mut ky_samplers = Vec::new();

    println!("Started creating base samplers");
    for i in 0..center_count {
        let center = i as f64 / center_count as f64;

        // Assuming BaseSampler is a struct or class in the cryptographic library.
        // peikert_samplers.push(BaseSampler::new(center, std_base, &bg, SamplerType::Peikert));
        // ky_samplers.push(BaseSampler::new(center, std_base, &bg, SamplerType::KnuthYao));
    }
    println!("Ended creating base samplers, Started sampling");

    let start = Instant::now();
    for k in 0..center_count {
        let center = k as f64 / center_count as f64;
        for _ in 0..count {
            // dgg_rejection.generate_integer(center, std, 8192);
        }
    }
    let finish = start.elapsed();
    println!("Sampling {} integers (Rejection): {:?} ms", count, finish.as_millis() / center_count as u128);

    let start = Instant::now();
    for k in 0..center_count {
        let center = k as f64 / center_count as f64;
        for _ in 0..count {
            // dgg.generate_integer_karney(center, std);
        }
    }
    let finish = start.elapsed();
    println!("Sampling {} integers (Karney): {:?} ms", count, finish.as_millis() / center_count as u128);

    let base = (center_count as f64).log2() as i32;

    // Assuming DiscreteGaussianGeneratorGeneric is a struct or class in the cryptographic library.
    // let dgg2 = DiscreteGaussianGeneratorGeneric::new(&peikert_samplers, std_base, base, smoothing_parameter);
    let start = Instant::now();
    for k in 0..center_count {
        let center = k as f64 / center_count as f64;
        for _ in 0..count {
            // dgg2.generate_integer(center, std);
        }
    }
    let finish = start.elapsed();
    println!("Sampling {} integers (Generic - Peikert): {:?} ms", count, finish.as_millis() / center_count as u128);

    // let dgg3 = DiscreteGaussianGeneratorGeneric::new(&ky_samplers, std_base, base, smoothing_parameter);
    let start = Instant::now();
    for k in 0..center_count {
        let center = k as f64 / center_count as f64;
        for _ in 0..count {
            // dgg3.generate_integer(center, std);
        }
    }
    let finish = start.elapsed();
    println!("Sampling {} integers (Generic - Knuth Yao): {:?} ms", count, finish.as_millis() / center_count as u128);
}



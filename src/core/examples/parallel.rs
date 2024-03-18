
/*
  This is an example demo file that demonstrates timing of Parallel operations using openmp
*/

use std::env;
use std::thread;
use std::time::{Duration, Instant};

fn verify(foo: &[f32], array_size: usize) {
    let mut goodflag = true;
    for i in 1..array_size {
        if (foo[i] - foo[i - 1]) != 1.0 {
            goodflag = goodflag && false;
        }
    }
    if goodflag {
        println!("verification succeeded");
    } else {
        println!("verification failed");
        for i in 0..array_size {
            print!("{} ", foo[i]);
        }
        println!();
    }
}

fn main() {
    // Assuming OPENFHE_DEBUG_FLAG and related functions are part of a specific library's debug utilities,
    // these will not be directly translated to Rust as there's no direct equivalent in standard Rust.
    // Similarly, OpenFHEParallelControls and related OpenFHE functionality are specific to the OpenFHE library
    // and would require a Rust equivalent library or manual implementation.

    let args: Vec<String> = env::args().collect();
    let mut array_size = 1000;
    if args.len() < 2 {
        println!("running {} with default array size of 1000", args[0]);
    } else {
        array_size = args[1].parse::<usize>().unwrap_or_else(|_| {
            println!("error in argument {} must be greater than zero", args[1]);
            std::process::exit(-1);
        });
    }

    let mut foo = vec![0f32; array_size];

    // Parallel computation demo message
    // Rust does not have a direct equivalent to OpenMP, so parallel execution would require
    // a crate like rayon or manual management of threads.
    println!("Parallel computation demo using Rust");

    // Demonstrating parallel execution with threads in Rust
    let start = Instant::now();
    let handles: Vec<_> = (0..array_size).map(|i| {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            i as f32
        })
    }).collect();

    for (i, handle) in handles.into_iter().enumerate() {
        foo[i] = handle.join().unwrap();
    }

    let time_total = start.elapsed();
    println!("Total time with internal delay: \t{:?} ms", time_total.as_millis());
    verify(&foo, array_size);
    println!();

    // Resetting the array
    foo.iter_mut().for_each(|x| *x = 0.0);

    let start = Instant::now();
    foo.iter_mut().enumerate().for_each(|(i, x)| *x = i as f32);
    let time_total = start.elapsed();
    println!("Total time without internal delay: \t{:?} us", time_total.as_micros());
    verify(&foo, array_size);
}



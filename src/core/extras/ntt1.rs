/*
  Example of NTT operations
  This is a main() file built to test and time NTT operations. D. Cousins
 */

 use std::time::{Instant};

fn test_NTT() {
    let nloop = 100;
    let mut time1af = 0.0;
    let mut time1bf = 0.0;
    let mut time2af = 0.0;
    let mut time2bf = 0.0;
    let mut time3af = 0.0;
    let mut time3bf = 0.0;
    let mut time1ar = 0.0;
    let mut time1br = 0.0;
    let mut time2ar = 0.0;
    let mut time2br = 0.0;
    let mut time3ar = 0.0;
    let mut time3br = 0.0;
    let mut failed = false;
    let mut ix = 0;
    println!("Startng timing");
    while ix < nloop {
        if ix % 100 == 0 {
            println!("{}", ix);
        }
        #ifdef TEST1
        let mut x1a = Poly::new();
        let mut x1b = Poly::new();
        let mut x1aClone = Poly::new();
        let mut x1bClone = Poly::new();
        let t1 = Instant::now();
        x1a.switch_format();
        time1af += t1.elapsed().as_micros() as f64;
        let t1 = Instant::now();
        x1b.switch_format();
        time1bf += t1.elapsed().as_micros() as f64;
        let t1 = Instant::now();
        x1a.switch_format();
        time1ar += t1.elapsed().as_micros() as f64;
        let t1 = Instant::now();
        x1b.switch_format();
        time1br += t1.elapsed().as_micros() as f64;
        failed |= clonetest(x1a, x1aClone, "x1a");
        failed |= clonetest(x1b, x1bClone, "x1b");
        #endif
        #ifdef TEST2
        let mut x2a = Poly::new();
        let mut x2b = Poly::new();
        let mut x2aClone = Poly::new();
        let mut x2bClone = Poly::new();
        let t1 = Instant::now();
        x2a.switch_format();
        time2af += t1.elapsed().as_micros() as f64;
        let t1 = Instant::now();
        x2b.switch_format();
        time2bf += t1.elapsed().as_micros() as f64;
        let t1 = Instant::now();
        x2a.switch_format();
        time2ar += t1.elapsed().as_micros() as f64;
        let t1 = Instant::now();
        x2b.switch_format();
        time2br += t1.elapsed().as_micros() as f64;
        failed |= clonetest(x2a, x2aClone, "x2a");
        failed |= clonetest(x2b, x2bClone, "x2b");
        #endif
        #ifdef TEST3
        let mut x3a = Poly::new();
        let mut x3b = Poly::new();
        let mut x3aClone = Poly::new();
        let mut x3bClone = Poly::new();
        let t1 = Instant::now();
        x3a.switch_format();
        time3af += t1.elapsed().as_micros() as f64;
        let t1 = Instant::now();
        x3b.switch_format();
        time3bf += t1.elapsed().as_micros() as f64;
        let t1 = Instant::now();
        x3a.switch_format();
        time3ar += t1.elapsed().as_micros() as f64;
        let t1 = Instant::now();
        x3b.switch_format();
        time3br += t1.elapsed().as_micros() as f64;
        failed |= clonetest(x3a, x3aClone, "x3a");
        failed |= clonetest(x3b, x3bClone, "x3b");
        #endif
        ix += 1;
    }
    if failed {
        println!("failure in loop number {}", ix);
    } else {
        time1af /= nloop as f64;
        time1bf /= nloop as f64;
        time2af /= nloop as f64;
        time2bf /= nloop as f64;
        time3af /= nloop as f64;
        time3bf /= nloop as f64;
        time1ar /= nloop as f64;
        time1br /= nloop as f64;
        time2ar /= nloop as f64;
        time2br /= nloop as f64;
        time3ar /= nloop as f64;
        time3br /= nloop as f64;
        println!("{} loops", nloop);
        println!("t1af: \t{} us", time1af);
        println!("t1bf: \t{} us", time1bf);
        println!("t2af: \t{} us", time2af);
        println!("t2bf: \t{} us", time2bf);
        println!("t3af: \t{} us", time3af);
        println!("t3bf: \t{} us", time3bf);
        println!("t1ar: \t{} us", time1ar);
        println!("t1br: \t{} us", time1br);
        println!("t2ar: \t{} us", time2ar);
        println!("t2br: \t{} us", time2br);
        println!("t3ar: \t{} us", time3ar);
        println!("t3br: \t{} us", time3br);
    }
}

fn main() {
    test_NTT();
}



/*
  This code provides basic structure for distribution generators. This should be inherited by
  all other distribution generators
*/
use rand::distributions::Uniform;
use rand::rngs::OsRng;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Once;
use std::thread;
use std::time::SystemTime;

lazy_static! {
    static ref PRNG: Mutex<Option<Blake2Engine>> = Mutex::new(None);
    static ref INIT_PRNG: Once = Once::new();
}

pub struct PseudoRandomNumberGenerator {}

impl PseudoRandomNumberGenerator {
    pub fn init_prng() {
        INIT_PRNG.call_once(|| {
            let threads = OpenFHEParallelControls::get_num_threads();
            if threads == 0 {
                threads = 1;
            }
            let mut handles = vec![];
            for _ in 0..threads {
                handles.push(thread::spawn(|| {
                    Self::get_prng();
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });
    }

    pub fn get_prng() -> Arc<Mutex<Blake2Engine>> {
        let mut prng = PRNG.lock().unwrap();
        if prng.is_none() {
            let mut seed: [u32; 16] = [0; 16];
            #[cfg(FIXED_SEED)]
            {
                eprintln!("**FOR DEBUGGING ONLY!!!!  Using fixed initializer for PRNG. Use a single thread only, e.g., OMP_NUM_THREADS=1!");
                seed[0] = 1;
                *prng = Some(Arc::new(Mutex::new(Blake2Engine::new(seed))));
            }
            #[cfg(not(FIXED_SEED))]
            {
                let mut init_key: [u32; 16] = [0; 16];
                init_key[0] = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as u32;
                let mut hasher = DefaultHasher::new();
                thread::current().id().hash(&mut hasher);
                init_key[1] = hasher.finish() as u32;
                #[cfg(not(any(target_arch = "arm", target_arch = "wasm32")))]
                {
                    if std::mem::size_of::<usize>() == 8 {
                        init_key[2] = (hasher.finish() >> 32) as u32;
                    }
                }
                let mem = Box::into_raw(Box::new(1));
                let counter = mem as u32;
                unsafe { Box::from_raw(mem) };
                let mut gen = Blake2Engine::new_with_counter(init_key, counter);
                let distribution = Uniform::new(0, std::u32::MAX);
                let mut seed: [u32; 16] = [0; 16];
                for i in 0..16 {
                    seed[i] = distribution.sample(&mut gen);
                }
                let mut rdseed: [u32; 16] = [0; 16];
                let mut attempts = 3;
                let mut rd_gen_passed = false;
                let mut idx = 0;
                while !rd_gen_passed && idx < attempts {
                    match OsRng::new() {
                        Ok(mut gen_r) => {
                            for i in 0..16 {
                                rdseed[i] = distribution.sample(&mut gen_r);
                            }
                            rd_gen_passed = true;
                        }
                        Err(_) => {}
                    }
                    idx += 1;
                }
                for i in 0..16 {
                    seed[i] += rdseed[i];
                }
                *prng = Some(Arc::new(Mutex::new(Blake2Engine::new(seed))));
            }
        }
        Arc::clone(prng.as_ref().unwrap())
    }
}


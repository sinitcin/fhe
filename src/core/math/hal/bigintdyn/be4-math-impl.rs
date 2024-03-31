/*
 This file contains template instantiations for all math classes & functions using math be4
*/


#[cfg(feature = "WITH_BE4")]
mod math {
    use crate::config_core::M4Vector;

    pub struct DiscreteGaussianGeneratorImpl<T>(std::marker::PhantomData<T>);
    pub struct BinaryUniformGeneratorImpl<T>(std::marker::PhantomData<T>);
    pub struct TernaryUniformGeneratorImpl<T>(std::marker::PhantomData<T>);
    pub struct DiscreteUniformGeneratorImpl<T>(std::marker::PhantomData<T>);

    pub fn RootOfUnity<M4Integer>(m: usint, modulo: &M4Integer) -> M4Integer {
        // implementation here
    }

    pub fn RootsOfUnity<M4Integer>(m: usint, moduli: &Vec<M4Integer>) -> Vec<M4Integer> {
        // implementation here
    }

    pub fn GreatestCommonDivisor(a: &M4Integer, b: &M4Integer) -> M4Integer {
        // implementation here
    }

    pub fn MillerRabinPrimalityTest(p: &M4Integer, niter: usint) -> bool {
        // implementation here
    }

    pub fn PollardRhoFactorization(n: &M4Integer) -> M4Integer {
        // implementation here
    }

    pub fn PrimeFactorize(n: M4Integer, primeFactors: &mut std::collections::HashSet<M4Integer>) {
        // implementation here
    }

    pub fn FirstPrime(nBits: u32, m: u32) -> M4Integer {
        // implementation here
    }

    pub fn LastPrime(nBits: u32, m: u32) -> M4Integer {
        // implementation here
    }

    pub fn NextPrime(q: &M4Integer, m: u32) -> M4Integer {
        // implementation here
    }

    pub fn PreviousPrime(q: &M4Integer, m: u32) -> M4Integer {
        // implementation here
    }

    pub fn GetTotientList(n: &M4Integer) -> Vec<M4Integer> {
        // implementation here
    }

    pub fn PolyMod(dividend: &M4Vector, divisor: &M4Vector, modulus: &M4Integer) -> M4Vector {
        // implementation here
    }

    pub fn PolynomialMultiplication(a: &M4Vector, b: &M4Vector) -> M4Vector {
        // implementation here
    }

    pub fn GetCyclotomicPolynomial(m: usint, modulus: &M4Integer) -> M4Vector {
        // implementation here
    }

    pub fn SyntheticRemainder(
        dividend: &M4Vector,
        a: &M4Integer,
        modulus: &M4Integer,
    ) -> M4Integer {
        // implementation here
    }

    pub fn SyntheticPolyRemainder(
        dividend: &M4Vector,
        aList: &M4Vector,
        modulus: &M4Integer,
    ) -> M4Vector {
        // implementation here
    }

    pub fn PolynomialPower<M4Vector>(input: &M4Vector, power: usint) -> M4Vector {
        // implementation here
    }

    pub fn SyntheticPolynomialDivision(
        dividend: &M4Vector,
        a: &M4Integer,
        modulus: &M4Integer,
    ) -> M4Vector {
        // implementation here
    }

    pub fn FindGeneratorCyclic(modulo: &M4Integer) -> M4Integer {
        // implementation here
    }

    pub fn IsGenerator(g: &M4Integer, modulo: &M4Integer) -> bool {
        // implementation here
    }

    pub fn GetDigits(u: &M4Integer, base: u64, k: u32) -> std::shared_ptr<Vec<i64>> {
        // implementation here
    }

    pub struct Matrix<T>(std::marker::PhantomData<T>);
}

#[cfg(feature = "WITH_BE4")]
mod lbcrypto {
    use crate::config_core::M4Integer;
    use crate::math::{
        BinaryUniformGeneratorImpl, DiscreteGaussianGeneratorImpl, DiscreteUniformGeneratorImpl,
        Matrix, TernaryUniformGeneratorImpl,
    };

    impl DiscreteGaussianGeneratorImpl<M4Vector> {}
    impl BinaryUniformGeneratorImpl<M4Vector> {}
    impl TernaryUniformGeneratorImpl<M4Vector> {}
    impl DiscreteUniformGeneratorImpl<M4Vector> {}

    impl M4Integer {
        pub fn RootOfUnity(m: usint, modulo: &M4Integer) -> M4Integer {
            // implementation here
        }

        pub fn RootsOfUnity(m: usint, moduli: &Vec<M4Integer>) -> Vec<M4Integer> {
            // implementation here
        }

        pub fn GreatestCommonDivisor(a: &M4Integer, b: &M4Integer) -> M4Integer {
            // implementation here
        }

        pub fn MillerRabinPrimalityTest(p: &M4Integer, niter: usint) -> bool {
            // implementation here
        }

        pub fn PollardRhoFactorization(n: &M4Integer) -> M4Integer {
            // implementation here
        }

        pub fn PrimeFactorize(
            n: M4Integer,
            primeFactors: &mut std::collections::HashSet<M4Integer>,
        ) {
            // implementation here
        }

        pub fn FirstPrime(nBits: u32, m: u32) -> M4Integer {
            // implementation here
        }

        pub fn LastPrime(nBits: u32, m: u32) -> M4Integer {
            // implementation here
        }

        pub fn NextPrime(q: &M4Integer, m: u32) -> M4Integer {
            // implementation here
        }

        pub fn PreviousPrime(q: &M4Integer, m: u32) -> M4Integer {
            // implementation here
        }

        pub fn GetTotientList(n: &M4Integer) -> Vec<M4Integer> {
            // implementation here
        }

        pub fn PolyMod(dividend: &M4Vector, divisor: &M4Vector, modulus: &M4Integer) -> M4Vector {
            // implementation here
        }

        pub fn PolynomialMultiplication(a: &M4Vector, b: &M4Vector) -> M4Vector {
            // implementation here
        }

        pub fn GetCyclotomicPolynomial(m: usint, modulus: &M4Integer) -> M4Vector {
            // implementation here
        }

        pub fn SyntheticRemainder(
            dividend: &M4Vector,
            a: &M4Integer,
            modulus: &M4Integer,
        ) -> M4Integer {
            // implementation here
        }

        pub fn SyntheticPolyRemainder(
            dividend: &M4Vector,
            aList: &M4Vector,
            modulus: &M4Integer,
        ) -> M4Vector {
            // implementation here
        }

        pub fn PolynomialPower<M4Vector>(input: &M4Vector, power: usint) -> M4Vector {
            // implementation here
        }

        pub fn SyntheticPolynomialDivision(
            dividend: &M4Vector,
            a: &M4Integer,
            modulus: &M4Integer,
        ) -> M4Vector {
            // implementation here
        }

        pub fn FindGeneratorCyclic(modulo: &M4Integer) -> M4Integer {
            // implementation here
        }

        pub fn IsGenerator(g: &M4Integer, modulo: &M4Integer) -> bool {
            // implementation here
        }

        pub fn GetDigits(u: &M4Integer, base: u64, k: u32) -> std::shared_ptr<Vec<i64>> {
            // implementation here
        }
    }

    impl Matrix<M4Integer> {}
    impl Matrix<M4Vector> {}
}

#[cfg(feature = "WITH_BE4")]
fn main() {
    // implementation here
}

#[cfg(not(feature = "WITH_BE4"))]
fn main() {
    // implementation here
}

#[cfg(feature = "WITH_BE4")]
mod cereal {
    use crate::config_core::{M4Integer, M4Vector};

    impl M4Integer {
        pub const fn SerializedVersion() -> u32 {
            // implementation here
        }
    }

    impl M4Vector {
        pub const fn SerializedVersion() -> u32 {
            // implementation here
        }
    }
}

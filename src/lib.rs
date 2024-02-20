pub mod core;
pub mod errors;
pub mod fhe_schemes;
pub mod gf_context;
pub mod gf_element;
pub mod pke;
pub mod ring_context;
pub mod ring_element;
pub mod scale_technique;
pub mod serializable;
pub mod vanilla_variant;

pub type Element = u64;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    macro_rules! carray {
        ( $( $x:expr ),* ) => ( array![ $( c64::new($x as f64,0f64) ),* ] )
    }

    use crate::gf_context::GFContext;
    use crate::ring_context::RingContext;
    use crate::vanilla_variant::encoder::encoder::CKKSEncoder;
    use ndarray::array;
    use ndarray_linalg::error::LinalgError;
    use ndarray_linalg::types::c64;
    use ndarray_linalg::Norm;

    /* константа допустимого отличия от исходного шифротекста */
    const E: f64 = 1e-10;
    const A: f64 = 0.1;

    #[test]
    fn test_unity() {
        let encoder = CKKSEncoder::new(8, 64);
        let x = encoder.unity();

        let diff = c64::new(-1f64, 0f64) - x * x * x * x;
        assert!(diff.norm() < E);
    }

    #[test]
    fn test_gf() {
        let gf5 = GFContext::new(5);

        assert_eq!(gf5.element(0), gf5.element(5));
        assert_eq!(gf5.element(1), gf5.element(21));
        assert_eq!(gf5.element(2), gf5.element(17));
        assert_eq!(gf5.element(3), gf5.element(73));
        assert_eq!(gf5.element(4), gf5.element(104));

        let gf97 = GFContext::new(97);
        let x = gf97.element(11);
        let y = gf97.element(53);

        assert_eq!(gf97.element(64), x + y);
        assert_eq!(gf97.element(42), y - x);
        assert_eq!(gf97.element(1), x * y);
        assert_eq!(y, x.inv());
        assert_eq!(gf97.element(1), x / x);
    }

    #[test]
    fn test_ring() {
        let ring5 = RingContext::new(5);

        assert_eq!(ring5.element(0), ring5.element(5));
        assert_eq!(ring5.element(1), ring5.element(21));
        assert_eq!(ring5.element(2), ring5.element(17));
        assert_eq!(ring5.element(3), ring5.element(73));
        assert_eq!(ring5.element(4), ring5.element(104));
    }

    #[test]
    fn test_sigma_and_invert() -> Result<(), LinalgError> {
        use ndarray_linalg::Norm;
        let encoder = CKKSEncoder::new(8, 64);
        let x = carray![1, 2, 3, 4];

        let plain_text = encoder.sigma_inverse(x.clone())?;
        let xd = encoder.sigma(plain_text)?;

        let diff = x - xd;
        assert!(diff.norm_l2() < E);

        Ok(())
    }

    #[test]
    fn test_sigma_isomorphism() -> Result<(), LinalgError> {
        let encoder = CKKSEncoder::new(8, 64);
        let x = carray![1, 2, 3, 4];
        let y = carray![1, 2, 3, 4];

        /* тестирование гомоморфного сложения */
        {
            let plain_text1 = encoder.sigma_inverse(x.clone())?;
            let plain_text2 = encoder.sigma_inverse(y.clone())?;

            let xy = &x + &y;
            let plain_text12 = plain_text1 + plain_text2;
            let xyd = encoder.sigma(plain_text12)?;

            let diff = xy - xyd;
            assert!(diff.norm_l2() < E);
        }

        /* тестирование гомоморфного умножения */
        {
            let plain_text1 = encoder.sigma_inverse(x.clone())?;
            let plain_text2 = encoder.sigma_inverse(y.clone())?;

            let xy = &x * &y;
            let plain_text12 = plain_text1 * plain_text2;
            let xyd = encoder.sigma(plain_text12)?;

            let diff = xy - xyd;
            assert!(diff.norm_l2() < E);
        }

        Ok(())
    }

    #[test]
    fn test_pi_and_pi_inverse() -> Result<(), LinalgError> {
        let encoder = CKKSEncoder::new(8, 64);
        let x = carray![0, 1];
        let y = carray![0, 1, 1, 0];
        let z = encoder.pi_inverse(&x);

        println!("{}", &z);
        let diff = y - z;
        assert!(diff.norm_l2() < E);

        Ok(())
    }

    #[test]
    fn test_encode_and_decode() -> Result<(), LinalgError> {
        let encoder = CKKSEncoder::new(8, 64);
        let x = array![c64::new(3., 4.), c64::new(2., 1.)];

        let plain_text = encoder.encode(x.clone())?;

        println!("result of encryption : {:?}", plain_text);

        let res = encoder.decode(plain_text)?;

        println!("result of decryption : {}", res);

        let diff = res - x;
        assert!(diff.norm_l2() < A);

        Ok(())
    }
}

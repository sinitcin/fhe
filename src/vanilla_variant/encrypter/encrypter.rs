use crate::vanilla_variant::{encoder::plain_text::PlainText, encrypter::cipher_text::CipherText};
struct CKKSEncrypter {
    public_key: (PlainText, PlainText),
    secret_key: PlainText,
}

impl CKKSEncrypter {
    /// a: choose on uniform with random
    /// s: small secret polynomial
    /// e: small noisy polynomial
    pub fn new(_mu: f64, _sigma: f64) -> CKKSEncrypter {
        // use rand::distributions::Distribution;
        // use rand::Rng;
        // let mut rng = rand::thread_rng();
        // let gaussian = Normal::new(mu, sigma.sqrt()).unwrap();

        // let a = rng.gen_range(0..10000);
        // let noise = gaussian.sample(&mut rng);

        // let public_key = (-a + secret * noise, a);
        // let secret_key = secret;
        // CKKSEncrypter {
        //     public_key,
        //     secret_key,
        // }
        todo!()
    }

    pub fn encrypt(&self, _plain_text: PlainText) -> CipherText {
        // CipherText::new(plain_text + public_key.0, public_key.1)
        todo!()
    }

    pub fn decrypt(&self, _ctxt: CipherText) -> PlainText {
        // PlainText::new(ctxt.0 + ctxt.1 * secret_key)
        todo!()
    }
}

pub type ExpdType = u64;

pub struct Ubint<T>(T);

impl<T: Copy + 'static> Ubint<T> {
    fn new(n: T) -> Self {
        Ubint(n)
    }
}

impl<T: Copy + std::ops::Add<Output = T> + 'static> Ubint<T> {
    fn add(&mut self, other: &Ubint<T>) {
        self.0 = self.0 + other.0;
    }
}
/** Define the mapping for ExpBigInteger (experimental) */
pub type xubint = Ubint<ExpdType>;
pub type BigInteger = xubint;

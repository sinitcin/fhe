
/*
  Parameter definitions for trapdoor-related schemes (GPV signature, IBE, ABE)
 */


 use std::sync::Arc;

pub trait Element {
    type Params;
    type DggType;
}

pub struct TrapdoorParams<E: Element> {
    stddev: f64,
    elemparams: Option<Arc<E::Params>>,
    dgg: E::DggType,
}

impl<E: Element> TrapdoorParams<E> {
    pub fn new() -> Self {
        Self {
            stddev: 0.0,
            elemparams: None,
            dgg: Default::default(), // Assuming DggType implements Default
        }
    }

    pub fn with_params(elemparams: Arc<E::Params>, dgg: E::DggType, stddev: f64) -> Self {
        Self {
            stddev,
            elemparams: Some(elemparams),
            dgg,
        }
    }

    pub fn get_elem_params(&self) -> Option<&Arc<E::Params>> {
        self.elemparams.as_ref()
    }

    pub fn set_elem_params(&mut self, elemparams: Arc<E::Params>) {
        self.elemparams = Some(elemparams);
    }

    pub fn get_dgg(&self) -> &E::DggType {
        &self.dgg
    }

    pub fn set_dgg(&mut self, dgg: E::DggType) {
        self.dgg = dgg;
    }

    pub fn get_std_dev(&self) -> f64 {
        self.stddev
    }

    pub fn set_std_dev(&mut self, stddev: f64) {
        self.stddev = stddev;
        // Assuming DggType has a method set_std
        // self.dgg.set_std(stddev);
    }
}

pub struct RLWETrapdoorParams<E: Element> {
    base_params: TrapdoorParams<E>,
    base: i64,
    k: usize,
    bal: bool,
    n: usize,
    dgg_large_sigma: E::DggType,
}

impl<E: Element> RLWETrapdoorParams<E> {
    pub fn new() -> Self {
        Self {
            base_params: TrapdoorParams::new(),
            base: 0,
            k: 0,
            bal: false,
            n: 0,
            dgg_large_sigma: Default::default(), // Assuming DggType implements Default
        }
    }

    // Additional methods for RLWETrapdoorParams can be implemented here
}

pub struct PerturbationVector<E: Element> {
    pvector: Option<Arc<Matrix<E>>>,
}

impl<E: Element> PerturbationVector<E> {
    pub fn new() -> Self {
        Self { pvector: None }
    }

    pub fn with_vector(pvector: Arc<Matrix<E>>) -> Self {
        Self { pvector: Some(pvector) }
    }

    pub fn set_vector(&mut self, pvector: Arc<Matrix<E>>) {
        self.pvector = Some(pvector);
    }

    pub fn get_vector(&self) -> Option<&Arc<Matrix<E>>> {
        self.pvector.as_ref()
    }
}

use std::fmt;
use std::marker::PhantomData;

pub struct ILParamsImpl<IntType> {
    order: u32,
    modulus: IntType,
    root_of_unity: IntType,
    big_modulus: Option<IntType>,
    big_root_of_unity: Option<IntType>,
    _marker: PhantomData<IntType>,
}

impl<IntType> ILParamsImpl<IntType> {
    pub fn new(order: u32, modulus: IntType, root_of_unity: IntType) -> Self {
        ILParamsImpl {
            order,
            modulus,
            root_of_unity,
            big_modulus: None,
            big_root_of_unity: None,
            _marker: PhantomData,
        }
    }

    pub fn with_big_params(order: u32, modulus: IntType, root_of_unity: IntType, big_modulus: IntType, big_root_of_unity: IntType) -> Self {
        ILParamsImpl {
            order,
            modulus,
            root_of_unity,
            big_modulus: Some(big_modulus),
            big_root_of_unity: Some(big_root_of_unity),
            _marker: PhantomData,
        }
    }
}

impl<IntType: PartialEq> PartialEq for ILParamsImpl<IntType> {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
            && self.modulus == other.modulus
            && self.root_of_unity == other.root_of_unity
            && self.big_modulus == other.big_modulus
            && self.big_root_of_unity == other.big_root_of_unity
    }
}

impl<IntType: fmt::Display> fmt::Display for ILParamsImpl<IntType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ILParams order: {}, modulus: {}, root_of_unity: {}", self.order, self.modulus, self.root_of_unity)?;
        if let Some(ref big_modulus) = self.big_modulus {
            write!(f, ", big_modulus: {}", big_modulus)?;
        }
        if let Some(ref big_root_of_unity) = self.big_root_of_unity {
            write!(f, ", big_root_of_unity: {}", big_root_of_unity)?;
        }
        Ok(())
    }
}



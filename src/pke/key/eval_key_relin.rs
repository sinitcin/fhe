use std::sync::Arc;
use std::vec::Vec;

pub struct EvalKeyRelinImpl<Element> {
    context: Option<Arc<CryptoContext<Element>>>,
    r_key: Vec<Vec<Element>>,
    dcrt_keys: Vec<DCRTPoly>,
}

impl<Element> EvalKeyRelinImpl<Element> {
    pub fn new(context: Option<Arc<CryptoContext<Element>>>) -> Self {
        EvalKeyRelinImpl {
            context,
            r_key: Vec::new(),
            dcrt_keys: Vec::new(),
        }
    }

    pub fn set_a_vector(&mut self, a: Vec<Element>) {
        if self.r_key.len() < 1 {
            self.r_key.push(a);
        } else {
            self.r_key[0] = a;
        }
    }

    pub fn get_a_vector(&self) -> Option<&Vec<Element>> {
        self.r_key.get(0)
    }

    pub fn set_b_vector(&mut self, b: Vec<Element>) {
        if self.r_key.len() < 2 {
            self.r_key.push(b);
        } else {
            self.r_key[1] = b;
        }
    }

    pub fn get_b_vector(&self) -> Option<&Vec<Element>> {
        self.r_key.get(1)
    }

    pub fn set_ain_dcrt(&mut self, a: DCRTPoly) {
        if self.dcrt_keys.len() < 1 {
            self.dcrt_keys.push(a);
        } else {
            self.dcrt_keys[0] = a;
        }
    }

    pub fn get_ain_dcrt(&self) -> Option<&DCRTPoly> {
        self.dcrt_keys.get(0)
    }

    pub fn set_bin_dcrt(&mut self, b: DCRTPoly) {
        if self.dcrt_keys.len() < 2 {
            self.dcrt_keys.push(b);
        } else {
            self.dcrt_keys[1] = b;
        }
    }

    pub fn get_bin_dcrt(&self) -> Option<&DCRTPoly> {
        self.dcrt_keys.get(1)
    }

    pub fn clear_keys(&mut self) {
        self.r_key.clear();
        self.dcrt_keys.clear();
    }

    pub fn key_compare(&self, other: &EvalKeyRelinImpl<Element>) -> bool {
        if Arc::ptr_eq(&self.context, &other.context) && self.r_key.len() == other.r_key.len() {
            for (i, sub_key) in self.r_key.iter().enumerate() {
                if sub_key.len() != other.r_key[i].len() {
                    return false;
                }
                for (j, element) in sub_key.iter().enumerate() {
                    if element != &other.r_key[i][j] {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    // Serialization and deserialization methods would be implemented here.
    // Rust uses different serialization frameworks like serde, and the exact
    // implementation would depend on the chosen framework and the format
    // (e.g., JSON, Bincode, etc.).
}



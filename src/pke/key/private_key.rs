use rand::{distributions::Uniform, Rng};
use std::fmt::Write;
use std::sync::Arc;

fn generate_unique_key_id() -> String {
    let ints_in_id = 128 / (std::mem::size_of::<u32>() * 8);
    let distribution = Uniform::new_inclusive(0, u32::MAX);
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    for _ in 0..ints_in_id {
        write!(s, "{:08x}", rng.sample(&distribution)).unwrap();
    }
    s
}

pub trait CryptoObject {
    type Element;
    fn get_crypto_context(&self) -> Arc<Self::Element>;
    fn get_key_tag(&self) -> String;
}

pub struct PrivateKeyImpl<Element> {
    context: Arc<Element>,
    key_tag: String,
    m_sk: Element,
}

impl<Element> CryptoObject for PrivateKeyImpl<Element> {
    type Element = Element;

    fn get_crypto_context(&self) -> Arc<Self::Element> {
        Arc::clone(&self.context)
    }

    fn get_key_tag(&self) -> String {
        self.key_tag.clone()
    }
}

impl<Element> PrivateKeyImpl<Element> {
    pub fn new(context: Arc<Element>) -> Self {
        PrivateKeyImpl {
            context,
            key_tag: generate_unique_key_id(),
            m_sk: Default::default(),
        }
    }

    pub fn from(other: &Self) -> Self {
        PrivateKeyImpl {
            context: Arc::clone(&other.context),
            key_tag: other.key_tag.clone(),
            m_sk: other.m_sk.clone(),
        }
    }

    pub fn from_parts(context: Arc<Element>, m_sk: Element) -> Self {
        PrivateKeyImpl {
            context,
            key_tag: generate_unique_key_id(),
            m_sk,
        }
    }

    pub fn get_private_element(&self) -> &Element {
        &self.m_sk
    }

    pub fn set_private_element(&mut self, x: Element) {
        self.m_sk = x;
    }

    pub fn is_valid(&self) -> bool {
        Arc::strong_count(&self.context) > 0
    }
}

impl<Element: PartialEq> PartialEq for PrivateKeyImpl<Element> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.context, &other.context) && self.m_sk == other.m_sk
    }
}

impl<Element: Eq> Eq for PrivateKeyImpl<Element> {}

impl<Element: Clone> Clone for PrivateKeyImpl<Element> {
    fn clone(&self) -> Self {
        PrivateKeyImpl::from(self)
    }
}

// Serialization and deserialization traits would need to be implemented
// according to the specific requirements and available libraries in Rust.

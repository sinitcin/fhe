use std::sync::Arc;
use std::vec::Vec;
use std::string::String;
use std::ops::{Deref, DerefMut};

use crate::pke::gen_cryptocontext::CryptoContext;

pub struct PublicKeyImpl<Element> {
    context: Arc<CryptoContext<Element>>,
    key_tag: String,
    pub m_h: Vec<Element>,
}

impl<Element> PublicKeyImpl<Element> {
    pub fn new(context: Arc<CryptoContext<Element>>, id: &str) -> Self {
        PublicKeyImpl {
            context,
            key_tag: id.to_string(),
            m_h: Vec::new(),
        }
    }

    pub fn from(other: &Self) -> Self {
        PublicKeyImpl {
            context: Arc::clone(&other.context),
            key_tag: other.key_tag.clone(),
            m_h: other.m_h.clone(),
        }
    }

    pub fn from_parts(context: Arc<CryptoContext<Element>>, key_tag: String, m_h: Vec<Element>) -> Self {
        PublicKeyImpl { context, key_tag, m_h }
    }

    pub fn is_valid(&self) -> bool {
        Arc::strong_count(&self.context) > 0 && !self.m_h.is_empty()
    }

    pub fn get_public_elements(&self) -> &Vec<Element> {
        &self.m_h
    }

    pub fn set_public_elements(&mut self, element: Vec<Element>) {
        self.m_h = element;
    }

    pub fn set_public_element_at_index(&mut self, idx: usize, element: Element) {
        if idx < self.m_h.len() {
            self.m_h[idx] = element;
        } else {
            self.m_h.insert(idx, element);
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.context == other.context && self.key_tag == other.key_tag && self.m_h == other.m_h
    }
}

impl<Element> PartialEq for PublicKeyImpl<Element> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl<Element> Eq for PublicKeyImpl<Element> {}

impl<Element> Clone for PublicKeyImpl<Element> {
    fn clone(&self) -> Self {
        PublicKeyImpl::from(self)
    }
}

// Serialization and deserialization traits would be implemented here,
// possibly using serde if Element supports it.



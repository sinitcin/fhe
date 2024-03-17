use std::rc::Rc;

use super::private_key::PrivateKeyImpl;

pub type PrivateKey = Rc<dyn PrivateKeyImpl<Element>>;
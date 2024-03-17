use std::string::String;
use std::sync::Arc;
use std::vec::Vec;

use crate::pke::key::private_key_fwd::PrivateKey;

pub trait KeySwitchBase<Element> {
    type Params;

    fn key_switch_gen_internal(
        &self,
        old_private_key: &PrivateKey<Element>,
        new_private_key: &PrivateKey<Element>,
    ) -> Result<EvalKey<Element>, String> {
        Err(format!(
            "{} is not supported",
            stringify!(key_switch_gen_internal)
        ))
    }

    fn key_switch_gen_internal_with_eval_key(
        &self,
        old_private_key: &PrivateKey<Element>,
        new_private_key: &PrivateKey<Element>,
        eval_key: &EvalKey<Element>,
    ) -> Result<EvalKey<Element>, String> {
        Err(format!(
            "{} is not supported",
            stringify!(key_switch_gen_internal_with_eval_key)
        ))
    }

    fn key_switch_gen_internal_with_public_key(
        &self,
        old_private_key: &PrivateKey<Element>,
        new_public_key: &PublicKey<Element>,
    ) -> Result<EvalKey<Element>, String> {
        Err(format!(
            "{} is not supported",
            stringify!(key_switch_gen_internal_with_public_key)
        ))
    }

    fn key_switch(
        &self,
        ciphertext: &Ciphertext<Element>,
        eval_key: &EvalKey<Element>,
    ) -> Result<Ciphertext<Element>, String>;

    fn key_switch_in_place(
        &self,
        ciphertext: &mut Ciphertext<Element>,
        eval_key: &EvalKey<Element>,
    ) -> Result<(), String> {
        Err("KeySwitch is not supported".to_string())
    }

    fn key_switch_ext(
        &self,
        ciphertext: &Ciphertext<Element>,
        add_first: bool,
    ) -> Result<Ciphertext<Element>, String> {
        Err("KeySwitchExt is not supported".to_string())
    }

    fn key_switch_down(
        &self,
        ciphertext: &Ciphertext<Element>,
    ) -> Result<Ciphertext<Element>, String> {
        Err("KeySwitchDown is not supported".to_string())
    }

    fn key_switch_down_first_element(
        &self,
        ciphertext: &Ciphertext<Element>,
    ) -> Result<Element, String> {
        Err("KeySwitchDownFirstElement is not supported".to_string())
    }

    fn key_switch_core(
        &self,
        a: &Element,
        eval_key: &EvalKey<Element>,
    ) -> Result<Arc<Vec<Element>>, String> {
        Err("KeySwitchCore is not supported".to_string())
    }

    fn eval_key_switch_precompute_core(
        &self,
        c: &Element,
        crypto_params_base: Arc<CryptoParametersBase<Element>>,
    ) -> Result<Arc<Vec<Element>>, String> {
        Err("EvalKeySwitchPrecomputeCore is not supported".to_string())
    }

    fn eval_fast_key_switch_core(
        &self,
        digits: Arc<Vec<Element>>,
        eval_key: &EvalKey<Element>,
        params_ql: Arc<Self::Params>,
    ) -> Result<Arc<Vec<Element>>, String> {
        Err("EvalFastKeySwitchCore is not supported".to_string())
    }

    fn eval_fast_key_switch_core_ext(
        &self,
        digits: Arc<Vec<Element>>,
        eval_key: &EvalKey<Element>,
        params_ql: Arc<Self::Params>,
    ) -> Result<Arc<Vec<Element>>, String> {
        Err("EvalFastKeySwitchCoreExt is not supported".to_string())
    }
}

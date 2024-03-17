//
// HOW TO GENERATE CRYPTOCONTEXT BY CALLING GenCryptoContext()
//
// 1. Pick the scheme you want to use. I choose CKKS for our tutorial example.
// 2. Your code must include this header file and the header with the scheme-specific
//    context generator (scheme/<scheme>/cryptocontext-<scheme>.h):
//       #include "scheme/ckks/cryptocontext-ckks.h"
//       #include "gen-cryptocontext.h"
// 3. Create a parameter object to be passed as a parameter in to GenCryptoContext(). Its generic
//    form would look like this: CCParams<GeneratorName<Element>> parameters
//    where
//    - GeneratorName is the name of the class defined in cryptocontext-<scheme>.h. In our case
//      it is CryptoContextCKKS.
//    - Element is a template parameter representing integer lattice. So, it can stay Element or
//      be replaced with Poly, NativePoly or DCRTPoly. I leave "Element".
//      As the result we can add this line:
//       CCParams<CryptoContextCKKS<Element>> parameters;
// 4. Adjust the parameters' values with set functions for CCParams<CryptoContextCKKS<Element>> as
//    the object is created using default values from scheme/cryptocontextparams-defaults.h.
// 5. Call GenCryptoContext() to generate cryptocontext.
//
// Now your code should look like this:
//       #include "scheme/ckks/cryptocontext-ckks.h"
//       #include "gen-cryptocontext.h"
//       ...........................................
//       CCParams<CryptoContextCKKS<Element>> parameters;
//       parameters.SetMultiplicativeDepth(1);
//       parameters.SetScalingModSize(50);
//       parameters.SetBatchSize(8);
//       parameters.SetSecurityLevel(HEStd_NotSet);
//       parameters.SetRingDim(16);
//
//       auto cryptoContext = GenCryptoContext(parameters);
//
//       cryptoContext->Enable(ENCRYPTION);
//       cryptoContext->Enable(KEYSWITCH);
//       cryptoContext->Enable(LEVELEDSHE);
//       ...........................................
//
// More examples can be found in src/pke/unittest/UnitTestAutomorphism.cpp or in
// src/pke/unittest/UnitTestEvalMult.cpp.
//

use std::{marker::PhantomData, rc::Rc};

use crate::{errors::FHEError, fhe_schemes::FHEScheme};

use super::{scheme::ckksrns::CryptoContextParams, schemebase::base_scheme::SchemeBase};

/**
 * @brief CryptoContextImpl
 *
 * A CryptoContextImpl is the object used to access the OpenFHE library
 *
 * All OpenFHE functionality is accessed by way of an instance of a
 * CryptoContextImpl; we say that various objects are "created in" a context,
 * and can only be used in the context in which they were created
 *
 * All OpenFHE methods are accessed through CryptoContextImpl methods. Guards
 * are implemented to make certain that only valid objects that have been
 * created in the context are used
 *
 * Contexts are created using GenCryptoContext(), and can be serialized
 * and recovered from a serialization
 */
pub struct CryptoContextImpl<Element> {
    m_scheme_id: FHEScheme,
    scheme: Rc<SchemeBase<Element>>,
    m_key_gen_level: Option<u32>,
    phantom_data: PhantomData<Element>,
}

impl<Element> CryptoContextImpl<Element> {
    /**
     * @brief VerifyCKKSScheme is to check if the cryptocontext scheme is CKKS. if it is not
     *        the function will thow an exception
     * @param functionName is the calling function name. __func__ can be used instead
     */
    #[inline]
    fn verify_ckks_scheme(self, function_name: &str) -> Result<(), FHEError> {
        if self.m_scheme_id != FHEScheme::CKKSRNS {
            let err_msg = format!(
                "Function {}  is available for the CKKS scheme only.
            The current scheme is {}.",
                function_name, self.m_scheme_id
            );
            Err(FHEError::InvalidScheme(err_msg))?;
        }
        Ok(())
    }

    fn set_key_switch_technique_in_scheme(&self) {
        // check if the scheme is an RNS scheme
        // check if the parameter object is RNS-based
    }
}

pub type CryptoContext<Element> = Rc<CryptoContextImpl<Element>>;

trait GenCryptoContext {
    fn gen_crypto_context<T>(params: &CryptoContextParams<T>) -> Result<(), FHEError>;
}

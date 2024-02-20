use std::marker::PhantomData;

// use crate::{errors::FHEError, pke::constants::PKESchemeFeature};

pub struct CallerInfo {
    caller_file: String,
    caller_function: String,
    caller_line: u32,
}

pub struct SchemeBase<Element> {
    phantom_data: PhantomData<Element>,
}

// pub trait SchemeBase<Element> {
//     type ParmType;
//     type IntType;
//     type DugType;
//     type DggType;
//     type TugType;

//     #[inline]
//     fn check_multiparty_decrypt_compatibility(
//         ciphertext: &ConstCiphertext<Element>,
//         caller_info: CallerInfo,
//     ) {
//         if (ciphertext.elements().size() > 2) {
//             let err_msg = format!("ciphertext's number of elements is [{}].  Must be 2 or less for Multiparty Decryption. {}
//             ", ciphertext.elements().size(), caller_info);
//             Err(FHEError::Unknown(err_msg))?;
//         }
//     }

//     /**
//      * Enable features with a bit mast of PKESchemeFeature codes
//      * @param mask
//      */
//     fn enable(mask: PKESchemeFeature) {
//         // if (mask & PKESchemeFeature::PKE) {
//         //     Enable(PKESchemeFeature::PKE);
//         // }
//         // if (mask & PKESchemeFeature::KEYSWITCH) {
//         //     Enable(PKESchemeFeature::KEYSWITCH);
//         // }
//         // if (mask & PKESchemeFeature::LEVELEDSHE) {
//         //     Enable(PKESchemeFeature::LEVELEDSHE);
//         // }
//         // if (mask & PKESchemeFeature::ADVANCEDSHE) {
//         //     Enable(PKESchemeFeature::ADVANCEDSHE);
//         // }
//         // if (mask & PKESchemeFeature::PRE) {
//         //     Enable(PKESchemeFeature::PRE);
//         // }
//         // if (mask & PKESchemeFeature::MULTIPARTY) {
//         //     Enable(PKESchemeFeature::MULTIPARTY);
//         // }
//         // if (mask & PKESchemeFeature::FHE) {
//         //     Enable(PKESchemeFeature::FHE);
//         // }
//         // if (mask & PKESchemeFeature::SCHEMESWITCH) {
//         //     Enable(PKESchemeFeature::SCHEMESWITCH);
//         // }
//         todo!()
//     }

//     fn  get_enabled() -> u32 {
//         todo!() }

//     fn is_feature_enabled(feature: PKESchemeFeature) -> bool {
//         match feature {
//             PKESchemeFeature::PKE => todo!(),
//             PKESchemeFeature::KEYSWITCH => todo!(),
//             PKESchemeFeature::PRE => todo!(),
//             PKESchemeFeature::LEVELEDSHE => todo!(),
//             PKESchemeFeature::ADVANCEDSHE => todo!(),
//             PKESchemeFeature::MULTIPARTY => todo!(),
//             PKESchemeFeature::FHE => todo!(),
//             PKESchemeFeature::SCHEMESWITCH => todo!(),
//         }
//     }

//     // fn  GetEnabled() -> u32 {
//     //     uint32_t flag = 0;
//     //     if (m_PKE != nullptr)
//     //         flag |= PKE;
//     //     if (m_KeySwitch != nullptr)
//     //         flag |= KEYSWITCH;
//     //     if (m_LeveledSHE != nullptr)
//     //         flag |= LEVELEDSHE;
//     //     if (m_AdvancedSHE != nullptr)
//     //         flag |= ADVANCEDSHE;
//     //     if (m_PRE != nullptr)
//     //         flag |= PRE;
//     //     if (m_Multiparty != nullptr)
//     //         flag |= MULTIPARTY;
//     //     if (m_FHE != nullptr)
//     //         flag |= FHE;
//     //     if (m_SchemeSwitch != nullptr)
//     //         flag |= SCHEMESWITCH;
//     //     return flag;
//     // }
// }

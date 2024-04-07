/*
  This file contains the functionality to switch between math backends
*/
use crate::math::hal::bigintbackend::BigInteger as bigintbackend_BigInteger;
use crate::math::hal::bigintbackend::BigVector as bigintbackend_BigVector;
use crate::math::hal::bigintdyn::ChineseRemainderTransformArbDyn as bigintdyn_ChineseRemainderTransformArbDyn;
use crate::math::hal::bigintdyn::ChineseRemainderTransformFTTDyn as bigintdyn_ChineseRemainderTransformFTTDyn;
use crate::math::hal::bigintfxd::ChineseRemainderTransformArbFxd as bigintfxd_ChineseRemainderTransformArbFxd;
use crate::math::hal::bigintfxd::ChineseRemainderTransformFTTFxd as bigintfxd_ChineseRemainderTransformFTTFxd;
use crate::math::hal::nativeintbackend::ChineseRemainderTransformArbNat as intnat_ChineseRemainderTransformArbNat;
use crate::math::hal::nativeintbackend::ChineseRemainderTransformFTTNat as intnat_ChineseRemainderTransformFTTNat;
use crate::math::hal::ntl::ChineseRemainderTransformArbNtl as NTL_ChineseRemainderTransformArbNtl;
use crate::math::hal::ntl::ChineseRemainderTransformFTTNtl as NTL_ChineseRemainderTransformFTTNtl;
use crate::math::hal::vector::NativeVector;

type BigInteger = bigintbackend_BigInteger;
type BigVector = bigintbackend_BigVector;

type NatChineseRemainderTransformFTT<VecType> = intnat_ChineseRemainderTransformFTTNat<VecType>;
type NatChineseRemainderTransformArb<VecType> = intnat_ChineseRemainderTransformArbNat<VecType>;

struct FTTTypedef<VecType> {
    type_: (),
}
impl FTTTypedef<NativeVector> {
    type type_ = NatChineseRemainderTransformFTT<NativeVector>;
}

struct FTTTypedef<M2Vector> {
    type_: (),
}
impl FTTTypedef<M2Vector> {
    type type_ = bigintfxd_ChineseRemainderTransformFTTFxd<M2Vector>;
}

struct FTTTypedef<M4Vector> {
    type_: (),
}
impl FTTTypedef<M4Vector> {
    type type_ = bigintdyn_ChineseRemainderTransformFTTDyn<M4Vector>;
}

struct FTTTypedef<M6Vector> {
    type_: (),
}
impl FTTTypedef<M6Vector> {
    type type_ = NTL_ChineseRemainderTransformFTTNtl<M6Vector>;
}

type ChineseRemainderTransformFTT<VecType> = FTTTypedef<VecType>::type_;

struct ArbTypedef<VecType> {
    type_: (),
}
impl ArbTypedef<NativeVector> {
    type type_ = NatChineseRemainderTransformArb<NativeVector>;
}

struct ArbTypedef<M2Vector> {
    type_: (),
}
impl ArbTypedef<M2Vector> {
    type type_ = bigintfxd_ChineseRemainderTransformArbFxd<M2Vector>;
}

struct ArbTypedef<M4Vector> {
    type_: (),
}
impl ArbTypedef<M4Vector> {
    type type_ = bigintdyn_ChineseRemainderTransformArbDyn<M4Vector>;
}

struct ArbTypedef<M6Vector> {
    type_: (),
}
impl ArbTypedef<M6Vector> {
    type type_ = NTL_ChineseRemainderTransformArbNtl<M6Vector>;
}

type ChineseRemainderTransformArb<VecType> = ArbTypedef<VecType>::type_;

#[cfg(PROFILE)]
use encoding::encodings;
use gtest::gtest;
use lattice::lat_hal;
use math::math_hal;
use utils::utilities;
use std::vec::Vec;
use std::string::String;
use std::i64;
use std::u32;
use std::shared::Shared;
use std::make_shared;
use std::cout;
use lbcrypto;

struct UTGENERAL_ENCODING;

impl UTGENERAL_ENCODING {
    fn setup(&self) {}
    fn teardown(&self) {}
}

#[test]
fn coef_packed_encoding() {
    let value: Vec<i64> = vec![32, 17, 8, -12, -32, 22, -101, 6];
    let m: u32 = 16;
    let lp = make_shared<ILParamsImpl<BigInteger>>(m);
    let ep = EncodingParams(make_shared<EncodingParamsImpl>(256));
    let se = CoefPackedEncoding(lp, ep, value);
    se.Encode();
    se.Decode();
    se.SetLength(value.len());
    assert_eq!(se.GetCoefPackedValue(), value, "COEF_PACKED_ENCODING");
}

#[test]
fn packed_int_ptxt_encoding() {
    let m: u32 = 22;
    let p: PlaintextModulus = 89;
    let modulusQ: BigInteger = BigInteger::from(955263939794561);
    let squareRootOfRoot: BigInteger = BigInteger::from(941018665059848);
    let bigmodulus: BigInteger = BigInteger::from(80899135611688102162227204937217);
    let bigroot: BigInteger = BigInteger::from(77936753846653065954043047918387);
    let cycloPoly = GetCyclotomicPolynomial<BigVector>(m, modulusQ);
    ChineseRemainderTransformArb<BigVector>().SetCylotomicPolynomial(cycloPoly, modulusQ);
    let lp = make_shared<ILParams>(m, modulusQ, squareRootOfRoot, bigmodulus, bigroot);
    let ep = EncodingParams(make_shared<EncodingParamsImpl>(p, 8));
    PackedEncoding::SetParams(m, ep);
    let vectorOfInts1: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0, 0];
    let se = PackedEncoding(lp, ep, vectorOfInts1);
    se.Encode();
    se.Decode();
    assert_eq!(se.GetPackedValue(), vectorOfInts1, "packed int");
}

#[test]
fn packed_int_ptxt_encoding_negative() {
    let m: u32 = 22;
    let p: PlaintextModulus = 89;
    let modulusQ: BigInteger = BigInteger::from(955263939794561);
    let squareRootOfRoot: BigInteger = BigInteger::from(941018665059848);
    let bigmodulus: BigInteger = BigInteger::from(80899135611688102162227204937217);
    let bigroot: BigInteger = BigInteger::from(77936753846653065954043047918387);
    let cycloPoly = GetCyclotomicPolynomial<BigVector>(m, modulusQ);
    ChineseRemainderTransformArb<BigVector>().SetCylotomicPolynomial(cycloPoly, modulusQ);
    let lp = make_shared<ILParams>(m, modulusQ, squareRootOfRoot, bigmodulus, bigroot);
    let ep = EncodingParams(make_shared<EncodingParamsImpl>(p, 8));
    PackedEncoding::SetParams(m, ep);
    let vectorOfInts1: Vec<i64> = vec![1, 2, -3, 4, 5, -6, 7, 8, 0, 0];
    let se = PackedEncoding(lp, ep, vectorOfInts1);
    se.Encode();
    se.Decode();
    assert_eq!(se.GetPackedValue(), vectorOfInts1, "packed int");
}

#[test]
fn packed_int_ptxt_encoding_DCRTPoly_prime_cyclotomics() {
    let init_size: u32 = 3;
    let dcrtBits: u32 = 24;
    let dcrtBitsBig: u32 = 57;
    let m: u32 = 1811;
    let p: PlaintextModulus = 2 * m + 1;
    let modulusP: BigInteger = BigInteger::from(p);
    let mArb: u32 = 2 * m;
    let mNTT: u32 = pow(2, ceil(log2(2 * m - 1)));
    
    let mut init_moduli: Vec<NativeInteger> = Vec::with_capacity(init_size);
    let mut init_rootsOfUnity: Vec<NativeInteger> = Vec::with_capacity(init_size);
    init_moduli[0] = LastPrime<NativeInteger>(dcrtBits, mArb);
    init_rootsOfUnity[0] = RootOfUnity(mArb, init_moduli[0]);
    for i in 1..init_size {
        init_moduli[i] = PreviousPrime(init_moduli[i - 1], mArb);
        init_rootsOfUnity[i] = RootOfUnity(mArb, init_moduli[i]);
    }
    
    let mut init_moduli_NTT: Vec<NativeInteger> = Vec::with_capacity(init_size);
    let mut init_rootsOfUnity_NTT: Vec<NativeInteger> = Vec::with_capacity(init_size);
    init_moduli_NTT[0] = LastPrime<NativeInteger>(dcrtBitsBig, mNTT);
    init_rootsOfUnity_NTT[0] = RootOfUnity(mNTT, init_moduli_NTT[0]);
    for i in 1..init_size {
        init_moduli_NTT[i] = PreviousPrime(init_moduli_NTT[i - 1], mNTT);
        init_rootsOfUnity_NTT[i] = RootOfUnity(mNTT, init_moduli_NTT[i]);
    }
    let paramsDCRT = make_shared<ILDCRTParams<BigInteger>>(m, init_moduli, init_rootsOfUnity, init_moduli_NTT,
                                                                 init_rootsOfUnity_NTT);
    let ep = EncodingParams(make_shared<EncodingParamsImpl>(p));
    PackedEncoding::SetParams(m, ep);
    let vectorOfInts1: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0, 0];
    let se = PackedEncoding(paramsDCRT, ep, vectorOfInts1);
    se.Encode();
    se.GetElement<DCRTPoly>().SwitchFormat();
    se.GetElement<DCRTPoly>().SwitchFormat();
    se.Decode();
    se.SetLength(vectorOfInts1.len());
    assert_eq!(se.GetPackedValue(), vectorOfInts1, "packed int - prime cyclotomics");
}

#[test]
fn packed_int_ptxt_encoding_DCRTPoly_prime_cyclotomics_negative() {
    let init_size: u32 = 3;
    let dcrtBits: u32 = 24;
    let dcrtBitsBig: u32 = 57;
    let m: u32 = 1811;
    let p: PlaintextModulus = 2 * m + 1;
    let modulusP: BigInteger = BigInteger::from(p);
    let mArb: u32 = 2 * m;
    let mNTT: u32 = pow(2, ceil(log2(2 * m - 1)));
    
    let mut init_moduli: Vec<NativeInteger> = Vec::with_capacity(init_size);
    let mut init_rootsOfUnity: Vec<NativeInteger> = Vec::with_capacity(init_size);
    init_moduli[0] = LastPrime<NativeInteger>(dcrtBits, mArb);
    init_rootsOfUnity[0] = RootOfUnity(mArb, init_moduli[0]);
    for i in 1..init_size {
        init_moduli[i] = PreviousPrime(init_moduli[i - 1], mArb);
        init_rootsOfUnity[i] = RootOfUnity(mArb, init_moduli[i]);
    }
    
    let mut init_moduli_NTT: Vec<NativeInteger> = Vec::with_capacity(init_size);
    let mut init_rootsOfUnity_NTT: Vec<NativeInteger> = Vec::with_capacity(init_size);
    init_moduli_NTT[0] = LastPrime<NativeInteger>(dcrtBitsBig, mNTT);
    init_rootsOfUnity_NTT[0] = RootOfUnity(mNTT, init_moduli_NTT[0]);
    for i in 1..init_size {
        init_moduli_NTT[i] = PreviousPrime(init_moduli_NTT[i - 1], mNTT);
        init_rootsOfUnity_NTT[i] = RootOfUnity(mNTT, init_moduli_NTT[i]);
    }
    let paramsDCRT = make_shared<ILDCRTParams<BigInteger>>(m, init_moduli, init_rootsOfUnity, init_moduli_NTT,
                                                                 init_rootsOfUnity_NTT);
    let ep = EncodingParams(make_shared<EncodingParamsImpl>(p));
    PackedEncoding::SetParams(m, ep);
    let vectorOfInts1: Vec<i64> = vec![1, 2, -3, 4, 5, 6, -7, 8, 0, 0];
    let se = PackedEncoding(paramsDCRT, ep, vectorOfInts1);
    se.Encode();
    se.GetElement<DCRTPoly>().SwitchFormat();
    se.GetElement<DCRTPoly>().SwitchFormat();
    se.Decode();
    se.SetLength(vectorOfInts1.len());
    assert_eq!(se.GetPackedValue(), vectorOfInts1, "packed int - prime cyclotomics");
}

#[test]
fn string_encoding() {
    let value: String = String::from("Hello, world!");
    let m: u32 = 64;
    let lp = make_shared<ILParamsImpl<BigInteger>>(m);
    let ep = EncodingParams(make_shared<EncodingParamsImpl>(256));
    let se = StringEncoding(lp, ep, value);
    se.Encode();
    se.Decode();
    assert_eq!(se.GetStringValue(), value, "string encode/decode");
    
    let lp2 = make_shared<ILParamsImpl<BigInteger>>(4);
    let se2 = StringEncoding(lp2, ep, value);
    se2.Encode();
    se2.Decode();
    assert_eq!(se2.GetStringValue(), value.substr(0, lp2.GetRingDimension()), "string truncate encode/decode");
}



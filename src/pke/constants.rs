/**
 * @brief Lists all features supported by public key encryption schemes
 */
pub enum PKESchemeFeature {
    PKE = 0x01,
    KEYSWITCH = 0x02,
    PRE = 0x04,
    LEVELEDSHE = 0x08,
    ADVANCEDSHE = 0x10,
    MULTIPARTY = 0x20,
    FHE = 0x40,
    SCHEMESWITCH = 0x80,
}

use std::mem;

use crate::{errors::FHEError, fhe_schemes::FHEScheme, scale_technique::ScalingTechnique};

pub type PlaintextModulus = u64;

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct SchemeParameters {
    scheme: FHEScheme,

    /// 🇷🇺 PlaintextModulus ptModulus используется в схемах типа BGV/BFV и влияет на рост шума
    /// 🇬🇧 PlaintextModulus ptModulus is used in BGV/BFV type schemes and impacts noise growth
    plain_text_modulus: PlaintextModulus,

    /// 🇷🇺 digitSize используется только при переключении ключей BV (KeySwitchTechnique = BV) и влияет на рост шума
    /// 🇬🇧 digitSize is used in BV Key Switching only (KeySwitchTechnique = BV) and impacts noise growth
    digit_size: u32,

    /// 🇷🇺 StandardDeviation используется для генерации гауссовых ошибок
    /// 🇬🇧 standardDeviation is used for Gaussian error generation
    standard_deviation: f64,

    /// 🇷🇺 Распределение секретных ключей: GAUSSIAN, UNIFORM_TERNARY и т. д.
    /// 🇬🇧 Secret key distribution: GAUSSIAN, UNIFORM_TERNARY, etc.
    secret_key_distribution: SecretKeyDistribution,

    /// 🇷🇺 Максимальная степень ре-линеаризации полинома секретного ключа (используется для ленивой ре-линеаризации)
    /// 🇬🇧 Max relinearization degree of secret key polynomial (used for lazy relinearization)
    max_relinearization_secret_key_degree: u32,

    /// 🇷🇺 Техника переключения ключей: BV или HYBRID в настоящее время
    /// Для BV у нас нет дополнительного модуля, поэтому безопасность зависит от модуля шифротекста Q.
    /// Для HYBRID у нас есть дополнительный модуль P, поэтому безопасность зависит от модуля P*Q.
    /// Для BV нам нужен digitSize - размер цифры в цифровом разложении
    /// Для HYBRID нам нужно numLargeDigits - количество цифр в цифровом разложении
    /// Хорошо бы иметь альтернативу numLargeDigits (возможно, numPrimesInDigit?).
    ///
    /// 🇬🇧 key switching technique: BV or HYBRID currently
    /// For BV we do not have extra modulus, so the security depends on ciphertext modulus Q.
    /// For HYBRID we do have extra modulus P, so the security depends on modulus P*Q
    /// For BV we need digitSize - digit size in digit decomposition
    /// For HYBRID we need numLargeDigits - number of digits in digit decomposition
    /// it is good to have alternative to numLargeDigits (possibly numPrimesInDigit?)
    key_switch_technique: KeySwitchTechnique,

    /// 🇷🇺 техника переключения масштаба/модуля, используемая в CKKS/BGV: FLEXIBLEAUTOEXT, FIXEDMANUL, FLEXIBLEAUTO и др.
    /// Подробности см. тут https://eprint.iacr.org/2022/915
    ///
    /// 🇬🇧 rescaling/modulus switching technique used in CKKS/BGV: FLEXIBLEAUTOEXT, FIXEDMANUL, FLEXIBLEAUTO, etc.
    /// see https://eprint.iacr.org/2022/915 for details
    scaling_technique: ScalingTechnique,

    /// 🇷🇺 максимальный размер пакета сообщений, которые необходимо упаковать в кодировку (количество слотов)
    /// 🇬🇧 max batch size of messages to be packed in encoding (number of slots)
    batch_size: u32,

    /// 🇷🇺 режим безопасности повторного шифрования прокси
    /// 🇬🇧 PRE security mode
    proxy_reencryption_mode: Option<ProxyReEncryptionMode>,

    /// 🇷🇺 Режим многосторонней безопасности в BFV/BGV
    /// NOISE_FLOODING_MULTIPARTY является более безопасным, чем FIXED_NOISE_MULTIPARTY.
    /// 🇬🇧 Multiparty security mode in BFV/BGV
    /// NOISE_FLOODING_MULTIPARTY is more secure than FIXED_NOISE_MULTIPARTY.
    multiparty_mode: MultipartyMode,

    /// 🇷🇺 Режим выполнения в CKKS
    /// В режиме EXEC_NOISE_ESTIMATION мы оцениваем уровень шума, который необходимо добавить к фактическим вычислениям, чтобы гарантировать хорошую безопасность.
    /// В режиме EXEC_EVALUATION мы вводим нашу оценку шума и выполняем требуемые безопасные зашифрованные вычисления.
    /// 🇬🇧 Execution mode in CKKS
    /// In EXEC_NOISE_ESTIMATION mode, we estimate the noise we need to add to the actual computation to guarantee good security.
    /// In EXEC_EVALUATION mode, we input our noise estimate and perform the desired secure encrypted computation.
    execution_mode: ExecutionMode,

    /// 🇷🇺 Режим шума при расшифровке в CKKS
    /// NOISE_FLOODING_DECRYPT более безопасен, чем FIXED_NOISE_DECRYPT, но он требует выполнения всех вычислений дважды.
    /// 🇬🇧 Decryption noise mode in CKKS
    /// NOISE_FLOODING_DECRYPT is more secure than FIXED_NOISE_DECRYPT, but it requires executing all computations twice.
    decryption_noise_mode: DecryptionNoiseMode,

    /// 🇷🇺 Оценка шума в CKKS для режима NOISE_FLOODING_DECRYPT.
    /// Эта оценка получена при выполнении вычислений в режиме EXEC_NOISE_ESTIMATION.
    /// 🇬🇧 Noise estimate in CKKS for NOISE_FLOODING_DECRYPT mode.
    /// This estimate is obtained from running the computation in EXEC_NOISE_ESTIMATION mode.
    noise_estimate: f64,

    /// 🇷🇺 Желаемая точность для 128-битных CKKS. Мы используем это значение в режиме NOISE_FLOODING_DECRYPT для определения коэффициента масштабирования.
    /// 🇬🇧 Desired precision for 128-bit CKKS. We use this value in NOISE_FLOODING_DECRYPT mode to determine the scaling factor.
    desired_precision: f64,

    /// 🇷🇺 Статистическая безопасность CKKS в режиме NOISE_FLOODING_DECRYPT. Это ограничение на вероятность успеха, которую может иметь любой противник.
    /// В частности, вероятность успеха не более 2^(-statisticalSecurity).
    /// 🇬🇧 Statistical security of CKKS in NOISE_FLOODING_DECRYPT mode. This is the bound on the probability of success
    /// that any adversary can have. Specifically, they a probability of success of at most 2^(-statisticalSecurity).
    statistical_security: u32,

    /// 🇷🇺 Это количество неблагоприятных запросов, ожидаемых пользователем для своего приложения, которое мы используем для
    /// обеспечения безопасности CKKS в режиме NOISE_FLOODING_DECRYPT.
    /// 🇬🇧 This is the number of adversarial queries a user is expecting for their application, which we use to ensure
    /// security of CKKS in NOISE_FLOODING_DECRYPT mode.
    num_adversarial_queries: u32,

    /// 🇷🇺 Это количество сторон в пороговом приложении, которое используется для привязки совместного секретного ключа
    /// 🇬🇧 This is the number of parties in a threshold application, which is used for bound on the joint secret key
    threshold_num_of_parties: u32,

    /// 🇷🇺 firstModSize и scalingModSize используются для вычисления модуля шифротекста.
    /// Модуль шифротекста следует рассматривать как:
    /// Q = q_0 * q_1 * ... * q_n * q'
    /// где q_0 - первая простая, а ее количество битов равно firstModSize
    /// Остальные q_i имеют одинаковое количество битов и равны scalingModSize
    /// Простое q' не задается явно, но оно используется внутри схем CKKS и BGV (в методах масштабирования *EXT)
    ///
    /// 🇬🇧 firstModSize and scalingModSize are used to calculate ciphertext modulus. The ciphertext modulus should be seen as:
    /// Q = q_0 * q_1 * ... * q_n * q'
    /// where q_0 is first prime, and it's number of bits is firstModSize
    /// other q_i have same number of bits and is equal to scalingModSize
    /// the prime q' is not explicitly given,
    /// but it is used internally in CKKS and BGV schemes (in *EXT scaling methods)
    first_mod_size: u32,
    scaling_mod_size: u32,

    /// 🇷🇺 см. KeySwitchTechnique - количество цифр при переключении ключей HYBRID
    /// 🇬🇧 see KeySwitchTechnique - number of digits in HYBRID key switching
    num_large_digits: u32,

    /// 🇷🇺 мультипликативная глубина (глубина умножения?)
    /// 🇬🇧 multiplicative depth
    multiplicative_depth: u32,

    /// 🇷🇺 Уровень безопасности:
    /// Мы используем значения из стандарта безопасности по адресу
    /// http://homomorphicencryption.org/wp-content/uploads/2018/11/HomomorphicEncryptionStandardv1.1.pdf
    /// Для заданной размерности кольца и уровня безопасности мы имеем верхнюю границу
    /// возможного наибольшего модуля (Q для BV или P*Q для HYBRID).

    /// 🇬🇧 security level:
    /// We use the values from the security standard  at
    /// http://homomorphicencryption.org/wp-content/uploads/2018/11/HomomorphicEncryptionStandardv1.1.pdf
    /// For given ring dimension and security level we have
    /// upper bound of possible highest modulus (Q for BV or P*Q for HYBRID)
    security_level: SecurityLevel,

    /// 🇷🇺 размерность кольца N схемы: кольцо равно Z_Q[x] / (X^N+1)
    /// 🇬🇧 ring dimension N of the scheme : the ring is Z_Q[x] / (X^N+1)
    ring_dimension: u32,

    /// 🇷🇺 количество дополнений (используется для настройки шума в BGV и BFV)
    /// 🇬🇧 number of additions (used for setting noise in BGV and BFV)
    eval_add_count: u32,

    /// 🇷🇺 количество операций переключения клавиш (используется для настройки шума в BGV и BFV)
    /// 🇬🇧 number of key switching operations (used for setting noise in BGV and BFV)
    key_switch_count: u32,

    /// 🇷🇺 размер модулей, используемых для PRE в доказываемой настройке HRA
    /// 🇬🇧 size of moduli used for PRE in the provable HRA setting
    multi_hop_mod_size: u32,

    /// 🇷🇺 СТАНДАРТНЫЙ или РАСШИРЕННЫЙ режим для шифрования BFV
    /// EXTENDED немного уменьшает размер Q (на несколько бит),
    /// но делает шифрование несколько медленнее подробности см. https://eprint.iacr.org/2022/915
    ///
    /// 🇬🇧 STANDARD or EXTENDED mode for BFV encryption
    /// EXTENDED slightly reduces the size of Q (by few bits) but makes encryption somewhat slower
    /// see https://eprint.iacr.org/2022/915 for details
    encryption_technique: EncryptionTechnique,

    /// 🇷🇺 метод умножения в BFV: BEHZ, HPS и т.д.
    /// Подробности см. https://eprint.iacr.org/2022/915
    ///
    /// 🇬🇧 multiplication method in BFV: BEHZ, HPS, etc.
    /// see https://eprint.iacr.org/2022/915 for details
    multiplication_technique: MultiplicationTechnique,

    /// 🇷🇺 Интерактивный многосторонний параметр бутстрапинга
    /// Установка степени сжатия шифротекста (SLACK или COMPACT)
    /// SLACK имеет более слабые предположения о безопасности, поэтому менее эффективен
    /// COMPACT имеет более сильные предположения о безопасности, поэтому более эффективен
    ///
    /// 🇬🇧 Interactive multi-party bootstrapping parameter
    /// Set the compression level in ciphertext (SLACK or COMPACT)
    /// SLACK has weaker security assumption, thus less efficient
    /// COMPACT has stronger security assumption, thus more efficient
    interactive_boot_compression_level: CompressionLevel,
}

impl SchemeParameters {
    pub fn new(scheme: FHEScheme) -> Self {
        let mut parameters = Self::default();
        parameters.set_to_defaults(scheme).expect("Unable to initialize scheme parameters");
        parameters
    }

    pub fn set_to_defaults(&mut self, scheme: FHEScheme) -> Result<(), FHEError> {
        match scheme {
            FHEScheme::CKKSRNS => {
                let _ = mem::replace(
                    self,
                    Self {
                        scheme,
                        plain_text_modulus: 0,
                        digit_size: 0,
                        standard_deviation: 3.19,
                        secret_key_distribution: SecretKeyDistribution::UniformTernary,
                        max_relinearization_secret_key_degree: 2,
                        key_switch_technique: KeySwitchTechnique::HYBRID,
                        scaling_technique: ScalingTechnique::FlexibleAutoExt,
                        batch_size: 0,
                        proxy_reencryption_mode: Some(ProxyReEncryptionMode::InDCPA),
                        multiparty_mode: MultipartyMode::FixedNoiseMultiparty,
                        execution_mode: ExecutionMode::Evaluation,
                        decryption_noise_mode: DecryptionNoiseMode::FixedNoiseDecrypt,
                        noise_estimate: 0.0,
                        desired_precision: 25.0,
                        statistical_security: 30,
                        num_adversarial_queries: 1,
                        threshold_num_of_parties: 1,
                        first_mod_size: 60,
                        scaling_mod_size: 59,
                        num_large_digits: 0,
                        multiplicative_depth: 1,
                        security_level: SecurityLevel::HEstd128Classic,
                        ring_dimension: 0,
                        eval_add_count: 0,
                        key_switch_count: 0,
                        multi_hop_mod_size: 0,
                        encryption_technique: EncryptionTechnique::STANDARD,
                        multiplication_technique: MultiplicationTechnique::HPS,
                        interactive_boot_compression_level: CompressionLevel::Slack,
                    },
                );
            }
            FHEScheme::BFVRNS => {
                let _ = mem::replace(
                    self,
                    Self {
                        scheme,
                        plain_text_modulus: 0,
                        digit_size: 0,
                        standard_deviation: 3.19,
                        secret_key_distribution: SecretKeyDistribution::UniformTernary,
                        max_relinearization_secret_key_degree: 2,
                        key_switch_technique: KeySwitchTechnique::BV,
                        scaling_technique: ScalingTechnique::NoRescale,
                        batch_size: 0,
                        proxy_reencryption_mode: Some(ProxyReEncryptionMode::InDCPA),
                        multiparty_mode: MultipartyMode::FixedNoiseMultiparty,
                        execution_mode: ExecutionMode::Evaluation,
                        decryption_noise_mode: DecryptionNoiseMode::FixedNoiseDecrypt,
                        noise_estimate: 0.0,
                        desired_precision: 0.0,
                        statistical_security: 30,
                        num_adversarial_queries: 1,
                        threshold_num_of_parties: 1,
                        first_mod_size: 60,
                        scaling_mod_size: 60,
                        num_large_digits: 0,
                        multiplicative_depth: 1,
                        security_level: SecurityLevel::HEstd128Classic,
                        ring_dimension: 0,
                        eval_add_count: 0,
                        key_switch_count: 0,
                        multi_hop_mod_size: 0,
                        encryption_technique: EncryptionTechnique::STANDARD,
                        multiplication_technique: MultiplicationTechnique::HPSPOVERQLEVELED,
                        interactive_boot_compression_level: CompressionLevel::Slack,
                    },
                );
            }
            FHEScheme::BGVRNS => {
                let _ = mem::replace(
                    self,
                    Self {
                        scheme,
                        plain_text_modulus: 0,
                        digit_size: 0,
                        standard_deviation: 3.19,
                        secret_key_distribution: SecretKeyDistribution::UniformTernary,
                        max_relinearization_secret_key_degree: 2,
                        key_switch_technique: KeySwitchTechnique::HYBRID,
                        scaling_technique: ScalingTechnique::FlexibleAutoExt,
                        batch_size: 0,
                        proxy_reencryption_mode: Some(ProxyReEncryptionMode::InDCPA),
                        multiparty_mode: MultipartyMode::FixedNoiseMultiparty,
                        execution_mode: ExecutionMode::Evaluation,
                        decryption_noise_mode: DecryptionNoiseMode::FixedNoiseDecrypt,
                        noise_estimate: 0.0,
                        desired_precision: 0.0,
                        statistical_security: 30,
                        num_adversarial_queries: 1,
                        threshold_num_of_parties: 1,
                        first_mod_size: 0,
                        scaling_mod_size: 0,
                        num_large_digits: 0,
                        multiplicative_depth: 1,
                        security_level: SecurityLevel::HEstd128Classic,
                        ring_dimension: 0,
                        eval_add_count: 5,
                        key_switch_count: 3,
                        multi_hop_mod_size: 0,
                        encryption_technique: EncryptionTechnique::STANDARD,
                        multiplication_technique: MultiplicationTechnique::HPS,
                        interactive_boot_compression_level: CompressionLevel::Slack,
                    },
                );
            }
        }
        Ok(())
    }

    fn validate_ring_dimension(ring_dimension: u32) -> Result<(), FHEError> {
        if !ring_dimension.is_power_of_two() {
            Err(FHEError::InvalidRingDimension(ring_dimension))?;
        }
        Ok(())
    }

    fn validate_multiplicative_depth(multiplicative_depth: u32) -> Result<(), FHEError> {
        const MAX_MULTIPLICATIVE_DEPTH_VALUE: u32 = 1000;
        if multiplicative_depth > MAX_MULTIPLICATIVE_DEPTH_VALUE {
            Err(FHEError::InvalidMultiplicativeDepth(multiplicative_depth))?;
        }
        Ok(())
    }

    // getters
    pub fn scheme(&self) -> FHEScheme {
        self.scheme
    }

    pub fn plaintext_modulus(&self) -> PlaintextModulus {
        self.plain_text_modulus
    }

    pub fn digit_size(&self) -> u32 {
        self.digit_size
    }

    pub fn standard_deviation(&self) -> f64 {
        self.standard_deviation
    }

    pub fn secret_key_dist(&self) -> SecretKeyDistribution {
        self.secret_key_distribution
    }

    pub fn max_relinearization_secret_key_degree(&self) -> u32 {
        self.max_relinearization_secret_key_degree
    }

    pub fn key_switch_technique(&self) -> KeySwitchTechnique {
        self.key_switch_technique
    }

    pub fn scaling_technique(&self) -> ScalingTechnique {
        self.scaling_technique
    }

    pub fn batch_size(&self) -> u32 {
        self.batch_size
    }

    pub fn proxy_reencryption_mode(&self) -> Option<ProxyReEncryptionMode> {
        self.proxy_reencryption_mode
    }

    pub fn multiparty_mode(&self) -> MultipartyMode {
        self.multiparty_mode
    }

    pub fn execution_mode(&self) -> ExecutionMode {
        self.execution_mode
    }

    pub fn decryption_noise_mode(&self) -> DecryptionNoiseMode {
        self.decryption_noise_mode
    }

    pub fn noise_estimate(&self) -> f64 {
        self.noise_estimate
    }

    pub fn desired_precision(&self) -> f64 {
        self.desired_precision
    }

    pub fn statistical_security(&self) -> u32 {
        self.statistical_security
    }

    pub fn num_adversarial_queries(&self) -> u32 {
        self.num_adversarial_queries
    }

    pub fn threshold_num_of_parties(&self) -> u32 {
        self.threshold_num_of_parties
    }

    pub fn first_mod_size(&self) -> u32 {
        self.first_mod_size
    }

    pub fn scaling_mod_size(&self) -> u32 {
        self.scaling_mod_size
    }

    pub fn num_large_digits(&self) -> u32 {
        self.num_large_digits
    }

    pub fn multiplicative_depth(&self) -> u32 {
        self.multiplicative_depth
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.security_level
    }

    pub fn ring_dimension(&self) -> u32 {
        self.ring_dimension
    }

    pub fn eval_add_count(&self) -> u32 {
        self.eval_add_count
    }

    pub fn key_switch_count(&self) -> u32 {
        self.key_switch_count
    }

    pub fn multi_hop_mod_size(&self) -> u32 {
        self.multi_hop_mod_size
    }

    pub fn encryption_technique(&self) -> EncryptionTechnique {
        self.encryption_technique
    }

    pub fn multiplication_technique(&self) -> MultiplicationTechnique {
        self.multiplication_technique
    }

    pub fn interactive_boot_compression_level(&self) -> CompressionLevel {
        self.interactive_boot_compression_level
    }

    // setters
    pub fn set_scheme(&mut self, scheme: FHEScheme) -> Result<(), FHEError> {
        self.scheme = scheme;
        Ok(())
    }

    pub fn set_plaintext_modulus(
        &mut self,
        plaintext_modulus: PlaintextModulus,
    ) -> Result<(), FHEError> {
        self.plain_text_modulus = plaintext_modulus;
        Ok(())
    }

    pub fn set_digit_size(&mut self, digit_size: u32) -> Result<(), FHEError> {
        self.digit_size = digit_size;
        Ok(())
    }

    pub fn set_standard_deviation(&mut self, standard_deviation: f64) -> Result<(), FHEError> {
        self.standard_deviation = standard_deviation;
        Ok(())
    }

    pub fn set_secret_key_dist(
        &mut self,
        secret_key_dist: SecretKeyDistribution,
    ) -> Result<(), FHEError> {
        self.secret_key_distribution = secret_key_dist;
        Ok(())
    }

    pub fn set_max_relinearization_secret_key_degree(
        &mut self,
        max_relinearization_secret_key_degree: u32,
    ) -> Result<(), FHEError> {
        self.max_relinearization_secret_key_degree = max_relinearization_secret_key_degree;
        Ok(())
    }

    pub fn set_key_switch_technique(
        &mut self,
        key_switch_technique: KeySwitchTechnique,
    ) -> Result<(), FHEError> {
        self.key_switch_technique = key_switch_technique;
        Ok(())
    }

    pub fn set_scaling_technique(
        &mut self,
        scaling_technique: ScalingTechnique,
    ) -> Result<(), FHEError> {
        self.scaling_technique = scaling_technique;
        Ok(())
    }

    pub fn set_batch_size(&mut self, batch_size: u32) -> Result<(), FHEError> {
        self.batch_size = batch_size;
        Ok(())
    }

    pub fn set_proxy_reencryption_mode(
        &mut self,
        proxy_reencryption_mode: Option<ProxyReEncryptionMode>,
    ) -> Result<(), FHEError> {
        self.proxy_reencryption_mode = proxy_reencryption_mode;
        Ok(())
    }

    pub fn set_multiparty_mode(&mut self, multiparty_mode: MultipartyMode) -> Result<(), FHEError> {
        self.multiparty_mode = multiparty_mode;
        Ok(())
    }

    pub fn set_execution_mode(&mut self, execution_mode: ExecutionMode) -> Result<(), FHEError> {
        self.execution_mode = execution_mode;
        Ok(())
    }

    pub fn set_decryption_noise_mode(
        &mut self,
        decryption_noise_mode: DecryptionNoiseMode,
    ) -> Result<(), FHEError> {
        self.decryption_noise_mode = decryption_noise_mode;
        Ok(())
    }

    pub fn set_noise_estimate(&mut self, noise_estimate: f64) -> Result<(), FHEError> {
        self.noise_estimate = noise_estimate;
        Ok(())
    }

    pub fn set_desired_precision(&mut self, desired_precision: f64) -> Result<(), FHEError> {
        self.desired_precision = desired_precision;
        Ok(())
    }

    pub fn set_statistical_security(&mut self, statistical_security: u32) -> Result<(), FHEError> {
        self.statistical_security = statistical_security;
        Ok(())
    }

    pub fn set_num_adversarial_queries(
        &mut self,
        num_adversarial_queries: u32,
    ) -> Result<(), FHEError> {
        self.num_adversarial_queries = num_adversarial_queries;
        Ok(())
    }

    pub fn set_threshold_num_of_parties(
        &mut self,
        threshold_num_of_parties: u32,
    ) -> Result<(), FHEError> {
        self.threshold_num_of_parties = threshold_num_of_parties;
        Ok(())
    }

    pub fn set_first_mod_size(&mut self, first_mod_size: u32) -> Result<(), FHEError> {
        self.first_mod_size = first_mod_size;
        Ok(())
    }

    pub fn set_scaling_mod_size(&mut self, scaling_mod_size: u32) -> Result<(), FHEError> {
        self.scaling_mod_size = scaling_mod_size;
        Ok(())
    }

    pub fn set_num_large_digits(&mut self, num_large_digits: u32) -> Result<(), FHEError> {
        self.num_large_digits = num_large_digits;
        Ok(())
    }

    pub fn set_multiplicative_depth(&mut self, multiplicative_depth: u32) -> Result<(), FHEError> {
        // TODO (dsuponit): move the check below ValidateMultiplicativeDepth() to a separate validating function. see
        // https://github.com/openfheorg/openfhe-development/issues/400
        Self::validate_multiplicative_depth(multiplicative_depth)?;
        self.multiplicative_depth = multiplicative_depth;
        Ok(())
    }

    pub fn set_security_level(&mut self, security_level: SecurityLevel) -> Result<(), FHEError> {
        self.security_level = security_level;
        Ok(())
    }

    pub fn set_ring_dimension(&mut self, ring_dimension: u32) -> Result<(), FHEError> {
        // TODO (dsuponit): move the check below ValidateRingDim() to a separate validating function. see
        // https://github.com/openfheorg/openfhe-development/issues/400
        Self::validate_ring_dimension(ring_dimension)?;
        self.ring_dimension = ring_dimension;
        Ok(())
    }

    pub fn set_eval_add_count(&mut self, eval_add_count: u32) -> Result<(), FHEError> {
        self.eval_add_count = eval_add_count;
        Ok(())
    }

    pub fn set_key_switch_count(&mut self, key_switch_count: u32) -> Result<(), FHEError> {
        self.key_switch_count = key_switch_count;
        Ok(())
    }

    pub fn set_multi_hop_mod_size(&mut self, multi_hop_mod_size: u32) -> Result<(), FHEError> {
        self.multi_hop_mod_size = multi_hop_mod_size;
        Ok(())
    }

    pub fn set_encryption_technique(
        &mut self,
        encryption_technique: EncryptionTechnique,
    ) -> Result<(), FHEError> {
        self.encryption_technique = encryption_technique;
        Ok(())
    }

    pub fn set_multiplication_technique(
        &mut self,
        multiplication_technique: MultiplicationTechnique,
    ) -> Result<(), FHEError> {
        self.multiplication_technique = multiplication_technique;
        Ok(())
    }
}

pub type Params = SchemeParameters;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecretKeyDistribution {
    Gaussian = 0,
    // UNIFORM_BINARY,
    #[default]
    UniformTernary = 1,
    SparseTernary = 2,
    // UNIFORM_QUATERNARY,
    // UNIFORM_QUINARY,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeySwitchTechnique {
    Invalid = 0,
    BV,
    #[default]
    HYBRID,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyReEncryptionMode {
    InDCPA,
    FixedNoiseHRA,
    NoiseFloodingHRA,
    DivideAndRoundHRA,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MultipartyMode {
    InvalidMultipartyMode = 0,
    #[default]
    FixedNoiseMultiparty,
    NoiseFloodingMultiparty,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionMode {
    #[default]
    Evaluation = 0,
    NoiseEstimation,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecryptionNoiseMode {
    #[default]
    FixedNoiseDecrypt = 0,
    NoiseFloodingDecrypt,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecurityLevel {
    #[default]
    HEstd128Classic,
    HEstd192Classic,
    HEstd256Classic,
    HEstd128Quantum,
    HEstd192Quantum,
    HEstd256Quantum,
    HEstdNotSet,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncryptionTechnique {
    #[default]
    STANDARD = 0,
    EXTENDED,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MultiplicationTechnique {
    BEHZ = 0,
    #[default]
    HPS,
    HPSPOVERQ,
    HPSPOVERQLEVELED,
}

/// 🇷🇺 Определение уровня, до которого доводится входной шифротекст перед
/// интерактивным многосторонним бутстрапингом
/// 🇬🇧 Defining the level to which the input ciphertext is brought to before
/// interactive multi-party bootstrapping
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompressionLevel {
    // we don't support 0 or 1 compression levels
    // do not change values here
    Compact = 2, // more efficient with stronger security assumption
    #[default]
    Slack = 3, // less efficient with weaker security assumption
}

// mod libcrypto {
// pub struct CryptoContextCKKSRNS;

// impl CryptoContextCKKSRNS {
//     pub type ContextType = CryptoContext<Element>;
//     pub type Factory = CryptoContextFactory<Element>;
//     pub type PublicKeyEncryptionScheme = SchemeCKKSRNS;
//     pub type CryptoParams = CryptoParametersCKKSRNS;
//     pub type Element = DCRTPoly;

//     pub fn gen_crypto_context(parameters: &CCParams<CryptoContextCKKSRNS>) -> CryptoContext<Element> {
//         gen_crypto_context_ckksrns_internal::<CryptoContextCKKSRNS, Element>(parameters)
//     }
// }
// }

// trait CryptoContext {
//     type Element; //                    = DCRTPoly;
//     type ContextType; //                = CryptoContext<Element>;  // required by GenCryptoContext() in gen-cryptocontext.h
//     type Factory; //                    = CryptoContextFactory<Element>;
//     type PublicKeyEncryptionScheme; //  = SchemeCKKSRNS;
//     type CryptoParams; /// .
//     //               = CryptoParametersCKKSRNS;

//     fn genCryptoContext<CryptoContextCKKSRNS>(parameters: &CCParams<CryptoContextCKKSRNS>) -> Rc<CryptoContextement> {
//         genCryptoContextCKKSRNSInternal<CryptoContextCKKSRNS, Element>(parameters);
//     }
// }

// pub struct CCParams<CryptoContext> {
//     params: SchemeParameters,
// }

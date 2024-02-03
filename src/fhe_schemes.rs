///❗🇷🇺❗
/// FHE реализует схемы BGV, BFV и CKKS.
/// Поддерживается несколько вариантов каждой схемы. Для повышения эффективности в настоящее время
/// поддерживаются только варианты системы остаточных чисел (Residue Number System - RNS).
/// RNS используется для эффективного выполнения операций над большими целыми числами путем их
/// разложения на маленькие числа, умещающиеся в машинных словах, например, в 64-битных целых числах.
/// Однако в будущем можно будет добавить многопрецизионные или смешанные многопрецизионные-RNS-варианты
/// схем без изменения текущего дизайна.
///
///❗🇬🇧❗
/// FHE implements BGV, BFV, and CKKS schemes.
/// Multiple variants of each scheme are supported. For efficiency, only the
/// Residue Number System (RNS) variants are currently supported.
/// RNS is used to efficiently perform operations on large integers by decomposing them into
/// small numbers that fit in machine words, e.g., in 64-bit integers. However,
/// multiprecision or mixed multiprecision-RNS variants of the schemes can be added in
/// the future without changing the current design
pub enum FHEScheme {
    Invalid = 0,
    /// 🇷🇺 Схема CKKS
    /// ---
    /// FHE реализует два RNS-варианта схемы CKKS (далее они делятся на четыре режима в зависимости
    /// от того, как выполняется перемасштабирование). Первый вариант RNS предполагает одинаковый
    /// коэффициент масштабирования 2p для всех уровней и задает RNS-модули qi ≈ 2p для всех i,
    /// соответствующих мультипликативным уровням (все RNS-модули, кроме первого и, возможно,
    /// последнего, в зависимости от режима). Этот метод был независимо предложен в [21] и [8].
    /// Во втором варианте RNS для каждого уровня используется свой масштабный коэффициент [43].
    /// В FHE реализованы следующие режимы изменения масштаба (обозначены так же, как и для BGV,
    /// так как между CKKS и BGV есть много общего):
    /// 👉 1. FIXEDMANUAL: вариант RNS [8, 21], в котором переключение модуля осуществляется
    /// пользователем вручную.
    /// 👉 2. FIXEDAUTO: вариант RNS [8, 21], в котором пересчет автоматически выполняется
    /// непосредственно перед гомоморфным умножением (за исключением первого умножения).
    /// 👉 3. FLEXIBLEAUTO: вариант RNS [43], в котором пересчет автоматически выполняется
    /// непосредственно перед гомоморфным умножением (за исключением первого умножения).
    /// 👉 4. FLEXIBLEAUTOEXT: вариант RNS [43], в котором перемасштабирование автоматически
    /// выполняется непосредственно перед гомоморфным умножением (включая первое умножение).
    /// За более подробным обсуждением алгоритмов для всех этих режимов мы отсылаем читателя к [43].
    ///
    /// При выборе метода изменения масштаба мы руководствуемся следующими рекомендациями:
    /// 👉 Режим FLEXIBLEAUTOEXT обеспечивает наивысшую точность для тех же параметров. Вычислительная
    /// сложность обычно в 1,5 раза выше по сравнению с самым быстрым подходом (FIXEDMANUAL или
    /// FIXEDAUTO) для тех же параметров.
    /// 👉 Режим FLEXIBLEAUTO обеспечивает точность, которая примерно на 3-4 бита меньше, чем
    /// FLEXIBLEAUTOEXT, но может быть немного быстрее.
    /// 👉 Режимы FIXEDMANUAL и FIXEDAUTO несут дополнительную потерю точности примерно на 3-4 бита
    /// по сравнению с FLEXIBLEAUTO, но имеют меньшую вычислительную сложность. Отметим, что FIXEDMANUAL
    /// может иметь лучшую производительность, чем FIXEDAUTO, если он адаптирован экспертом по FHE
    /// или компилятором для конкретного приложения.
    /// 👉 Режимы AUTO гораздо проще в использовании, чем режим FIXEDMANUAL, так как изменение масштаба
    /// происходит автоматически с помощью FHE.
    /// 👉 Если целью является минимизация модуля шифротекста Q при той же точности, то лучшим вариантом
    /// будет режим FLEXIBALEAUTOEXT. В некоторых сценариях уменьшение Q может также привести к уменьшению
    /// размера кольца при том же уровне безопасности, что дает лучшую производительность для
    /// FLEXIBLEAUTOEXT по сравнению со всеми остальными режимами. Обратите внимание, что по умолчанию
    /// используется метод FLEXIBLEAUTOEXT, так как он прост в использовании и достигает наивысшей точности.
    /// Другие режимы могут быть рассмотрены, если необходимо сократить время выполнения (обычно не
    /// более чем в 1,5 раза).
    ///
    /// 🇬🇧 CKKS scheme.
    /// ---
    /// OpenFHE implements two RNS variants of the CKKS scheme (they are further
    /// split into four modes based on how rescaling is done). The first RNS variant assumes the same
    /// scaling factor 2p for all levels and sets RNS moduli qi ≈ 2p for all i corresponding to
    /// multiplicative levels (all RNS moduli except for the first and possibly last ones, depending
    /// on the mode). This method was independently proposed in [21] and [8]. The second RNS variant
    /// uses a different scaling factor for each level [43].
    /// The following rescaling modes are implemented in OpenFHE (labeled the same way as for the
    /// BGV scheme as there are a lot of similarities between CKKS and BGV):
    /// 👉 1. FIXEDMANUAL: the RNS variant [8, 21] where modulus switching is manually done by the
    /// user.
    /// 👉 2. FIXEDAUTO: the RNS variant [8, 21] where rescaling is automatically done right before a
    /// homomorphic multiplication (except for the first multiplication).
    /// 👉 3. FLEXIBLEAUTO: the RNS variant [43] where rescaling is automatically done right before a
    /// homomorphic multiplication (except for the first multiplication).
    /// 👉 4. FLEXIBLEAUTOEXT: the RNS variant [43] where rescaling is automatically done right
    /// before a homomorphic multiplication (including the first multiplication).
    /// For a more detailed discussion of the algorithms for all these modes, the reader is referred
    /// to [43].
    /// We suggest the following guidelines when choosing the rescaling method:
    /// 👉 The FLEXIBLEAUTOEXT mode provides the highest precision for the same parameters.
    /// The computational complexity is typically up to 1.5x higher, as compared to the fastest
    /// approach (FIXEDMANUAL or FIXEDAUTO) for the same parameters.
    /// 👉 The FLEXIBLEAUTO mode provides a precision that is about 3-4 bits smaller than FLEXIBLEAUTOEXT, but can be slightly faster.
    /// 👉 The FIXEDMANUAL and FIXEDAUTO modes incur additional precision loss of about 3-4
    /// bits as compared FLEXIBLEAUTO, but have smaller computational complexity. Note that
    /// FIXEDMANUAL can yield better performance than FIXEDAUTO if it is tailored by an FHE
    /// expert or a compiler for a given application.
    /// 👉 The AUTO modes are much easier to use than the FIXEDMANUAL mode as rescaling is
    /// done automatically by OpenFHE.
    /// 👉 If the goal is to minimize the ciphertext modulus Q for the same precision, then the FLEXIBALEAUTOEXT mode is the best option. In some scenarios, the decrease in Q may also
    /// result in reduced ring dimension for the same security level, yielding better performance for
    /// FLEXIBLEAUTOEXT as compared to all other modes.
    /// Note that the default method is FLEXIBLEAUTOEXT as it easy to use and achieves highest
    /// precision. Other modes can be considered when there is a need to reduce the runtime (typically by
    /// no more than 1.5x).
    CKKSRNS,

    /// 🇷🇺 Схема BFV
    /// 🇬🇧 BFV scheme.
    /// ---
    /// OpenFHE implements four different RNS variants of the BFV scheme. These
    /// variants differ in the way the homomorphic multiplication is performed. There are also some
    /// differences in evaluating the decryption operation for some of the variants. These four variants are:
    /// 👉 HPS: the homomorphic multiplication and decryption are implemented using the RNS procedures proposed by Halevi, Polyakov, and Shoup [37]. These RNS procedures use a mix of
    /// integer and floating-point operations.
    /// 👉 BEHZ: the homomorphic multiplication and decryption are implemented using the RNS procedures described by Bajard, Eynard, Hasan, and Zucca [6]. These RNS procedures are based
    /// on integer arithmetic.
    /// 👉 HPSPOVERQ: the HPS variant where the homomorphic encryption is optimized using the
    /// technique described in [44].
    /// 👉 HPSPOVERQLEVELED: the HPSOVERQ variant where modulus switching is applied inside
    /// homomorphic encryption to further reduce the computational complexity [44].
    /// Note that all four methods use the modified BFV encryption method proposed in [44], which
    /// has smaller noise than the original BFV encryption method [31].
    /// The HPSPOVERQLEVELED method is the fastest when floating-point arithmetic is available.
    /// The BEHZ method can be used when floating-point arithmetic is not available (it is slightly slower
    /// than HPS, typically by no more than 1.2x). The other two modes, namely, HPS and HPSOVERQ,
    /// are available mostly for academic purposes. For a more detailed comparison of the HPS and
    /// BEHZ variants, the reader is referred to [1, 7]. The default method for BFV in OpenFHE is
    /// HPSPOVERQLEVELED.
    /// OpenFHE also provides two different options for BFV encryption: STANDARD and EXTENDED. For the STANDARD option, the encryption is done using fresh modulus Q. For the EXTENDED setting, a larger modulus is used for encryption by employing auxiliary moduli available
    /// for homomorphic multiplication and then modulus switching to Q is executed. The EXTENDED
    /// option requires a slightly smaller value of Q (around 5 bits less in the case of public key encryption)
    /// but makes encryption more computationally expensive. The STANDARD option is used as the
    /// default.
    BFVRNS,

    /// 🇷🇺 Схема BGV
    /// 🇬🇧 BGV scheme.
    /// ---
    /// OpenFHE implements both the original BGV scheme (with unscaled messages) [14]
    /// and the Gentry-Halevi-Smart (GHS) variant (with scaled messages) [33]. The main advantage
    /// of the GHS variant is that the RNS moduli qi do not need to satisfy the congruence relation
    /// qi ≡ 1 mod t to perform modulus switching. OpenFHE currently supports only the static noise
    /// estimation method [44] to choose the size of RNS moduli.
    /// Four modes for BGV are currently implemented in OpenFHE (these modes are distinguished
    /// by the way the modulus switching is performed):
    /// 👉 1. FIXEDMANUAL: original BGV variant [14] with RNS optimizations from [33, 44] where
    /// modulus switching is manually done by the user.
    /// 👉 2. FIXEDAUTO: original BGV variant [14] with RNS optimizations from [33, 44] where modulus
    /// switching is automatically done right before a homomorphic multiplication (except for the
    /// first multiplication).
    /// 👉 3. FLEXIBLEAUTO: GHS variant [33] with RNS optimizations from [44] where modulus switching is automatically done right before a homomorphic multiplication (except for the first
    /// multiplication).
    /// 👉 4. FLEXIBLEAUTOEXT: GHS variant [33] with RNS optimizations from [44] where modulus
    /// switching is automatically done right before a homomorphic multiplication (including the first
    /// multiplication).
    /// For a more detailed discussion of the algorithms for all these modes, the reader is referred to [44].
    /// We suggest the following guidelines when choosing the modulus switching method:
    /// 👉 The FLEXIBLEAUTOEXT mode requires the smallest ciphertext modulus Q, but is somewhat slower for most cases than other options (typically less than 1.5x than the fastest mode).
    /// However, FLEXIBLEAUTOEXT is the fastest when a smaller ring dimension N can be chosen for its smaller Q to satisfy the same level of security.
    /// 👉 The FIXEDMANUAL and FIXEDAUTO methods are often the fastest (when the ring dimension N needed to achieve the desired level of security is the same for all four modes).
    /// Note that FIXEDMANUAL can yield better performance than FIXEDAUTO if it is tailored
    /// by an FHE expert or a compiler for a given application.
    /// 👉 The FLEXIBLEAUTO mode can be selected in relatively rare cases where the ring dimension N is smaller than for FIXEDAUTO (note that FLEXIBLEAUTO is often faster than
    /// FLEXIBLEAUTOEXT for the same ring dimension N).
    /// 👉 The AUTO modes are much easier to use than the FIXEDMANUAL mode as modulus
    /// switching is done automatically by OpenFHE.
    /// 👉 The FLEXIBLEAUTOEXT mode supports larger plaintext moduli than other modes.
    /// Note that the default method is FLEXIBLEAUTOEXT as it easy to use and supports largest
    /// plaintext moduli. Other modes can be considered when there is a need to reduce the runtime
    /// (typically by no more than 1.5x).
    BGVRNS,
}
pub type Scheme = FHEScheme;

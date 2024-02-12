///❗🇷🇺❗
/// FHE реализует два RNS-варианта схемы CKKS (далее они делятся на четыре режима в
/// зависимости от того, как выполняется перемасштабирование).
///
/// 👉 Первый вариант RNS предполагает одинаковый масштабный коэффициент 2^p для всех уровней
/// и задает RNS-модули qₗ ≈ 2^p для всех i, соответствующих мультипликативным уровням
/// (все RNS-модули, кроме первого и, возможно, последнего, в зависимости от режима).
/// Этот метод был независимо предложен в [21] и [8] в https://eprint.iacr.org/2022/915.pdf.
///
/// 👉 Во втором варианте RNS для каждого уровня используется свой масштабный коэффициент [43].
/// В FHE реализованы следующие режимы перемасштабирования (обозначены так же, как
/// и для схемы BGV, поскольку между CKKS и BGV много общего):
///
///❗🇬🇧❗
/// FHE implements two RNS variants of the CKKS scheme (they are further split
/// into four modes based on how rescaling is done). The first RNS variant assumes
/// the same scaling factor 2^p for all levels and sets RNS moduli qₗ ≈ 2^p
/// for all i corresponding to multiplicative levels (all RNS moduli except for
/// the first and possibly last ones, depending on the mode). This method was
/// independently proposed in [21] and [8] at https://eprint.iacr.org/2022/915.pdf.
/// The second RNS variant uses a different scaling factor for each level [43].
/// The following rescaling modes are implemented in FHE (labeled the same way
/// as for the BGV scheme as there are a lot of similarities between CKKS and BGV):
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalingTechnique {
    /// 🇷🇺 Вариант RNS [8, 21], в котором переключение модуля осуществляется пользователем вручную.
    ///
    /// Режимы FIXEDMANUAL и FIXEDAUTO несут дополнительные потери точности примерно на 3-4 бита
    /// по сравнению с FLEXIBLEAUTO, но имеют меньшую вычислительную сложность. Отметим, что
    /// FIXEDMANUAL может иметь лучшую производительность, чем FIXEDAUTO, если он адаптирован
    /// экспертом по FHE или компилятором для конкретного приложения.
    ///
    /// Режимы AUTO гораздо проще в использовании, чем FIXEDMANUAL, так как изменение масштаба
    /// происходит автоматически с помощью крейта FHE.
    ///
    /// 🇬🇧 The RNS variant [8, 21] where modulus switching is manually done by the user.
    ///
    /// The FIXEDMANUAL and FIXEDAUTO modes incur additional precision loss of about 3-4
    /// bits as compared FLEXIBLEAUTO, but have smaller computational complexity. Note that
    /// FIXEDMANUAL can yield better performance than FIXEDAUTO if it is tailored by an FHE
    /// expert or a compiler for a given application.
    ///
    /// The AUTO modes are much easier to use than the FIXEDMANUAL mode as rescaling is
    /// done automatically by FHE crate.
    FixedManual,

    /// 🇷🇺 Вариант RNS [8, 21], в котором перемасштабирование автоматически выполняется
    /// непосредственно перед гомоморфным умножением (за исключением первого умножения).
    ///
    /// 🇬🇧 The RNS variant [8, 21] where rescaling is automatically done right before a
    /// homomorphic multiplication (except for the first multiplication).
    FixedAuto,

    /// 🇷🇺 Вариант RNS [43], в котором масштабирование автоматически выполняется непосредственно
    /// перед гомоморфным умножением (за исключением первого умножения).
    ///
    /// Режим FLEXIBLEAUTO обеспечивает точность, которая примерно на 3-4 бита меньше, чем
    /// FLEXIBLEAUTOEXT, но может быть немного быстрее.
    ///
    /// 🇬🇧 The RNS variant [43] where rescaling is automatically done right before a
    /// homomorphic multiplication (except for the first multiplication).
    ///
    /// The FLEXIBLEAUTO mode provides a precision that is about 3-4 bits smaller than
    /// FLEXIBLEAUTOEXT, but can be slightly faster
    FlexibleAuto,

    /// 🇷🇺 Вариант RNS [43], в котором перемасштабирование автоматически выполняется
    /// непосредственно перед гомоморфным умножением (включая первое умножение).
    ///
    /// Режим FLEXIBLEAUTOEXT обеспечивает наивысшую точность для тех же параметров.
    /// Вычислительная сложность обычно в 1,5 раза выше по сравнению с самым быстрым подходом
    ///  (FIXEDMANUAL или FIXEDAUTO) для тех же параметров
    ///
    /// 🇬🇧 The RNS variant [43] where rescaling is automatically done right before a
    /// homomorphic multiplication (including the first multiplication)
    ///
    /// The FLEXIBLEAUTOEXT mode provides the highest precision for the same parameters.
    /// The computational complexity is typically up to 1.5x higher, as compared to the fastest
    /// approach (FIXEDMANUAL or FIXEDAUTO) for the same parameters
    ///
    /// If the goal is to minimize the ciphertext modulus Q for the same precision, then the FLEXIBALEAUTOEXT mode is the best option. In some scenarios, the decrease in Q may also
    /// result in reduced ring dimension for the same security level, yielding better performance for
    /// FLEXIBLEAUTOEXT as compared to all other modes.
    #[default]
    FlexibleAutoExt,

    /// 🇷🇺 Без изменения масштаба
    NoRescale,

    /// TODO: Неизвестная техника переключения ключей, уточнить подробности
    InvalidTechnique,
}

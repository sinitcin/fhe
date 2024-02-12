use fhe::{
    errors::FHEError,
    fhe_schemes::FHEScheme,
    gen_cryptocontext_parameters::{self, SchemeParameters},
    pke::scheme::ckksrns::{CryptoContextCKKSRNS, CryptoContextParams},
    scale_technique::ScalingTechnique,
};

fn main() {
    // ❗🇷🇺❗
    // Реализация CKKS включает четыре метода изменения масштаба,
    // которые называются "FIXEDMANUAL*", "*FIXEDAUTO*", "FLEXIBLEAUTO" и "FLEXIBLEAUTOEXT".
    // Эти методы изменения масштаба описаны в разделе CKKS на https://eprint.iacr.org/2022/915.

    // Прежде чем мы начнем, необходимо сказать несколько слов об операции изменения масштаба
    // которая занимает центральное место в CKKS. Всякий раз, когда мы перемножаем два
    // шифротекста c1 и c2, которые соответственно шифруют числа m1*D и m2*D, мы получаем результат,
    // который выглядит как m1*m2*D^2.
    //
    // Поскольку масштабный коэффициент этого числа равен D^2, мы говорим, что результат имеет глубину 2.
    // Понятно, что шифротекст глубины 2 не может быть добавлен к шифротекстам глубины 1,
    // потому что их коэффициенты масштаба разные. При пересчете берется шифротекст глубины 2
    // и превращается в шифр глубины глубины 1 с помощью операции, которая очень похожа
    // на деление на D=2^p.
    //
    // По соображениям эффективности наша реализация CKKS работает в пространстве RNS, что означает,
    // что мы избегаем работы с большими числами и мы работаем только с собственными целыми числами.
    // Одно из осложнений, которое возникает заключается в том, что мы можем изменять масштаб только
    // путем деления на определенные простые числа, а не D=2^p. Есть два способа решить эту проблему.
    //
    // 👉 Первый - выбрать простые числа, как можно ближе к 2^p, и предположить, что масштабный коэффициент
    // остается неизменным. Это неизбежно влечет за собой некоторую ошибку аппроксимации и для этого
    // сценария есть два варианта: FLEXIBLEAUTO и FLEXIBLEAUTOEXT.
    // 👉 Второй способ решения этой проблемы - отследить, как изменяется масштабный коэффициент и попытаться
    // подстроиться под него. Это то, что мы делаем для FLEXIBLEAUTO и FLEXIBLEAUTOEXT вариантов CKKS.
    //
    // Компромисс заключается в том. что вычисления в FLEXIBLEAUTO - обычно выполняются несколько
    // медленнее (по нашему опыту замедление составляет около 5-35 % в зависимости от сложности вычислений)
    // это происходит из-за необходимости корректировки значений. Мы разработали FLEXIBLEAUTO(EXT) таким образом,
    // что она скрывает все нюансы отслеживания глубины шифротекстов и необходимости вызова операции
    // изменения масштаба. Поэтому FLEXIBLEAUTO(EXT) больше подходит для пользователей, которые
    // не хотят вникать в детали криптовалют и математики, или для создания быстрого прототипа.

    // Напротив, FIXEDMANUAL больше подходит для приложений в проде, которые были оптимизированы экспертами.
    // Первые две части этой демонстрации реализуют одно и то же вычисление, то есть функцию
    // f(x) = x^18 + x^9 + 1, используя все четыре метода.

    // ❗🇬🇧❗
    //  Our implementation of CKKS includes four rescaling methods called
    // "FIXEDMANUAL*, *FIXEDAUTO*, "FLEXIBLEAUTO", and "FLEXIBLEAUTOEXT".
    // THese rescaling methods are explained in the CKKS section of
    // https://eprint.iacr.org/2022/915.

    // Before we start, we need to say a few words about the rescale
    // operation, which is central in CKKS. Whenever we multiply two
    // ciphertexts c1 and c2 which encrypt numbers m1*D and m2*D
    // respectively, we get a result that looks like m1*m2*D^2. Since the
    // scaling factor of this number is D^2, we say that the result is of
    // depth 2. It is clear that a ciphertext of depth 2 cannot be added
    // to ciphertexts of depth 1, because their scaling factors are
    // different. Rescaling takes a ciphertext of depth 2, and makes it of
    // depth 1 by an operation that looks a lot like dividing by D=2^p.

    // For efficiency reasons, our implementation of CKKS works in the
    // RNS space, which means that we avoid working with big numbers and
    // we only work with native integers. One complication that arises
    // from this is that we can only rescale by dividing by certain prime
    // numbers and not D=2^p.

    // There are two ways to deal with this. The first is to choose prime
    // numbers as close to 2^p as possible, and assume that the scaling
    // factor remains the same. This inevitably incurs some approximation
    // error, and there are two variants for this scenario: FLEXIBLEAUTO
    // and FLEXIBLEAUTOEXT.

    // The second way of dealing with this is to track how the scaling
    // factor changes and try to adjust for it. This is what we do for the
    // FLEXIBLEAUTO and FLEXIBALEAUTOEXT variants of CKKS. The tradeoff is
    // that FLEXIBLEAUTO*    * computations are typically somewhat slower (based on our experience
    // the slowdown is around 5-35% depending on the complexity of the
    // computation), because of the adjustment of values that need to
    // take place.

    // We have designed FLEXIBLEAUTO(EXT) so it hides all the nuances of
    // tracking the depth of ciphertexts and having to call the rescale
    // operation. Therefore, FLEXIBLEAUTO(EXT) is more appropriate for users
    // who do not want to get into the details of the underlying crypto
    // and math, or who want to put together a quick prototype. On the
    // contrary, FIXEDMANUAL is more appropriate for production
    // applications that have been optimized by experts.

    // The first two parts of this demo implement the same computation, i.e, the function
    // f(x) = x^18 + x^9 + 1, using all four methods.

    automatic_rescale_demo(ScalingTechnique::FlexibleAuto);
    automatic_rescale_demo(ScalingTechnique::FlexibleAutoExt);
    automatic_rescale_demo(ScalingTechnique::FixedAuto);
    manual_rescale_demo(ScalingTechnique::FixedManual);

    // ❗🇷🇺❗
    // Реализация CKKS поддерживает два различных алгоритма переключения ключей,
    // а именно BV и HYBRID.
    //
    // 👉 BV соответствует технике, известной также как поразрядное разложение (как RNS, так и
    //   основанное на размере разряда). GHS (больше не реализуется отдельно) соответствует
    // удвоению модуля шифротекста
    //
    // 👉 HYBRID сочетает в себе характеристики и BV, и GHS.
    //
    // Более подробную информацию о различных техниках переключения ключей можно найти в документации
    // KeySwitchGen в keyswitch-bv.h/cpp и keyswitch-hybrid.h/cpp.
    //
    // В большинстве случаев HYBRID будет наиболее подходящей и эффективной техникой переключения ключей,
    // поэтому мы посвящаем третью и четвертую части этого примера переключения ключей с помощью HYBRID.

    // ❗🇬🇧❗
    //      Our implementation of CKKS supports two different algorithms
    // for key switching, namely BV and HYBRID. BV corresponds to
    // a technique also known as digit decomposition (both RNS and based
    // on a digit size). GHS (not implemented separately anymore) corresponds to ciphertext
    // modulus doubling, and HYBRID combines the characteristics of both
    // BV and GHS. Please refer to the documentation of KeySwitchGen in
    // keyswitch-bv.h/cpp and keyswitch-hybrid.h/cpp for more
    // details about the different key switch techniques.
    //      For most cases, HYBRID will be the most appropriate and efficient
    // key switching technique, and this is why we devote the third and
    // fourth part of this demo to HYBRID key switching.

    hybrid_key_switching_demo1();
    hybrid_key_switching_demo2();
}

fn automatic_rescale_demo(scale_technique: ScalingTechnique) -> Result<(), FHEError> {
    // ❗🇷🇺❗
    // Пожалуйста, прочитайте комментарии в main(), чтобы узнать, что представляет собой
    // операция rescale (перемасштабирование). Понимание Rescale() не обязательно для
    // использования варианта CKKS FLEXIBLEAUTO, однако оно необходимо для понимания того,
    // что происходит внутри алгоритма.
    //
    // FLEXIBLEAUTO - это вариант CKKS, который имеет две основные особенности:
    // 👉 1 - Автоматически выполняет перемасштабирование перед каждым умножением.
    //      Это сделано для того, чтобы пользователям было проще делать вычисления над FHE,
    //      не заботясь о глубине шифротекстов и масштабировании.
    // 👉 2 - Отслеживает точный коэффициент масштабирования всех шифротекстов.
    //      Это означает, что вычисления в FLEXIBLEAUTO будут более точными, чем те же
    //      вычисления в FIXEDMANUAL. Имейте в виду, что эта разница становится очевидной
    //      только при вычислениях с большой глубиной умножения; это происходит потому,
    //      что большая глубина умножения означает, что нам нужно найти больше простых чисел,
    //      достаточно близких к D=2^p, а это становится все труднее и труднее по мере
    //      увеличения глубины умножения.

    // ❗🇬🇧❗
    // Please read comments in main() for an introduction to what the
    // rescale operation is. Knowing about Rescale() is not necessary
    // to use the FLEXIBLEAUTO CKKS variant, it is however needed to
    // understand what's happening underneath.
    //
    // FLEXIBLEAUTO is a variant of CKKS that has two main features:
    // 👉 1 - It automatically performs rescaling before every multiplication.
    //    This is done to make it easier for users to write FHE
    //    computations without worrying about the depth of ciphertexts
    //    or rescaling.
    // 👉 2 - It tracks the exact scaling factor of all ciphertexts.
    //    This means that computations in FLEXIBLEAUTO will be more
    //    accurate than the same computations in FIXEDMANUAL. Keep
    //    in mind that this difference only becomes apparent when
    //    dealing with computations of large multiplicative depth; this
    //    is because a large multiplicative depth means we need to find
    //    more prime numbers sufficiently close to D=2^p, and this
    //    becomes harder and harder as the multiplicative depth
    //    increases.

    println!(
        "\n\n\n{}\n",
        match scale_technique {
            ScalingTechnique::FlexibleAuto => "FLEXIBLE AUTO DEMO",
            _ => "FIXED AUTO DEMO",
        }
    );

    let batch_size = 8;
    let mut parameters = CryptoContextParams::<CryptoContextCKKSRNS>::new();
    parameters.set_to_defaults(FHEScheme::CKKSRNS)?;
    parameters.set_multiplicative_depth(5)?;
    parameters.set_scaling_mod_size(50)?;
    parameters.set_scaling_technique(scale_technique)?;
    parameters.set_batch_size(batch_size)?;

    // let cc = gen_crypto_context(parameters)?;

    println!(
        "CKKS scheme is using ring dimension {}\n\n",
        parameters.ring_dimension()
    );

    todo!()
}

fn manual_rescale_demo(_scale_technique: ScalingTechnique) {
    todo!()
}

fn hybrid_key_switching_demo1() {
    todo!()
}

fn hybrid_key_switching_demo2() {
    todo!()
}

use super::Element;
use std::ops::*;

/// Элемент кольца
/// --
/// Основное назначение этого кода — предоставить инфраструктуру для безопасной
/// работы с числами в зашифрованном виде, используя арифметику по модулю для
/// совместимости с механизмами FHE, в частности, с алгоритмом CKKS.
/// CKKS позволяет выполнять операции с плавающей точкой над зашифрованными данными,
/// поддерживая при этом шифрование.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct RingElement {
    /// Значение элемента
    pub value: Element,
    /// Модуль относительно которого выполняются операции с элементом
    pub modulo: Element,
}

use std::fmt;
impl fmt::Display for RingElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0} mod {1}",
            self.value.to_string(),
            self.modulo.to_string()
        )
    }
}

impl RingElement {
    pub fn new(value: Element, modulo: Element) -> Self {
        Self {
            value: value % modulo,
            modulo: modulo,
        }
    }
}

impl Add for RingElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value, self.modulo)
    }
}

impl Sub for RingElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.modulo != other.modulo {
            panic!("This operation is not enclose in its field.")
        }
        Self::new(
            if self.value < other.value {
                self.modulo - (other.value - self.value)
            } else {
                self.value - other.value
            },
            self.modulo,
        )
    }
}

impl Mul for RingElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value + self.modulo, self.modulo)
    }
}

impl AddAssign for RingElement {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for RingElement {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for RingElement {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

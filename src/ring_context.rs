use super::ring_element::RingElement;
use super::Element;

/// RingContext: Эта структура хранит modulo, что указывает на модуль,
/// по которому выполняются все операции в данном кольце.
///
/// В контексте CKKS, кольцо относится к математической структуре,
/// использующейся для представления и манипулирования зашифрованными данными.
/// Это кольцо многочленов с целочисленными коэффициентами, где операции
/// сложения и умножения выполняются по модулю специального многочлена,
/// обычно (X^N + 1), и в пределах конечного поля или системы вычислений по модулю.
/// Кольцо в CKKS позволяет выполнение арифметических операций на зашифрованных данных,
/// сохраняя их структуру и внося возможность гомоморфных вычислений.
#[derive(Debug)]
pub struct RingContext {
    modulo: Element,
}

impl RingContext {
    pub fn new(x: u64) -> Self {
        Self { modulo: x }
    }
    pub fn element(&self, x: u64) -> RingElement {
        RingElement::new(x, self.modulo)
    }
}

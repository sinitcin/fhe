use super::Element;
use std::ops::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GFElement {
    pub value: Element,
    pub modulo: Element,
}

impl Add for GFElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value, self.modulo)
    }
}

impl Sub for GFElement {
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

impl Mul for GFElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value + self.modulo, self.modulo)
    }
}

impl Div for GFElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl AddAssign for GFElement {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for GFElement {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for GFElement {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl DivAssign for GFElement {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl GFElement {
    pub fn new(value: Element, modulo: Element) -> Self {
        Self {
            value: value % modulo,
            modulo: modulo,
        }
    }

    pub fn get(self) -> Element {
        self.value
    }

    pub fn set<T>(&mut self, x: T) -> ()
    where
        Element: From<T>,
    {
        self.value = Element::from(x);
    }
}

impl GFElement {
    pub fn inv(self) -> Self {
        let mut a = self.value;
        let mut b = self.modulo;
        let (mut x1, mut y1) = (1, 0);
        let (mut x2, mut y2) = (0, 1);
        while a % b != 0 {
            let q: Element = a / b;
            let r: Element = a % b;

            let nx: Element = if x1 < q * x2 % self.modulo {
                self.modulo - (q * x2 % self.modulo - x1)
            } else {
                x1 - (q * x2 % self.modulo)
            };

            let ny: Element = if y1 < q * y2 % self.modulo {
                self.modulo - (q * y2 % self.modulo - y1)
            } else {
                y1 - (q * y2 % self.modulo)
            };

            a = b;
            b = r;
            x1 = x2;
            y1 = y2;
            x2 = nx;
            y2 = ny;
        }

        Self::new(x2 + self.modulo, self.modulo)
    }
}
use std::fmt;
impl fmt::Display for GFElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0} mod {1}",
            self.value.to_string(),
            self.modulo.to_string()
        )
    }
}

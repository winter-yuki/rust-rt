extern crate num_traits;

use std::ops;
use std::ops::{AddAssign, Mul};

use num_traits::{Bounded, CheckedAdd, Float, Num, ToPrimitive, Unsigned, Zero};

#[derive(Clone, Copy, Debug)]
pub struct Color<T: Num> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Unsigned + Zero> Default for Color<T> {
    fn default() -> Self {
        Color::black()
    }
}

impl<T: Unsigned + Zero> Color<T> {
    pub fn black() -> Self {
        Color {
            r: T::zero(),
            g: T::zero(),
            b: T::zero(),
        }
    }
}

impl<T: Bounded + Unsigned> Color<T> {
    pub fn white() -> Self {
        Color {
            r: T::max_value(),
            g: T::max_value(),
            b: T::max_value(),
        }
    }
}

impl<T> ops::Add<Color<T>> for Color<T>
    where T: Num + Bounded + CheckedAdd
{
    type Output = Color<T>;

    fn add(self, Color { r, g, b }: Self) -> Self::Output {
        return Color {
            r: add_helper(self.r, r),
            g: add_helper(self.g, g),
            b: add_helper(self.b, b),
        };

        fn add_helper<T: CheckedAdd + Bounded>(x: T, y: T) -> T {
            if let Some(v) = x.checked_add(&y) {
                v
            } else {
                T::max_value()
            }
        }
    }
}

impl<C: AddAssign + Float> ops::AddAssign for Color<C> {
    fn add_assign(&mut self, Color { r, g, b }: Self) {
        self.r += r;
        self.g += g;
        self.b += b;
    }
}

impl<C: ToPrimitive + Num> ops::Mul<Color<C>> for f64 {
    type Output = Color<f64>;

    fn mul(self, Color { r, g, b }: Color<C>) -> Self::Output {
        Color {
            r: self * r.to_f64().unwrap(),
            g: self * g.to_f64().unwrap(),
            b: self * b.to_f64().unwrap(),
        }
    }
}

impl<T: Mul + Float> ops::Mul for Color<T> {
    type Output = Color<T>;

    fn mul(self, Color { r, g, b }: Self) -> Self::Output {
        Color {
            r: self.r * r,
            g: self.g * g,
            b: self.b * b,
        }
    }
}

impl From<Color<f64>> for Color<u8> {
    fn from(Color { r, g, b }: Color<f64>) -> Self {
        Color {
            r: r.round() as u8,
            g: g.round() as u8,
            b: b.round() as u8,
        }
    }
}

impl From<Color<u8>> for Color<f64> {
    fn from(Color { r, g, b }: Color<u8>) -> Self {
        Color {
            r: r as f64,
            g: g as f64,
            b: b as f64,
        }
    }
}

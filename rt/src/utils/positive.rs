use std::ops::{Deref, DerefMut};

use num_traits::{Signed, Zero};

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Positive<T: Signed>(T);

impl<T: Signed + PartialOrd + Zero + Copy> Positive<T> {
    pub fn new(x: T) -> Option<Self> {
        if x > T::zero() {
            Some(Positive(x))
        } else {
            None
        }
    }

    pub fn get(&self) -> T {
        self.0
    }
}

impl From<Positive<f64>> for f64 {
    fn from(pf: Positive<f64>) -> Self {
        pf.0
    }
}

impl<T: Signed> Deref for Positive<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Signed> DerefMut for Positive<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

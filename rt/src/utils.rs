use std::ops::{Deref, DerefMut};

use approx::AbsDiffEq;
use num_traits::{Signed, Zero};

use crate::Vector;

#[derive(Debug, Clone)]
pub(crate) struct NormVector(Vector);

impl NormVector {
    pub fn new(v: Vector) -> Self {
        NormVector(v.normalize())
    }

    pub fn new_unchecked(v: Vector) -> Self {
        debug_assert!(v.norm().abs_diff_eq(&1., 1e-5));
        NormVector(v)
    }
}

impl From<Vector> for NormVector {
    fn from(v: Vector) -> Self {
        NormVector::new(v)
    }
}

impl From<NormVector> for Vector {
    fn from(nv: NormVector) -> Self {
        nv.0
    }
}

impl Deref for NormVector {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NormVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

    pub fn val(&self) -> T {
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

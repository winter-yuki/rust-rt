use std::ops::{Deref, DerefMut};

use approx::AbsDiffEq;

use crate::{Vector, VFloat};
use crate::utils::clone_vec;

#[derive(Debug)]
pub struct NormVector(Vector);

impl NormVector {
    pub fn new(x: VFloat, y: VFloat, z: VFloat) -> Self {
        NormVector::from(Vector::new(x, y, z))
    }

    pub fn new_unchecked(x: VFloat, y: VFloat, z: VFloat) -> Self {
        NormVector::from_unchecked(Vector::new(x, y, z))
    }

    pub fn from_unchecked(v: Vector) -> Self {
        debug_assert!(v.norm().abs_diff_eq(&1., 1e-5));
        NormVector(v)
    }

    pub fn get(&self) -> &Vector {
        &self.0
    }
}

impl From<Vector> for NormVector {
    fn from(v: Vector) -> Self {
        NormVector(v.normalize())
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

impl Clone for NormVector {
    fn clone(&self) -> Self {
        NormVector::from_unchecked(clone_vec(&self.0))
    }
}

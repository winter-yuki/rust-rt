use std::f64::consts::PI;

use crate::utils::NormVector;
use crate::Vector;

pub(crate) fn random_unit() -> Vector {
    let a = 2. * PI * rand::random::<f64>();
    let z = -1. + 2. * rand::random::<f64>();
    let r = (1. - z * z).sqrt();
    Vector::new(
        r * a.cos(),
        r * a.sin(),
        z,
    )
}

pub(crate) fn reflect(v: &NormVector, n: &NormVector) -> NormVector {
    NormVector::from(v.get() - 2. * v.dot(n) * n.get())
}

pub(crate) fn clone_vec(v: &Vector) -> Vector {
    Vector::from_data(v.data)
}

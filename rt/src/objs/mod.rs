pub(crate) use {
    sphere::*,
};
use color::Color;

use crate::ray::Ray;
use crate::utils::{NormVector, Positive};

mod sphere;

pub(crate) trait Touch {
    fn touch(&self, r: &Ray) -> Option<Touching>;
}

pub(crate) type TouchBox = Box<dyn Touch>;

pub(crate) struct Touching {
    pub(crate) c: Color<u8>,
    pub(crate) n: NormVector,
    pub(crate) t: Positive<f64>,
}

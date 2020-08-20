pub(crate) use {
    sphere::*,
};
use image::Color;

use crate::ray::Ray;
use crate::Vector;

mod sphere;

pub(crate) trait Touch {
    fn touch(&self, r: &Ray) -> Option<Touching>;
}

pub(crate) type TouchBox = Box<dyn Touch>;

pub(crate) struct Touching {
    c: Color,
    n: Vector,
    t: f64,
}

impl Touching {
    pub(crate) fn c(&self) -> Color {
        self.c
    }

    /// `||n|| == 1`
    pub(crate) fn n(&self) -> &Vector {
        &self.n
    }

    pub(crate) fn t(&self) -> f64 {
        self.t
    }
}

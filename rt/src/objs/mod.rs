use std::rc::Rc;

pub(crate) use {
    lambertian::*,
    metal::*,
    sphere::*,
};
use image::Color;

use crate::ray::Ray;
use crate::utils::{NormVector, Positive};
use crate::Vector;

mod sphere;
mod lambertian;
mod metal;

pub(crate) trait Touch {
    fn touch(&self, r: &Ray) -> Option<Touching>;
}

pub(crate) type TouchBox = Box<dyn Touch>;

pub(crate) struct Touching {
    pub(crate) p: Vector,
    pub(crate) t: Positive<f64>,
    pub(crate) normal: NormVector,
    pub(crate) material: Rc<dyn Material>,
}

pub(crate) struct Scatter {
    pub(crate) attenuation: Color,
    pub(crate) scattered: Ray,
}

pub(crate) trait Material {
    fn scatter(&self, r: &Ray, t: &Touching) -> Option<Scatter>;
}

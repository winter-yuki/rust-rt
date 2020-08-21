use std::ops::Deref;

use image::Color;

use crate::objs::{Material, Scatter, Touching};
use crate::ray::Ray;
use crate::utils::{clone_vec, NormVector, random_unit};
use crate::Vector;

pub(crate) struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, Touching { p, normal, .. }: &Touching) -> Option<Scatter> {
        Some(Scatter {
            attenuation: self.albedo,
            scattered: Ray {
                orig: clone_vec(p),
                dir: NormVector::new(normal.deref() + random_unit()),
            },
        })
    }
}

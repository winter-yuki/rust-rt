use std::ops::Deref;

use image::Color;

use crate::objs::{Material, Scatter, Touching};
use crate::ray::Ray;
use crate::utils::{clone_vec, NormVector, random_unit, reflect, UniFloat};

pub(crate) struct Metal {
    pub(crate) albedo: Color,
    pub(crate) fuzz: UniFloat<f64>,
}

impl Material for Metal {
    fn scatter(
        &self,
        Ray { dir, .. }: &Ray,
        Touching { normal, p, .. }: &Touching,
    ) -> Option<Scatter> {
        let reflected = reflect(dir, normal);
        if reflected.dot(normal) > 0. {
            Some(Scatter {
                attenuation: self.albedo,
                scattered: Ray {
                    orig: clone_vec(p),
                    dir: NormVector::new(reflected.deref() + self.fuzz.val() * random_unit()),
                },
            })
        } else {
            None
        }
    }
}

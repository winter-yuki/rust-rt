use image::Color;

use crate::objs::{Material, Scatter, Touching};
use crate::ray::Ray;
use crate::utils::{clone_vec, reflect};

pub struct Metal {
    pub albedo: Color
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
                    dir: reflected,
                },
            })
        } else {
            None
        }
    }
}

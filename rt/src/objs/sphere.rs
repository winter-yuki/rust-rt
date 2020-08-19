use image::Color;

use crate::objs::Object;
use crate::ray::Ray;
use crate::Vector;

pub(crate) struct Sphere {
    c: Vector,
    r: f64,
    color: Color,
}

impl Sphere {
    pub(crate) fn from(center: Vector, radius: f64, color: Color) -> Sphere {
        assert!(radius >= 0.);
        Sphere {
            c: center,
            r: radius,
            color,
        }
    }
}

impl Object for Sphere {
    fn touch(&self, r: &Ray) -> Option<Color> {
        let oc = r.orig() - &self.c;
        let a = r.dir().dot(r.dir());
        let b = 2. * oc.dot(r.dir());
        let c = oc.dot(&oc) - self.r * self.r;
        let d = b * b - 4. * a * c;
        if d > 0. {
            Some(self.color)
        } else {
            None
        }
    }
}

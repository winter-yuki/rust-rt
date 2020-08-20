use std::cmp;

use float_ord::FloatOrd;

use image::Color;

use crate::objs::{Touch, Touching};
use crate::ray::Ray;
use crate::Vector;

pub(crate) struct Sphere {
    c: Vector,
    r: f64,
    color: Color<u8>,
}

impl Sphere {
    pub(crate) fn new(center: Vector, radius: f64, color: Color<u8>) -> Sphere {
        assert!(radius >= 0.);
        Sphere {
            c: center,
            r: radius,
            color,
        }
    }
}

impl Touch for Sphere {
    fn touch(&self, r: &Ray) -> Option<Touching> {
        let oc = r.orig() - &self.c;
        let a = r.dir().dot(r.dir());
        let b = oc.dot(r.dir());
        let c = oc.dot(&oc) - self.r * self.r;
        let d = b * b - a * c;
        if d <= 0. {
            return None;
        }

        let root = d.sqrt();
        let t1 = (-b - root) / a;
        let t2 = (-b + root) / a;
        let t = cmp::max(FloatOrd(t1), FloatOrd(t2)).0;
        if t > 0. {
            Some(Touching {
                c: self.color,
                n: (r.orig() - &self.c).normalize(),
                t,
            })
        } else {
            None
        }
    }
}

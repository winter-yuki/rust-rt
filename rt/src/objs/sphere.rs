use std::cmp;

use float_ord::FloatOrd;

use color::Color;

use crate::objs::{Touch, Touching};
use crate::ray::Ray;
use crate::utils::{NormVector, Positive};
use crate::Vector;

pub(crate) struct Sphere {
    pub(crate) c: Vector,
    pub(crate) r: Positive<f64>,
    pub(crate) color: Color<u8>,
}

impl Sphere {
    pub(crate) fn new(center: Vector, radius: f64, color: Color<u8>) -> Sphere {
        Sphere {
            c: center,
            r: Positive::new(radius).unwrap(),
            color,
        }
    }
}

impl Touch for Sphere {
    fn touch(&self, Ray { orig, dir }: &Ray) -> Option<Touching> {
        let oc = orig - &self.c;
        let a = dir.dot(dir);
        let b = oc.dot(dir);
        let c = oc.dot(&oc) - self.r.val() * self.r.val();
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
                n: NormVector::new(orig - &self.c),
                t: Positive::new(t).unwrap(),
            })
        } else {
            None
        }
    }
}

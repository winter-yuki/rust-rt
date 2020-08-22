use std::cmp;
use std::sync::Arc;

use float_ord::FloatOrd;

use crate::objs::{MaterialArc, Touch, Touching};
use crate::ray::Ray;
use crate::utils::{NormVector, Positive};
use crate::Vector;

pub(crate) struct Sphere {
    pub(crate) center: Vector,
    pub(crate) radius: Positive<f64>,
    pub(crate) material: MaterialArc,
}

impl Touch for Sphere {
    fn touch(&self, r: &Ray) -> Option<Touching> {
        let Ray { orig, dir } = r;
        let oc = orig - &self.center;

        let a = dir.dot(dir);
        let b = oc.dot(dir);
        let c = oc.dot(&oc) - self.radius.get() * self.radius.get();
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
                normal: NormVector::new(orig - &self.center),
                p: r.point(t),
                t: Positive::new(t).unwrap(),
                material: Arc::clone(&self.material),
            })
        } else {
            None
        }
    }
}

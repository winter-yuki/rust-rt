use std::cmp;
use std::sync::Arc;

use float_ord::FloatOrd;

use crate::{Lambertian, Metal, Vector};
use crate::objs::{MaterialArc, Touch, Touching};
use crate::ray::Ray;
use crate::utils::{NormVector, Positive};

pub struct Sphere {
    pub(crate) center: Vector,
    pub(crate) radius: Positive<f64>,
    pub(crate) material: MaterialArc,
}

impl Sphere {
    pub fn new() -> SphereBuilder {
        SphereBuilder::new()
    }
}

pub struct SphereBuilder {
    center: Option<Vector>,
    radius: Option<Positive<f64>>,
    material: Option<MaterialArc>,
}

impl SphereBuilder {
    pub fn new() -> Self {
        SphereBuilder {
            center: None,
            radius: None,
            material: None,
        }
    }

    pub fn center(mut self, center: Vector) -> Self {
        self.center = Some(center);
        self
    }

    pub fn radius(mut self, radius: Positive<f64>) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn metal(mut self, metal: Metal) -> Self {
        self.material = Some(Arc::new(metal));
        self
    }

    pub fn lambertian(mut self, lambertian: Lambertian) -> Self {
        self.material = Some(Arc::new(lambertian));
        self
    }

    pub fn build(self) -> Sphere {
        Sphere {
            center: self.center.unwrap(),
            radius: self.radius.unwrap(),
            material: self.material.unwrap(),
        }
    }
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

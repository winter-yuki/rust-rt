use image::Color;

use crate::render::Ray;
use crate::Vector;

pub trait Object {
    fn touch(&self, r: &Ray) -> Option<Color>;
}

pub type ObjectBox = Box<dyn Object>;

pub struct Plane;

impl Object for Plane {
    fn touch(&self, _r: &Ray) -> Option<Color> {
        todo!()
    }
}

pub struct Sphere {
    pub c: Vector,
    pub r: f64,
    pub color: Color,
}

// TODO make API pub(crate)
impl Object for Sphere {
    fn touch(&self, r: &Ray) -> Option<Color> {
        let oc = &r.orig - &self.c;
        let a = r.dir.dot(&r.dir);
        let b = 2. * oc.dot(&r.dir);
        let c = oc.dot(&oc) - self.r * self.r;
        let d = b * b - 4. * a * c;
        if d > 0. {
            Some(self.color)
        } else {
            None
        }
    }
}

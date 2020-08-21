use std::ops::Deref;

use crate::scene::Camera;
use crate::utils::{clone_vec, NormVector};
use crate::Vector;

#[derive(Clone, Debug)]
pub(crate) struct Ray {
    pub(crate) orig: Vector,
    pub(crate) dir: NormVector,
}

impl Ray {
    pub(crate) fn from_cam(c: &Camera, w: f64, h: f64) -> Ray {
        let right: Vector = c.to.cross(&c.up).normalize();
        let viewport_w = f64::from(c.viewport_w);
        let viewport_h = f64::from(c.viewport_h);
        let left_top = &c.to - &right * (viewport_w / 2.) + c.up.deref() * (viewport_h / 2.);
        let dir = left_top - c.up.deref() * h * viewport_h + right * w * viewport_w;
        Ray {
            orig: clone_vec(&c.pos),
            dir: NormVector::new(dir),
        }
    }

    pub(crate) fn point(&self, t: f64) -> Vector {
        &self.orig + t * self.dir.deref()
    }
}

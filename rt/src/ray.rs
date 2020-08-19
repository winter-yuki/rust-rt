use approx::AbsDiffEq;

use crate::scene::Camera;
use crate::Vector;

#[derive(Clone, Debug)]
pub(crate) struct Ray {
    orig: Vector,
    dir: Vector,
}

impl Ray {
    pub(crate) fn from_cam(c: &Camera, w: f64, h: f64) -> Ray {
        assert!(c.up.norm().abs_diff_eq(&1., 1e-8));
        assert!(c.to.norm() > 0.);
        assert!(c.viewport_w > 0.);
        assert!(c.viewport_h > 0.);

        assert!(0. <= w && w <= 1.);
        assert!(0. <= h && h <= 1.);

        // TODO remove repeating computations
        let right: Vector = c.to.cross(&c.up).normalize();
        let left_top = &c.to - &right * (c.viewport_w / 2.) + &c.up * (c.viewport_h / 2.);
        let dir = (left_top - &c.up * h * c.viewport_h + right * w * c.viewport_w).normalize();
        Ray {
            orig: Vector::from_data(c.pos.data),
            dir,
        }
    }

    pub(crate) fn orig(&self) -> &Vector {
        &self.orig
    }

    pub(crate) fn dir(&self) -> &Vector {
        &self.dir
    }
}

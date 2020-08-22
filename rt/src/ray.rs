use crate::scene::Camera;
use crate::utils::{clone_vec, NormVector, Positive};
use crate::Vector;

#[derive(Clone, Debug)]
pub(crate) struct Ray {
    pub(crate) orig: Vector,
    pub(crate) dir: NormVector,
}

impl Ray {
    pub(crate) fn from_cam(
        Camera { to, pos, up, vfov, aspect_ratio }: &Camera,
        w: f64, h: f64,
    ) -> Ray {
        let vp_h = Positive::new(2. * (vfov.to_radians() / 2.).tan()).unwrap();
        let vp_w = Positive::new(aspect_ratio.get() * vp_h.get()).unwrap();

        let cam_look = (to - pos).normalize();
        let right = cam_look.cross(up).normalize();
        let cam_up = right.cross(&cam_look).normalize();
        let hor = vp_w.get() * right;
        let ver = vp_h.get() * cam_up;
        let left_top = pos + cam_look - (0.5 * hor) + (0.5 * ver);
        let dir = left_top - h * ver + w * hor - pos;
        Ray {
            orig: clone_vec(pos),
            dir: NormVector::new(dir),
        }
    }

    pub(crate) fn point(&self, t: f64) -> Vector {
        &self.orig + t * self.dir.get()
    }
}

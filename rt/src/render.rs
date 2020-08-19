use image::{Color, Image};

use crate::{scene, Vector};
use crate::scene::{Camera, Scene};

pub struct Ray {
    pub orig: Vector,
    pub dir: Vector,
}

impl Ray {
    fn from(c: &Camera, w: f64, h: f64) -> Ray {
        assert!(0. <= w && w <= 1.);
        assert!(0. <= h && h <= 1.);

        let right: Vector = c.to.cross(&c.up).normalize();
        let up = c.up.normalize();
        let left_bottom = &c.to - &right * (c.viewport_w / 2.) - &up * (c.viewport_h / 2.);
        let dir = (left_bottom + up * h * c.viewport_h + right * w * c.viewport_w).normalize();
        Ray {
            orig: Vector::from_data(c.pos.data),
            dir,
        }
    }
}

impl Ray {
    fn trace(&self, scene: &Scene) -> Color {
        // TODO
        for obj in &scene.objs {
            // TODO compare distance
            if let Some(c) = obj.touch(self) {
                return c;
            }
        }
        (scene.background_getter)(self)
    }
}

pub type Logger = Box<dyn Fn(usize, usize)>;

pub fn render(scene: &Scene, logger: Logger) -> Image {
    let mut img = Image::from_size(scene.width, scene.height);
    for i_row in 0..scene.height {
        logger(i_row + 1, scene.height);
        for i_col in 0..scene.width {
            let h = i_row as f64 / (scene.height - 1) as f64;
            let w = i_col as f64 / (scene.width - 1) as f64;
            let ray = Ray::from(&scene.cam, w, h);
            img[(i_row, i_col)] = ray.trace(scene);
        }
    }
    img
}

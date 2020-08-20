use std::f64::consts::PI;
use std::ops::Deref;

use color::Color;
use image::Image;

use crate::objs::Touching;
use crate::ray::Ray;
use crate::scene::{Camera, Scene};
use crate::Vector;

pub type Logger = Box<dyn Fn(usize, usize)>;

pub struct Render<'a> {
    scene: &'a Scene,
    logger: Logger,
    antialiasing_samples_per_pixel: usize,
    diffuse_depth: usize,
}

impl<'a> From<&'a Scene> for Render<'a> {
    fn from(scene: &'a Scene) -> Self {
        Render {
            scene,
            logger: Box::new(|_, _| {}),
            antialiasing_samples_per_pixel: 1,
            diffuse_depth: 1,
        }
    }
}

impl<'a> Render<'a> {
    pub fn set_logger(self, logger: Logger) -> Self {
        Render { logger, ..self }
    }

    pub fn set_antialiasing_samples_per_pixel(self, n: usize) -> Self {
        Render { antialiasing_samples_per_pixel: n, ..self }
    }

    pub fn set_diffuse_depth(self, n: usize) -> Self {
        Render { diffuse_depth: n, ..self }
    }

    pub fn render(&self) -> Image {
        let height = usize::from(self.scene.height);
        let width = usize::from(self.scene.width);
        let mut image = Image::from_size(self.scene.width, self.scene.height);
        for i_row in 0..height {
            (self.logger)(i_row + 1, height);
            for i_col in 0..width {
                let mut color = Color { r: 0., g: 0., b: 0. };
                for _ in 0..self.antialiasing_samples_per_pixel {
                    let h = (i_row as f64 + rand::random::<f64>() - 0.5) / (height - 1) as f64;
                    let w = (i_col as f64 + rand::random::<f64>() - 0.5) / (width - 1) as f64;
                    let ray = Ray::from_cam(&self.scene.cam, w, h);
                    color += self.trace(&ray, self.diffuse_depth);
                }
                let k = 1. / self.antialiasing_samples_per_pixel as f64;
                image[(i_row, i_col)] = Color::from(k * color);
            }
        }
        image
    }

    fn trace(&self, r: &Ray, depth: usize) -> Color<f64> {
        if depth == 0 {
            Color { r: 0., g: 0., b: 0. }
        } else {
            if let Some(Touching { t, n, .. }) = self.touch_all(r) {
                let p = r.point(*t);
                let target = &p + n.deref() + random_unit();
                let new_r = Ray::new(p, target - &p);
                0.6 * self.trace(&new_r, depth - 1)
            } else {
                Color::from((self.scene.background_getter)(r))
            }
        }
    }

    fn touch_all(&self, r: &Ray) -> Option<Touching> {
        const SELF_TOUCHING_THRESHOLD: f64 = 0.001;
        let mut t_min = f64::MAX;
        let mut res = None;
        for obj in &self.scene.objs {
            if let Some(touching) = obj.touch(r) {
                if touching.t.val() < t_min && touching.t.val() > SELF_TOUCHING_THRESHOLD {
                    t_min = touching.t.val();
                    res = Some(touching);
                }
            }
        }
        res
    }
}

fn random_unit() -> Vector {
    let a = 2. * PI * rand::random::<f64>();
    let z = -1. + 2. * rand::random::<f64>();
    let r = (1. - z * z).sqrt();
    Vector::new(
        r * a.cos(),
        r * a.sin(),
        z,
    )
}

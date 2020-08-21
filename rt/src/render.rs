use std::f64::consts::PI;
use std::ops::Deref;

use color::Color;
use image::Image;

use crate::objs::{Scatter, Touching};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::utils::random_unit;
use crate::Vector;

pub type Logger = Box<dyn Fn(usize, usize)>;

pub struct Render<'a> {
    scene: &'a Scene,
    logger: Logger,
    samples_per_pixel: usize,
    diffuse_depth: usize,
}

impl<'a> Render<'a> {
    pub fn new(scene: &'a Scene) -> Self {
        Render {
            scene,
            logger: Box::new(|_, _| {}),
            samples_per_pixel: 1,
            diffuse_depth: 1,
        }
    }

    pub fn logger(self, logger: Logger) -> Self {
        Render { logger, ..self }
    }

    pub fn samples_per_pixel(self, n: usize) -> Self {
        Render { samples_per_pixel: n, ..self }
    }

    pub fn diffuse_depth(self, n: usize) -> Self {
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
                for _ in 0..self.samples_per_pixel {
                    let h = (i_row as f64 + rand::random::<f64>() - 0.5) / (height - 1) as f64;
                    let w = (i_col as f64 + rand::random::<f64>() - 0.5) / (width - 1) as f64;
                    let ray = Ray::from_cam(&self.scene.cam, w, h);
                    color += self.trace(&ray, self.diffuse_depth);
                }
                let k = 1. / self.samples_per_pixel as f64;
                image[(i_row, i_col)] = Color::from(k * color);
            }
        }
        image
    }

    fn trace(&self, r: &Ray, depth: usize) -> Color<f64> {
        if depth == 0 { return Color { r: 0., g: 0., b: 0. }; }
        if let Some(touching) = self.touch_all(r) {
            if let Some(scatter) = touching.material.scatter(r, &touching) {
                let a: Color<f64> = Color::from(scatter.attenuation);
                (1. / 255.) * a * self.trace(&scatter.scattered, depth - 1)
            } else {
                Color { r: 0., g: 0., b: 0. }
            }
        } else {
            Color::from((self.scene.background_getter)(r))
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

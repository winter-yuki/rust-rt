use rayon::prelude::*;

use color::Color;
use image::Image;

use crate::objs::Touching;
use crate::ray::Ray;
use crate::scene::Scene;

pub type Current = usize;
pub type Total = usize;
pub type Logger = Box<dyn Fn(Current, Total) + Send + Sync + 'static>;

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

    pub fn logger(mut self, logger: Logger) -> Self {
        self.logger = logger;
        self
    }

    pub fn samples_per_pixel(mut self, n: usize) -> Self {
        self.samples_per_pixel = n;
        self
    }

    pub fn diffuse_depth(mut self, n: usize) -> Self {
        self.diffuse_depth = n;
        self
    }

    pub fn render(&self) -> Image {
        let height = self.scene.height.get();
        let width = self.scene.width.get();
        let mut image = vec![Vec::new(); height];
        image
            .par_iter_mut()
            .enumerate()
            .for_each(|(i_row, row)| {
                *row = self.render_row(i_row, height, width);
                (self.logger)(i_row, height);
            });
        Image::from(image)
    }

    fn render_row(&self, i_row: usize, height: usize, width: usize) -> Vec<image::Color> {
        let mut row = Vec::with_capacity(width);
        for i_col in 0..width {
            let mut color = Color { r: 0., g: 0., b: 0. };
            for _ in 0..self.samples_per_pixel {
                let h = (i_row as f64 + rand::random::<f64>() - 0.5) / (height - 1) as f64;
                let w = (i_col as f64 + rand::random::<f64>() - 0.5) / (width - 1) as f64;
                let ray = Ray::from_cam(&self.scene.cam, w, h);
                color += self.trace(&ray, self.diffuse_depth);
            }
            let k = 1. / self.samples_per_pixel as f64;
            row.push(Color::from(k * color));
        }
        row
    }

    fn trace(&self, r: &Ray, depth: usize) -> Color<f64> {
        if depth == 0 { return Color { r: 0., g: 0., b: 0. }; }
        if let Some(touching) = self.touch_all(r) {
            if let Some(scatter) = touching.material.scatter(r, &touching) {
                let a: Color<f64> = Color::from(scatter.attenuation);
                (1. / u8::MAX as f64) * a * self.trace(&scatter.scattered, depth - 1)
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
                if touching.t.get() < t_min && touching.t.get() > SELF_TOUCHING_THRESHOLD {
                    t_min = touching.t.get();
                    res = Some(touching);
                }
            }
        }
        res
    }
}

use image::{Color, Image};

use crate::{scene, Vector};
use crate::objs::Touching;
use crate::ray::Ray;
use crate::scene::{Camera, Scene};

pub type Logger = Box<dyn Fn(usize, usize)>;

pub struct Render<'a> {
    scene: &'a Scene,
    logger: Logger,
    antialiasing_samples_per_pixel: usize,
}

impl<'a> From<&'a Scene> for Render<'a> {
    fn from(scene: &'a Scene) -> Self {
        Render {
            scene,
            logger: Box::new(|_, _| {}),
            antialiasing_samples_per_pixel: 1,
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

    pub fn render(&self) -> Image {
        let mut img = Image::from_size(self.scene.width, self.scene.height);
        for i_row in 0..self.scene.height {
            (self.logger)(i_row + 1, self.scene.height);
            for i_col in 0..self.scene.width {
                let mut r = 0.;
                let mut g = 0.;
                let mut b = 0.;
                for _ in 0..self.antialiasing_samples_per_pixel {
                    let h = (i_row as f64 + rand::random::<f64>() - 0.5)
                        / (self.scene.height - 1) as f64;
                    let w = (i_col as f64 + rand::random::<f64>() - 0.5)
                        / (self.scene.width - 1) as f64;
                    let ray = Ray::from_cam(&self.scene.cam, w, h);
                    let c = self.trace(&ray);
                    r += c.r as f64;
                    g += c.g as f64;
                    b += c.b as f64;
                }
                img[(i_row, i_col)] = Color {
                    r: (r / self.antialiasing_samples_per_pixel as f64).round() as u8,
                    g: (g / self.antialiasing_samples_per_pixel as f64).round() as u8,
                    b: (b / self.antialiasing_samples_per_pixel as f64).round() as u8,
                }
            }
        }
        img
    }

    fn trace(&self, r: &Ray) -> Color {
        // TODO
        for obj in &self.scene.objs {
            // TODO choose closest
            if let Some(ting) = obj.touch(r) {
                return ting.c();
            }
        }
        (self.scene.background_getter)(r)
    }
}

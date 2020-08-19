use image::{Color, Image};

use crate::{scene, Vector};
use crate::ray::Ray;
use crate::scene::{Camera, Scene};

pub type Logger = Box<dyn Fn(usize, usize)>;

pub fn render(scene: &Scene, logger: Logger) -> Image {
    let mut img = Image::from_size(scene.width, scene.height);
    for i_row in 0..scene.height {
        logger(i_row + 1, scene.height);
        for i_col in 0..scene.width {
            let h = i_row as f64 / (scene.height - 1) as f64;
            let w = i_col as f64 / (scene.width - 1) as f64;
            let ray = Ray::from(&scene.cam, w, h);
            img[(i_row, i_col)] = trace(&ray, scene);
        }
    }
    img
}

fn trace(r: &Ray, s: &Scene) -> Color {
    // TODO
    for obj in &s.objs {
        // TODO compare distance
        if let Some(c) = obj.touch(r) {
            return c;
        }
    }
    (s.background_getter)(r)
}

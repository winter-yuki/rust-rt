use std::num::NonZeroUsize;
use std::sync::Arc;

use color::Color;

use crate::objs::{Lambertian, Metal, Sphere, TouchBox};
use crate::ray::Ray;
use crate::utils::{NormVector, Positive, UniFloat};
use crate::Vector;

pub struct Scene {
    pub(crate) width: NonZeroUsize,
    pub(crate) height: NonZeroUsize,
    pub(crate) cam: Camera,
    pub(crate) objs: Vec<TouchBox>,
    pub(crate) background_getter: Box<dyn Fn(&Ray) -> Color<u8> + Send + Sync + 'static>,
}

#[derive(Debug)]
pub(crate) struct Camera {
    pub(crate) pos: Vector,
    pub(crate) up: NormVector,
    pub(crate) to: Vector,
    pub(crate) vfov: Positive<f64>,
    pub(crate) aspect_ratio: Positive<f64>,
}

impl Default for Scene {
    fn default() -> Self {
        let aspect_ratio = 16. / 9.;
        let width = 600;
        Scene {
            width: NonZeroUsize::new(width).unwrap(),
            height: NonZeroUsize::new((width as f64 / aspect_ratio).round() as usize).unwrap(),
            cam: Camera {
                pos: Vector::new(0., 0., 1.),
                up: NormVector::new(Vector::new(0.3, 1., 1.)),
                to: Vector::new(0., 0., -1.),
                vfov: Positive::new(60.).unwrap(),
                aspect_ratio: Positive::new(aspect_ratio).unwrap(),
            },
            objs: vec![
                Box::new(Sphere {
                    center: Vector::new(0., -1., -5.),
                    radius: Positive::new(2.).unwrap(),
                    material: Arc::new(Lambertian {
                        albedo: Color { g: 200, b: 255, ..Color::black() }
                    }),
                }),
                Box::new(Sphere {
                    center: Vector::new(-3., 1., -5.),
                    radius: Positive::new(2.).unwrap(),
                    material: Arc::new(Lambertian {
                        albedo: Color { r: 200, ..Color::black() }
                    }),
                }),
                Box::new(Sphere {
                    center: Vector::new(1., 3.5, -6.),
                    radius: Positive::new(2.).unwrap(),
                    material: Arc::new(Metal {
                        albedo: Color { r: 210, g: 100, b: 235 },
                        fuzz: UniFloat::new(0.1).unwrap(),
                    }),
                }),
                Box::new(Sphere {
                    center: Vector::new(5., 0., -6.),
                    radius: Positive::new(2.).unwrap(),
                    material: Arc::new(Metal {
                        albedo: Color { b: 200, r: 200, g: 200 },
                        fuzz: UniFloat::new(0.2).unwrap(),
                    }),
                }),
                Box::new(Sphere {
                    center: Vector::new(0., -101., -5.),
                    radius: Positive::new(100.).unwrap(),
                    material: Arc::new(Lambertian {
                        albedo: Color { b: 200, r: 200, ..Color::black() }
                    }),
                }),
            ],
            background_getter: Box::new(|Ray { dir, .. }| {
                let t = 0.5 * (dir.get() + Vector::new(1., 1., 1.));
                let r = 255. * (1. - t[0] * 0.5);
                let g = 255. * (1. - t[1] * 0.7);
                let b = 255. * (1. - t[2] * 1.0);
                Color {
                    r: r as u8,
                    g: g as u8,
                    b: b as u8,
                }
            }),
        }
    }
}

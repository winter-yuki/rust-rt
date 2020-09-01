use std::num::NonZeroUsize;
use std::sync::Arc;

use color::Color;

use crate::objs::{Lambertian, Metal, Sphere, TouchBox};
use crate::ray::Ray;
use crate::utils::{NormVector, Positive, UniFloat};
use crate::Vector;

#[derive(Debug)]
pub struct Camera {
    pub pos: Vector,
    pub up: NormVector,
    pub to: Vector,
    pub vfov: Positive<f64>,
    pub aspect_ratio: Positive<f64>,
}

pub type Background = Box<dyn Fn(&Ray) -> Color<u8> + Send + Sync + 'static>;

pub struct Scene {
    pub(crate) width: NonZeroUsize,
    pub(crate) height: NonZeroUsize,
    pub(crate) cam: Camera,
    pub(crate) objs: Vec<TouchBox>,
    pub(crate) background_getter: Background,
}

impl Scene {
    pub fn new() -> SceneBuilder {
        SceneBuilder::new()
    }
}

pub struct SceneBuilder {
    width: Option<NonZeroUsize>,
    height: Option<NonZeroUsize>,
    cam: Option<Camera>,
    objs: Vec<TouchBox>,
    background_getter: Option<Background>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        SceneBuilder {
            width: None,
            height: None,
            cam: None,
            objs: vec![],
            background_getter: None,
        }
    }

    pub fn width(mut self, width: NonZeroUsize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: NonZeroUsize) -> Self {
        self.height = Some(height);
        self
    }

    pub fn cam(mut self, cam: Camera) -> Self {
        self.cam = Some(cam);
        self
    }

    pub fn add_sphere(mut self, sphere: Sphere) -> Self {
        self.objs.push(Box::new(sphere));
        self
    }

    pub fn background_getter(mut self, bg: Background) -> Self {
        self.background_getter = Some(bg);
        self
    }

    pub fn build(self) -> Scene {
        Scene {
            width: self.width.unwrap(),
            height: self.height.unwrap(),
            cam: self.cam.unwrap(),
            objs: self.objs,
            background_getter: self.background_getter.unwrap(),
        }
    }
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
                up: NormVector::new(0.3, 1., 1.),
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

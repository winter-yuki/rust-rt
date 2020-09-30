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

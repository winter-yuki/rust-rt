pub(crate) use {
    plane::*,
    sphere::*,
};
use image::Color;

use crate::ray::Ray;
use crate::Vector;

mod sphere;
mod plane;

pub(crate) trait Object {
    fn touch(&self, r: &Ray) -> Option<Color>;
}

pub(crate) type ObjectBox = Box<dyn Object>;

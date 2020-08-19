use image::Color;

use crate::objs::Object;
use crate::ray::Ray;

pub(crate) struct Plane;

impl Object for Plane {
    fn touch(&self, _r: &Ray) -> Option<Color> {
        todo!()
    }
}

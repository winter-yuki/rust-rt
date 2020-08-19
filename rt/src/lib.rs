extern crate nalgebra as na;
#[macro_use]
extern crate serde_derive;

pub use crate::{
    image::Image,
    rt::render,
    scene::Scene,
};

pub mod scene;
mod light;
mod obj;
mod rt;
pub mod image;

type Vector = na::Vector3<f64>;

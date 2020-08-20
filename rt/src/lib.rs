extern crate approx;
extern crate float_ord;
extern crate image;
extern crate nalgebra as na;
extern crate rand;
#[macro_use]
extern crate serde_derive;

use image::Color;

pub use crate::{
    render::Logger,
    render::Render,
};

pub mod scene;

mod light;
mod objs;
mod render;
mod ray;

type Vector = na::Vector3<f64>;

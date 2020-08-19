#[macro_use]
extern crate approx;
extern crate image;
extern crate nalgebra as na;
#[macro_use]
extern crate serde_derive;

pub use crate::{
    render::Logger,
    render::render,
};

pub mod scene;

mod light;
mod objs;
mod render;
mod ray;

type Vector = na::Vector3<f64>;

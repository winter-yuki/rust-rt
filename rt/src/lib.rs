extern crate image;
extern crate nalgebra as na;
#[macro_use]
extern crate serde_derive;

pub use crate::{
    render::Logger,
    render::render,
};

pub mod light;
pub mod obj;
pub mod scene;
mod render;

type Vector = na::Vector3<f64>;

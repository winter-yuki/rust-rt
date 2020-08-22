extern crate approx;
extern crate color;
extern crate float_ord;
extern crate image;
extern crate nalgebra as na;
extern crate num_traits;
extern crate rand;
extern crate rayon;

pub use crate::{
    render::Logger,
    render::Render,
};

pub mod scene;

mod objs;
mod render;
mod ray;
mod utils;

type Vector = na::Vector3<f64>;

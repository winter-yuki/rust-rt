extern crate approx;
extern crate color;
extern crate float_ord;
extern crate image;
extern crate nalgebra as na;
extern crate num_traits;
extern crate rand;
extern crate rayon;

pub use crate::{
    objs::{Lambertian, Metal, Sphere, SphereBuilder},
    ray::Ray,
    render::Logger,
    render::Render,
    scene::{Camera, Scene, SceneBuilder},
    utils::*,
};

mod scene;
mod objs;
mod render;
mod ray;
mod utils;

pub type Vector = na::Vector3<f64>;

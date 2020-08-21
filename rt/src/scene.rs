use std::io;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use color::Color;

use crate::objs::{Lambertian, Metal, Sphere, TouchBox};
use crate::ray::Ray;
use crate::utils::{NormVector, Positive, UniFloat};
use crate::Vector;

pub struct Scene {
    pub(crate) width: NonZeroUsize,
    pub(crate) height: NonZeroUsize,
    pub(crate) cam: Camera,
    pub(crate) objs: Vec<TouchBox>,
    pub(crate) background_getter: Box<dyn Fn(&Ray) -> Color<u8>>,
}

#[derive(Debug)]
pub(crate) struct Camera {
    pub(crate) pos: Vector,
    pub(crate) up: NormVector,
    pub(crate) to: Vector,
    pub(crate) viewport_w: Positive<f64>,
    pub(crate) viewport_h: Positive<f64>,
}

pub enum Error {
    ConfReadIO(io::Error),
    ConfSerde(serde_json::Error),
}

type Result<T> = std::result::Result<T, Error>;

impl Scene {
    pub fn from_json_file(_path: &Path) -> Result<Scene> {
        // TODO make scene normalized
        // TODO
        Ok(Scene::default())
    }
}

impl Default for Scene {
    fn default() -> Self {
        let aspect_ratio = 16. / 9.;
        let width = 400;
        let viewport_h = 2.;
        Scene {
            width: NonZeroUsize::new(width).unwrap(),
            height: NonZeroUsize::new((width as f64 / aspect_ratio).round() as usize).unwrap(),
            cam: Camera {
                pos: Vector::new(0., 0., 0.),
                up: NormVector::new(Vector::new(0., 1., 0.)),
                to: Vector::new(0., 0., -1.),
                viewport_h: Positive::new(viewport_h).unwrap(),
                viewport_w: Positive::new(viewport_h * aspect_ratio).unwrap(),
            },
            objs: vec![
                Box::new(Sphere {
                    center: Vector::new(0., -1., -5.),
                    radius: Positive::new(2.).unwrap(),
                    material: Rc::new(Lambertian {
                        albedo: Color { g: 200, b: 255, ..Color::black() }
                    }),
                }),
                Box::new(Sphere {
                    center: Vector::new(-3., 1., -5.),
                    radius: Positive::new(2.).unwrap(),
                    material: Rc::new(Lambertian {
                        albedo: Color { r: 200, ..Color::black() }
                    }),
                }),
                Box::new(Sphere {
                    center: Vector::new(5., 0., -6.),
                    radius: Positive::new(2.).unwrap(),
                    material: Rc::new(Metal {
                        albedo: Color { b: 200, r: 200, g: 200 },
                        fuzz: UniFloat::new(0.3).unwrap(),
                    }),
                }),
                Box::new(Sphere {
                    center: Vector::new(0., -101., -5.),
                    radius: Positive::new(100.).unwrap(),
                    material: Rc::new(Lambertian {
                        albedo: Color { b: 200, r: 200, ..Color::black() }
                    }),
                }),
            ],
            background_getter: Box::new(|Ray { dir, .. }| {
                let t = 0.5 * (dir.deref() + Vector::new(1., 1., 1.));
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

//     pub fn from_json(json: &str) -> serde_json::Result<Scene> {
//         let scene: Value = serde_json::from_str(json)?;
//         // TODO Ok(Scene {
//         //     width: Self::get_width(&scene)?,
//         //     height: Self::get_height(&scene)?,
//         //     save_path: Self::get_path(&scene)?,
//         //     cam: Self::get_cam(&scene)?,
//         //     objs: Self::get_objs(&scene)?,
//         //     lights: Self::get_lights(&scene)?,
//         // })
//         Ok(Scene {
//             width: 300,
//             height: 200,
//             save_path: PathBuf::from("render.png"),
//             cam: Camera {
//                 pos: todo!(),
//                 up: todo!(),
//                 to: todo!(),
//             },
//             objs: vec![todo!()],
//             lights: vec![todo!()],
//         })
//     }
//
//     fn get_width(scene: &Value) -> serde_json::Result<usize> {
//         serde_json::from_value(scene["width"].clone())
//     }
//
//     fn get_height(scene: &Value) -> serde_json::Result<usize> {
//         serde_json::from_value(scene["height"].clone())
//     }
//
//     fn get_path(scene: &Value) -> serde_json::Result<PathBuf> {
//         serde_json::from_value(scene["path"].clone())
//     }
//
//     fn get_cam(scene: &Value) -> serde_json::Result<Camera> {
//         todo!()
//     }
//
//     fn get_objs(scene: &Value) -> serde_json::Result<Vec<ObjectBox>> {
//         const OBJS_TAG: &'static str = "objs";
//
//         let obj_values = scene[OBJS_TAG]
//             .as_array()
//             .ok_or_else(|| {
//                 serde_json::error::Error::custom(
//                     format!("Key {} should contain list", OBJS_TAG)
//                 )
//             })?;
//
//         obj_values
//             .iter()
//             .map(|value| {
//                 let t = value["type"].to_string();
//                 match &t[..] {
//                     "\"plane\"" => Ok(Box::new(Plane) as ObjectBox),
//                     "\"sphere\"" => Ok(Box::new(Sphere) as ObjectBox),
//                     _ => Err(error::Error::custom(format!("Unknown figure: {}", t))),
//                 }
//             })
//             .collect()
//     }
//
//     fn get_lights(scene: &Value) -> serde_json::Result<Vec<LightBox>> {
//         todo!()
//     }
// }

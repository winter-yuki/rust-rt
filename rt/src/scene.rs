use std::io;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use image::Color;

use crate::light::{Ambient, LightBox};
use crate::objs::{Sphere, TouchBox};
use crate::ray::Ray;
use crate::Vector;

pub struct Scene {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) cam: Camera,
    pub(crate) objs: Vec<TouchBox>,
    pub(crate) lights: Vec<LightBox>,
    pub(crate) background_getter: Box<dyn Fn(&Ray) -> Color<u8>>,
}

#[derive(Debug)]
pub(crate) struct Camera {
    pub(crate) pos: Vector,
    pub(crate) up: Vector,
    pub(crate) to: Vector,
    pub(crate) viewport_w: f64,
    pub(crate) viewport_h: f64,
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
            width,
            height: (width as f64 / aspect_ratio).round() as usize,
            cam: Camera {
                pos: Vector::new(0., 0., 0.),
                up: Vector::new(0., 1., 0.).normalize(),
                to: Vector::new(0., 0., -1.),
                viewport_h,
                viewport_w: viewport_h * aspect_ratio,
            },
            objs: vec![
                Box::new(Sphere::new(
                    Vector::new(0., 1., -3.), 2., Color { r: 0, g: 200, b: 0 },
                )),
                Box::new(Sphere::new(
                    Vector::new(0., -101., -1.), 100., Color { r: 0, g: 0, b: 200 },
                ))
            ],
            lights: vec![Box::new(Ambient)],
            background_getter: Box::new(|ray| {
                let t = 0.5 * (ray.dir().normalize() + Vector::new(1., 1., 1.));
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

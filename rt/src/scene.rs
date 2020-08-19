use std::path::{Path, PathBuf};

use image::Color;

use crate::light::{Ambient, LightBox};
use crate::obj::{ObjectBox, Sphere};
use crate::render::Ray;
use crate::Vector;

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub cam: Camera,
    pub objs: Vec<ObjectBox>,
    pub lights: Vec<LightBox>,
    pub background_getter: Box<dyn Fn(&Ray) -> Color>,
}

#[derive(Debug)]
pub struct Camera {
    pub pos: Vector,
    pub up: Vector,
    pub to: Vector,
    pub viewport_w: f64,
    pub viewport_h: f64,
}

pub enum Error {
    IO,
    Serde,
}

type Result<T> = std::result::Result<T, Error>;

impl Scene {
    pub fn from_json_file(_path: &Path) -> Result<Scene> {
        let aspect_ratio = 16. / 9.;
        let width = 400;
        let viewport_h = 2.;
        Ok(Scene {
            width,
            height: (width as f64 / aspect_ratio).round() as usize,
            cam: Camera {
                pos: Vector::new(0., 0., 0.),
                up: Vector::new(0., 1., 0.),
                to: Vector::new(1., 0., 0.),
                viewport_h,
                viewport_w: viewport_h * aspect_ratio,
            },
            objs: vec![
                Box::new(Sphere {
                    c: Vector::new(3., 0., 0.),
                    r: 2.,
                    color: Color { r: 0, g: 200, b: 0 },
                })
            ],
            lights: vec![Box::new(Ambient)],
            background_getter: Box::new(|ray| {
                let t = 0.5 * (ray.dir.normalize() + Vector::new(1., 1., 1.));
                let r = 255. * (1. - t[0] * 0.5);
                let g = 255. * (1. - t[1] * 0.7);
                let b = 255. * (1. - t[2] * 1.0);
                Color {
                    r: r as u8,
                    g: g as u8,
                    b: b as u8,
                }
            }),
        })
        // TODO
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

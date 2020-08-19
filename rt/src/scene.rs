use std::path::{Path, PathBuf};

use crate::light::{Ambient, LightBox};
use crate::obj::{ObjectBox, Sphere};
use crate::Vector;

pub struct Scene {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) cam: Camera,
    pub(crate) objs: Vec<ObjectBox>,
    pub(crate) lights: Vec<LightBox>,
}

#[derive(Debug)]
pub struct Camera {
    pub pos: Vector,
    pub up: Vector,
    pub to: Vector,
}

pub enum Error {
    IO,
    Serde,
}

type Result<T> = std::result::Result<T, Error>;

impl Scene {
    pub fn from_json_file(_path: &Path) -> Result<Scene> {
        Ok(Scene {
            width: 256,
            height: 256,
            cam: Camera {
                pos: Vector::new(0., 0., 0.),
                up: Vector::new(0., 0., 1.),
                to: Vector::new(1., 0., 0.),
            },
            objs: vec![Box::new(Sphere)],
            lights: vec![Box::new(Ambient)],
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

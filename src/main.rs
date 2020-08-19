extern crate rt;

use std::{env, io, process};
use std::path::PathBuf;

use rt::{image, scene};

pub struct Cli {
    pub scene_json_path: PathBuf,
    pub save_path: PathBuf,
}

impl Cli {
    pub fn new(args: &[String]) -> Option<Cli> {
        match args {
            [_, jp, sp] => Some(Cli {
                scene_json_path: PathBuf::from(jp),
                save_path: PathBuf::from(sp),
            }),
            _ => None
        }
    }
}

enum Error {
    Cli,
    ConfIO,
    Parse,
    ImgIO(io::Error),
}

impl From<scene::Error> for Error {
    fn from(e: scene::Error) -> Error {
        match e {
            scene::Error::IO => Error::ConfIO,
            scene::Error::Serde => Error::Parse
        }
    }
}

impl From<image::Error> for Error {
    fn from(e: image::Error) -> Error {
        match e {
            image::Error::WriteIO(e) => Error::ImgIO(e)
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let res = run(args);
    match res {
        Ok(()) => println!("Finished!"),
        Err(e) => match e {
            Error::Cli => {
                eprintln!(
                    "Only two args should be passed: \
                     path to json config and render save path"
                );
                process::exit(exitcode::NOINPUT)
            }
            Error::ConfIO => todo!("Conf io error"),
            Error::Parse => todo!("Parse error"),
            Error::ImgIO(_) => todo!("Image io error")
        }
    }
}

fn run(args: Vec<String>) -> Result<(), Error> {
    let Cli { scene_json_path, save_path } = Cli::new(&args).ok_or(Error::Cli)?;
    let scene = rt::Scene::from_json_file(&scene_json_path)?;
    let image = rt::render(&scene);
    Ok(image.write_png(&save_path)?)
}

extern crate image;
extern crate rt;
extern crate serde_json;

use std::{env, io, process};
use std::path::PathBuf;

use rt::Logger;
use rt::scene;
use rt::scene::Scene;

struct Cli {
    scene_json_path: PathBuf,
    save_path: PathBuf,
}

impl Cli {
    fn new(args: &[String]) -> Option<Cli> {
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
    ConfIO(io::Error),
    ConfParse(serde_json::Error),
    ImgWriteIO(io::Error),
}

impl From<scene::Error> for Error {
    fn from(e: scene::Error) -> Error {
        match e {
            scene::Error::ConfReadIO(e) => Error::ConfIO(e),
            scene::Error::ConfSerde(e) => Error::ConfParse(e),
        }
    }
}

impl From<image::Error> for Error {
    fn from(e: image::Error) -> Error {
        match e {
            image::Error::WriteIO(e) => Error::ImgWriteIO(e)
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let logger = Box::new(|i, n| println!("Progress {} of {}", i, n));
    let res = run(args, logger);
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
            Error::ConfIO(e) => {
                eprintln!("Error while reading json config: {}", e);
                process::exit(exitcode::IOERR)
            }
            Error::ConfParse(e) => {
                eprintln!("Error while parsing config: {}", e);
                process::exit(exitcode::CONFIG)
            }
            Error::ImgWriteIO(e) => {
                eprintln!("Error while writing rendered image to file: {}", e);
                process::exit(exitcode::IOERR)
            }
        }
    }
}

fn run(args: Vec<String>, logger: Logger) -> Result<(), Error> {
    let Cli { scene_json_path, save_path } = Cli::new(&args).ok_or(Error::Cli)?;
    let scene = Scene::from_json_file(&scene_json_path)?;
    let image = rt::Render::from(&scene)
        .set_logger(logger)
        .set_samples_per_pixel(500)
        .set_diffuse_depth(100)
        .render();
    Ok(image.write_png(&save_path)?)
}

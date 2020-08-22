extern crate image;
extern crate rt;

use std::{env, io, process};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use rt::Logger;
use rt::scene::Scene;

struct Cli {
    save_path: PathBuf,
}

impl Cli {
    fn new(args: &[String]) -> Option<Cli> {
        match args {
            [_, sp] => Some(Cli {
                save_path: PathBuf::from(sp),
            }),
            _ => None
        }
    }
}

enum Error {
    Cli,
    ImgWriteIO(io::Error),
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
    let process = AtomicUsize::new(0);
    let logger = Box::new(move |_, _| {
        if process.fetch_add(1, Ordering::SeqCst) % 10 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }
    });
    let res = run(args, logger);
    match res {
        Ok(()) => println!("Finished!"),
        Err(e) => match e {
            Error::Cli => {
                eprintln!("Only one arg should be passed: render save path");
                process::exit(exitcode::NOINPUT)
            }
            Error::ImgWriteIO(e) => {
                eprintln!("Error while writing rendered image to file: {}", e);
                process::exit(exitcode::IOERR)
            }
        }
    }
}

fn run(args: Vec<String>, logger: Logger) -> Result<(), Error> {
    let Cli { save_path } = Cli::new(&args).ok_or(Error::Cli)?;
    let scene = Scene::default();
    let image = rt::Render::new(&scene)
        .logger(logger)
        .samples_per_pixel(1000)
        .diffuse_depth(100)
        .render();
    Ok(image.write_png(&save_path)?)
}

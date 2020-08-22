extern crate image;
extern crate rt;

use std::{env, io, process};
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use image::Color;
use rt::{Camera, Lambertian, Logger, Metal, Positive, Ray, Scene, Sphere, UniFloat};
use rt::NormVector;
use rt::Vector;

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
    let scene = scene();
    let image = rt::Render::new(&scene)
        .logger(logger)
        .samples_per_pixel(1000)
        .diffuse_depth(100)
        .render();
    Ok(image.write_png(&save_path)?)
}

fn scene() -> Scene {
    let aspect_ratio = 16. / 9.;
    let width = 600;
    Scene::new()
        .width(NonZeroUsize::new(width).unwrap())
        .height(NonZeroUsize::new((width as f64 / aspect_ratio).round() as usize).unwrap())
        .cam(Camera {
            pos: Vector::new(0., 0., 1.),
            up: NormVector::new(Vector::new(0.3, 1., 1.)),
            to: Vector::new(0., 0., -1.),
            vfov: Positive::new(60.).unwrap(),
            aspect_ratio: Positive::new(aspect_ratio).unwrap(),
        })
        .add_sphere(
            Sphere::new()
                .center(Vector::new(0., -1., -5.))
                .radius(Positive::new(2.).unwrap())
                .lambertian(Lambertian {
                    albedo: Color { g: 200, b: 255, ..Color::black() }
                })
                .build()
        )
        .add_sphere(
            Sphere::new()
                .center(Vector::new(-3., 1., -5.))
                .radius(Positive::new(2.).unwrap())
                .lambertian(Lambertian {
                    albedo: Color { r: 200, ..Color::black() }
                })
                .build()
        )
        .add_sphere(
            Sphere::new()
                .center(Vector::new(1., 3.5, -6.))
                .radius(Positive::new(2.).unwrap())
                .metal(Metal {
                    albedo: Color { r: 210, g: 100, b: 235 },
                    fuzz: UniFloat::new(0.1).unwrap(),
                })
                .build()
        )
        .add_sphere(
            Sphere::new()
                .center(Vector::new(5., 0., -6.))
                .radius(Positive::new(2.).unwrap())
                .metal(Metal {
                    albedo: Color { b: 200, r: 200, g: 200 },
                    fuzz: UniFloat::new(0.2).unwrap(),
                })
                .build()
        )
        .add_sphere(
            Sphere::new()
                .center(Vector::new(0., -101., -5.))
                .radius(Positive::new(100.).unwrap())
                .lambertian(Lambertian {
                    albedo: Color { b: 200, r: 200, ..Color::black() }
                })
                .build()
        )
        .background_getter(Box::new(|Ray { dir, .. }| {
            let t = 0.5 * (dir.get() + Vector::new(1., 1., 1.));
            let r = 255. * (1. - t[0] * 0.5);
            let g = 255. * (1. - t[1] * 0.7);
            let b = 255. * (1. - t[2] * 1.0);
            Color {
                r: r as u8,
                g: g as u8,
                b: b as u8,
            }
        }))
        .build()
}

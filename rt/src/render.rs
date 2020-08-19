use crate::{scene, Scene, Vector};
use crate::image::{Color, Image};

pub(crate) struct Ray {
    from: Vector,
    a: Vector,
}

impl Ray {
    fn trace(&self, scene: &Scene) -> Color {
        todo!()
    }
}

pub fn render(scene: &Scene) -> Image {
    let mut img = Image::from_size(scene.width, scene.height);
    for i_row in 0..scene.height {
        for i_col in 0..scene.width {
            // TODO Execute in the thread pool
            for ray in gen_rays(scene, i_row, i_col) {
                img[(i_row, i_col)] = ray.trace(scene);
            }
        }
    }
    img

    // Image::from(vec![
    //     vec![Color { r: 100, g: 0, b: 0 }, Color { r: 0, g: 100, b: 0 }],
    //     vec![Color { r: 0, g: 0, b: 0 }, Color { r: 0, g: 0, b: 0 }],
    //     vec![Color { r: 100, g: 0, b: 0 }, Color { r: 0, g: 100, b: 0 }]
    // ]) // TODO
}

fn gen_rays(scene: &Scene, i_row: usize, i_col: usize) -> Vec<Ray> {
    todo!()
}

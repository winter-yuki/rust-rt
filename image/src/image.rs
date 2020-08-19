use std::{fs, io};
use std::ops::{Index, IndexMut};
use std::path::Path;

pub enum Error {
    WriteIO(io::Error)
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::WriteIO(e)
    }
}

impl From<png::EncodingError> for Error {
    fn from(e: png::EncodingError) -> Error {
        match e {
            png::EncodingError::IoError(e) => Error::WriteIO(e),
            png::EncodingError::Format(e) =>
                panic!("Unable to encode image as png \
                        (inconsistent image state): {}", e)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Debug)]
pub struct Image(Vec<Vec<Color>>);

impl Image {
    pub fn from_size(w: usize, h: usize) -> Image {
        assert!(w > 0);
        assert!(h > 0);
        Image(vec![vec![Color { r: 0, g: 0, b: 0 }; w]; h])
    }

    pub fn write_png(&self, path: &Path) -> Result<(), Error> {
        let file = fs::File::create(path)?;
        let writer = io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(writer, self.w() as u32, self.h() as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let data = self.linearized();
        Ok(encoder
            .write_header()?
            .write_image_data(&data)?)
    }

    pub fn h(&self) -> usize {
        self.0.len()
    }

    pub fn w(&self) -> usize {
        assert!(self.h() > 0);
        self.0[0].len()
    }

    pub fn linearized(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(3 * self.h() * self.w());
        for row in self.0.iter() {
            for Color { r, g, b } in row.iter() {
                res.push(*r);
                res.push(*g);
                res.push(*b);
            }
        }
        res
    }
}

impl From<Vec<Vec<Color>>> for Image {
    fn from(data: Vec<Vec<Color>>) -> Self {
        let img = Image(data);
        assert!(img.h() > 0);
        assert!(img.w() > 0);
        img
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Color;

    fn index(&self, (i_row, i_col): (usize, usize)) -> &Self::Output {
        &self.0[i_row][i_col]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (i_row, i_col): (usize, usize)) -> &mut Self::Output {
        &mut self.0[i_row][i_col]
    }
}

use crate::utils::canvas::Canvas;
use crate::utils::error::Error;
use image::Rgb;
use rand::{self, Rng};
pub struct Reference {
    pub canvas: Canvas,
    colors: Vec<Rgb<u8>>,
}

impl Reference {
    pub fn from_file(path: &str) -> Result<Reference, Error> {
        let canvas = Canvas::from_file(path)?;
        let colors = canvas.compute_unique_colors();

        Ok(Reference { canvas, colors })
    }

    pub fn width(&self) -> u32 {
        self.canvas.image.width()
    }

    pub fn height(&self) -> u32 {
        self.canvas.image.height()
    }

    pub fn random_color(&self) -> Rgb<u8> {
        let random_index = rand::thread_rng().gen_range(0, self.colors.len());
        Rgb(self.colors[random_index].0)
    }
}

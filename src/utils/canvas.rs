use crate::utils::error::Error;
use image::io::Reader as ImageReader;
use image::{ImageBuffer, Rgb};
use imageproc::drawing;

/// Compute the distance between two pixels
///
/// The distance is a float between 0 and 1
fn pixel_distance(left: &Rgb<u8>, right: &Rgb<u8>) -> f32 {
    let left_channels = left.0.iter();
    let right_channels = right.0.iter();

    let mut distance = 0;
    let max_distance = (255 * left_channels.len()) as f32;

    for (left_channel, right_channel) in left_channels.zip(right_channels) {
        let difference = *left_channel as i16 - *right_channel as i16;
        distance += difference.abs();
    }

    1.0 - distance as f32 / max_distance
}

pub struct Canvas {
    pub image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let storage = vec![0; 4 * width as usize * height as usize];
        let image = ImageBuffer::from_raw(width, height, storage)
            .expect("Something went wrong instancing the new image");
        Canvas { image }
    }

    pub fn from_canvas(reference: &Canvas) -> Canvas {
        let (width, height) = reference.image.dimensions();
        Canvas::new(width, height)
    }

    pub fn from_file(path: &str) -> Result<Canvas, Error> {
        let image = ImageReader::open(path)?.decode()?.to_rgb8();
        Ok(Canvas { image })
    }

    pub fn compare(&self, other: &Canvas) -> f32 {
        let self_pixels = self.image.pixels();
        let other_pixels = other.image.pixels();
        let both_pixels = self_pixels.zip(other_pixels);

        let score: Option<f32> = both_pixels
            .map(|(left_pixel, right_pixel)| pixel_distance(left_pixel, right_pixel))
            .reduce(|a, b| a + b);
        let score = match score {
            Some(score) => score,
            None => 1.0,
        };

        (score / self.count_pixels() as f32) * 100.0
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: Rgb<u8>) {
        drawing::draw_filled_circle_mut(&mut self.image, (x, y), radius, color);
    }

    fn count_pixels(&self) -> u32 {
        let (width, height) = self.image.dimensions();
        width * height
    }

    pub fn compute_unique_colors(&self) -> Vec<Rgb<u8>> {
        let all_colors = self.image.pixels();
        let mut unique_colors = Vec::new();
        for color in all_colors {
            if !unique_colors.contains(color) {
                unique_colors.push(*color);
            }
        }
        unique_colors
    }
}

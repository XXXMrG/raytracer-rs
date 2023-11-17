use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use rand::distributions::Uniform;
use rand::prelude::*;
use std::env::{self, var};
use std::path::Path;

struct Canvas {
    width: u32,
    height: u32,
    img: RgbImage,
}

impl Canvas {
    fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            img: RgbImage::new(width, height),
        }
    }

    fn put_pixel(&mut self, x: i32, y: i32, color: Rgb<u8>) {
        let transformed_x = self.width as i32 / 2 + x;
        let transformed_y = self.height as i32 / 2 - y;

        if transformed_x < 0 || transformed_y < 0 {
            panic!("out of bounds");
        }

        self.img
            .put_pixel(transformed_x as u32, transformed_y as u32, color);
    }

    fn save(&self, path: &Path) {
        self.img.save(path).unwrap();
    }
}

fn main() {
    println!("Hello, world!");
    let path = env::var("IMG_PATH").expect("missing");
    println!("{:?}", path);

    let mut canvas = Canvas::new(11, 11);
    canvas.put_pixel(-1, -1, Rgb([255, 0, 0]));
    canvas.save(Path::new(&path));
}

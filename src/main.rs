use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use rand::distributions::Uniform;
use rand::prelude::*;
use std::env;
use std::path::Path;

type Point3D = [i32; 3];
type Point2D = [i32; 2];

fn dot_product(a: Point3D, b: Point3D) -> f32 {
    (a[0] * b[0] + a[1] * b[1] + a[2] * b[2]) as f32
}

fn subtract(a: Point3D, b: Point3D) -> Point3D {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn intersect_ray_sphere(origin: Point3D, direction: Point3D, sphere: &Sphere) -> [f32; 2] {
    let sphere_to_origin = subtract(origin, sphere.center);

    let k1 = dot_product(direction, direction);
    let k2 = 2.0 * dot_product(sphere_to_origin, direction);
    let k3 = dot_product(sphere_to_origin, sphere_to_origin) - sphere.radius * sphere.radius;

    let discriminant = (k2 * k2 - 4.0 * k1 * k3) as f32;
    if discriminant < 0.0 {
        return [f32::MAX, f32::MAX];
    }

    let t1 = 0.5 * (-k2 - discriminant.sqrt()) / k1;
    let t2 = 0.5 * (-k2 + discriminant.sqrt()) / k1;
    [t1, t2]
}

struct Sense {
    canvas: Canvas,
    viewport_size: i32,
    projection_plane_z: i32,
    camera_pos: Point3D,
    back_color: Rgb<u8>,
    spheres: Vec<Sphere>,
}

impl Sense {
    fn new(canvas: Canvas, spheres: Vec<Sphere>) -> Sense {
        Sense {
            canvas,
            spheres,
            viewport_size: 1,
            projection_plane_z: 1,
            camera_pos: [0; 3],
            back_color: Rgb([0; 3]),
        }
    }

    fn canvas_to_viewport(&self, point2d: Point2D) -> Point3D {
        let [x, y] = point2d;
        [
            x * self.viewport_size / self.canvas.width as i32,
            y * self.viewport_size / self.canvas.height as i32,
            self.projection_plane_z,
        ]
    }
}

struct Sphere {
    center: Point3D,
    radius: f32,
    color: Rgb<u8>,
}

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

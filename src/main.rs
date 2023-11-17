use image::{Rgb, RgbImage};
use std::path::Path;

type Point3D = [f32; 3];
type Point2D = [f32; 2];

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

    let t1 = (-k2 - discriminant.sqrt()) / (2.0 * k1);
    let t2 = 0.5 * (-k2 + discriminant.sqrt()) / (2.0 * k1);

    [t1, t2]
}

struct Sense<'a> {
    canvas: &'a mut Canvas,
    viewport_size: f32,
    projection_plane_z: f32,
    camera_pos: Point3D,
    back_color: Rgb<u8>,
    spheres: Vec<Sphere>,
}

impl<'a> Sense<'a> {
    fn new(canvas: &mut Canvas, spheres: Vec<Sphere>) -> Sense {
        Sense {
            canvas,
            spheres,
            viewport_size: 1.0,
            projection_plane_z: 1.0,
            camera_pos: [0.0; 3],
            back_color: Rgb([255; 3]),
        }
    }

    fn canvas_to_viewport(&self, point2d: Point2D) -> Point3D {
        let [x, y] = point2d;
        [
            x * self.viewport_size / self.canvas.width as f32,
            y * self.viewport_size / self.canvas.height as f32,
            self.projection_plane_z,
        ]
    }

    fn trace_ray(&self, direction: Point3D, min_t: f32, max_t: f32) -> Rgb<u8> {
        let origin = self.camera_pos;
        let mut closest_t = f32::MAX;
        let mut closest_sphere = None;

        for sphere in &self.spheres {
            let ts = intersect_ray_sphere(origin, direction, sphere);
            let [x, y] = ts;

            if x < closest_t && min_t < x && x < max_t {
                closest_t = x;
                closest_sphere = Some(sphere);
            }
            if y < closest_t && min_t < y && y < max_t {
                closest_t = y;
                closest_sphere = Some(sphere);
            }
        }

        match closest_sphere {
            None => self.back_color,
            Some(sphere) => sphere.color,
        }
    }

    fn put_pixel(&mut self, x: i32, y: i32, color: Rgb<u8>) {
        self.canvas.put_pixel(x, y, color);
    }

    fn render(&self, path: &Path) {
        self.canvas.save(path);
    }
}

#[derive(Debug)]
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

        if transformed_x < 0
            || transformed_y < 0
            || transformed_x >= self.width as i32
            || transformed_y >= self.height as i32
        {
            return;
        }

        self.img
            .put_pixel(transformed_x as u32, transformed_y as u32, color);
    }

    fn save(&self, path: &Path) {
        self.img.save(path).unwrap();
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

fn main() {
    println!("Hello, world!");

    let mut canvas = Canvas::new(600, 600);
    let mut sense = Sense::new(
        &mut canvas,
        vec![
            Sphere {
                center: [0.0, -1.0, 3.0],
                radius: 1.0,
                color: Rgb([255, 0, 0]),
            },
            Sphere {
                center: [2.0, 0.0, 4.0],
                radius: 1.0,
                color: Rgb([0, 0, 255]),
            },
            Sphere {
                center: [-2.0, 0.0, 4.0],
                radius: 1.0,
                color: Rgb([0, 255, 0]),
            },
        ],
    );

    for x in -(sense.canvas.width() as i32) / 2..sense.canvas.width() as i32 / 2 {
        for y in -(sense.canvas.height() as i32) / 2..sense.canvas.height() as i32 / 2 {
            let direction = sense.canvas_to_viewport([x as f32, y as f32]);
            let color = sense.trace_ray(direction, 1.0, f32::MAX);
            sense.put_pixel(x, y, color);
        }
    }

    sense.render(Path::new("./target/test.png"));
}

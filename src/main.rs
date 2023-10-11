use std::{fs, ops};

fn main() {
    println!("Hello, world!");

    let _ = fs::write("./output.ppm", write_gradient());
}

fn write_gradient() -> String {
    let mut s = String::new();

    // image
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // viewport
    let focal_length = 1.;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3(Vec3::zero());

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::create(viewport_width, 0., 0.);
    let viewport_v = Vec3::create(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = Point3(
        camera_center.0 - Vec3::create(0., 0., focal_length) - (viewport_u / 2.) - viewport_v / 2.,
    );
    let pixel00_loc = viewport_upper_left.0 + (pixel_delta_u + pixel_delta_v) * 0.5;

    s += "P3\n";
    s.push_str(&format!("{} {}\n", image_width, image_height));
    s.push_str(&format!("255\n"));

    for j in 0..image_height {
        // println!("Scanlines remaining: {}", height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center.0;
            let ray = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let pixel_color = ray_color(ray);
            s.push_str(pixel_color.write().as_str());
        }
    }
    // println!("Done");
    return s;
}

fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    return Color(Vec3::create(1., 1., 1.) * (1.0 - a) + Vec3::create(0.5, 0.7, 1.0) * a);
}

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn zero() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    pub fn create(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: Vec3) -> Vec3 {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[derive(Debug)]
struct Color(Vec3);
impl Color {
    pub fn write(&self) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * self.0.x) as i32,
            (255.999 * self.0.y) as i32,
            (255.999 * self.0.z) as i32
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Point3(Vec3);

#[derive(Debug)]
struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Point3 {
        Point3(self.origin.0 + (self.direction * t))
    }
}

use std::{fs, ops};

fn main() {
    println!("Hello, to the ray tracer!");

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Point3(Vec3::create(0., 0., -1.)),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3(Vec3::create(0., -100.5, -1.)),
        radius: 100.,
    }));

    let mut cam = Camera::initialize();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world);
}

struct Camera {
    aspect_ratio: f64,   // Ratio of image width over height
    image_width: usize,  // Rendered image width in pixel count
    image_height: usize, // Rendered image height
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
    pub fn render(self: &Self, world: &impl Hittable) {
        // render
        let mut image = String::new();
        image += "P3\n";
        image.push_str(&format!("{} {}\n", self.image_width, self.image_height));
        image.push_str(&format!("255\n"));

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc.0
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center.0;
                let ray = Ray {
                    origin: self.center,
                    direction: ray_direction,
                };

                let pixel_color = Self::ray_color(ray, world);
                image.push_str(pixel_color.write().as_str());
            }
        }
        fs::write("./output.ppm", image).unwrap();
    }
    fn initialize() -> Self {
        // image
        let aspect_ratio = 16. / 9.;
        let image_width = 400;
        let mut image_height = (image_width as f64 / aspect_ratio) as usize;
        image_height = if image_height < 1 { 1 } else { image_height };

        // camera
        let focal_length = 1.;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3(Vec3::zero());

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::create(viewport_width, 0., 0.);
        let viewport_v = Vec3::create(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = Point3(
            center.0 - Vec3::create(0., 0., focal_length) - (viewport_u / 2.) - viewport_v / 2.,
        );
        let pixel00_loc = Point3(viewport_upper_left.0 + (pixel_delta_u + pixel_delta_v) * 0.5);

        return Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        };
    }
    fn ray_color(ray: Ray, world: &impl Hittable) -> Color {
        if let Some(rec) = world.hit(&ray, Interval::new(0., f64::INFINITY)) {
            return Color((rec.normal + Vec3::create(1., 1., 1.)) * 0.5);
        }

        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        return Color(Vec3::create(1., 1., 1.) * (1.0 - a) + Vec3::create(0.5, 0.7, 1.0) * a);
    }
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

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = outward_normal.dot(ray.direction) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        }
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.origin.0 - self.center.0;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return Option::None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return Option::None;
            }
        }

        let ray_at = ray.at(root);
        let outward_normal = (ray_at.0 - self.center.0) / self.radius;

        let mut hit_record = HitRecord {
            t: root,
            p: ray_at,
            normal: outward_normal,
            front_face: false,
        };
        hit_record.set_face_normal(ray, &outward_normal);

        return Option::Some(hit_record);
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    fn new() -> Self {
        HittableList { objects: vec![] }
    }

    fn clear(self: &mut Self) {
        self.objects.clear();
    }

    fn add(self: &mut Self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_option = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_record.t;
                hit_option = Some(hit_record);
            }
        }

        return hit_option;
    }
}

struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }
    fn empty() -> Self {
        Self::new(f64::INFINITY, f64::NEG_INFINITY)
    }
    fn universe() -> Self {
        Self::new(f64::NEG_INFINITY, f64::INFINITY)
    }

    fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }
}

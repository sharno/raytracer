use std::{fs, ops};

fn main() {
    println!("Hello, world!");

    let _ = fs::write("./output.ppm", write_gradient());
}

fn write_gradient() -> String {
    let mut s = String::new();
    let width = 256;
    let height = 256;
    s += "P3\n";
    s.push_str(&format!("{} {}\n", width, height));
    s.push_str(&format!("255\n"));

    for j in 0..height {
        // println!("Scanlines remaining: {}", height - j);
        for i in 0..width {
            s.push_str(
                &Color(Vec3 {
                    x: i as f64 / (width - 1) as f64,
                    y: j as f64 / (height - 1) as f64,
                    z: 0.0,
                })
                .write(),
            );
        }
    }
    // println!("Done");
    return s;
}

#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
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

    pub fn unit(&self, rhs: Self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f64> for &Vec3 {
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

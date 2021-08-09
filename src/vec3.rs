use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_squared())
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalized(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        }
    }

    pub fn random_range(r_min: f64, r_max: f64) -> Vec3 {
        Vec3 {
            x: thread_rng().gen_range(r_min..r_max),
            y: thread_rng().gen_range(r_min..r_max),
            z: thread_rng().gen_range(r_min..r_max),
        }
    }

    pub fn near_zero(&self) -> bool {
        static EPS: f64 = 1e-8;
        f64::abs(self.x) < EPS && f64::abs(self.y) < EPS && f64::abs(self.z) < EPS
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * Vec3::dot(self, n) * *n
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(&-*self, n), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.len_squared())) * *n;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ToString for Vec3 {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}

pub type Point3 = Vec3;
pub type Colour = Vec3;

pub fn colour_string(colour: &Colour) -> String {
    format!(
        "{} {} {}",
        (255.999f64 * colour.x) as i64,
        (255.999f64 * colour.y) as i64,
        (255.999f64 * colour.z) as i64
    )
}

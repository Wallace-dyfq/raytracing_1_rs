use crate::utils::random_f64_range;
use std::fmt;
use std::ops;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn set_with_other(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }

    pub fn length(&self) -> f64 {
        let tmp = self.length_squared();
        tmp.sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn make_unit_vector(&self) -> Self {
        self / self.length()
    }
    pub fn unit_vector(v: &Vec3) -> Self {
        v / v.length()
    }
    // generate a random Vec3 where each element is between min and max
    pub fn random(min: f64, max: f64) -> Self {
        Vec3 {
            x: random_f64_range(min, max),
            y: random_f64_range(min, max),
            z: random_f64_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    pub fn random_unit_vec3() -> Self {
        return Self::unit_vector(&Self::random_in_unit_sphere());
    }
    pub fn random_unit_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vec3();
        if on_unit_sphere.dot(normal) > 0.0 {
            // in the same hemisphere as the normal
            return on_unit_sphere;
        } else {
            return on_unit_sphere * -1.0;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        let tmplength = 2.0 * v.dot(&n);
        v - &(n * tmplength)
    }

    pub fn reverse(&self) -> Self {
        self * (-1.0)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = uv.reverse().dot(n).min(1.0);
        let r_out_perp: Vec3 = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("cannot divide by zero")
        } else {
            Vec3 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }
}
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("cannot divide by zero")
        } else {
            Vec3 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }
}
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            panic!("cannot divide by zero!");
        }
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}
impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let v = Vec3::default();
        assert_eq!(
            v,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn test_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            v,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_mul_number() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let x: f64 = 3.0;
        assert_eq!(
            v * x,
            Vec3 {
                x: 3.0,
                y: 6.0,
                z: 9.0,
            }
        )
    }

    #[test]
    fn test_add_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 3.0);
        assert_eq!(
            v1 + v2,
            Vec3 {
                x: 5.0,
                y: 7.0,
                z: 6.0,
            }
        );
    }

    #[test]
    fn test_div_number() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let x: f64 = 4.0;
        assert_eq!(
            v / x,
            Vec3 {
                x: 0.25,
                y: 0.5,
                z: 0.75,
            }
        );
    }
    #[test]
    fn test_mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 3.0);
        assert_eq!(
            &v1 * &v2,
            Vec3 {
                x: 4.0,
                y: 10.0,
                z: 9.0,
            }
        );
    }
    #[test]
    fn test_mul_assign_number() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let x = 4;
        v1 *= x as f64;
        assert_eq!(
            v1,
            Vec3 {
                x: 4.0,
                y: 8.0,
                z: 12.0,
            }
        );
    }
    #[test]
    fn test_div_assign_number() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let x = 4;
        v1 /= x as f64;
        assert_eq!(
            v1,
            Vec3 {
                x: 0.25,
                y: 0.5,
                z: 0.75,
            }
        );
    }
    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let mut v2 = Vec3::new(10.0, 20.0, 30.0);
        v1 += &v2;
        assert_eq!(
            v1,
            Vec3 {
                x: 11.0,
                y: 22.0,
                z: 33.0
            }
        );
    }
    #[test]
    fn test_length() {
        let mut v1 = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v1.length(), 5.0);
    }

    #[test]
    fn test_dot() {
        let mut v1 = Vec3::new(3.0, 4.0, 8.0);
        let mut v2 = Vec3::new(2.0, 4.0, 0.0);
        assert_eq!(v1.dot(&v2), 22.0);
    }
}

use std::{fmt::{Display, Formatter, Result}, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x,y,z]}
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

}

pub type Point3 = Vec3;

// Format for output
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

//-vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

// vec3 += vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v:Vec3) {
        *self = *self + v;
    }
}

// vec3 + vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v:Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, v:Vec3) {
        *self = *self - v;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v:Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

// vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t:f64) {
        *self = *self * t;
    }
}

// vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: f64) -> Vec3 {
        Vec3::new(v * self.e[0], v* self.e[1], v* self.e[2])
    }
}

//f64 * vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.e[0], self* v.e[1], self * v.e[2])
    }
}

//vec3 * vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new( self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2])
    }
}

// Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t:f64) {
        *self = *self / t;
    }
}

//Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, v: f64) -> Vec3 {
        Vec3::new(self.x() / v, self.y() / v, self.z() /v)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
use std::ops::{Add, AddAssign, Neg, Mul, MulAssign, Sub, SubAssign, Div, DivAssign};

// Basic 3D vector type
#[derive(Debug, Clone, Copy)]
pub struct Vec3D {
    arr: [f32; 3]
}

// Trait magic did not work, resort to just type aliasing
pub type ColorRGB = Vec3D;

impl Vec3D {
    pub fn new(x0: f32, x1: f32, x2: f32) -> Self {
        Self{ arr: [ x0, x1, x2 ] }
    }

    // vector coefficient access
    pub fn x0(&self) -> f32 {
        self.arr[0]
    }

    pub fn x1(&self) -> f32 {
        self.arr[1]
    }

    pub fn x2(&self) -> f32 {
        self.arr[2]
    }

    // Vec3D coefficient access
    pub fn x(&self) -> f32 {
        self.arr[0]
    }

    pub fn y(&self) -> f32 {
        self.arr[1]
    }

    pub fn z(&self) -> f32 {
        self.arr[2]
    }

    // ColorRGB coefficient access
    pub fn r(&self) -> f32 {
        self.arr[0]
    }

    pub fn g(&self) -> f32 {
        self.arr[1]
    }

    pub fn b(&self) -> f32 {
        self.arr[2]
    }

    // Vec3D predefs
    pub fn zeros() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    // ColorRGB predefs
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn bytes(&self) -> &[f32] {
        &self.arr
    }

    // General vector math
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.arr[0] * self.arr[0] + self.arr[1] * self.arr[1] + self.arr[2] * self.arr[2]
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.arr[0] * other.arr[0] + self.arr[1] * other.arr[1] + self.arr[2] * other.arr[2]
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            arr: [
                self.arr[1] * other.arr[2] - self.arr[2] * other.arr[1],
                self.arr[2] * other.arr[0] - self.arr[0] * other.arr[2],
                self.arr[0] * other.arr[1] - self.arr[1] * other.arr[0],
            ]
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }
}

impl Add for Vec3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x0() + other.x0(), self.x1() + other.x1(), self.x2() + other.x2())
    }
}

impl AddAssign for Vec3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.x0() + other.x0(), self.x1() + other.x1(), self.x2() + other.x2());
    }
}

impl Neg for Vec3D {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x0(), -self.x1(), -self.x2())
    }
}

impl Mul<f32> for Vec3D {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x0() * scalar, self.x1() * scalar, self.x2() * scalar)
    }
}

impl MulAssign<f32> for Vec3D {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self::new(self.x0() * scalar, self.x1() * scalar, self.x2() * scalar)
    }
}

impl Sub for Vec3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x0() - other.x0(), self.x1() - other.x1(), self.x2() - other.x2())
    }
}

impl SubAssign for Vec3D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self.x0() - other.x0(), self.x1() - other.x1(), self.x2() - other.x2())
    }
}

impl Div<f32> for Vec3D {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self::new(self.x0() * (1.0 / scalar), self.x1() * (1.0 / scalar), self.x2() * (1.0 / scalar))
    }
}

impl DivAssign<f32> for Vec3D {
    fn div_assign(&mut self, scalar: f32) {
        *self = Self::new(self.x0() * (1.0 / scalar), self.x1() * (1.0 / scalar), self.x2() * (1.0 / scalar))
    }
}

// Also implement multiplication by Vec3D for f32
impl Mul<Vec3D> for f32 {
    type Output = Vec3D;

    fn mul(self, vec: Vec3D) -> Vec3D {
        Vec3D::new(vec.x0() * self, vec.x1() * self, vec.x2() * self)
    }
}

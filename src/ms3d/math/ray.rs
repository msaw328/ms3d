use super::vec3d::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3D,
    dir: Vec3D
}

impl Ray {
    pub fn new(origin: Vec3D, dir: Vec3D) -> Self {
        Self { origin, dir: dir.unit() }
    }

    pub fn at(&self, delta: f32) -> Vec3D {
        self.origin + delta * self.dir
    }

    pub fn origin(&self) -> Vec3D {
        self.origin
    }

    pub fn dir(&self) -> Vec3D {
        self.dir
    }
}

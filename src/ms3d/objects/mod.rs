use super::math::*;

// Trait for things that can be moved around in the 3D space, including objects visible on renders as well as the renderer itself (the camera)
pub trait Moveable {
    fn location(&self) -> Vec3D; // Return the current location of the thing
    fn move_to(&mut self, destination: Vec3D) -> (); // Moves to an absolute location
    fn move_by(&mut self, x: f32, y: f32, z: f32) -> (); // Move by an offset
}

// Basic implementation of the Moveable trait - an object representing an origin of something
pub struct Origin {
    location: Vec3D
}

impl Origin {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            location: Vec3D::new(x, y, z)
        }
    }

    pub fn at_center() -> Self {
        Self {
            location: Vec3D::zeros()
        }
    }
}

impl Moveable for Origin {
    fn location(&self) -> Vec3D {
        self.location
    }

    fn move_to(&mut self, destination: Vec3D) -> () {
        self.location = destination;
    }

    fn move_by(&mut self, x: f32, y: f32, z: f32) -> () {
        self.location += Vec3D::new(x, y, z);
    }
}

pub struct HitRecord {
    pub dist: f32,
    pub normal: Vec3D,
    pub point: Vec3D,
    pub color: ColorRGB,
    pub is_front: bool
}

pub trait Hittable {
    fn hit(&self, ray: Ray, dist_min: f32, dist_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3D,
    radius: f32
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32) -> Self {
        Self {
            center: Vec3D::new(x, y, z),
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, dist_min: f32, dist_max: f32) -> Option<HitRecord> {
        let vec_diff = ray.origin() - self.center;

        // Quadratic formula coefficients
        let a = ray.dir().dot(&ray.dir());
        let b = ray.dir().dot(&vec_diff) * 2.0;
        let c = vec_diff.dot(&vec_diff) - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;

        // If ray hits sphere its either 1 or 2 zeros
        if delta < 0.0 {
            return Option::None;
        }

        let dist = (-b - delta.sqrt()) / (2.0 * a);

        if dist < dist_min || dist > dist_max {
            return Option::None;
        }

        let point = ray.at(dist);
        let normal = (point - self.center) / self.radius;

        Option::<HitRecord>::Some(
            HitRecord {
                color: normal,
                normal,
                dist,
                point,
                is_front: normal.dot(&ray.dir()) < 0.0
            }
        )
    }
}

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>
}

impl Scene {
    // TODO: Remove this testing function
    // Instead add actual ray casting and 3d objects, probably spheres first cause theyre super simple
    fn ray_color(ray: &Ray) -> ColorRGB {
        let t: f32 = 0.5 * (ray.dir().unit().y() + 1.0);
        (1.0 - t) * ColorRGB::new(1.0, 1.0, 1.0) + t * ColorRGB::new(0.2, 0.4, 0.2)
    }

    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(8)
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) -> () {
        self.objects.push(obj);
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: Ray, dist_min: f32, dist_max: f32) -> Option<HitRecord> {
        let mut current_hitrec: Option<HitRecord> = Option::None;

        for obj in &self.objects {
            let hitrec_option = obj.hit(ray, dist_min, dist_max);

            if hitrec_option.is_none() {
                continue
            }

            if current_hitrec.is_none() || hitrec_option.as_ref().unwrap().dist < current_hitrec.as_ref().unwrap().dist {
                current_hitrec = hitrec_option;
            }
        }

        if current_hitrec.is_none() {
            return Option::<HitRecord>::Some(
                HitRecord {
                    color: Self::ray_color(&ray),
                    dist: 0.0,
                    normal: Vec3D::zeros(),
                    point: Vec3D::zeros(),
                    is_front: true
                }
            )
        }

        current_hitrec
    }
}

use super::math::*;
use super::objects::*;

use rand::distributions::Uniform;
use rand::{Rng, thread_rng};

#[derive(Debug)]
pub struct Camera {
    width: f32,
    height: f32,
    focal_length: f32,
    dir: Vec3D,
    pub origin: Vec3D,
    viewport_width: u32,
    viewport_height: u32
}

impl Camera {
    // Default values for width and height as well as focal length  set to 1.0 which result in the same aspect ratio as viewport and 90% FOV
    pub fn with_default_dimensions(viewport_width: u32, viewport_height: u32) -> Self {
        let DEFAULT_DIRECTION: Vec3D = Vec3D::new(0.0, 0.0, -1.0);
        const DEFAULT_FOCAL_LENGTH: f32 = 1.0;
        const DEFAULT_WIDTH: f32 = 2.0;
        let aspect_ratio = viewport_width as f32 / viewport_height as f32;
        let height = DEFAULT_WIDTH / aspect_ratio;

        Self {
            viewport_width,
            viewport_height,
            origin: Vec3D::zeros(),
            dir: DEFAULT_DIRECTION,
            focal_length: DEFAULT_FOCAL_LENGTH,
            width: DEFAULT_WIDTH,
            height
        }
    }

    pub fn look_at(&mut self, point: Vec3D) -> () {
        self.dir = (point - self.origin).unit();
    }

    pub fn render(&self, scene: &Scene) -> Vec<ColorRGB> {
        // The distance between centers of two pixels in 3D units
        let horizontal_step = self.width / self.viewport_width as f32;
        let vertical_step = self.height / self.viewport_height as f32;

        // Total amount of slide
        let total_horizontal_slide = horizontal_step * (self.viewport_width - 1) as f32;
        let total_vertical_slide = vertical_step * (self.viewport_height - 1) as f32;

        // Center of the viewport rectangle
        let rect_center = self.origin + (self.dir.unit() * self.focal_length);

        // Center of the top left pixel in the viewport rectangle
        let bottom_left_corner = rect_center - Vec3D::new(0.0, total_vertical_slide / 2.0, 0.0) - Vec3D::new(total_horizontal_slide / 2.0, 0.0, 0.0);

        // Uniform ranges for vector variation during multisampling antialiasing
        let horizontal_aa_range = Uniform::new(-horizontal_step / 2.0, horizontal_step / 2.0);
        let vertical_aa_range = Uniform::new(-vertical_step / 2.0, vertical_step / 2.0);
        let mut rng = thread_rng();

        // Rebind step variables as vectors
        let horizontal_step = Vec3D::new(horizontal_step, 0.0, 0.0);
        let vertical_step = Vec3D::new(0.0, vertical_step, 0.0);

        // List which holds the samples
        let mut sample_vec: Vec<ColorRGB> = Vec::with_capacity(self.viewport_width as usize * self.viewport_height as usize);
        
        for y in (0..self.viewport_height).rev() { // Since we are based on the bottom left corner we have to subtract from y isntead of add
            for x in 0..self.viewport_width {
                let base_dir_vec = bottom_left_corner + (horizontal_step * x as f32) + (vertical_step * y as f32) - self.origin;

                let mut color = ColorRGB::zeros();

                let sample_num = 64;

                for _ in 0..sample_num {
                    let (x, y) = (rng.sample(horizontal_aa_range), rng.sample(vertical_aa_range));

                    let dir_vec = base_dir_vec + Vec3D::new(x, y, 0.0);
                    let ray = Ray::new(self.origin, dir_vec);
    
                    let hitrec_option = scene.hit(ray, 0.0, f32::INFINITY);

                    color += hitrec_option.unwrap().color;
                }

                color /= sample_num as f32;

                sample_vec.push(color);
            }
        }

        sample_vec
    }
}

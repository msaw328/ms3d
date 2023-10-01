mod ms3d;

use ms3d::math::*;
use ms3d::objects::*;
use ms3d::render::*;

use png;

fn write_to_png(data: &[ColorRGB], width: u32, height: u32, filename: &str) {
    let pixels = data.iter().map(|x| x.bytes()).flatten().map(|x| (x * 255.0) as u8).collect::<Vec<u8>>();

    let file = std::fs::File::create(filename).unwrap();
    let ref mut w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
}

fn main() {
    let width = 640;
    let height = 480;

    let mut camera = Camera::with_default_dimensions(width, height);

    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(-0.3, 0.3, -1.5, 0.5)));
    scene.add(Box::new(Sphere::new(0.1, -0.2, -2.5, 0.2)));
    scene.add(Box::new(Sphere::new(0.0, -0.5, -2.0, 0.3)));


    let num_steps = 1;
    let total_angle = 2.0 * std::f32::consts::PI / 3.0;

    for step in 0..num_steps {
        let angle = step as f32 / num_steps as f32 * total_angle;

        let radius = 2.0;

        //camera.origin = Point3d::new(radius * angle.cos(), 0.0, radius * angle.sin());
        //camera.look_at(Point3d::empty());

        let data = camera.render(&scene);

        let filename = format!("test_renders/frame{}.png", step);
        write_to_png(&data, width, height, filename.as_str());
        println!("Frame {} done out of {}", step, num_steps - 1);
    }
    
}

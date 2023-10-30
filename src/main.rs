mod vectors;
mod image;
mod ray;
mod sphere;
mod progress;
mod world;
mod camera;

use rand::Rng;
use sphere::Sphere;
use vectors::{Vec3, Color, Point3};
use image::{ASPECT_RATIO, IMAGE_WIDTH, FOCAL_LENGTH, ORIGIN_POINT, VIEWPORT_HEIGHT};
use ray::Ray;
use world::World;
use camera::Camera;

use std::fs::File;
use std::io::{Write, BufWriter, stderr};
use std::time::SystemTime;

fn ray_color(r: &Ray, recurse_count: u16) -> Color {
    if recurse_count <= 0 {
        return Color::new(0.0, 0.0, 0.0)
    }
  
    let unit_direction = r.direction().normalized();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    
  
    let SPHERES: [Sphere; 4] = [
        Sphere::new(Point3::new(-0.50,0.5,-1.0),0.3, [0.7, 0.2, 0.3], 0.0),
        Sphere::new(Point3::new(0.5,0.0,-2.0),0.6, [0.3, 0.1, 0.6], 0.0),
        Sphere::new(Point3::new(1.5,-1.0,-1.8),0.8, [0.1, 0.8, 0.5], 0.0),
        Sphere::new(Point3::new(-0.5,-0.8,-1.5),0.5, [1.0, 1.0, 1.0], 20.0)
    ];
  
  
    let mut closest_object: &Sphere = &SPHERES[0];
    let mut closest_distance: f64 = -1.0;
  
    for i in 0..SPHERES.len() {
        if SPHERES[i].intersects_ray(r) {
            let distance = SPHERES[i].intersects_ray_at(r);
  
            if distance < closest_distance || closest_distance == -1.0 {
                closest_object = &SPHERES[i];
                closest_distance = distance;
            }
        }
    }
  
    if closest_distance != -1.0 {
        let diffuse_ray = Ray::new_random(r.at(closest_distance), &closest_object);
        let traced_color = ray_color(&diffuse_ray, recurse_count - 1);
  
  
        return (closest_object.color() * (closest_object.emission() + 1 as f64)) * traced_color
    }
  
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
  }

fn main() {
    // Image settings
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;

    // Camera
    let viewport_height = VIEWPORT_HEIGHT;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = FOCAL_LENGTH;

    let origin = Point3::new(ORIGIN_POINT[0], ORIGIN_POINT[1], ORIGIN_POINT[2]);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let image_file = File::create("image.ppm").expect("Uh oh");
    let mut image_writer = BufWriter::new(image_file);

    image_writer.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes()).expect("Uh oh");

    const SAMPLE_SIZE: u16 = 512;
    const MAX_BOUNCES: u16 = 4;

    // let mut world = World::initial_scene(Camera::new(&origin, &ASPECT_RATIO, IMAGE_WIDTH, &FOCAL_LENGTH));
    // world.add_obj(Sphere::new(Point3::new(-0.50,0.5,-1.0),0.3, [0.7, 0.2, 0.3], 0.0));
    // world.add_obj(Sphere::new(Point3::new(0.5,0.0,-2.0),0.6, [0.3, 0.1, 0.6], 0.0));
    // world.add_obj(Sphere::new(Point3::new(1.5,-1.0,-1.8),0.8, [0.1, 0.8, 0.5], 0.0));
    // world.add_obj(Sphere::new(Point3::new(-0.5,-0.8,-1.5),0.5, [1.0, 1.0, 1.0], 20.0));

    // world.camera().capture(&world)

    let mut rng = rand::thread_rng();
    // let mut progress_bar: Progress = Progress::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    // let start_time = SystemTime::now();

    for j in (0..IMAGE_HEIGHT).rev() {
        let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);

            // TODO: Multi-sampling
            let mut pixel_color_sum = Color::new(0.0, 0.0, 0.0);

            for _ in 0..(SAMPLE_SIZE - 1) {
                let rand_u: f64 = rng.gen();
                let rand_v: f64 = rng.gen();


                let r = Ray::new(origin, lower_left_corner + (u + (rand_u / ((IMAGE_WIDTH - 1) as f64))) * horizontal + (v + (rand_v / ((IMAGE_HEIGHT - 1) as f64))) * vertical - origin);
                let sample_pixel_color = ray_color(&r, MAX_BOUNCES);

                pixel_color_sum += sample_pixel_color;
            }

            let pixel_color = pixel_color_sum / SAMPLE_SIZE as f64;


            image_writer.write(format!("{}\n", pixel_color.format_color()).as_bytes()).expect("UH oh");
            // print!("\r{}", progress_bar.update(i * j, 4));
            // stderr().flush().unwrap()
        }
    }
}

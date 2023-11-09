use crate::image::VIEWPORT_HEIGHT;

use super::vectors::{Vec3, Point3, Color};
use super::ray::Ray;
use super::world::World;
use super::sphere::Sphere;

use std::fs::File;
use std::io::{Write, BufWriter, stderr};
use std::time::SystemTime;
use rand::Rng;


pub struct Camera {
  origin: Point3,

  viewport_height: u64,
  viewport_width: u64,
  focal_length: f64,


  starting_corner: Point3,
  horizontal: Vec3,
  vertical: Vec3,
}

struct Quality {
  sample_size: u64,
  max_bounces: u64,
}

fn ray_color(r: &Ray, recurse_count: u16, objects: &Vec<Sphere>) -> Color {
  if recurse_count <= 0 {
      return Color::new(0.0, 0.0, 0.0)
  }

  let unit_direction = r.direction().normalized();
  let t: f64 = 0.5 * (unit_direction.y() + 1.0);
  


  let mut closest_object: &Sphere = &objects[0];
  let mut closest_distance: f64 = -1.0;

  for i in 0..objects.len() {
    if objects[i].intersects_ray(r) {
      let distance = objects[i].intersects_ray_at(r);

      if distance < closest_distance || closest_distance == -1.0 {
          closest_object = &objects[i];
          closest_distance = distance;
      }
    }
  }

  if closest_distance != -1.0 {
      let new_ray = closest_object.material().trace_color(&r, closest_object, closest_distance);
      let traced_color = ray_color(&new_ray, recurse_count - 1, objects);


      return closest_object.material().return_color(&traced_color, &closest_object)
  }

  (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

impl Camera {
  pub fn new(origin: &Point3, aspect_ratio: &f64, viewport_width: u64, focal_length: &f64) -> Camera {
    let horizontal = Vec3::new(aspect_ratio * 1.0 as f64, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 1.0,0.0);
    
    Camera {
      origin: *origin,

      viewport_height: (viewport_width as f64 / aspect_ratio) as u64,
      viewport_width: viewport_width,
      focal_length: *focal_length,

      starting_corner: origin - &(horizontal / 2.0) + vertical / 2.0 - Vec3::new(0.0, 0.0, *focal_length),
      horizontal: horizontal,
      vertical: vertical,
    }
  }

  pub fn capture(&self, world: &World) {
    let image_file = File::create("image.ppm").expect("Uh oh");
    let mut image_writer = BufWriter::new(image_file);

    image_writer.write(format!("P3\n{} {}\n255\n", self.viewport_width, self.viewport_height).as_bytes()).expect("Uh oh");

    const SAMPLE_SIZE: u16 = 1024;
    const MAX_BOUNCES: u16 = 4;


    let mut rng = rand::thread_rng();

    for j in (0..self.viewport_height) {
        let v = (j as f64) / ((self.viewport_height - 1) as f64);

        for i in 0..self.viewport_width {
            let u = (i as f64) / ((self.viewport_width - 1) as f64);

            let mut pixel_color_sum = Color::new(0.0, 0.0, 0.0);

            for _ in 0..(SAMPLE_SIZE - 1) {
                let rand_u: f64 = rng.gen();
                let rand_v: f64 = rng.gen();


                let r = Ray::new(self.origin, self.starting_corner + (u + (rand_u / ((self.viewport_width - 1) as f64))) * self.horizontal - (v + (rand_v / ((self.viewport_height - 1) as f64))) * self.vertical - self.origin);
                let sample_pixel_color = ray_color(&r, MAX_BOUNCES, world.list_objects());

                pixel_color_sum += sample_pixel_color;
            }

            let pixel_color = pixel_color_sum / SAMPLE_SIZE as f64;


            image_writer.write(format!("{}\n", pixel_color.format_color()).as_bytes()).expect("UH oh");
        }
    }
  }
}


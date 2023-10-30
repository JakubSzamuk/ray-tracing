use rand::Rng;

use super::vectors::{Vec3, Point3};
use super::sphere::Sphere;

pub struct Ray {
  orig: Point3,
  dir: Vec3,
}

impl Ray {
  pub fn new(origin: Point3, direction: Vec3) -> Ray {
    Ray {
      orig: origin,
      dir: direction,
    }
  }
  
  pub fn new_random(origin: Point3, avoid: &Sphere) -> Ray {
    let mut rng = rand::thread_rng();

    
    let random_vector = loop {
      let new_vec = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

      if new_vec.length() <= 1.0 {
        break new_vec;
      }
    };

    let target = random_vector + (origin - avoid.center()).normalized();
    Ray::new(origin, target)
  }

  pub fn origin(&self) -> Point3 {
    self.orig
  }
  
  pub fn direction(&self) -> Vec3 {
    self.dir
  }

  pub fn at(&self, t: f64) -> Point3 {
    self.orig + t * self.dir
  }
}
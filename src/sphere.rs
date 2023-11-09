use crate::vectors::Vec3;

use super::vectors::{Point3, Color};
use super::ray::Ray;
use super::material::Hit;

pub struct Sphere {
  center: Point3,
  radius: f64,
  emission: f64,
  material: Box<dyn Hit>
}

impl Sphere {
  pub fn new(origin: Point3, radius: f64, e_strength: f64, material: Box<dyn Hit>) -> Sphere {
    Sphere {
      center: origin,
      radius: radius,
      emission: e_strength,
      material
    }
  }

  pub fn center(&self) -> Point3 {
    self.center
  }

  pub fn radius(&self) -> f64 {
    self.radius
  }

  pub fn material(&self) -> &Box<dyn Hit> {
    &self.material
  }

  pub fn emission(&self) -> f64 {
    self.emission
  }

  pub fn intersects_ray_at(&self, ray: &Ray) -> f64 {
    let origin_center = ray.origin() - self.center();
    let a = ray.direction().dot(ray.direction());
    let b = ray.direction().dot(origin_center);
    let c = origin_center.dot(origin_center) - self.radius() * self.radius();

    let top_root: f64 = b * b - a * c;
    if top_root >= 0.01 {
      -top_root.sqrt() - b / a
    } else {
      -1.0
    }
  }

  pub fn intersects_ray(&self, ray: &Ray) -> bool {
    let t = self.intersects_ray_at(ray);

    t > 0.0
  }

  pub fn normal_vector(&self, at: Point3) -> Vec3 {
    (at - self.center()).normalized()
  }
}

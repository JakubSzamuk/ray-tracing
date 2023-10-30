use super::vectors::{Point3, Color};
use super::ray::Ray;

pub struct Sphere {
  center: Point3,
  radius: f64,
  color: Color,
  emission: f64,
}

impl Sphere {
  pub fn new(origin: Point3, radius: f64, color: [f64; 3], e_strength: f64) -> Sphere {
    Sphere {
      center: origin,
      radius: radius,
      color: Color::new(color[0], color[1], color[2]),
      emission: e_strength,
    }
  }

  pub fn center(&self) -> Point3 {
    self.center
  }

  pub fn radius(&self) -> f64 {
    self.radius
  }

  pub fn color(&self) -> Color {
    self.color
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
}

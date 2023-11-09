use rand::Rng;

use super::sphere::Sphere;
use super::ray::Ray;
use super::vectors::Color;

pub trait Hit {
  fn trace_color(&self, r: &Ray, hit_object: &Sphere, hit_distance: f64) -> Ray;
  fn return_color(&self, traced_color: &Color, hit_object: &Sphere) -> Color;
}

pub struct Diffuse {
  pub albedo: Color
}

impl Diffuse {
  pub fn new(color: [f64; 3]) -> Diffuse {
    Diffuse {
      albedo: Color::new(color[0], color[1], color[2])
    }
  }
  pub fn color(&self) -> Color {
    self.albedo
  }
}

impl Hit for Diffuse {
  fn trace_color(&self, r: &Ray, hit_object: &Sphere, hit_distance: f64) -> Ray {
    Ray::new_random(r.at(hit_distance), &hit_object)
  }
  fn return_color(&self, traced_color: &Color, hit_object: &Sphere) -> Color {
    (self.color() * (hit_object.emission() + 1 as f64)) * *traced_color
  }
}

pub struct Metal {
  roughness: f64
}

impl Metal {
  pub fn new(roughness: f64) -> Metal {
    Metal {
      roughness: roughness
    }
  }
}

impl Hit for Metal {
  fn trace_color(&self, r: &Ray, hit_object: &Sphere, hit_distance: f64) -> Ray {
    let normal = hit_object.normal_vector(r.at(hit_distance));
    let mut rng = rand::thread_rng();
    if rng.gen::<f64>() > self.roughness {
      return Ray::new(r.at(hit_distance), r.direction() - 2.0 * r.direction().dot(normal) * normal);
    }
    
    Ray::new_random(r.at(hit_distance), &hit_object)
  }
  fn return_color(&self, traced_color: &Color, _hit_object: &Sphere) -> Color {
    *traced_color
  }
}
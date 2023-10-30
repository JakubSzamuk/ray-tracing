use super::camera::Camera;
use super::sphere::Sphere;

pub struct World {
  objects: Vec<Sphere>,
  camera: Camera,
}

impl World {
  pub fn initial_scene(camera: Camera) -> World {
    World {
      objects: Vec::new(),
      camera: camera,
    }
  }

  pub fn add_obj(&mut self, object: Sphere) {
    self.objects.push(object);
  }

  pub fn list_objects(&self) -> &Vec<Sphere> {
    &self.objects
  }

  pub fn camera(&self) -> &Camera {
    &self.camera
  }

}
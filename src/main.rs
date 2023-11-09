mod vectors;
mod image;
mod ray;
mod sphere;
mod progress;
mod world;
mod camera;
mod material;


use material::{Hit, Diffuse, Metal};
use sphere::Sphere;
use vectors::{Point3};
use image::{ASPECT_RATIO, IMAGE_WIDTH, FOCAL_LENGTH, ORIGIN_POINT};
use world::World;
use camera::Camera;

fn main() {

    let origin = Point3::new(ORIGIN_POINT[0], ORIGIN_POINT[1], ORIGIN_POINT[2]);


    let mut world = World::initial_scene(Camera::new(&origin, &ASPECT_RATIO, IMAGE_WIDTH, &FOCAL_LENGTH));
    world.add_obj(Sphere::new(Point3::new(-0.50,0.5,-1.0),0.3, 0.0, Box::new(Diffuse::new([0.4, 0.5, 0.8]))));
    world.add_obj(Sphere::new(Point3::new(0.5,0.0,-2.0),0.6, 0.0, Box::new(Diffuse::new([0.4, 0.5, 0.8]))));
    world.add_obj(Sphere::new(Point3::new(1.5,-1.0,-1.8),0.8, 0.0, Box::new(Metal::new(0.1))));
    world.add_obj(Sphere::new(Point3::new(-0.5,-0.8,-1.5),0.5, 20.0, Box::new(Diffuse::new([0.4, 0.5, 0.8]))));

    world.camera().capture(&world)
}

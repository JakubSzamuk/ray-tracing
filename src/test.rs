// Purpose: Image settings

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: u64 = 1920;
pub const VIEWPORT_HEIGHT: f64 = 2.0;
pub const FOCAL_LENGTH: f64 = 1.0;
pub const ORIGIN_POINT: [f64; 3] = [0.0, 0.0, 0.0];

pub struct Sphere {
  center: Point3,
  radius: f64,
}

impl Sphere {
  pub fn new(origin: Point3, radius: f64) -> Sphere {
    Sphere {
      center: origin,
      radius: radius
    }
  }

  pub fn center(&self) -> Point3 {
    self.center
  }

  pub fn radius(&self) -> f64 {
    self.radius
  }

  pub fn intersects_ray_at(&self, ray: &Ray) -> f64 {
    let origin_center = ray.origin() - self.center();
    let a = ray.direction().dot(ray.direction());
    let b = ray.direction().dot(origin_center);
    let c = origin_center.dot(origin_center) - self.radius() * self.radius();

    let top_root = b * b - a * c;
    if top_root >= 0.0 {
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


#[derive(Clone, Copy)]
pub struct Vec3 {
  e: [f64; 3]
}


pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
  pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
    Vec3 {
      e: [e0, e1, e2]
    }
  }

  
  pub fn x(self: Vec3) -> f64 {
    self[0]
  }
  
  pub fn y(self: Vec3) -> f64 {
    self[1]
  }
  
  pub fn z(self: Vec3) -> f64 {
    self[2]
  }
  
  pub fn dot(self, other: Vec3) -> f64 {
    self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
  }
  
  pub fn length(self) -> f64 {
    self.dot(self).sqrt()
  }
  
  pub fn cross(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [
        self[1] * other[2] - self[2] * other[1],
        self[2] * other[0] - self[0] * other[2],
        self[0] * other[1] - self[1] * other[0]
      ]
    }
  }
  
  pub fn normalized(self) -> Vec3 {
    self / self.length()
  }
}


impl Color {
  pub fn format_color(self) -> String {
    format!("{} {} {}", 
                        (255.999 * self[0]) as u64,          
                        (255.999 * self[1]) as u64,          
                        (255.999 * self[2]) as u64,                    
    )
  }
}

impl Display for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {}, {})", self[0], self[1], self[2])
  }
}

impl Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, index: usize) -> &f64 {
    &self.e[index]
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, index: usize) -> &mut f64 {
    &mut self.e[index]
  }
}

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
    }
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Vec3) -> () {
    *self = Vec3 {
      e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
    }
  }
}

impl Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
    }
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, other: Vec3) -> () {
    *self = Vec3 {
      e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
    }
  }
}

impl Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, other: f64) -> Vec3 {
    Vec3 {
      e: [self[0] * other, self[1] * other, self[2] * other]
    }
  }
}

impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, other: f64) -> () {
    *self = Vec3 {
      e: [self[0] * other, self[1] * other, self[2] * other]
    }
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [self * other[0], self * other[1], self * other[2]]
    }
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, other: f64) -> Vec3 {
    Vec3 {
      e: [self[0] / other, self[1] / other, self[2] / other]
    }
  }
}

impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, other: f64) -> () {
    *self = Vec3 {
      e: [self[0] / other, self[1] / other, self[2] / other]
    }
  }
}

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













fn ray_color(r: &Ray) -> Color {
  let unit_direction = r.direction().normalized();
  let t: f64 = 0.5 * (unit_direction.y() + 1.0);
  

  let SPHERES: [Sphere; 3] = [
      Sphere::new(Point3::new(0.0, -0.2, -1.0),0.3),
      Sphere::new(Point3::new(0.0, 0.5, -1.0),0.6),
      Sphere::new(Point3::new(0.0, 0.0, -4.0),0.5),
  ];


  let mut closest_object: &Sphere = &SPHERES[0];
  let mut closest_distance: f64 = 20.0;

  for i in 0..SPHERES.len() {
      if SPHERES[i].intersects_ray(r) {
          let distance = SPHERES[i].intersects_ray_at(r);

          if distance < closest_distance {
              closest_object = &SPHERES[i];
              closest_distance = distance;
          }
      }
  }

  if closest_distance != 20.0 {
      let normal_vec = (r.at(t) - closest_object.center()).normalized();

      return 0.5 * Color::new(normal_vec.x() + 1.0, normal_vec.y() + 1.0, normal_vec.z() + 1.0);
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


  println!("P3");
  println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
  println!("255");

  for j in (0..IMAGE_HEIGHT).rev() {
      let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

      for i in 0..IMAGE_WIDTH {
          let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);

          let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
          let pixel_color = ray_color(&r);

          println!("{}", pixel_color.format_color());

      }
  }
}


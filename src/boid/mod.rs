mod vector;

use vector::Vector;
use sdl2::{pixels::Color, render::Canvas, video::Window, rect::Point};

const UNIT_SIZE: f32 = 5.0;
const PI: f32 = std::f32::consts::PI;

pub struct Boid {
  _id: u32,
  color: Color,
  pos: Point,
  v: Vector,
  a: Vector,
  scale: f32
}

/// Public Functions
impl Boid {
  pub fn new(_id: u32, color: Color, pos: (i32, i32)) -> Self {
    Boid {
      _id,
      color,
      pos: Point::new(pos.0, pos.1),
      v: Vector::new(0.5f32, 0.2f32),
      a: Vector::new(0.0, 0.0),
      scale: 1f32
    }
  }

  pub fn draw(&self, canvas: &mut Canvas<Window>) {

    // Draw Outline
    canvas.set_draw_color(self.color);
    let tip = self.pos;
    let back_left = self.pos.offset((-40f32 * self.scale) as i32, (-20f32 * self.scale) as i32);
    let back_left = Boid::rotate_point(&back_left, &tip, self.v.direction());
    let middle = self.pos.offset((-30f32 * self.scale) as i32, (0f32 * self.scale) as i32);
    let middle = Boid::rotate_point(&middle,&tip, self.v.direction());
    let back_right = self.pos.offset((-40f32 * self.scale) as i32, (20f32 * self.scale) as i32);
    let back_right = Boid::rotate_point(&back_right, &tip, self.v.direction());
    let points = vec![tip, back_left, middle, back_right, tip];

    canvas.draw_lines(&*points).expect("Unable to Draw Boid {self._id}");
  }

  pub fn tick(&mut self, canvas: &mut Canvas<Window>) {
    self.check_boundary_collision(canvas);
    self.update_physics();
    self.draw(canvas);
    dbg!(&self.pos, &self.v, &self.a);
  }
}

// Tick Functions
impl Boid {
  fn update_physics(&mut self) {
    self.v.offset_self(&self.a);
    self.pos = self.pos.offset((self.v.x() * UNIT_SIZE) as i32, (self.v.y() * UNIT_SIZE) as i32);
  }

  fn check_boundary_collision(&mut self, canvas: &Canvas<Window>) {
    // Setup window max size and proposed movement position
    let window_max_size = canvas.window().drawable_size();
    let new_pos = self.pos.offset(self.v.x() as i32, self.v.y() as i32);
    
    // Test if proposed position is out of bounds,
    // if it is then reverse the velocity in that direction and 
    // update the proposed position accordingly.
    if new_pos.x >= window_max_size.0 as i32 || new_pos.x <= 0 {
      println!("Bounce X!");
      self.v.reverse_x();
      if new_pos.x >= window_max_size.0 as i32 {
        self.pos = Point::new(window_max_size.0 as i32 - 1, self.pos.y);
      } else {
        self.pos = Point::new(1, self.pos.y);
      }
    }
    if new_pos.y >= window_max_size.1 as i32 || new_pos.y <= 0 {
      println!("Bounce Y!");
      self.v.reverse_y();
      if new_pos.y >= window_max_size.1 as i32 {
        self.pos = Point::new(self.pos.x, window_max_size.1 as i32 - 1);
      } else {
        self.pos = Point::new(self.pos.x, 1);
      }
    }
  } 
}

// Utility Functions
impl Boid {
    pub fn set_scale(&mut self, scale: f32) {
      self.scale = scale;
    } 

    /// Rotate the given point around the center point given, by the angle theta,
    /// Positive theta = rotation counterclockwise
    fn rotate_point(p: &Point, c: &Point, theta: f32) -> Point {
      let x = p.x as f32;
      let y = p.y as f32;
      let cx = c.x as f32;
      let cy = c.y as f32;
      let x_rot = (theta.cos() * (x - cx)) - (theta.sin() * (y - cy)) + cx;
      let y_rot = (theta.sin() * (x - cx)) + (theta.cos() * (y - cy)) + cy;
      Point::new(x_rot as i32, y_rot as i32)
    }
}

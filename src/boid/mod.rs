mod vector;

use vector::Vector;
use sdl2::{pixels::Color, render::Canvas, video::Window, rect::Point};

const BLACK: Color = Color::RGB(0, 0, 0);
const UNIT_SIZE: f32 = 5.0;
const COLLISION_ENERGY_LOSS: Vector = Vector::new(-0.01, -0.01);
const G: Vector = Vector::new(0.0, 0.1);

pub struct Boid {
  _id: u32,
  color: Color,
  pos: Point,
  v: Vector,
  a: Vector,
  m: f32,
  scale: f32
}

/// Public Functions
impl Boid {
  pub fn new(_id: u32, color: Color, pos: (i32, i32)) -> Self {
    Boid {
      _id,
      color,
      pos: Point::new(pos.0, pos.1),
      v: Vector::new(0.25f32, 0.1f32),
      a: G,
      m: 10f32,
      scale: 1f32
    }
  }

  pub fn draw(&self, canvas: &mut Canvas<Window>) {
    // Draw Outline
    canvas.set_draw_color(self.color);
    let tip = self.pos;
    let back_left = self.pos.offset((-40f32 * self.scale) as i32, (-20f32 * self.scale) as i32);
    let middle = self.pos.offset((-30f32 * self.scale) as i32, (0f32 * self.scale) as i32);
    let back_right = self.pos.offset((-40f32 * self.scale) as i32, (20f32 * self.scale) as i32);
    canvas.draw_line(tip, back_left).unwrap();
    canvas.draw_line(back_left, middle).unwrap();
    canvas.draw_line(middle, back_right).unwrap();
    canvas.draw_line(middle, back_right).unwrap();
    canvas.draw_line(back_right, tip).unwrap();
  }

  pub fn tick(&mut self, canvas: &mut Canvas<Window>) {
    self.check_boundary_collision(canvas);
    self.update_physics();
    self.draw(canvas);
    println!("pos: {:?} v: {:?} a: {:?} ", (self.pos.x, self.pos.y), (self.v.x(), self.v.y()), (self.a.x(), self.a.y()));
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
}
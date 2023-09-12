mod vector;

use vector::Vector;
use sdl2::{pixels::Color, render::Canvas, video::Window, rect::Point};

const BLACK: Color = Color::RGB(0, 0, 0);

pub struct Boid {
  _id: u32,
  color: Color,
  pos: Point,
  v: Vector,
  a: Vector
}

impl Boid {
  pub fn new(_id: u32, color: Color) -> Self {
    Boid {
      _id,
      color,
      pos: Point::new(50, 50),
      v: Vector::new(0.25f32, 0.25f32),
      a: Vector::new(0.1f32, 0.1f32)
    }
  }

  pub fn draw(&self, canvas: &mut Canvas<Window>) {
    // Draw Outline
    canvas.set_draw_color(self.color);
    let tip = self.pos;
    let back_left = self.pos.offset(-40, -20);
    let middle = self.pos.offset(-30, 0);
    let back_right = self.pos.offset(-40, 20);
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
  }

  fn update_physics(&mut self) {
    self.v.offset_self(&self.a);
    self.pos = self.pos.offset(self.v.x() as i32, self.v.y() as i32);
  }

  fn check_boundary_collision(&mut self, canvas: &mut Canvas<Window>) {
    // Setup window max size and proposed movement position
    let window_max_size = canvas.window().drawable_size();
    let new_pos = self.pos.offset(self.v.x() as i32, self.v.y() as i32);
    
    // Test if proposed position is out of bounds,
    // if it is then reverse the velocity in that direction and 
    // update the proposed position accordingly.
    if new_pos.x >= window_max_size.0 as i32 || new_pos.x <= 0 {
      self.v.reverse_x();
    }
    if new_pos.y >= window_max_size.1 as i32 || new_pos.y <= 0 {
      self.v.reverse_y();
    }
  } 
}

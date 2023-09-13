pub struct Vector {
  x: f32,
  y: f32
}

impl Vector {
  pub const fn new(x: f32, y: f32) -> Self {
    Vector{
      x,
      y
    }
  }

  pub fn offset(&self, other: &Vector) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y
    }
  }

  pub fn offset_self(&mut self, other: &Vector) {
    self.x += other.x;
    self.y += other.y;
  }

  pub fn x(&self) -> f32 {
    self.x
  }

  pub fn y(&self) -> f32 {
    self.y
  }

  pub fn component_x(&self) -> Vector {
    Vector::new(self.x, 0.0)
  }

  pub fn component_y(&self) -> Vector {
    Vector::new(0.0, self.y) 
  }

  pub fn reverse(&self) -> Vector {
    Vector::new(self.x * -1.0, self.y * -1.0)
  }

  pub fn reverse_self(&mut self) {
    self.x *= -1f32;
    self.y *= -1f32;
  }

  pub fn reverse_x(&mut self) {
    self.x *= -1f32;
  }

  pub fn reverse_y(&mut self) {
    self.y *= -1f32;
  }
}
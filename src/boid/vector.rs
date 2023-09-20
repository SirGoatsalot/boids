use std::f32::consts::PI; 

#[derive(Debug)]
pub struct Vector {
  x: f32,
  y: f32,
  magnitude: f32,
  direction: f32
}

// Public Vector functions
/// 2D Vector implementation.
/// Direction is in radians relative to the x-axis.
/// Positive is a clockwise rotation, negative is counterclockwise.
impl Vector {
  pub fn new(x: f32, y: f32) -> Self {
    Vector{
      x,
      y,
      magnitude: ((x * x) + (y * y)).sqrt(),
      direction: 0.0,
    }
  }

  pub fn magnitude(&self) -> f32 {
    self.magnitude
  }

  pub fn direction(&self) -> f32 {
    self.direction
  }

  pub fn offset(&self, other: &Vector) -> Vector {
    let x = self.x + other.x;
    let y = self.y + other.y;
    Vector::new(x, y)
  }

  pub fn offset_self(&mut self, other: &Vector) {
    self.x += other.x;
    self.y += other.y;
    self.magnitude = ((self.x * self.x) + (self.y * self.y)).sqrt();
    self.update_direction(); 
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
    self.update_direction();
  }

  pub fn reverse_x(&mut self) {
    self.x *= -1f32;
    self.update_direction();
  }

  pub fn reverse_y(&mut self) {
    self.y *= -1f32;
    self.update_direction();
  }
}

/// Private Vector functions
impl Vector {
  fn update_direction(&mut self) {
    match (self.x , self.y) {
      (0.0, 0.0) => {}       
      (0.0, 0.0..) => self.direction = 0.5 * PI,                             
      (0.0, ..=0.0) => self.direction = -(0.5 * PI),                         
      (0.0.., _) => self.direction = (self.y / self.x).atan(),
      (..=0.0, _) => self.direction = PI + (self.y / self.x).atan(),
      _ => {panic!("Invalid Velocity, could not update direction");}                                                        
    }                                                                                            
  }
}

pub mod color;

pub use color::*;
use nalgebra::{Matrix4, Vector2};
use rand::Rng;

pub fn u_sqrt(x: usize) -> usize {
  (x as f64).sqrt().round() as usize
}

pub fn vec2_rand() -> Vector2<f32> {
  let mut rng = rand::thread_rng();

  Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
}

pub type Position = Vector2<f32>;
pub type Size = Vector2<f32>;
pub type Velocity = Vector2<f32>;
pub type Mass = f32;

pub type Projection = Matrix4<f32>;
pub type ProjectionUniform = [[f32; 4]; 4];
pub type View = Matrix4<f32>;
pub type ViewUniform = [[f32; 4]; 4];
pub type ViewProjection = Matrix4<f32>;
pub type ViewProjectionUniform = [[f32; 4]; 4];

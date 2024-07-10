pub mod color;
pub use color::*;
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
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

#[inline]
pub const fn vec2(x: f32, y: f32) -> Vector2<f32> {
  Vector2::new(x, y)
}

#[inline]
pub const fn vec3(x: f32, y: f32, z: f32) -> Vector3<f32> {
  Vector3::new(x, y, z)
}

#[inline]
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vector4<f32> {
  Vector4::new(x, y, z, w)
}

#[inline]
pub const fn vec2i(x: i32, y: i32) -> Vector2<i32> {
  Vector2::new(x, y)
}

#[inline]
pub const fn vec3i(x: i32, y: i32, z: i32) -> Vector3<i32> {
  Vector3::new(x, y, z)
}

#[inline]
pub const fn vec4i(x: i32, y: i32, z: i32, w: i32) -> Vector4<i32> {
  Vector4::new(x, y, z, w)
}

#[inline]
pub const fn vec2u(x: u32, y: u32) -> Vector2<u32> {
  Vector2::new(x, y)
}

#[inline]
pub const fn vec3u(x: u32, y: u32, z: u32) -> Vector3<u32> {
  Vector3::new(x, y, z)
}

#[inline]
pub const fn vec4u(x: u32, y: u32, z: u32, w: u32) -> Vector4<u32> {
  Vector4::new(x, y, z, w)
}

#[inline]
pub const fn vec2b(x: bool, y: bool) -> Vector2<bool> {
  Vector2::new(x, y)
}

#[inline]
pub const fn vec3b(x: bool, y: bool, z: bool) -> Vector3<bool> {
  Vector3::new(x, y, z)
}

#[inline]
pub const fn vec4b(x: bool, y: bool, z: bool, w: bool) -> Vector4<bool> {
  Vector4::new(x, y, z, w)
}

#[inline]
pub const fn quat(x: f32, y: f32, z: f32, w: f32) -> nalgebra::Quaternion<f32> {
  nalgebra::Quaternion::new(w, x, y, z)
}

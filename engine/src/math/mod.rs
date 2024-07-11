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

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

pub type Vec2i = Vector2<i32>;
pub type Vec3i = Vector3<i32>;
pub type Vec4i = Vector4<i32>;

pub type Vec2u = Vector2<u32>;
pub type Vec3u = Vector3<u32>;
pub type Vec4u = Vector4<u32>;

pub type Vec2b = Vector2<bool>;
pub type Vec3b = Vector3<bool>;
pub type Vec4b = Vector4<bool>;

pub type Position = Vec2;
pub type Size = Vec2;
pub type Velocity = Vec2;
pub type Mass = f32;

pub type Projection = Matrix4<f32>;
pub type ProjectionUniform = [[f32; 4]; 4];
pub type View = Matrix4<f32>;
pub type ViewUniform = [[f32; 4]; 4];
pub type ViewProjection = Matrix4<f32>;
pub type ViewProjectionUniform = [[f32; 4]; 4];

// SURELY there isnt a function to do this already in nalgebra with a horrible name
#[inline]
pub fn vec2_mul(a: Vec2, b: Vec2) -> Vec2 {
  let x = a.x * b.x;
  let y = a.y * b.y;

  vec2(x, y)
}

#[inline]
pub fn vec3_mul(a: Vec3, b: Vec3) -> Vec3 {
  let x = a.x * b.x;
  let y = a.y * b.y;
  let z = a.z * b.z;

  vec3(x, y, z)
}

#[inline]
pub fn vec4_mul(a: Vec4, b: Vec4) -> Vec4 {
  let x = a.x * b.x;
  let y = a.y * b.y;
  let z = a.z * b.z;
  let w = a.w * b.w;

  vec4(x, y, z, w)
}

#[inline]
pub const fn vec2(x: f32, y: f32) -> Vec2 {
  Vec2::new(x, y)
}

#[inline]
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
  Vec3::new(x, y, z)
}

#[inline]
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
  Vec4::new(x, y, z, w)
}

#[inline]
pub const fn vec2i(x: i32, y: i32) -> Vec2i {
  Vec2i::new(x, y)
}

#[inline]
pub const fn vec3i(x: i32, y: i32, z: i32) -> Vec3i {
  Vec3i::new(x, y, z)
}

#[inline]
pub const fn vec4i(x: i32, y: i32, z: i32, w: i32) -> Vec4i {
  Vec4i::new(x, y, z, w)
}

#[inline]
pub const fn vec2u(x: u32, y: u32) -> Vec2u {
  Vec2u::new(x, y)
}

#[inline]
pub const fn vec3u(x: u32, y: u32, z: u32) -> Vec3u {
  Vec3u::new(x, y, z)
}

#[inline]
pub const fn vec4u(x: u32, y: u32, z: u32, w: u32) -> Vec4u {
  Vec4u::new(x, y, z, w)
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

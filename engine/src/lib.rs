#![feature(trait_alias)]
#![feature(array_chunks)]

#[macro_use]
extern crate log;

use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use engine::layer::Layer;
pub use engine::{Engine, EngineContext};
pub use log::{debug, error, info, trace, warn};
pub use math::*;
use pretty_env_logger::init;
pub use render::camera::ortho::{OrthoCamera, OrthoCameraController};
pub use {glium, glutin, nalgebra, winit, egui};

#[allow(unused_imports)]
use crate::macros::*;

pub mod app;
pub mod math;
pub mod render;
#[macro_use]
pub mod macros;
pub mod engine;
pub mod update;
pub mod window;

pub mod mesh;
pub mod debug;

pub const VSYNC: bool = true;

pub fn init_logger() {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info");
  }

  init();
}

#[allow(dead_code)]
pub(crate) fn id() -> usize {
  static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

  NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

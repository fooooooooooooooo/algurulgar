use nalgebra::Matrix4;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

use crate::engine::events::EventHandler;
use crate::engine::input::key_pressed;
use crate::math::{Position, Projection, View, ViewProjection};
use crate::update::UpdateHandler;
use crate::vec3;
use crate::window::aspect_ratio;

pub struct OrthoCamera {
  projection: Projection,
  view: View,
  view_projection: ViewProjection,
  pub position: Position,
}

impl OrthoCamera {
  pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
    let projection = Projection::new_orthographic(left, right, bottom, top, -1.0, 1.0);
    let view = View::identity();
    let view_projection = projection * view;

    Self {
      projection,
      view,
      view_projection,
      position: Position::zeros(),
    }
  }

  #[inline]
  pub const fn projection(&self) -> &Projection {
    &self.projection
  }

  #[inline]
  pub const fn view(&self) -> &View {
    &self.view
  }

  #[inline]
  pub const fn view_projection(&self) -> &ViewProjection {
    &self.view_projection
  }

  pub fn set_projection(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
    self.projection = Projection::new_orthographic(left, right, bottom, top, -1.0, 1.0);
    self.view_projection = self.projection * self.view;
  }

  pub fn recalculate_view_matrix(&mut self) {
    let transform = Matrix4::identity().prepend_translation(&vec3(self.position.x, self.position.y, 0.0));

    self.view = transform.try_inverse().unwrap_or_else(Matrix4::zeros);
    self.view_projection = self.projection * self.view;
  }
}

pub struct OrthoCameraController {
  camera: OrthoCamera,
  zoom: f32,
  pub speed: f32,
}

impl OrthoCameraController {
  pub fn new() -> Self {
    let zoom = 1.0;
    let aspect = aspect_ratio();

    debug_assert_ne!(aspect, 0.0, "aspect ratio should never be zero");

    let camera = OrthoCamera::new(-aspect * zoom, aspect * zoom, -zoom, zoom);

    Self {
      camera,
      zoom,
      speed: 1.0,
    }
  }

  #[inline]
  pub const fn camera(&self) -> &OrthoCamera {
    &self.camera
  }

  pub fn set_zoom(&mut self, zoom: f32) {
    self.zoom = zoom.clamp(0.25, 10.0);
    self.update_projection();
  }

  fn update_projection(&mut self) {
    let aspect = aspect_ratio();

    let left = -aspect * self.zoom;
    let mut right = aspect * self.zoom;
    let bottom = -self.zoom;
    let mut top = self.zoom;

    if left == right {
      right += 0.0001;
    }

    if left == bottom {
      top += 0.0001;
    }

    self.camera.set_projection(left, right, bottom, top);
  }
}

impl Default for OrthoCameraController {
  fn default() -> Self {
    Self::new()
  }
}

impl UpdateHandler for OrthoCameraController {
  fn update(&mut self, delta_time: f32) {
    if key_pressed(PhysicalKey::Code(KeyCode::KeyA)) {
      self.camera.position.x -= self.speed * delta_time;
      self.camera.recalculate_view_matrix();
    } else if key_pressed(PhysicalKey::Code(KeyCode::KeyD)) {
      self.camera.position.x += self.speed * delta_time;
      self.camera.recalculate_view_matrix();
    }

    if key_pressed(PhysicalKey::Code(KeyCode::KeyW)) {
      self.camera.position.y += self.speed * delta_time;
      self.camera.recalculate_view_matrix();
    } else if key_pressed(PhysicalKey::Code(KeyCode::KeyS)) {
      self.camera.position.y -= self.speed * delta_time;
      self.camera.recalculate_view_matrix();
    }

    if key_pressed(PhysicalKey::Code(KeyCode::KeyQ)) {
      self.set_zoom(self.zoom + self.speed * delta_time);
      self.update_projection();
    } else if key_pressed(PhysicalKey::Code(KeyCode::KeyE)) {
      self.set_zoom(self.zoom - self.speed * delta_time);
      self.update_projection();
    }
  }
}

impl EventHandler for OrthoCameraController {
  fn handle_event(&mut self, event: &WindowEvent, _window: &Window) {
    match event {
      WindowEvent::MouseWheel { delta, .. } => match delta {
        MouseScrollDelta::LineDelta(_, y) => {
          self.set_zoom(self.zoom + y * self.speed);
          self.update_projection();
        }
        MouseScrollDelta::PixelDelta(delta) => {
          self.set_zoom(self.zoom + delta.y as f32 * self.speed);
          self.update_projection();
        }
      },
      WindowEvent::Resized(_) => {
        self.update_projection();
      }
      _ => (),
    }
  }
}

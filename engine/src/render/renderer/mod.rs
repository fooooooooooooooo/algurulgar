// 3d renderer
// todo: maybe make this a trait and have vulkan and opengl implementations

use glium::{Frame, VertexBuffer};

use super::shader::Shader;
use crate::window::Viewport;
use crate::{OrthoCameraController, ViewProjection};

pub mod mesh;

pub struct Renderer {
  // todo: individual renderers
}

impl Renderer {
  pub fn new() -> Self {
    // todo
    Self {}
  }

  pub fn on_viewport_resize(&mut self, viewport: Viewport) {
    // todo: update renderers
    let _ = viewport;
  }

  /// Initialize the renderer context to begin drawing
  pub fn begin<'a>(&'a mut self, camera: &OrthoCameraController, frame: &'a mut Frame) -> RendererContext<'a> {
    let view_projection = *camera.view_projection();

    RendererContext {
      renderer: self,
      frame,
      view_projection,
    }
  }
}

impl Default for Renderer {
  fn default() -> Self {
    Self::new()
  }
}

pub struct RendererContext<'a> {
  pub renderer: &'a mut Renderer,

  /// The frame that is currently being drawn to
  pub frame: &'a mut Frame,

  /// The view projection matrix that is currently being used
  pub view_projection: ViewProjection,
}

impl<'a> RendererContext<'a> {
  /// Draw vertex buffer
  pub fn draw<V: Copy>(&mut self, vertex_buffer: &VertexBuffer<V>, shader: Box<Shader>) {
    //
  }

  pub fn flush(&mut self) {
    // todo: flush renderers
    // todo: self.renderer.x_renderer.flush();
  }

  /// Finish drawing and flush the renderer
  /// consumes the renderer context
  pub fn finish(mut self) {
    self.flush();
  }
}

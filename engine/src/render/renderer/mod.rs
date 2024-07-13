// 3d renderer
// todo: maybe make this a trait and have vulkan and opengl implementations

use glium::{Display, Frame};
use glutin::surface::WindowSurface;
use mesh::{Mesh, MeshRenderer};
use nalgebra::Matrix4;

use super::shader::Shader;
use crate::window::Viewport;
use crate::{OrthoCameraController, Vec3, ViewProjection};

pub mod mesh;

pub struct Renderer {
  mesh_renderer: MeshRenderer,
}

impl Renderer {
  pub fn new(display: &Display<WindowSurface>, shader: Shader) -> Self {
    Self {
      mesh_renderer: MeshRenderer::new(display, shader),
    }
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

pub struct RendererContext<'a> {
  pub renderer: &'a mut Renderer,

  /// The frame that is currently being drawn to
  pub frame: &'a mut Frame,

  /// The view projection matrix that is currently being used
  pub view_projection: ViewProjection,
}

impl<'a> RendererContext<'a> {
  /// Draw vertex buffer
  pub fn draw(&mut self, position: Vec3, mesh: &Mesh) {
    let translation = Matrix4::identity().prepend_translation(&position);
    let transform = translation;

    self
      .renderer
      .mesh_renderer
      .draw_mesh(self.frame, &self.view_projection, transform, mesh)
  }

  pub fn flush(&mut self) {
    self.renderer.mesh_renderer.flush(self.frame, &self.view_projection);
  }

  /// Finish drawing and flush the renderer
  /// consumes the renderer context
  pub fn finish(mut self) {
    self.flush();
  }
}

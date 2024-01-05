pub mod quad;
pub mod text;

use glium::uniforms::{EmptyUniforms, UniformsStorage};
use glium::{uniform, Display, Frame, IndexBuffer, Surface, VertexBuffer};
use glutin::surface::WindowSurface;

use self::quad::QuadRenderer;
use self::text::{TextParams, TextRenderer};
use super::camera::ortho::OrthoCameraController;
use super::shader::{Shader, DRAW_PARAMETERS};
use crate::math::{Position, Size};

pub const MAX_QUADS: usize = 20000;
pub const MAX_VERTICES: usize = MAX_QUADS * 4;
pub const MAX_INDICES: usize = MAX_QUADS * 6;
pub const MAX_TEXTURE_SLOTS: usize = 32;

// calculate quad indices once
const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];
static QUAD_INDEX_ARRAY: [u16; MAX_INDICES] = {
  let mut indices = [0; MAX_INDICES];

  let mut i = 0;
  while i < MAX_QUADS {
    let start = i * 6;

    let mut j = 0;
    while j < 6 {
      indices[start + j] = QUAD_INDICES[j] + (i * 4) as u16;
      j += 1;
    }

    i += 1;
  }

  indices
};

pub struct Renderer {
  pub shaders: Vec<Shader>,

  quad_renderer: QuadRenderer,
  text_renderer: TextRenderer,
}

impl Renderer {
  pub fn new(display: &Display<WindowSurface>) -> Self {
    let quad_renderer = QuadRenderer::new(display);
    let text_renderer = TextRenderer::new(display);

    Self {
      shaders: vec![],

      quad_renderer,
      text_renderer,
    }
  }

  pub fn add_shader(&mut self, shader: Shader) {
    self.shaders.push(shader);
  }

  pub fn begin<'a>(&'a mut self, camera: &OrthoCameraController, frame: &'a mut Frame) -> RendererContext<'a> {
    let view_projection = camera.camera().view_projection();
    let camera_uniforms = uniform! {
      u_view_projection: *view_projection.as_ref(),
    };

    self.quad_renderer.clear();
    self.text_renderer.clear();

    RendererContext {
      renderer: self,
      frame,
      camera_uniforms,
    }
  }
}

pub struct RendererContext<'a> {
  pub renderer: &'a mut Renderer,
  pub frame: &'a mut Frame,
  pub camera_uniforms: CameraUniforms<'a>,
}

impl<'a> RendererContext<'a> {
  pub fn draw_quad(&mut self, position: Position, size: Size) {
    let shader = &self.renderer.shaders[0];

    self
      .renderer
      .quad_renderer
      .draw_quad(shader, self.frame, &self.camera_uniforms, position, size);
  }

  pub fn draw_text(&mut self, text: &str, position: Position, params: &TextParams) {
    let shader = &self.renderer.shaders[0];

    self
      .renderer
      .text_renderer
      .draw_text(shader, self.frame, &self.camera_uniforms, position, params, text);
  }

  fn flush(&mut self) {
    let shader = &self.renderer.shaders[0];

    self
      .renderer
      .quad_renderer
      .flush(shader, self.frame, &self.camera_uniforms)
  }

  pub fn finish(mut self) {
    self.flush();
  }
}

type CameraUniforms<'a> = UniformsStorage<'a, [[f32; 4]; 4], EmptyUniforms>;

pub(crate) fn copy_and_draw<V: Copy>(
  vertex_buffer: &mut VertexBuffer<V>,
  vertex_array: &mut Vec<V>,
  index_buffer: &IndexBuffer<u16>,
  shader: &Shader,
  frame: &mut Frame,
  camera_uniforms: &CameraUniforms,
) {
  vertex_buffer.invalidate();

  unsafe {
    vertex_buffer
      .slice_mut(0..vertex_array.len())
      .unwrap_unchecked()
      .write(vertex_array.as_slice());
  }

  vertex_buffer
    .slice_mut(0..vertex_array.len())
    .unwrap()
    .write(vertex_array.as_slice());

  frame
    .draw(
      &*vertex_buffer,
      index_buffer,
      shader.program(),
      camera_uniforms,
      &DRAW_PARAMETERS,
    )
    .unwrap();
}

pub mod quad;
pub mod text;

use std::ops::Deref;
use std::rc::Rc;

use glium::uniforms::Uniforms;
use glium::{Display, Frame, IndexBuffer, Surface, VertexBuffer};
use glutin::surface::WindowSurface;

use self::quad::QuadRenderer;
use self::text::{TextParams, TextRenderer};
use super::camera::ortho::OrthoCameraController;
use super::shader::{Shader, DRAW_PARAMETERS};
use super::text::font::FontBitmap;
use crate::math::{Position, Size, ViewProjection};

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
  quad_renderer: QuadRenderer,
  text_renderer: TextRenderer,
}

impl Renderer {
  pub fn new(display: &Display<WindowSurface>, quad_shader: Shader, text_shader: Shader, font: Rc<FontBitmap>) -> Self {
    let quad_renderer = QuadRenderer::new(display, quad_shader);
    let text_renderer = TextRenderer::new(display, text_shader, font);

    Self {
      quad_renderer,
      text_renderer,
    }
  }

  pub fn begin<'a>(&'a mut self, camera: &OrthoCameraController, frame: &'a mut Frame) -> RendererContext<'a> {
    let view_projection = *camera.camera().view_projection();

    self.quad_renderer.clear();
    self.text_renderer.clear();

    RendererContext {
      renderer: self,
      frame,
      view_projection,
    }
  }
}

pub struct RendererContext<'a> {
  pub renderer: &'a mut Renderer,
  pub frame: &'a mut Frame,
  pub view_projection: ViewProjection,
}

impl<'a> RendererContext<'a> {
  #[inline]
  pub fn draw_quad(&mut self, position: Position, size: Size) {
    self
      .renderer
      .quad_renderer
      .draw_quad(self.frame, &self.view_projection, position, size);
  }

  #[inline]
  pub fn draw_text(&mut self, text: &str, position: Position, params: &TextParams) {
    self
      .renderer
      .text_renderer
      .draw_text(self.frame, &self.view_projection, position, params, text);
  }

  fn flush(&mut self) {
    self
      .renderer
      .quad_renderer
      .next_batch(self.frame, &self.view_projection);
    self
      .renderer
      .text_renderer
      .next_batch(self.frame, &self.view_projection);
  }

  pub fn finish(mut self) {
    self.flush();
  }
}

pub(crate) fn copy_and_draw<V: Copy, U: Uniforms>(
  vertex_buffer: &mut VertexBuffer<V>,
  vertex_array: &mut Vec<V>,
  index_buffer: &IndexBuffer<u16>,
  shader: &Shader,
  frame: &mut Frame,
  uniforms: &U,
) {
  if vertex_array.len() == vertex_buffer.len() {
    vertex_buffer.write(vertex_array.as_slice());
  } else {
    vertex_buffer.invalidate();

    unsafe {
      vertex_buffer
        .slice_mut(0..vertex_array.len())
        .unwrap_unchecked()
        .write(vertex_array.as_slice());
    }
  }

  frame
    .draw(
      vertex_buffer.deref(),
      index_buffer,
      shader.program(),
      uniforms,
      &DRAW_PARAMETERS,
    )
    .unwrap();
}

use std::marker::Copy;
use std::rc::Rc;

use glium::index::PrimitiveType;
use glium::uniforms::MagnifySamplerFilter;
use glium::{implement_vertex, uniform, Display, Frame, IndexBuffer, VertexBuffer};
use glutin::surface::WindowSurface;
use nalgebra::Matrix4;

use crate::math::{Position, Size, ViewProjection};
use crate::render::renderer2d::text::font::FontBitmap;
use crate::render::renderer2d::{copy_and_draw, MAX_VERTICES, QUAD_INDEX_ARRAY};
use crate::render::shader::Shader;
use crate::{vec2, vec3, vec4, Color};

pub mod font;

const TAB_SIZE: f32 = 4.0;

#[derive(Default, Copy, Clone)]
pub struct TextVertex {
  position: [f32; 2],
  tex_coords: [f32; 2],
  color: [f32; 4],
}

implement_vertex!(TextVertex, position, tex_coords, color);

pub struct TextRenderer {
  vertex_array: Vec<TextVertex>,
  vertex_buffer: VertexBuffer<TextVertex>,
  index_buffer: IndexBuffer<u16>,
  shader: Shader,
  font: Rc<FontBitmap>,
}

impl TextRenderer {
  pub fn new(display: &Display<WindowSurface>, shader: Shader, font: Rc<FontBitmap>) -> Self {
    let vertex_array = Vec::with_capacity(MAX_VERTICES);
    let vertex_buffer = VertexBuffer::empty_dynamic(display, MAX_VERTICES).unwrap();

    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &QUAD_INDEX_ARRAY).unwrap();

    Self {
      vertex_buffer,
      vertex_array,
      index_buffer,
      shader,
      font,
    }
  }

  pub fn draw_text(
    &mut self,
    frame: &mut Frame,
    view_projection: &ViewProjection,
    position: Position,
    params: &TextParams,
    text: &str,
  ) {
    let position = vec3(position.x, position.y, 0.0);
    let size = vec3(params.scale.x, params.scale.y, 1.0);
    let translation = Matrix4::identity().prepend_translation(&position);
    let scaling = Matrix4::identity().prepend_nonuniform_scaling(&size);
    let transform = translation * scaling;

    self.draw_text_transform(frame, view_projection, transform, params, text);
  }

  fn draw_text_transform(
    &mut self,
    frame: &mut Frame,
    view_projection: &ViewProjection,
    transform: Matrix4<f32>,
    params: &TextParams,
    text: &str,
  ) {
    if text.is_empty() {
      return;
    }

    if self.vertex_array.len() + text.len() * 4 >= MAX_VERTICES {
      self.next_batch(frame, view_projection);
    }

    let mut x = 0.0;
    let mut y = 0.0;

    let width = self.font.cell_size.x as f32;
    let height = self.font.cell_size.y as f32;

    let advance = width * params.scale.x;

    for c in text.chars() {
      if c == '\r' {
        continue;
      }

      if c == '\n' {
        x = 0.0;
        y -= self.font.cell_size.y as f32 * params.scale.y * params.line_spacing;
        continue;
      }

      if c == '\t' {
        x += advance * TAB_SIZE;
        continue;
      }

      let glyph = self.font.get_info(c);

      let tex_coords = glyph.quad_atlas_bounds();

      let mut quad = glyph.quad_plane_bounds(width, height);

      quad.left *= params.scale.x;
      quad.right *= params.scale.x;
      quad.top *= params.scale.y;
      quad.bottom *= params.scale.y;

      quad.left += x;
      quad.right += x;
      quad.top += y;
      quad.bottom += y;

      let positions = [
        transform * vec4(quad.left, quad.top, 0.0, 1.0),
        transform * vec4(quad.left, quad.bottom, 0.0, 1.0),
        transform * vec4(quad.right, quad.bottom, 0.0, 1.0),
        transform * vec4(quad.right, quad.top, 0.0, 1.0),
      ];

      let tex_coords = [
        vec2(tex_coords.left, tex_coords.top),
        vec2(tex_coords.left, tex_coords.bottom),
        vec2(tex_coords.right, tex_coords.bottom),
        vec2(tex_coords.right, tex_coords.top),
      ];

      for i in 0..4 {
        self.vertex_array.push(TextVertex {
          position: [positions[i].x, positions[i].y],
          tex_coords: [tex_coords[i].x, tex_coords[i].y],
          color: params.color.into(),
        });
      }

      x += advance;
    }
  }

  pub fn start_batch(&mut self) {
    self.clear();
  }

  pub fn next_batch(&mut self, frame: &mut Frame, view_projection: &ViewProjection) {
    self.flush(frame, view_projection);
    self.start_batch();
  }

  pub fn flush(&mut self, frame: &mut Frame, view_projection: &ViewProjection) {
    if !self.vertex_array.is_empty() {
      let uniforms = uniform! {
        u_font_atlas: self.font.texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest),
        u_view_projection: *view_projection.as_ref(),
      };

      copy_and_draw(
        &mut self.vertex_buffer,
        &mut self.vertex_array,
        &self.index_buffer,
        &self.shader,
        frame,
        &uniforms,
      );
    }
  }

  pub fn clear(&mut self) {
    unsafe {
      self.vertex_array.set_len(0);
    }
  }
}

pub struct TextParams {
  line_spacing: f32,
  scale: Size,
  color: Color,
}

impl TextParams {
  pub fn new() -> Self {
    Self {
      line_spacing: 1.0,
      scale: vec2(1.0, 1.0),
      color: Color::WHITE,
    }
  }

  pub fn line_spacing(mut self, line_spacing: f32) -> Self {
    self.line_spacing = line_spacing;
    self
  }

  pub fn scale(mut self, scale: f32) -> Self {
    self.scale = vec2(scale, scale);
    self
  }

  pub fn scale_xy(mut self, scale: Size) -> Self {
    self.scale = scale;
    self
  }

  pub fn color(mut self, color: Color) -> Self {
    self.color = color;
    self
  }
}

impl Default for TextParams {
  fn default() -> Self {
    Self::new()
  }
}

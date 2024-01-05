use std::marker::Copy;
use std::rc::Rc;

use glium::index::PrimitiveType;
use glium::{implement_vertex, Display, Frame, IndexBuffer, VertexBuffer};
use glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Vector2};

use crate::math::{Position, Size};
use crate::render::renderer::{copy_and_draw, CameraUniforms, MAX_VERTICES, QUAD_INDEX_ARRAY};
use crate::render::shader::Shader;
use crate::render::text::font::FontBitmap;
use crate::{vec2, vec3, vec4, Color};

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
}

impl TextRenderer {
  pub fn new(display: &Display<WindowSurface>) -> Self {
    let vertex_array = Vec::with_capacity(MAX_VERTICES);
    let vertex_buffer = VertexBuffer::empty_dynamic(display, MAX_VERTICES).unwrap();

    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &QUAD_INDEX_ARRAY).unwrap();

    Self {
      vertex_buffer,
      vertex_array,
      index_buffer,
    }
  }

  pub fn draw_text(
    &mut self,
    shader: &Shader,
    frame: &mut Frame,
    camera_uniforms: &CameraUniforms,
    position: Position,
    params: &TextParams,
    text: &str,
  ) {
    let position = vec3!(position.x, position.y, 0.0);
    let size = vec3!(params.scale.x, params.scale.y, 1.0);
    let translation = Matrix4::identity().prepend_translation(&position);
    let scaling = Matrix4::identity().prepend_nonuniform_scaling(&size);
    let transform = translation * scaling;

    self.draw_text_transform(shader, frame, camera_uniforms, transform, params, text);
  }

  fn draw_text_transform(
    &mut self,
    shader: &Shader,
    frame: &mut Frame,
    camera_uniforms: &CameraUniforms,
    transform: Matrix4<f32>,
    params: &TextParams,
    text: &str,
  ) {
    if text.is_empty() {
      return;
    }

    if self.vertex_array.len() + text.len() * 4 >= MAX_VERTICES {
      self.next_batch(shader, frame, camera_uniforms);
    }

    let mut x = 0.0;
    let mut y = 0.0;

    let width = params.font.cell_size.x as f32;
    let height = params.font.cell_size.y as f32;

    let advance = width * params.scale.x;

    for c in text.chars() {
      if c == '\r' {
        continue;
      }

      if c == '\n' {
        x = 0.0;
        y -= params.font.cell_size.y as f32;
        continue;
      }

      if c == '\t' {
        x += advance * TAB_SIZE;
        continue;
      }

      let glyph = params.font.get_info(c);

      let (al, ab, ar, at) = glyph.quad_atlas_bounds();
      let mut tex_coord_min = vec2!(al, ab);
      let mut tex_coord_max = vec2!(ar, at);

      let (pl, pb, pr, pt) = glyph.quad_plane_bounds(width, height);
      let mut quad_min = vec2!(pl, pb);
      let mut quad_max = vec2!(pr, pt);

      quad_min = quad_min.component_mul(&params.scale);
      quad_max = quad_max.component_mul(&params.scale);

      quad_min += vec2!(x, y);
      quad_max += vec2!(x, y);

      let texel_width = 1.0 / params.font.atlas_size.x as f32;
      let texel_height = 1.0 / params.font.atlas_size.y as f32;
      let texel = vec2!(texel_width, texel_height);

      tex_coord_min = tex_coord_min.component_mul(&texel);
      tex_coord_max = tex_coord_max.component_mul(&texel);

      let positions = [
        transform * vec4!(quad_min.x, quad_min.y, 0.0, 1.0),
        transform * vec4!(quad_min.x, quad_max.y, 0.0, 1.0),
        transform * vec4!(quad_max.x, quad_max.y, 0.0, 1.0),
        transform * vec4!(quad_max.x, quad_min.y, 0.0, 1.0),
      ];

      let tex_coords = [
        tex_coord_min,
        vec2!(tex_coord_min.x, tex_coord_max.y),
        tex_coord_max,
        vec2!(tex_coord_max.x, tex_coord_min.y),
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

  pub fn next_batch(&mut self, shader: &Shader, frame: &mut Frame, camera_uniforms: &CameraUniforms) {
    self.flush(shader, frame, camera_uniforms);
    self.start_batch();
  }

  pub fn flush(&mut self, shader: &Shader, frame: &mut Frame, camera_uniforms: &CameraUniforms) {
    if !self.vertex_array.is_empty() {
      copy_and_draw(
        &mut self.vertex_buffer,
        &mut self.vertex_array,
        &self.index_buffer,
        shader,
        frame,
        camera_uniforms,
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
  font: Rc<FontBitmap>,
  line_spacing: f32,
  scale: Size,
  color: Color,
}

impl TextParams {
  pub fn new(font: Rc<FontBitmap>) -> Self {
    Self {
      font,
      line_spacing: 1.0,
      scale: vec2!(1.0, 1.0),
      color: Color::WHITE,
    }
  }

  pub fn line_spacing(mut self, line_spacing: f32) -> Self {
    self.line_spacing = line_spacing;
    self
  }

  pub fn scale(mut self, scale: f32) -> Self {
    self.scale = vec2!(scale, scale);
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

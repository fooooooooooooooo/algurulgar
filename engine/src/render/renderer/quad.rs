use glium::index::PrimitiveType;
use glium::{implement_vertex, uniform, Display, Frame, IndexBuffer, VertexBuffer};
use glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};

use crate::math::{Position, ViewProjection};
use crate::render::renderer::{copy_and_draw, MAX_VERTICES, QUAD_INDEX_ARRAY};
use crate::render::shader::Shader;
use crate::vec4;

#[derive(Debug, Default, Copy, Clone)]
pub struct QuadVertex {
  position: [f32; 2],
  tex_coords: [f32; 2],
  color: [f32; 4],
}

implement_vertex!(QuadVertex, position, tex_coords, color);

pub struct QuadRenderer {
  vertex_array: Vec<QuadVertex>,
  vertex_buffer: VertexBuffer<QuadVertex>,
  index_buffer: IndexBuffer<u16>,
  shader: Shader,
}

impl QuadRenderer {
  pub fn new(display: &Display<WindowSurface>, shader: Shader) -> Self {
    let vertex_array = Vec::with_capacity(MAX_VERTICES);
    let vertex_buffer = VertexBuffer::empty_dynamic(display, MAX_VERTICES).unwrap();

    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &QUAD_INDEX_ARRAY).unwrap();

    Self {
      vertex_buffer,
      vertex_array,
      index_buffer,
      shader,
    }
  }

  pub fn draw_quad(
    &mut self,
    frame: &mut Frame,
    view_projection: &ViewProjection,
    position: Position,
    size: Vector2<f32>,
  ) {
    let position = Vector3::new(position.x, position.y, 0.0);
    let size = Vector3::new(size.x, size.y, 1.0);
    let translation = Matrix4::identity().prepend_translation(&position);
    let scaling = Matrix4::identity().prepend_nonuniform_scaling(&size);
    let transform = translation * scaling;

    self.draw_quad_transform(frame, view_projection, transform);
  }

  fn draw_quad_transform(&mut self, frame: &mut Frame, view_projection: &ViewProjection, transform: Matrix4<f32>) {
    const QUAD_VERTEX_COUNT: usize = 4;
    const TEX_COORDS: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    const QUAD_VERTEX_POSITIONS: [Vector4<f32>; 4] = [
      vec4!(-0.5, -0.5, 0.0, 1.0),
      vec4!(0.5, -0.5, 0.0, 1.0),
      vec4!(0.5, 0.5, 0.0, 1.0),
      vec4!(-0.5, 0.5, 0.0, 1.0),
    ];

    if self.vertex_array.len() + QUAD_VERTEX_COUNT >= MAX_VERTICES {
      self.next_batch(frame, view_projection);
    }

    for i in 0..QUAD_VERTEX_COUNT {
      let vertex = QuadVertex {
        position: *(transform * QUAD_VERTEX_POSITIONS[i]).xy().as_ref(),
        tex_coords: TEX_COORDS[i],
        color: [1.0, 1.0, 1.0, 1.0],
      };

      self.vertex_array.push(vertex);
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

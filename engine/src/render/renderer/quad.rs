use glium::index::PrimitiveType;
use glium::{implement_vertex, Display, Frame, IndexBuffer, VertexBuffer};
use glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};

use crate::math::Position;
use crate::render::renderer::{copy_and_draw, CameraUniforms, MAX_VERTICES, QUAD_INDEX_ARRAY};
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
}

impl QuadRenderer {
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

  pub fn draw_quad(
    &mut self,
    shader: &Shader,
    frame: &mut Frame,
    camera_uniforms: &CameraUniforms,
    position: Position,
    size: Vector2<f32>,
  ) {
    let position = Vector3::new(position.x, position.y, 0.0);
    let size = Vector3::new(size.x, size.y, 1.0);
    let translation = Matrix4::identity().prepend_translation(&position);
    let scaling = Matrix4::identity().prepend_nonuniform_scaling(&size);
    let transform = translation * scaling;

    self.draw_quad_transform(shader, frame, camera_uniforms, transform);
  }

  fn draw_quad_transform(
    &mut self,
    shader: &Shader,
    frame: &mut Frame,
    camera_uniforms: &CameraUniforms,
    transform: Matrix4<f32>,
  ) {
    const QUAD_VERTEX_COUNT: usize = 4;
    const TEX_COORDS: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    const QUAD_VERTEX_POSITIONS: [Vector4<f32>; 4] = [
      vec4!(-0.5, -0.5, 0.0, 1.0),
      vec4!(0.5, -0.5, 0.0, 1.0),
      vec4!(0.5, 0.5, 0.0, 1.0),
      vec4!(-0.5, 0.5, 0.0, 1.0),
    ];

    if self.vertex_array.len() + QUAD_VERTEX_COUNT >= MAX_VERTICES {
      self.next_batch(shader, frame, camera_uniforms);
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

use glium::{implement_vertex, index::PrimitiveType, Display, IndexBuffer, VertexBuffer};
use glutin::surface::WindowSurface;

use crate::render::shader::Shader;

pub const MAX_TRIS: usize = 20000;
pub const MAX_VERTICES: usize = MAX_TRIS * 3;
pub const MAX_INDICES: usize = MAX_TRIS * 3;

// calculate triangle indices once
const TRI_INDICES: [u16; 3] = [0, 1, 2];
static TRI_INDEX_ARRAY: [u16; MAX_INDICES] = {
  let mut indices = [0; MAX_INDICES];

  let mut i = 0;
  while i < MAX_TRIS {
    let start = i * 3;

    let mut j = 0;
    while j < 3 {
      indices[start + j] = TRI_INDICES[j] + (i * 3) as u16;
      j += 1;
    }

    i += 1;
  }

  indices
};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
  pub position: [f32; 3],
  pub color: [f32; 4],
}

implement_vertex!(Vertex, position, color);

pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>,
}

impl Mesh {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
    Self { vertices, indices }
  }
}

pub struct MeshRenderer {
  vertex_array: Vec<Vertex>,
  vertex_buffer: VertexBuffer<Vertex>,
  index_buffer: IndexBuffer<u16>,
  shader: Box<Shader>,
}

impl MeshRenderer {
  pub fn new(display: &Display<WindowSurface>, shader: Box<Shader>) -> Self {
    let vertex_array = Vec::with_capacity(MAX_VERTICES);
    let vertex_buffer = VertexBuffer::empty_dynamic(display, MAX_VERTICES).unwrap();

    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &TRI_INDEX_ARRAY).unwrap();

    Self {
      vertex_buffer,
      vertex_array,
      index_buffer,
      shader,
    }
  }
}

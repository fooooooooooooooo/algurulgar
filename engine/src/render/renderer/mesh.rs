use std::ops::Deref;

use glium::index::PrimitiveType;
use glium::uniforms::Uniforms;
use glium::{implement_vertex, uniform, Display, Frame, IndexBuffer, Surface, VertexBuffer};
use glutin::surface::WindowSurface;
use nalgebra::Matrix4;
use tobj::LoadOptions;

use crate::render::shader::{Shader, DRAW_PARAMETERS};
use crate::ViewProjection;

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

  pub fn load_obj(obj: &str) -> Self {
    let mut reader = std::io::BufReader::new(obj.as_bytes());
    let (obj, ..) = tobj::load_obj_buf(&mut reader, &LoadOptions::default(), |_| {
      Err(tobj::LoadError::OpenFileFailed)
    })
    .unwrap();

    let obj = obj.first().unwrap();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    const COLORS: [[f32; 4]; 3] = [[1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]];

    for (i, [x, y, z]) in obj.mesh.positions.array_chunks::<3>().enumerate() {
      vertices.push(Vertex {
        position: [*x, *y, *z],
        color: COLORS[i % COLORS.len()],
      });
    }

    for index in &obj.mesh.indices {
      indices.push(*index as u16);
    }

    println!("vertices: {:?}", vertices);
    println!("indices: {:?}", indices);

    Self { vertices, indices }
  }
}

pub struct MeshRenderer {
  vertex_array: Vec<Vertex>,
  vertex_buffer: VertexBuffer<Vertex>,
  index_buffer: IndexBuffer<u16>,
  shader: Shader,
}

impl MeshRenderer {
  pub fn new(display: &Display<WindowSurface>, shader: Shader) -> Self {
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

  pub fn draw_mesh(
    &mut self,
    frame: &mut glium::Frame,
    view_projection: &ViewProjection,
    transform: Matrix4<f32>,
    mesh: &Mesh,
  ) {
    self.vertex_array.clear();
    self.vertex_array.extend_from_slice(&mesh.vertices);

    let uniforms = uniform! {
      u_view_projection: *view_projection.as_ref(),
      u_transform: *transform.as_ref(),
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

  pub fn flush(&mut self, _frame: &mut Frame, _view_projection: &ViewProjection) {
    // if !self.vertex_array.is_empty() {
    //   let uniforms = uniform! {
    //     u_view_projection: *view_projection.as_ref(),
    //   };

    //   copy_and_draw(
    //     &mut self.vertex_buffer,
    //     &mut self.vertex_array,
    //     &self.index_buffer,
    //     &self.shader,
    //     frame,
    //     &uniforms,
    //   );
    // }
  }

  pub fn clear(&mut self) {
    unsafe {
      self.vertex_array.set_len(0);
    }
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

use glium::implement_vertex;

// pub mod batch;
pub mod font;

#[derive(Default, Copy, Clone)]
pub struct Vertex {
  position: [f32; 2],
  tex_coords: [f32; 2],
  color: [f32; 4],
}

implement_vertex!(Vertex, position, tex_coords, color);

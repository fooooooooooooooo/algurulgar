use std::rc::Rc;

use glium::index::{IndexBuffer, PrimitiveType};
use glium::uniforms::MagnifySamplerFilter;
use glium::{program, uniform, Display, Frame, Program, Surface, VertexBuffer};
use glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Vector2};

use crate::math::color::Color;
use crate::math::ViewProjection;
use crate::render::shader::DRAW_PARAMETERS;
use crate::render::text::font::{CharInfo, FontBitmap};
use crate::render::text::Vertex;

const BATCH_SIZE: usize = 64;

const INDICES: [u16; 6] = [
  0, 1, 3, //
  1, 2, 3,
];

pub struct TextRenderer {
  vertices: [Vertex; BATCH_SIZE * 4],
  pub cursor: usize,

  index_buffer: IndexBuffer<u16>,
  vertex_buffer: VertexBuffer<Vertex>,

  view_projection: ViewProjection,

  program: Program,
  font: Rc<FontBitmap>,
}

impl TextRenderer {
  pub fn new(font: Rc<FontBitmap>, display: &Display<WindowSurface>) -> Self {
    let program = program!(
      display,
      100 => {
        vertex: include_str!("../../../shaders/text_vert.glsl"),
        fragment: include_str!("../../../shaders/text_frag.glsl"),
      },
    )
    .unwrap();

    let index_buffer = gen_index_buffer(display);
    let vertex_buffer = VertexBuffer::empty_dynamic(display, BATCH_SIZE * 4).unwrap();

    let view_projection = Matrix4::identity();

    Self {
      vertices: [Vertex::default(); BATCH_SIZE * 4],
      cursor: 0,
      view_projection,

      index_buffer,
      vertex_buffer,

      program,
      font,
    }
  }

  pub fn draw_text(
    &mut self,
    frame: &mut Frame,
    text: &str,
    origin: Vector2<f32>,
    scale: Vector2<f32>,
    color: Color,
    view_projection: &ViewProjection,
  ) {
    self.view_projection = *view_projection;

    let mut x = origin.x;
    let mut y = origin.y;

    for c in text.chars() {
      if c == '\n' {
        x = origin.x;
        y -= self.font.cell_size.y as f32 * scale.y;
        continue;
      }

      let info = self.font.get_info(c);

      self.draw_char(frame, x, y, info, scale, color);

      x += self.font.cell_size.x as f32 * scale.x;
    }

    // could probably get better performance manually flushing at the end of the
    // frame but that is an extra line of code and it looks ugly
    self.flush(frame);
  }

  fn draw_char(&mut self, frame: &mut Frame, x: f32, y: f32, info: CharInfo, scale: Vector2<f32>, color: Color) {
    if self.cursor > BATCH_SIZE - 4 {
      self.flush(frame);
    }

    let x0 = x;
    let y0 = y;
    let x1 = x + self.font.cell_size.x as f32 * scale.x;
    let y1 = y + self.font.cell_size.y as f32 * scale.y;

    let ux0 = info.texture_coords[0][0];
    let uy0 = info.texture_coords[0][1];
    let ux1 = info.texture_coords[1][0];
    let uy1 = info.texture_coords[1][1];

    let i = self.cursor;

    self.vertices[i] = Vertex {
      color: color.into(),
      position: [x1, y0],
      tex_coords: [ux1, uy0],
    };
    self.vertices[i + 1] = Vertex {
      color: color.into(),
      position: [x1, y1],
      tex_coords: [ux1, uy1],
    };
    self.vertices[i + 2] = Vertex {
      color: color.into(),
      position: [x0, y1],
      tex_coords: [ux0, uy1],
    };
    self.vertices[i + 3] = Vertex {
      color: color.into(),
      position: [x0, y0],
      tex_coords: [ux0, uy0],
    };

    self.cursor += 4;
  }

  pub fn flush(&mut self, frame: &mut Frame) {
    // fails because self.vertices[..self.cursor]
    // is not the same size as vertex buffer
    // self.vertex_buffer.write(&self.vertices[..self.cursor]);

    if self.cursor == 0 {
      return;
    }

    if self.cursor == self.vertex_buffer.len() {
      self.vertex_buffer.write(&self.vertices[..self.cursor]);
    } else {
      self.vertex_buffer.invalidate();
      self
        .vertex_buffer
        .slice_mut(0..self.cursor)
        .unwrap()
        .write(&self.vertices[..self.cursor]);
    }

    self.cursor = 0;

    let uniforms = uniform! {
      u_view_projection: *self.view_projection.as_ref(),
      u_texture: self.font.texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest),
    };

    frame
      .draw(
        &self.vertex_buffer,
        &self.index_buffer,
        &self.program,
        &uniforms,
        &DRAW_PARAMETERS,
      )
      .unwrap();
  }
}

fn gen_index_buffer(display: &Display<WindowSurface>) -> IndexBuffer<u16> {
  let elements: Vec<u16> = (0..BATCH_SIZE * 3)
    .map(|i| INDICES[i % 6] + (i / 6 * 4) as u16)
    .collect();

  IndexBuffer::new(display, PrimitiveType::TrianglesList, &elements).unwrap()
}

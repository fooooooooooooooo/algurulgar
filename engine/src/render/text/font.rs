use std::collections::HashMap;
#[cfg(debug_assertions)]
use std::fs;

use bdf::Font;
use glium::texture::{RawImage2d, Texture2d};
use glium::Display;
use glutin::surface::WindowSurface;
use nalgebra::Vector2;

use crate::math::u_sqrt;

const CHAR_START: usize = 32;
const CHAR_END: usize = 127;
const CHARS: usize = CHAR_END - CHAR_START;

const fn generate_font_charset() -> [char; CHARS] {
  let mut chars: [char; CHARS] = ['\0'; CHARS];
  let mut i = 0;

  while i < CHARS {
    chars[i] = (i + CHAR_START) as u8 as char;
    i += 1;
  }

  chars
}

const CHARSET: [char; CHARS] = generate_font_charset();

struct FontAtlas {
  atlas: Vec<u32>,
  cell_size: Vector2<usize>,
  atlas_size: Vector2<usize>,
  char_info: HashMap<char, CharInfo>,
}

#[derive(Default, Debug, Clone)]
pub struct CharInfo {
  pub atlas_position: Vector2<usize>,
  pub texture_coords: [[f32; 2]; 2],
}

pub struct Bounds {
  pub left: f32,
  pub right: f32,
  pub top: f32,
  pub bottom: f32,
}

impl CharInfo {
  pub fn quad_atlas_bounds(&self) -> Bounds {
    let [left, top] = self.texture_coords[0];
    let [right, bottom] = self.texture_coords[1];

    Bounds {
      left,
      right,
      top,
      bottom,
    }
  }

  /// Outputs the bounding box of the glyph as it should be placed on the
  /// baseline
  /// bitmap font so the top left corner is at (0, 0)
  /// and the bottom right corner is at (width, height)
  pub fn quad_plane_bounds(&self, width: f32, height: f32) -> Bounds {
    Bounds {
      left: 0.0,
      right: width,
      top: 0.0,
      bottom: height,
    }
  }
}

fn generate_font_atlas(font: &Font) -> FontAtlas {
  // get largest glyph size
  let mut glyph_cell_width = 0;
  let mut glyph_cell_height = 0;

  for c in CHARSET.iter() {
    let glyph = font.glyphs().get(c).unwrap();

    glyph_cell_width = glyph_cell_width.max(glyph.width() as usize);
    glyph_cell_height = glyph_cell_height.max(glyph.height() as usize);
  }

  let cell_width = glyph_cell_width + 1;
  let cell_height = glyph_cell_height + 1;

  let line_height = cell_height;
  let width = u_sqrt(CHARS) * cell_width + 1;

  let mut height = 0;

  let mut x = 0;
  let mut y = 0;

  let mut char_info = HashMap::with_capacity(CHARS);

  for c in CHARSET.iter() {
    char_info.insert(*c, Vector2::new(x, y));

    x += cell_width;

    if x + cell_width > width {
      x = 0;
      y += line_height;
      height += line_height;
    }
  }

  height += line_height;

  let char_info: HashMap<char, CharInfo> = {
    let width = width as f32;
    let height = height as f32;
    let font_w = cell_width as f32;
    let font_h = cell_height as f32;

    char_info
      .into_iter()
      .map(|(c, pos)| {
        (
          c,
          CharInfo {
            atlas_position: pos,
            texture_coords: get_texture_coords(width, height, pos.x as f32, pos.y as f32, font_w, font_h),
          },
        )
      })
      .collect()
  };

  let mut atlas: Vec<u32> = vec![0; width * height];

  for c in CHARSET.iter() {
    let glyph = font.glyphs().get(c).unwrap_or_else(|| font.glyphs().get(&' ').unwrap());

    let CharInfo { atlas_position, .. } = char_info.get(c).unwrap();

    // offset character down if it is not the same height as the tallest character
    let glyph_height = glyph.height() as usize;
    let y_offset = cell_height - glyph_height;
    // center character horizontally
    let glyph_width = glyph.width() as usize;
    let x_offset = (cell_width - glyph_width) / 2;

    for glyph_x in 0..glyph_width {
      for glyph_y in 0..glyph_height {
        let atlas_x = atlas_position.x + glyph_x + x_offset;
        let atlas_y = atlas_position.y + glyph_y + y_offset;

        let atlas_index = atlas_y * width + atlas_x;

        let pixel = glyph.get(glyph_x as u32, glyph_y as u32);

        if pixel {
          atlas[atlas_index] = 0xFF_FF_FF_FF;
        }
      }
    }
  }

  let font_characters = {
    let mut characters = String::new();

    for c in CHARSET.iter() {
      characters.push(*c);
    }

    characters
  };

  println!("atlas: {}x{}", width, height);
  println!("characters: {}", font_characters);

  FontAtlas {
    atlas,
    char_info,
    cell_size: Vector2::new(cell_width, cell_height),
    atlas_size: Vector2::new(width, height),
  }
}

pub struct FontBitmap {
  pub font: Font,
  pub texture: Texture2d,
  pub atlas_size: Vector2<usize>,
  pub cell_size: Vector2<usize>,
  pub char_info: HashMap<char, CharInfo>,
}

impl FontBitmap {
  pub fn from_bytes(display: &Display<WindowSurface>, bytes: &[u8]) -> Self {
    let font = bdf::read(bytes).unwrap();

    Self::new(display, font)
  }

  pub fn new(display: &Display<WindowSurface>, font: Font) -> Self {
    let FontAtlas {
      atlas,
      cell_size,
      atlas_size,
      char_info,
    } = generate_font_atlas(&font);

    let atlas = atlas.iter().flat_map(|x| x.to_be_bytes()).collect::<Vec<u8>>();

    #[cfg(debug_assertions)]
    fs::write("atlas.bin", &atlas).unwrap();

    let image = RawImage2d::from_raw_rgba(atlas, (atlas_size.x as u32, atlas_size.y as u32));
    let texture = Texture2d::new(display, image).unwrap();

    Self {
      font,
      texture,
      atlas_size,
      cell_size,
      char_info,
    }
  }

  pub fn get_info(&self, c: char) -> CharInfo {
    self.char_info.get(&c).cloned().unwrap_or_default()
  }
}

fn get_texture_coords(w: f32, h: f32, x: f32, y: f32, font_w: f32, font_h: f32) -> [[f32; 2]; 2] {
  let y = y + font_h;

  let x0 = x / w;
  let x1 = (x + font_w) / w;
  let y0 = (y - font_h) / h;
  let y1 = y / h;

  [[x0, y1], [x1, y0]]
}

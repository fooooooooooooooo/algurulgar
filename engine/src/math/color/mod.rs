#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
  pub const BLACK: Self = Self(0.0, 0.0, 0.0, 1.0);
  pub const WHITE: Self = Self(1.0, 1.0, 1.0, 1.0);

  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self(r, g, b, a)
  }

  pub fn from_hex(hex: u32) -> Self {
    let r = ((hex >> 24) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let b = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let a = (hex & 0xFF) as f32 / 255.0;

    Self(r, g, b, a)
  }

  pub fn to_array(&self) -> [f32; 4] {
    [self.0, self.1, self.2, self.3]
  }

  pub fn opacity(&self, arg: f64) -> Color {
    Color(self.0, self.1, self.2, self.3 * arg as f32)
  }
}

impl Default for Color {
  fn default() -> Self {
    Self(0.0, 0.0, 0.0, 1.0)
  }
}

impl From<[f32; 4]> for Color {
  fn from(arr: [f32; 4]) -> Self {
    Self(arr[0], arr[1], arr[2], arr[3])
  }
}

impl From<[f32; 3]> for Color {
  fn from(arr: [f32; 3]) -> Self {
    Self(arr[0], arr[1], arr[2], 1.0)
  }
}

impl From<[u8; 4]> for Color {
  fn from(arr: [u8; 4]) -> Self {
    Self(
      arr[0] as f32 / 255.0,
      arr[1] as f32 / 255.0,
      arr[2] as f32 / 255.0,
      arr[3] as f32 / 255.0,
    )
  }
}

impl From<[u8; 3]> for Color {
  fn from(arr: [u8; 3]) -> Self {
    Self(arr[0] as f32 / 255.0, arr[1] as f32 / 255.0, arr[2] as f32 / 255.0, 1.0)
  }
}

impl From<u32> for Color {
  fn from(hex: u32) -> Self {
    Self::from_hex(hex)
  }
}

impl From<(f32, f32, f32, f32)> for Color {
  fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
    Self(r, g, b, a)
  }
}

impl From<(f32, f32, f32)> for Color {
  fn from((r, g, b): (f32, f32, f32)) -> Self {
    Self(r, g, b, 1.0)
  }
}

impl From<(u8, u8, u8, u8)> for Color {
  fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
    Self(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
  }
}

impl From<(u8, u8, u8)> for Color {
  fn from((r, g, b): (u8, u8, u8)) -> Self {
    Self(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
  }
}

impl From<Color> for [f32; 4] {
  fn from(val: Color) -> Self {
    [val.0, val.1, val.2, val.3]
  }
}

impl From<Color> for [f32; 3] {
  fn from(val: Color) -> Self {
    [val.0, val.1, val.2]
  }
}

impl From<Color> for [u8; 4] {
  fn from(val: Color) -> Self {
    [
      (val.0 * 255.0) as u8,
      (val.1 * 255.0) as u8,
      (val.2 * 255.0) as u8,
      (val.3 * 255.0) as u8,
    ]
  }
}

impl From<Color> for [u8; 3] {
  fn from(val: Color) -> Self {
    [(val.0 * 255.0) as u8, (val.1 * 255.0) as u8, (val.2 * 255.0) as u8]
  }
}

impl From<Color> for u32 {
  fn from(val: Color) -> Self {
    ((val.0 * 255.0) as u32) << 24
      | ((val.1 * 255.0) as u32) << 16
      | ((val.2 * 255.0) as u32) << 8
      | ((val.3 * 255.0) as u32)
  }
}

impl From<Color> for (f32, f32, f32, f32) {
  fn from(val: Color) -> Self {
    (val.0, val.1, val.2, val.3)
  }
}

impl From<Color> for (f32, f32, f32) {
  fn from(val: Color) -> Self {
    (val.0, val.1, val.2)
  }
}

impl From<Color> for (u8, u8, u8, u8) {
  fn from(val: Color) -> Self {
    (
      (val.0 * 255.0) as u8,
      (val.1 * 255.0) as u8,
      (val.2 * 255.0) as u8,
      (val.3 * 255.0) as u8,
    )
  }
}

impl From<Color> for (u8, u8, u8) {
  fn from(val: Color) -> Self {
    ((val.0 * 255.0) as u8, (val.1 * 255.0) as u8, (val.2 * 255.0) as u8)
  }
}

// obj parser because it is easy

use std::io::BufRead;
use std::num::{ParseFloatError, ParseIntError};

use crate::Vec3;

#[derive(Debug)]
#[deprecated = "incomplete implementation, use tobj::load_obj instead"]
pub struct Obj {
  pub name: Option<String>,
  pub vertices: Vec<Vertex>,
  pub edges: Vec<(u16, u16)>,
}

#[derive(Debug)]
pub struct Vertex {
  pub position: Vec3,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("missing name")]
  MissingName,
  #[error("missing x")]
  MissingX,
  #[error("missing y")]
  MissingY,
  #[error("missing z")]
  MissingZ,
  #[error("missing a")]
  MissingA,
  #[error("missing b")]
  MissingB,
  #[error(transparent)]
  ParseFloat(#[from] ParseFloatError),
  #[error(transparent)]
  ParseInt(#[from] ParseIntError),
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

#[allow(deprecated)]
impl Obj {
  pub fn parse<R: BufRead>(reader: R) -> Result<Self, Error> {
    let mut name = None;
    let mut vertices = Vec::new();
    let mut edges = Vec::new();

    for line in reader.lines() {
      let line = line?;
      let mut parts = line.split_whitespace();

      match parts.next() {
        Some("o") => {
          name = Some(parts.next().ok_or(Error::MissingName)?.to_string());
        }
        Some("v") => {
          let x: f32 = parts.next().ok_or(Error::MissingX)?.parse()?;
          let y: f32 = parts.next().ok_or(Error::MissingY)?.parse()?;
          let z: f32 = parts.next().ok_or(Error::MissingZ)?.parse()?;

          vertices.push(Vertex {
            position: Vec3::new(x, y, z),
          });
        }
        Some("l") => {
          let a: u16 = parts.next().ok_or(Error::MissingA)?.parse()?;
          let b: u16 = parts.next().ok_or(Error::MissingB)?.parse()?;

          edges.push((a, b));
        }
        _ => {}
      }
    }

    Ok(Self { name, vertices, edges })
  }
}

use egui::{Margin, Response, RichText, Rounding, Widget};
use nalgebra::{ComplexField, Matrix4, Vector3};

pub struct MatrixWidget<'a> {
  matrix: &'a Matrix4<f32>,
}

impl<'a> MatrixWidget<'a> {
  pub fn new(matrix: &'a Matrix4<f32>) -> Self {
    Self { matrix }
  }
}

impl<'a> Widget for MatrixWidget<'a> {
  // display elements in a 4x4 grid within a frame
  // if the element is 0, display it gray otherwise display it white
  // find longest element in column and space pad all elements to that length
  // format all elements as 0.000
  fn ui(self, ui: &mut egui::Ui) -> Response {
    let matrix = self.matrix.as_slice();

    egui::Frame::default()
      .inner_margin(Margin::symmetric(4.0, 4.0))
      .rounding(Rounding::default())
      .show(ui, |ui| {
        ui.horizontal(|ui| {
          for col in 0..4 {
            // min 7 characters to allow up to -99.999 without shift
            let mut max_len = 7;
            for row in 0..4 {
              let num = matrix[row * 4 + col];
              // skip calculating if number is between -99.999 and 99.999
              if num.abs() < 100.0 {
                continue;
              }
              max_len = max_len.max(num_string_length(num));
            }

            ui.vertical(|ui| {
              for row in 0..4 {
                let num = matrix[row * 4 + col];
                // format number as 0.000
                let text = format!("{:.3}", num);
                // then pad start with spaces until the length is equal to max_len
                let text = format!("{: >1$}", text, max_len);

                let color = if num == 0.0 {
                  egui::Color32::from_gray(128)
                } else {
                  egui::Color32::WHITE
                };

                ui.label(RichText::new(text).color(color).monospace());
              }
            });
          }
        })
      })
      .response
  }
}

// find length of f32 without string conversion
// assume 3 decimal places always, only count the number of digits above 0
fn num_string_length(num: f32) -> usize {
  let sign = num < 0.0;
  let mut num = num.floor().abs() as u32;
  let mut len = 0;

  while num > 0 {
    num /= 10;
    len += 1;
  }

  // 3 decimal places, decimal point, and a sign
  len + 4 + if sign { 1 } else { 0 }
}

pub struct VectorWidget<'a> {
  vector: &'a Vector3<f32>,
}

impl<'a> VectorWidget<'a> {
  pub fn new(vector: &'a Vector3<f32>) -> Self {
    Self { vector }
  }
}

impl<'a> Widget for VectorWidget<'a> {
  fn ui(self, ui: &mut egui::Ui) -> Response {
    let vector = self.vector;

    ui.horizontal(|ui| {
      for i in 0..3 {
        let num = vector[i];
        let text = format!("{:.3}", num);
        let color = if num == 0.0 {
          egui::Color32::from_gray(128)
        } else {
          egui::Color32::WHITE
        };

        ui.label(RichText::new(text).color(color).monospace());
      }
    })
    .response
  }
}

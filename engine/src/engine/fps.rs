use std::time::Instant;

use crate::window::viewport;

pub struct FpsStats {
  last_update: Instant,
  current: u32,
  average: u32,
  max: u32,
  min: u32,
  avg_window: [u32; 10],
  avg_cursor: usize,
  pub text: String,
}

impl FpsStats {
  pub fn new() -> Self {
    Self {
      last_update: Instant::now(),
      current: 0,
      average: 0,
      max: 0,
      min: 0,
      avg_window: [0; 10],
      avg_cursor: 0,
      text: String::new(),
    }
  }

  pub fn update(&mut self, fps: u32) {
    self.current = fps;

    if self.last_update.elapsed().as_secs_f32() > 0.1 {
      if self.current > self.max {
        self.max = self.current;
      }

      if self.current < self.min || self.min == 0 {
        self.min = self.current;
      }

      self.last_update = Instant::now();

      self.update_average();

      self.text = format!(
        "FPS: {: >4} AVG: {: >4} MAX: {: >4} MIN: {: >4}\t{:?}",
        self.current,
        self.average,
        self.max,
        self.min,
        viewport(),
      );
    }
  }

  fn update_average(&mut self) {
    if self.avg_cursor >= self.avg_window.len() {
      self.avg_cursor = 0;
    }

    self.average = self.avg_window.iter().sum::<u32>() / self.avg_window.len() as u32;

    self.avg_window[self.avg_cursor] = self.current;
    self.avg_cursor += 1;
  }
}

impl Default for FpsStats {
  fn default() -> Self {
    Self::new()
  }
}

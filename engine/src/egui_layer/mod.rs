use egui::{Context, ViewportId};
use egui_glium::EguiGlium;
use glium::glutin::surface::WindowSurface;
use glium::Display;
use raw_window_handle::HasDisplayHandle;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct Egui {
  egui_glium: EguiGlium,
}

impl Egui {
  pub fn new(
    viewport_id: ViewportId,
    display: &Display<WindowSurface>,
    window: &Window,
    display_target: &dyn HasDisplayHandle,
  ) -> Self {
    let egui_glium = EguiGlium::new(viewport_id, display, window, display_target);

    Self { egui_glium }
  }

  pub fn handle_window_event(&mut self, event: &WindowEvent, window: &Window) -> bool {
    self.egui_glium.on_event(window, event).consumed
  }

  pub fn run(&mut self, window: &Window, run_ui: impl FnMut(&Context)) {
    self.egui_glium.run(window, run_ui)
  }

  pub fn paint(&mut self, display: &Display<WindowSurface>, frame: &mut glium::Frame) {
    self.egui_glium.paint(display, frame)
  }
}

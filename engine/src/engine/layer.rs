use glium::Frame;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::engine::EngineContext;

pub trait Layer {
  /// Called every frame before draw to update state
  fn update(&mut self, context: &mut EngineContext) {
    let _ = context;
  }

  /// Called every frame after update to draw to display
  fn draw(&mut self, context: &mut EngineContext, display: &mut Frame) {
    let _ = context;
    let _ = display;
  }

  /// Called every frame for egui rendering
  fn egui(&mut self, context: &mut EngineContext, ctx: egui::Context) {
    let _ = context;
    let _ = ctx;
  }

  /// Called when a window event is received
  /// Returns true if the event was consumed
  fn handle_window_event(&mut self, context: &mut EngineContext, event: &WindowEvent, window: &Window) -> bool {
    let _ = context;
    let _ = event;
    let _ = window;

    false
  }
}

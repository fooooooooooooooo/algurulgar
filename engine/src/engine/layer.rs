use glium::Frame;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::engine::EngineContext;

pub trait Layer {
  fn update(&mut self, context: &mut EngineContext);
  fn draw(&mut self, context: &mut EngineContext, display: &mut Frame);
  fn handle_window_event(&mut self, context: &mut EngineContext, event: &WindowEvent, window: &Window);
}

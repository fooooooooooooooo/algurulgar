use winit::event::WindowEvent;
use winit::window::Window;

pub trait EventHandler {
  fn handle_event(&mut self, event: &WindowEvent, window: &Window);
}

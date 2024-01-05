pub mod keyboard;
pub mod mouse;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputState {
  PressedThisFrame,
  ReleasedThisFrame,
  Held,
  None,
}

pub fn update_input_state() {
  keyboard::update_keyboard_state();
  mouse::update_mouse_state();
}

pub use keyboard::*;
pub use mouse::*;

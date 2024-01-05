use std::cell::RefCell;
use std::collections::HashMap;

use winit::event::{ElementState, MouseButton};

use crate::engine::input::InputState;
use crate::math::Position;

thread_local! {
  static MOUSE_STATE: RefCell<HashMap<MouseButton, InputState>> = RefCell::new(HashMap::new());
  static MOUSE_POSITION: RefCell<Position> = RefCell::new(Position::zeros());
}

pub fn set_mouse_state(button: MouseButton, state: ElementState) {
  MOUSE_STATE.with(|mouse_state| {
    let mut mouse_state = mouse_state.borrow_mut();
    let current_state = mouse_state.get(&button).copied().unwrap_or(InputState::None);

    if state == ElementState::Pressed && current_state == InputState::None {
      mouse_state.insert(button, InputState::PressedThisFrame);
    } else if state == ElementState::Released && current_state == InputState::Held {
      mouse_state.insert(button, InputState::ReleasedThisFrame);
    }
  });
}

pub fn mouse_state(button: MouseButton) -> InputState {
  MOUSE_STATE.with(|mouse_state| mouse_state.borrow().get(&button).copied().unwrap_or(InputState::None))
}

pub fn set_mouse_position(pos: Position) {
  MOUSE_POSITION.with(|mouse_position| {
    *mouse_position.borrow_mut() = pos;
  });
}

pub fn mouse_position() -> Position {
  MOUSE_POSITION.with(|mouse_position| *mouse_position.borrow())
}

pub fn button_pressed_this_frame(button: MouseButton) -> bool {
  mouse_state(button) == InputState::PressedThisFrame
}

pub fn button_released_this_frame(button: MouseButton) -> bool {
  mouse_state(button) == InputState::ReleasedThisFrame
}

pub fn button_held(button: MouseButton) -> bool {
  mouse_state(button) == InputState::Held
}

pub fn button_pressed(button: MouseButton) -> bool {
  mouse_state(button) == InputState::PressedThisFrame || mouse_state(button) == InputState::Held
}

pub fn button_released(button: MouseButton) -> bool {
  mouse_state(button) == InputState::ReleasedThisFrame
}

pub(crate) fn update_mouse_state() {
  MOUSE_STATE.with(|mouse_state| {
    let mut mouse_state = mouse_state.borrow_mut();

    for (_, state) in mouse_state.iter_mut() {
      match state {
        InputState::PressedThisFrame => *state = InputState::Held,
        InputState::ReleasedThisFrame => *state = InputState::None,
        _ => {}
      }
    }
  });
}

use std::cell::RefCell;
use std::collections::HashMap;

use winit::event::{ElementState, VirtualKeyCode as VK};

use crate::engine::input::InputState;

thread_local! {
  static KEYBOARD_STATE: RefCell<HashMap<VK, InputState>> = RefCell::new(HashMap::new());
}

static mut LAST_KEY_PRESSED: VK = VK::Key0;

pub fn set_key_state(key: VK, state: ElementState) {
  KEYBOARD_STATE.with(|input_state| {
    let mut input_state = input_state.borrow_mut();
    let current_state = input_state.get(&key).copied().unwrap_or(InputState::None);

    match state {
      ElementState::Pressed if current_state == InputState::None => {
        input_state.insert(key, InputState::PressedThisFrame);
      }
      ElementState::Released if current_state == InputState::Held || current_state == InputState::PressedThisFrame => {
        input_state.insert(key, InputState::ReleasedThisFrame);
      }
      _ => {}
    }
  });

  unsafe {
    LAST_KEY_PRESSED = key;
  }
}

pub fn key_state(key: VK) -> InputState {
  KEYBOARD_STATE.with(|input_state| input_state.borrow().get(&key).copied().unwrap_or(InputState::None))
}

pub fn key_pressed_this_frame(key: VK) -> bool {
  key_state(key) == InputState::PressedThisFrame
}

pub fn key_released_this_frame(key: VK) -> bool {
  key_state(key) == InputState::ReleasedThisFrame
}

pub fn key_held(key: VK) -> bool {
  key_state(key) == InputState::Held
}

pub fn key_pressed(key: VK) -> bool {
  let state = key_state(key);
  state == InputState::PressedThisFrame || state == InputState::Held
}

pub(crate) fn update_keyboard_state() {
  KEYBOARD_STATE.with(|input_state| {
    let mut input_state = input_state.borrow_mut();

    for (_, state) in input_state.iter_mut() {
      match state {
        InputState::PressedThisFrame => *state = InputState::Held,
        InputState::ReleasedThisFrame => *state = InputState::None,
        _ => (),
      }
    }
  });
}

pub fn last_key_pressed() -> VK {
  unsafe { LAST_KEY_PRESSED }
}

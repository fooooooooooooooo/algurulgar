use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};

use winit::event::ElementState;
use winit::keyboard::{KeyCode, NativeKeyCode, PhysicalKey};

use crate::engine::input::InputState;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
  PhyiscalKey(PhysicalKey),
}

impl Key {
  pub fn code(&self) -> Option<KeyCode> {
    match self {
      Key::PhyiscalKey(PhysicalKey::Code(code)) => Some(*code),
      _ => None,
    }
  }

  pub fn native_code(&self) -> Option<NativeKeyCode> {
    match self {
      Key::PhyiscalKey(PhysicalKey::Unidentified(code)) => Some(*code),
      _ => None,
    }
  }
}

impl From<KeyCode> for Key {
  fn from(key_code: KeyCode) -> Self {
    Key::PhyiscalKey(PhysicalKey::Code(key_code))
  }
}

impl From<PhysicalKey> for Key {
  fn from(physical_key: PhysicalKey) -> Self {
    Key::PhyiscalKey(physical_key)
  }
}

impl From<NativeKeyCode> for Key {
  fn from(native_key_code: NativeKeyCode) -> Self {
    Key::PhyiscalKey(PhysicalKey::Unidentified(native_key_code))
  }
}

impl Debug for Key {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Key::PhyiscalKey(PhysicalKey::Code(code)) => write!(f, "Key::Code({:?})", code),
      Key::PhyiscalKey(PhysicalKey::Unidentified(code)) => write!(f, "Key::Unidentified({:?})", code),
    }
  }
}

impl Display for Key {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Key::PhyiscalKey(PhysicalKey::Code(code)) => write!(f, "{:?}", code),
      Key::PhyiscalKey(PhysicalKey::Unidentified(code)) => write!(f, "{:?}", code),
    }
  }
}

thread_local! {
  static KEYBOARD_STATE: RefCell<HashMap<Key, InputState>> = RefCell::new(HashMap::new());
}

// not using option becuase once a key is pressed, it will always have a value
// just using whatever `NativeKeyCode::Unidentified` is as a default
static mut LAST_KEY_PRESSED: Key = Key::PhyiscalKey(PhysicalKey::Unidentified(NativeKeyCode::Unidentified));

pub fn set_key_state(key: Key, state: ElementState) {
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

pub fn key_state<K: Into<Key>>(key: K) -> InputState {
  KEYBOARD_STATE.with(|input_state| {
    input_state
      .borrow()
      .get(&key.into())
      .copied()
      .unwrap_or(InputState::None)
  })
}

pub fn key_pressed_this_frame<K: Into<Key>>(key: K) -> bool {
  key_state(key) == InputState::PressedThisFrame
}

pub fn key_released_this_frame<K: Into<Key>>(key: K) -> bool {
  key_state(key) == InputState::ReleasedThisFrame
}

pub fn key_held<K: Into<Key>>(key: K) -> bool {
  key_state(key) == InputState::Held
}

pub fn key_pressed<K: Into<Key>>(key: K) -> bool {
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

pub fn last_key_pressed() -> Key {
  unsafe { LAST_KEY_PRESSED }
}

use std::collections::HashMap;

use glium::glutin::event::VirtualKeyCode;

pub struct Keyboard {
    keys: HashMap<VirtualKeyCode, bool>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: HashMap::with_capacity(32),
        }
    }

    /// Set the state of the given key
    pub fn set_key(&mut self, key: VirtualKeyCode, pressed: bool) {
        self.keys.insert(key, pressed);
    }

    /// Get if given key is pressed
    pub fn get_key(&self, key: &VirtualKeyCode) -> bool {
        match self.keys.get(key) {
            Some(pressed) => *pressed,
            None => false,
        }
    }

    /// Get if either Left Shift or Right Shift is pressed.
    pub fn get_shift(&self) -> bool {
        self.get_key(&VirtualKeyCode::LShift) || self.get_key(&VirtualKeyCode::RShift)
    }

    /// Get if either Left Windows or Right Windows is pressed.
    pub fn get_win(&self) -> bool {
        self.get_key(&VirtualKeyCode::LWin) || self.get_key(&VirtualKeyCode::RWin)
    }

    /// Get if either Left Alt or Right Alt is pressed.
    pub fn get_alt(&self) -> bool {
        self.get_key(&VirtualKeyCode::LAlt) || self.get_key(&VirtualKeyCode::RAlt)
    }

    /// Get if either Left Control or Right Control is pressed.
    pub fn get_ctrl(&self) -> bool {
        self.get_key(&VirtualKeyCode::LControl) || self.get_key(&VirtualKeyCode::RControl)
    }
}

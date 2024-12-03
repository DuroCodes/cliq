use std::time::Instant;

use enigo::{Button, Enigo, Mouse, Settings};
use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyEventReceiver, GlobalHotKeyManager, HotKeyState,
};

pub struct CliqApp {
    pub hotkey: HotKey,
    pub manager: GlobalHotKeyManager,
    pub enigo: Enigo,
    pub receiver: GlobalHotKeyEventReceiver,
    pub delay_ms: u64,
    pub is_enabled: bool,
    pub last_click: Instant,
    pub is_setting_hotkey: bool,
    pub is_sticky: bool,
    pub is_right_click: bool,
}

impl Default for CliqApp {
    fn default() -> Self {
        Self {
            hotkey: HotKey::new(None, Code::F6),
            manager: GlobalHotKeyManager::new().unwrap(),
            enigo: Enigo::new(&Settings::default()).unwrap(),
            receiver: GlobalHotKeyEvent::receiver().clone(),
            delay_ms: 100,
            is_enabled: false,
            last_click: Instant::now(),
            is_setting_hotkey: false,
            is_sticky: true,
            is_right_click: false,
        }
    }
}

impl CliqApp {
    pub fn check_hotkey(&mut self) {
        if let Ok(event) = self.receiver.try_recv() {
            if event.state == HotKeyState::Pressed && !self.is_setting_hotkey {
                self.is_enabled = !self.is_enabled;
            }
        }
    }

    pub fn handle_click(&mut self) {
        if self.is_enabled {
            let now = Instant::now();
            if now.duration_since(self.last_click).as_millis() >= self.delay_ms as u128 {
                let button = match self.is_right_click {
                    true => Button::Right,
                    false => Button::Left,
                };

                self.enigo.button(button, enigo::Direction::Click).unwrap();
                self.last_click = now;
            }
        }
    }
}

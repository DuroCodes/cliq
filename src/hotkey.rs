use eframe::egui;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};

pub fn egui_key_to_code(key: egui::Key) -> Option<Code> {
    match key {
        egui::Key::A => Some(Code::KeyA),
        egui::Key::B => Some(Code::KeyB),
        egui::Key::C => Some(Code::KeyC),
        egui::Key::D => Some(Code::KeyD),
        egui::Key::E => Some(Code::KeyE),
        egui::Key::F => Some(Code::KeyF),
        egui::Key::G => Some(Code::KeyG),
        egui::Key::H => Some(Code::KeyH),
        egui::Key::I => Some(Code::KeyI),
        egui::Key::J => Some(Code::KeyJ),
        egui::Key::K => Some(Code::KeyK),
        egui::Key::L => Some(Code::KeyL),
        egui::Key::M => Some(Code::KeyM),
        egui::Key::N => Some(Code::KeyN),
        egui::Key::O => Some(Code::KeyO),
        egui::Key::P => Some(Code::KeyP),
        egui::Key::Q => Some(Code::KeyQ),
        egui::Key::R => Some(Code::KeyR),
        egui::Key::S => Some(Code::KeyS),
        egui::Key::T => Some(Code::KeyT),
        egui::Key::U => Some(Code::KeyU),
        egui::Key::V => Some(Code::KeyV),
        egui::Key::W => Some(Code::KeyW),
        egui::Key::X => Some(Code::KeyX),
        egui::Key::Y => Some(Code::KeyY),
        egui::Key::Z => Some(Code::KeyZ),
        egui::Key::F1 => Some(Code::F1),
        egui::Key::F2 => Some(Code::F2),
        egui::Key::F3 => Some(Code::F3),
        egui::Key::F4 => Some(Code::F4),
        egui::Key::F5 => Some(Code::F5),
        egui::Key::F6 => Some(Code::F6),
        egui::Key::F7 => Some(Code::F7),
        egui::Key::F8 => Some(Code::F8),
        egui::Key::F9 => Some(Code::F9),
        egui::Key::F10 => Some(Code::F10),
        egui::Key::F11 => Some(Code::F11),
        egui::Key::F12 => Some(Code::F12),
        // there are theoretically up to F35 😭
        egui::Key::ArrowDown => Some(Code::ArrowDown),
        egui::Key::ArrowLeft => Some(Code::ArrowLeft),
        egui::Key::ArrowRight => Some(Code::ArrowRight),
        egui::Key::ArrowUp => Some(Code::ArrowUp),
        egui::Key::Backslash => Some(Code::Backslash),
        egui::Key::Backspace => Some(Code::Backspace),
        egui::Key::Backtick => Some(Code::Backquote),
        egui::Key::CloseBracket => Some(Code::BracketRight),
        egui::Key::Comma => Some(Code::Comma),
        egui::Key::Colon => Some(Code::Semicolon),
        egui::Key::Copy => Some(Code::Copy),
        egui::Key::Cut => Some(Code::Cut),
        egui::Key::Delete => Some(Code::Delete),
        egui::Key::End => Some(Code::End),
        egui::Key::Enter => Some(Code::Enter),
        egui::Key::Equals => Some(Code::Equal),
        egui::Key::Escape => Some(Code::Escape),
        egui::Key::Home => Some(Code::Home),
        egui::Key::Insert => Some(Code::Insert),
        egui::Key::Minus => Some(Code::Minus),
        egui::Key::Num0 => Some(Code::Digit0),
        egui::Key::Num1 => Some(Code::Digit1),
        egui::Key::Num2 => Some(Code::Digit2),
        egui::Key::Num3 => Some(Code::Digit3),
        egui::Key::Num4 => Some(Code::Digit4),
        egui::Key::Num5 => Some(Code::Digit5),
        egui::Key::Num6 => Some(Code::Digit6),
        egui::Key::Num7 => Some(Code::Digit7),
        egui::Key::Num8 => Some(Code::Digit8),
        egui::Key::Num9 => Some(Code::Digit9),
        egui::Key::PageDown => Some(Code::PageDown),
        egui::Key::PageUp => Some(Code::PageUp),
        egui::Key::Paste => Some(Code::Paste),
        egui::Key::OpenBracket => Some(Code::BracketLeft),
        egui::Key::Period => Some(Code::Period),
        egui::Key::Pipe => Some(Code::Backslash),
        egui::Key::Plus => Some(Code::Equal),
        egui::Key::Questionmark => Some(Code::Slash),
        egui::Key::Quote => Some(Code::Quote),
        egui::Key::Semicolon => Some(Code::Semicolon),
        egui::Key::Slash => Some(Code::Slash),
        egui::Key::Space => Some(Code::Space),
        egui::Key::Tab => Some(Code::Tab),
        _ => None, // there's probably some weird edge case
    }
}

pub fn format_hotkey(hotkey: &HotKey) -> String {
    let modifier_map = [
        (Modifiers::CONTROL, "Ctrl"),
        (Modifiers::SHIFT, "Shift"),
        (Modifiers::ALT, "Alt"),
        (Modifiers::SUPER, "Cmd"),
    ];

    let mut parts = modifier_map
        .iter()
        .filter(|(m, _)| hotkey.mods.contains(*m))
        .map(|(_, name)| *name)
        .collect::<Vec<&str>>();

    let hotkey_name = hotkey.to_string();
    parts.push(match &hotkey.key {
        // alphabetic keys have 'Key' prefix, which is ugly
        Code::KeyA => "A",
        Code::KeyB => "B",
        Code::KeyC => "C",
        Code::KeyD => "D",
        Code::KeyE => "E",
        Code::KeyF => "F",
        Code::KeyG => "G",
        Code::KeyH => "H",
        Code::KeyI => "I",
        Code::KeyJ => "J",
        Code::KeyK => "K",
        Code::KeyL => "L",
        Code::KeyM => "M",
        Code::KeyN => "N",
        Code::KeyO => "O",
        Code::KeyP => "P",
        Code::KeyQ => "Q",
        Code::KeyR => "R",
        Code::KeyS => "S",
        Code::KeyT => "T",
        Code::KeyU => "U",
        Code::KeyV => "V",
        Code::KeyW => "W",
        Code::KeyX => "X",
        Code::KeyY => "Y",
        Code::KeyZ => "Z",
        _ => &hotkey_name, // all other keys have normal names
    });

    parts.join(" + ")
}

#[derive(Default, Clone, PartialEq)]
pub struct KeyStates {
    pub left: bool,
    pub right: bool,
    pub shift: bool,
    pub tab: bool,
    pub enter: bool,
}

impl KeyStates {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            shift: false,
            tab: false,
            enter: false,
        }
    }

    pub fn update_from_key(&mut self, key: dioxus::events::Key, pressed: bool) {
        match key {
            dioxus::events::Key::ArrowLeft => self.left = pressed,
            dioxus::events::Key::ArrowRight => self.right = pressed,
            dioxus::events::Key::Shift => self.shift = pressed,
            dioxus::events::Key::Tab => self.tab = pressed,
            dioxus::events::Key::Enter => self.enter = pressed,

            _ => {}
        }
    }
}

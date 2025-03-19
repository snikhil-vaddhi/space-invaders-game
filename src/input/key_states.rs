use dioxus::events::Key;
/// Tracks the state of keyboard inputs relevant to the game
/// This struct maintains boolean flags for each key that the game responds to,
/// allowing the game logic to easily check which keys are currently pressed.

#[derive(Default, Clone, PartialEq, Debug)]
pub struct KeyStates {
    pub left: bool,
    pub right: bool,
    pub shift: bool,
    pub tab: bool,
    pub enter: bool,
}

/// Creates a new KeyStates instance with all keys unpressed
/// # Returns -> A new KeyStates instance with all boolean flags set to false

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

    /// Updates the state of a specific key based on keyboard events
    /// # Arguments
    /// * `key` - The key that was pressed or released
    /// * `pressed` - Whether the key was pressed (true) or released (false)

    pub fn update_from_key(&mut self, key: dioxus::events::Key, pressed: bool) {
        match key {
            Key::ArrowLeft => self.left = pressed,
            Key::ArrowRight => self.right = pressed,
            Key::Shift => self.shift = pressed,
            Key::Tab => self.tab = pressed,
            Key::Enter => self.enter = pressed,

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::events::Key;

    // Test that a new KeyStates instance has all keys unpressed
    #[test]
    fn test_key_states_new() {
        let key_states = KeyStates::new();
        assert!(!key_states.left);
        assert!(!key_states.right);
        assert!(!key_states.shift);
        assert!(!key_states.tab);
        assert!(!key_states.enter);
    }

    // Test that default() creates an instance with all keys unpressed
    #[test]
    fn test_key_states_default() {
        let key_states = KeyStates::default();
        assert!(!key_states.left);
        assert!(!key_states.right);
        assert!(!key_states.shift);
        assert!(!key_states.tab);
        assert!(!key_states.enter);
    }

    // Test that update_from_key correctly updates key states
    #[test]
    fn test_update_from_key() {
        let mut key_states = KeyStates::default();

        key_states.update_from_key(Key::ArrowLeft, true);
        assert!(key_states.left);

        key_states.update_from_key(Key::ArrowRight, true);
        assert!(key_states.right);

        key_states.update_from_key(Key::Shift, true);
        assert!(key_states.shift);

        key_states.update_from_key(Key::Tab, true);
        assert!(key_states.tab);

        key_states.update_from_key(Key::Enter, true);
        assert!(key_states.enter);

        key_states.update_from_key(Key::ArrowLeft, false);
        assert!(!key_states.left);

        key_states.update_from_key(Key::ArrowRight, false);
        assert!(!key_states.right);
    }

    // Test that unhandled keys don't affect the state
    #[test]
    fn test_unhandled_keys() {
        let mut key_states = KeyStates::default();

        key_states.update_from_key(Key::Backspace, true);

        assert!(!key_states.left);
        assert!(!key_states.right);
        assert!(!key_states.shift);
        assert!(!key_states.tab);
        assert!(!key_states.enter);
    }
}

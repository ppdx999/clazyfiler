use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn is_ctrl_c(key: &KeyEvent) -> bool {
    key.code == KeyCode::Char('c')
        && key.modifiers == KeyModifiers::CONTROL
}

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Up => app.items.previous(),
        KeyCode::Down => app.items.next(),
        KeyCode::Right => app.goto_child(),
        KeyCode::Left => app.goto_parent(),
        KeyCode::Char(' ') => app.select(),
        _ => {}
    };
}

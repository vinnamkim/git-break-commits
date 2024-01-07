use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, CurrentScreen};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::FileNavigator => update_file_navigator(app, key_event),
        CurrentScreen::CommitMessageEditor => {
            update_commit_message_editor(app, key_event)
        }
        CurrentScreen::ErrorMessagePopUp(_, _) => app.close_popup(),
        CurrentScreen::HelpMessagePopUp(_) => app.close_popup(),
    }
}

fn update_file_navigator(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Char('h') | KeyCode::Char('H') => app.open_help_popup(),
        KeyCode::Up => app.items.previous(),
        KeyCode::Down => app.items.next(),
        KeyCode::Right => app.goto_child(),
        KeyCode::Left => app.goto_parent(),
        KeyCode::Char(' ') => app.select(),
        KeyCode::Enter => app.open_editor(),
        _ => {}
    };
}

fn update_commit_message_editor(app: &mut App, key_event: KeyEvent) {
    if key_event.code == KeyCode::Esc {
        app.close_editor();
        return;
    }

    if (key_event.code == KeyCode::Char('w')
        || key_event.code == KeyCode::Char('W'))
        && key_event.modifiers.contains(KeyModifiers::CONTROL)
    {
        app.save_commit().expect("Failed to save commit");
        return;
    }

    app.textarea.input(key_event);
}

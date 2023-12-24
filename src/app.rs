use ratatui::widgets::ListState;

pub enum Action {
    Tick,
    Increment,
    Decrement,
    Quit,
    None,
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn new(items: Vec<T>) -> StatefulList<T> {
        let state = if items.len() > 0 {
            let mut state = ListState::default();
            state.select(Some(0));
            state
        } else {
            ListState::default()
        };

        StatefulList { state, items }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

// Application
#[derive(Debug, Default)]
pub struct App<T> {
    pub items: StatefulList<T>,
    pub should_quit: bool,
}

impl<T> App<T> {
    /// Constructs a new instance of [`App`].
    pub fn new(items: Vec<T>) -> Self {
        App {
            items,
            should_quit: false,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_app_increment_counter() {
//         let mut app = App::default();
//         app.increment_counter();
//         assert_eq!(app.counter, 1);
//     }

//     #[test]
//     fn test_app_decrement_counter() {
//         let mut app = App::default();
//         app.decrement_counter();
//         assert_eq!(app.counter, 0);
//     }
// }

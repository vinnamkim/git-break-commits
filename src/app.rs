use std::ffi::OsString;

use ratatui::widgets::ListState;

use crate::node::{
    ItemMarkable, NameGettable, Node, NodeData, NodeItem, Pointer,
};
use crate::tree::{Tree, TreeError};

pub enum Action {
    Tick,
    Increment,
    Decrement,
    Quit,
    None,
}

#[derive(Debug, Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
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

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

// Application
#[derive(Debug, Default)]
pub struct App<T> {
    pub items: StatefulList<T>,
    pub should_quit: bool,
}

pub struct AppItem(OsString, Pointer<NodeData>);

impl NameGettable for AppItem {
    fn get_name(&self) -> &str {
        self.0.as_os_str().to_str().expect("")
    }
}

impl ItemMarkable for AppItem {
    fn marked(&self) -> bool {
        self.1.marked()
    }
}

impl App<AppItem> {
    /// Constructs a new instance of [`App`].
    pub fn new(root: Pointer<NodeData>) -> Result<Self, TreeError> {
        let items = root
            .borrow()
            .children
            .as_ref()
            .ok_or(TreeError::EmptyTreeError)?
            .iter()
            .map(|item| {
                let key = item.0.to_owned();
                let pointer = item.1.clone();
                AppItem(key, pointer)
            })
            .collect();

        let items = StatefulList::new(items);

        Ok(App {
            items,
            should_quit: false,
        })
    }
}

impl<T> App<T> {
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
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

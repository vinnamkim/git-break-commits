use std::ffi::OsString;
use std::{cmp, path::PathBuf};

use ratatui::widgets::ListState;
use tui_textarea::TextArea;

use crate::tree::{Mark, NodeId, TreeError, TreePtr};

#[derive(Clone)]
pub enum CurrentScreen {
    FileNavigator,
    CommitMessageEditor,
    ErrorMessagePopUp(&'static str, Box<CurrentScreen>),
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

pub struct AppItem {
    pub key: OsString,
    node_id: NodeId,
    tree: TreePtr,
}

impl AppItem {
    pub fn is_directory(&self) -> bool {
        !self.tree.borrow().get_node(self.node_id).is_leaf_node()
    }

    pub fn get_mark(&self) -> Mark {
        self.tree.borrow().get_node(self.node_id).mark
    }
}

// Application
pub struct App<'a> {
    pub tree: TreePtr,
    pub items: StatefulList<AppItem>,
    pub should_quit: bool,
    pub curr_node_id: NodeId,
    pub current_screen: CurrentScreen,
    pub textarea: TextArea<'a>,
}

impl<'a> App<'a> {
    fn get_item_list(tree: TreePtr, node_id: NodeId) -> StatefulList<AppItem> {
        let mut items: Vec<AppItem> = tree
            .borrow()
            .get_node(node_id)
            .children
            .iter()
            .map(|item| AppItem {
                key: item.0.to_owned(),
                node_id: *item.1,
                tree: tree.clone(),
            })
            .collect();

        items.sort_by(|a, b| {
            if a.is_directory() == b.is_directory() {
                a.key.cmp(&b.key)
            } else if a.is_directory() {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        });

        StatefulList::new(items)
    }

    /// Constructs a new instance of [`App`].
    pub fn new(tree: TreePtr) -> Result<Self, TreeError> {
        let curr_node_id = tree.borrow().root_id();
        let items = App::get_item_list(tree.clone(), curr_node_id);

        Ok(Self {
            tree,
            items,
            should_quit: false,
            curr_node_id,
            current_screen: CurrentScreen::FileNavigator,
            textarea: TextArea::default(),
        })
    }

    pub fn goto_child(&mut self) {
        if let Some(selected_item_idx) = self.items.state.selected() {
            let next_node_id = self.items.items[selected_item_idx].node_id;

            if !self.tree.borrow().get_node(next_node_id).is_leaf_node() {
                self.items =
                    App::get_item_list(self.tree.clone(), next_node_id);
                self.curr_node_id = next_node_id;
            }
        }
    }

    pub fn goto_parent(&mut self) {
        if let Some(next_node_id) =
            self.tree.borrow().get_node(self.curr_node_id).parent
        {
            self.items = App::get_item_list(self.tree.clone(), next_node_id);
            self.curr_node_id = next_node_id;
        }
    }

    pub fn select(&mut self) {
        if let Some(selected_item_idx) = self.items.state.selected() {
            let node_id = self.items.items[selected_item_idx].node_id;
            let node_mark = self.tree.borrow().get_node(node_id).mark;
            match node_mark {
                Mark::Unselected | Mark::PartiallySelected => {
                    self.tree.borrow_mut().mark(node_id, Mark::Selected);
                }
                Mark::Selected => {
                    self.tree.borrow_mut().mark(node_id, Mark::Unselected);
                }
            }
        }
    }

    pub fn get_current_path(&self) -> PathBuf {
        self.tree.borrow().get_path_buf(self.curr_node_id)
    }

    pub fn get_stats(&self) -> (usize, usize) {
        let borrowed = self.tree.borrow();

        (borrowed.num_leaf_node, borrowed.num_selected)
    }

    pub fn open_editor(&mut self) {
        let (_, num_selected) = self.get_stats();
        if num_selected == 0 {
            let msg = "You should select more than one file before writing your commit message!";
            self.current_screen = CurrentScreen::ErrorMessagePopUp(
                msg,
                Box::new(self.current_screen.clone()),
            );
        } else {
            self.current_screen = CurrentScreen::CommitMessageEditor;
        }
    }

    pub fn close_editor(&mut self) {
        self.current_screen = CurrentScreen::FileNavigator;
    }

    pub fn save_commit(&mut self) {
        self.current_screen = CurrentScreen::FileNavigator;
    }

    pub fn close_popup(&mut self) {
        if let CurrentScreen::ErrorMessagePopUp(_, prev) =
            self.current_screen.clone()
        {
            self.current_screen = *prev;
        }
    }

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

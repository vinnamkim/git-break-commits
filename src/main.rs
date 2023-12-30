/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

pub mod tree;

use std::{path::PathBuf, str::FromStr};

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};

use ratatui::{backend::CrosstermBackend, Terminal};
use tree::Tree;
use tui::Tui;
use update::update;
// ANCHOR_END: imports_main

// ANCHOR: main
fn main() -> Result<()> {
    let mut tree = Tree::new_ptr();

    let y = PathBuf::from_str("./a/b/file.txt").expect("");
    let x = PathBuf::from_str("./a/b/c/file.txt").expect("");
    let z = PathBuf::from_str("./a/c/file.txt").expect("");
    let z2 = PathBuf::from_str("a/b/c/file2.txt").expect("");

    tree.borrow_mut().add(x);
    tree.borrow_mut().add(y);
    tree.borrow_mut().add(z);
    tree.borrow_mut().add(z2);

    // .
    // └── a
    //     ├── b
    //     │   ├── c
    //     │   │   ├── file.txt
    //     │   │   └── file2.txt
    //     │   └── file.txt
    //     └── c
    //         └── file.txt
    // # of nodes => 8 + 1 (including root)

    // Create an application.
    let mut app = App::new(tree);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

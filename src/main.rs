// ANCHOR: imports_main
// ANCHOR: declare_mods
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

pub mod node;
pub mod tree;

use std::{path::PathBuf, str::FromStr};

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use node::NodeData;
use ratatui::{backend::CrosstermBackend, Terminal};
use tree::Tree;
use tui::Tui;
use update::update;
// ANCHOR_END: imports_main

// ANCHOR: main
fn main() -> Result<()> {
    let tree = Tree::new();

    let x = NodeData::from_str("./a/b/c/file.txt").expect("");
    let y = NodeData::from_str("./a/b/file.txt").expect("");
    let z = NodeData::from_str("./a/c/file.txt").expect("");
    let z2 = NodeData::from_str("./a/b/c/file2.txt").expect("");

    tree.add(x);
    tree.add(y);
    tree.add(z);
    tree.add(z2);

    // Create an application.
    let mut app = App::new(tree.get_root()).expect("true");

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
// ANCHOR_END: main

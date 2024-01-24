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

pub mod git_helper;

use git_helper::{GitCommandError, GitHelper};

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use eyre::eyre;

use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

use clap::Parser;

const ABOUT: &str = r#"
Split Git commits interactively.

This is an interactive CLI tool that executes the following Git commands to break and reconstruct your existing top commits:

$ git reset --soft HEAD~{depth}

The following commands are executed repeatedly until all the reset files are committed:

$ git add {some-selected-files}
$ git commit -m "{msg}"
"#;

#[derive(Parser, Debug)]
#[command(author, version, about = ABOUT)]
struct Args {
    /// Depth of commits to split
    #[arg(short, long, default_value_t = 1)]
    depth: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut git_helper = GitHelper::new(args.depth)?;

    let file_paths = git_helper.list()?;

    // Create an application.
    let mut app = App::new(file_paths)?;

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

    // Apply Git changes
    if app.tree.borrow().num_leaf_node == 0 {
        let mut do_commit = || -> Result<(), GitCommandError> {
            git_helper.checkout_to_temp_branch()?;
            git_helper.reset()?;
            git_helper.commit(&app.commits)?;
            Ok(())
        };

        let result = do_commit();
        git_helper.restore_branch()?;
        result?;

        Ok(())
    } else {
        Err(eyre!("Nothing changed"))
    }
}

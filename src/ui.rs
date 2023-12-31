use crate::app::{App, CurrentScreen};
use crate::tree::Mark;
use ratatui::{prelude::*, widgets::*};

pub fn render(app: &mut App, f: &mut Frame) {
    match app.current_screen {
        CurrentScreen::FileNavigator => render_file_navigator(app, f),
        CurrentScreen::CommitMessageEditor => {
            render_commit_message_editor(app, f)
        }
        CurrentScreen::ErrorMessagePopUp(msg, _) => {
            render_error_message_pop_up(f, msg)
        }
    }
}

pub fn render_file_navigator(app: &mut App, f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Min(3)])
        .split(f.size());

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let prefix = match i.get_mark() {
                Mark::Selected => "☑",
                Mark::Unselected => "☐",
                Mark::PartiallySelected => "⚀",
            };
            let line = if i.is_directory() {
                format!("{} {}/", prefix, i.key.as_os_str().to_str().expect(""))
            } else {
                format!("{} {}", prefix, i.key.as_os_str().to_str().expect(""))
            };
            let lines = vec![Line::from(line)];
            ListItem::new(lines)
                .style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    let path_buf = app.get_current_path();
    let path_str = path_buf
        .as_os_str()
        .to_str()
        .expect("Cannot convert current path to str.");

    let title = format!(" Current path: {} ", path_str);
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    let commit_no = app.commits.len() + 1;
    let (num_total, num_selected) = app.get_stats();
    let text = format!(
        "[Commit {}] # of total files: {}, # of selected files: {}",
        commit_no, num_total, num_selected
    );

    let bottom_widget =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(bottom_widget, chunks[1]);
}

pub fn render_commit_message_editor(app: &mut App, f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Min(3)])
        .split(f.size());
    let title = " Enter commit message ";

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .padding(Padding::new(1, 1, 0, 0));

    app.textarea.set_block(block);
    let widget = app.textarea.widget();

    f.render_widget(widget, chunks[0]);

    let text = "Press 'Esc' to cancel or 'Ctrl + W' to save the message";
    let bottom_widget =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(bottom_widget, chunks[1]);
}

pub fn render_error_message_pop_up(f: &mut Frame, msg: &str) {
    let text = msg;

    let pop_up = Paragraph::new(text).wrap(Wrap { trim: false }).block(
        Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 0, 0))
            .title(" Error! - Press any key to close this pop up - "),
    );

    let area = centered_rect(60, 60, f.size());

    f.render_widget(pop_up, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

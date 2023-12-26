use crate::app::App;
use crate::node::{ItemMarkable, NameGettable};
use ratatui::{prelude::*, widgets::*};

pub fn render(app: &mut App, f: &mut Frame)
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(f.size());

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let prefix = if i.marked() { "☑" } else { "☐" };
            let line = format!("{} {}", prefix, i.get_name());
            let lines = vec![Line::from(line)];
            ListItem::new(lines)
                .style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}

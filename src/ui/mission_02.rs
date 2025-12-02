use crate::levels::mission_02::Mission02State;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &Mission02State, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let status_color = if state.is_finished {
        Color::Green
    } else {
        Color::Red
    };
    f.render_widget(
        Paragraph::new(format!(" RADIO STATUS: {}", state.frequency_output)).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(status_color)),
        ),
        chunks[0],
    );

    let art = vec![
        "         |",
        "        / \\",
        "       /   \\",
        "      |     |",
        "      |  O  |  <-- [RADIO TOWER]",
        "      |     |",
        "     /_______\\",
        "",
        "INSTRUCTIONS:",
        "1. Open 'missions/02_radio.rs'",
        "2. Implement 'get_frequency()' to return 742.5",
        "3. Press [C] to Compile and Tune",
    ];
    let styled_art: Vec<ListItem> = art.iter().map(|s| ListItem::new(Span::raw(*s))).collect();
    f.render_widget(
        List::new(styled_art).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" VISUAL FEED "),
        ),
        chunks[1],
    );
}

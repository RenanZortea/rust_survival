use crate::levels::mission_02::Mission02State;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &Mission02State, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let status_color = if state.is_finished { Color::Green } else { Color::Red };
    
    // Status Header
    f.render_widget(
        Paragraph::new(format!(" WPU-7 STATUS: {}", state.output_log)).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(status_color)),
        ),
        chunks[0],
    );

    // Visual Art & Instructions
    // We use #[rustfmt::skip] to prevent the auto-formatter from breaking the ASCII art alignment
    #[rustfmt::skip]
    let art = vec![
        format!(" SENSORS: Turbidity {:.1} NTU | pH {:.1}", state.turbidity, state.ph),
        String::from(" "),
        String::from("      |~~~|      "),
        String::from("    __|   |__    "),
        String::from("   |  FILTER |   "),
        String::from("   | [=====] |   "),
        String::from("   |_________|   "),
        String::from("     ||   ||     "),
        String::from("    /_______\\    "),
        String::from("   |  PURE   |   "),
        String::from("   |_________|   "),
        String::from(""),
        String::from("MISSION OBJECTIVES:"),
        String::from("1. Open 'missions/02_water.rs'"),
        String::from("2. Calculate correct Chlorine injection"),
        String::from("3. Press [C] to Compile and Inject"),
    ];
    
    let styled_art: Vec<ListItem> = art.iter().map(|s| ListItem::new(Span::raw(s.clone()))).collect();
    
    f.render_widget(
        List::new(styled_art).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" HYDRAULICS CONTROL "),
        ),
        chunks[1],
    );
}

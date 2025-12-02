use crate::app::{App, MenuItem};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

const RUST_ORANGE: Color = Color::Rgb(183, 65, 14);

pub fn render_main_menu(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(8), // Logo
            Constraint::Min(5),    // Menu body
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Logo
    let logo_text = vec![
    "██████╗ ██╗   ██╗███████╗████████╗     ███████╗██╗   ██╗██████╗ ██╗   ██╗██╗██╗   ██╗ █████╗ ██╗     ", 
    "██╔══██╗██║   ██║██╔════╝╚══██╔══╝     ██╔════╝██║   ██║██╔══██╗██║   ██║██║██║   ██║██╔══██╗██║     ",
    "██████╔╝██║   ██║███████╗   ██║        ███████╗██║   ██║██████╔╝██║   ██║██║██║   ██║███████║██║     ",
    "██╔══██╗██║   ██║╚════██║   ██║        ╚════██║██║   ██║██╔══██╗╚██╗ ██╔╝██║╚██╗ ██╔╝██╔══██║██║     ",
    "██║  ██║╚██████╔╝███████║   ██║███████╗███████║╚██████╔╝██║  ██║ ╚████╔╝ ██║ ╚████╔╝ ██║  ██║███████╗",
    "╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝╚══════╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝  ╚═══╝  ╚═╝  ╚═══╝  ╚═╝  ╚═╝╚══════╝",
    ];
    let logo_spans: Vec<Line> = logo_text
        .iter()
        .map(|s| {
            Line::from(Span::styled(
                *s,
                Style::default()
                    .fg(RUST_ORANGE)
                    .add_modifier(Modifier::BOLD),
            ))
        })
        .collect();
    f.render_widget(
        Paragraph::new(logo_spans).alignment(Alignment::Center),
        chunks[0],
    );

    // Menu Content
    let menu_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    let menu_list = MenuItem::all();
    let items: Vec<ListItem> = menu_list
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected_item_index {
                Style::default()
                    .fg(RUST_ORANGE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let prefix = if i == app.selected_item_index {
                ">> "
            } else {
                "   "
            };
            ListItem::new(Line::from(format!("{}{}", prefix, item.label()))).style(style)
        })
        .collect();

    f.render_widget(
        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" SYSTEM MENU "),
        ),
        menu_chunks[0],
    );

    let diag_text = vec![
        Line::from(Span::styled(
            "SYSTEM DIAGNOSTICS:",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("KERNEL: "),
            Span::styled("ONLINE", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("MEMORY: "),
            Span::styled("OK", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("NETWORK: "),
            Span::styled("OFFLINE", Style::default().fg(Color::Red)),
        ]),
    ];
    f.render_widget(
        Paragraph::new(diag_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" STATUS "),
        ),
        menu_chunks[1],
    );

    // Footer
    f.render_widget(
        Paragraph::new(" [UP/DOWN] Select | [ENTER] Confirm ")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray)),
        chunks[2],
    );
}

pub fn render_level_selection(f: &mut Frame, app: &App) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(0)])
        .split(f.area())[0];
    let missions = App::get_mission_list();

    let items: Vec<ListItem> = missions
        .iter()
        .enumerate()
        .map(|(i, (id, title, desc))| {
            let is_selected = i == app.mission_selection_index;
            let header_style = if is_selected {
                Style::default()
                    .fg(RUST_ORANGE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if is_selected { " [X] " } else { " [ ] " };

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(prefix, header_style),
                    Span::styled(format!("MISSION {:02}: {}", id, title), header_style),
                ]),
                Line::from(Span::styled(
                    format!("      {}", desc),
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
            ])
        })
        .collect();

    f.render_widget(
        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(" MISSION SELECT "),
        ),
        area,
    );
}

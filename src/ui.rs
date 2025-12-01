use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, Tabs},
    Frame,
};

use crate::app::{App, CurrentScreen, MenuItem};

const RUST_ORANGE: Color = Color::Rgb(183, 65, 14);
const HUD_GREEN: Color = Color::Rgb(50, 205, 50);

pub fn ui(f: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::MainMenu => render_menu(f, app),
        CurrentScreen::Gameplay => render_gameplay(f, app),
        CurrentScreen::Exiting => {}
    }
}

fn render_menu(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    let title_text = Paragraph::new(" RUST RECLAMATION OS v0.9.1 ")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(RUST_ORANGE)),
        );
    f.render_widget(title_text, chunks[0]);

    let menu_list = MenuItem::all();
    let menu_items: Vec<ListItem> = menu_list
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected_item_index {
                Style::default().bg(RUST_ORANGE).fg(Color::Black)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(item.label())).style(style)
        })
        .collect();

    let list = List::new(menu_items).block(Block::default().borders(Borders::ALL).title(" MENU "));
    f.render_widget(list, chunks[1]);
}

fn render_gameplay(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content (Map or Logs)
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // 1. TABS
    let titles = vec![" [1] NAVIGATION ", " [2] SYSTEM LOGS (DEBUG) "];
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" MODE SELECTION "),
        )
        .select(app.current_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(RUST_ORANGE)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, main_chunks[0]);

    // 2. CONTENT
    match app.current_tab {
        0 => render_map_tab(f, app, main_chunks[1]),
        1 => render_debug_tab(f, app, main_chunks[1]),
        _ => {}
    };

    // 3. FOOTER
    let footer_text = if app.current_tab == 0 {
        " [Arrows] Move | [TAB] Switch View | [C] Compile Code "
    } else {
        " [Up/Down] Scroll Logs | [TAB] Switch View | [C] Compile Code "
    };
    let footer = Paragraph::new(footer_text)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(footer, main_chunks[2]);
}

fn render_map_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // HUD
    let gps_style = if app.gps_output.contains("CRASH") || app.gps_output.contains("ERR") {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(HUD_GREEN).add_modifier(Modifier::BOLD)
    };
    let hud_text = vec![Line::from(vec![
        Span::raw(" STATUS: "),
        Span::styled(
            if app.is_gps_compiled {
                "ONLINE"
            } else {
                "OFFLINE"
            },
            Style::default().fg(if app.is_gps_compiled {
                Color::Green
            } else {
                Color::Red
            }),
        ),
        Span::raw(" | "),
        Span::styled(format!(" READING: {} ", app.gps_output), gps_style),
    ])];
    f.render_widget(
        Paragraph::new(hud_text).block(Block::default().borders(Borders::ALL)),
        chunks[0],
    );

    // GRID
    let mut grid_visuals = Vec::new();
    for y in 0..app.grid_height {
        let mut row_spans = Vec::new();
        for x in 0..app.grid_width {
            if x == app.player_x && y == app.player_y {
                row_spans.push(Span::styled(
                    "@",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ));
            } else if x == app.target_x
                && y == app.target_y
                && app.player_x == x
                && app.player_y == y
            {
                row_spans.push(Span::styled("H", Style::default().fg(Color::Green)));
            } else {
                row_spans.push(Span::styled(".", Style::default().fg(Color::DarkGray)));
            }
            row_spans.push(Span::raw(" "));
        }
        grid_visuals.push(Line::from(row_spans));
    }
    f.render_widget(
        Paragraph::new(grid_visuals)
            .block(Block::default().borders(Borders::ALL).title(" SECTOR MAP "))
            .alignment(Alignment::Center),
        chunks[1],
    );
}

fn render_debug_tab(f: &mut Frame, app: &App, area: Rect) {
    // Get the log text
    let log_content = if let crate::gameplay::MissionStatus::Failed(e) = &app.active_mission.status
    {
        e.clone()
    } else if let crate::gameplay::MissionStatus::Success = &app.active_mission.status {
        "COMPILATION SUCCESSFUL.\n\nFIRMWARE LOADED.\n\nSWITCH TO NAVIGATION TAB TO BEGIN."
            .to_string()
    } else {
        "NO LOGS AVAILABLE. PRESS 'C' TO COMPILE.".to_string()
    };

    let p = Paragraph::new(log_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" COMPILER OUTPUT (SCROLLABLE) "),
        )
        .style(Style::default().fg(Color::Yellow))
        .scroll((app.vertical_scroll, 0)); // Apply the scroll!

    f.render_widget(p, area);

    // Render Scrollbar
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        // FIXED: Removed the '&' borrow here, and imported Margin at the top
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.scroll_state.clone(),
    );
}

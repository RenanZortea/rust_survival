use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, Tabs, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, MenuItem};
use crate::gameplay::GameState;
use crate::levels::mission_01::Mission01State;
use crate::levels::mission_02::Mission02State;

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
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
        .split(f.area());

    let title = Paragraph::new(" RUST SURVIVAL OS ").alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(RUST_ORANGE)));
    f.render_widget(title, chunks[0]);

    let menu_list = MenuItem::all();
    let items: Vec<ListItem> = menu_list.iter().enumerate().map(|(i, item)| {
        let style = if i == app.selected_item_index { Style::default().bg(RUST_ORANGE).fg(Color::Black) } else { Style::default().fg(Color::Gray) };
        ListItem::new(Line::from(item.label())).style(style)
    }).collect();
    f.render_widget(List::new(items).block(Block::default().borders(Borders::ALL).title(" MENU ")), chunks[1]);
}

fn render_gameplay(f: &mut Frame, app: &App) {
    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)]).split(f.area());

    // Display binary size in the title
    let size_info = match app.active_mission.binary_size {
        Some(bytes) => format!(" [BIN: {} bytes]", bytes),
        None => "".to_string(),
    };

    let tabs = Tabs::new(vec![" [1] MISSION ", " [2] LOGS "])
        .block(Block::default().borders(Borders::ALL).title(format!(" MISSION: {}{} ", app.active_mission.title, size_info)))
        .select(app.current_tab)
        .highlight_style(Style::default().fg(RUST_ORANGE).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[0]);

    match app.current_tab {
        0 => match &app.state {
            GameState::Mission01(s) => render_mission_01(f, s, chunks[1]),
            GameState::Mission02(s) => render_mission_02(f, s, chunks[1]),
            _ => {},
        },
        1 => render_logs(f, app, chunks[1]),
        _ => {}
    }

    // Dynamic Footer
    let footer_text = match &app.state {
        GameState::Mission01(s) if s.is_finished => " MISSION COMPLETE. PRESS [ENTER] TO CONTINUE. ",
        GameState::Mission02(s) if s.is_finished => " SIGNAL ESTABLISHED. PRESS [ENTER] TO EXTRACT. ",
        _ => if app.current_tab == 0 { " [Arrows] Move | [C] Compile Code " } else { " [Up/Down] Scroll | [C] Re-Compile " }
    };
    
    // Check if finished to style the footer green
    let is_finished = match &app.state {
        GameState::Mission01(s) => s.is_finished,
        GameState::Mission02(s) => s.is_finished,
        _ => false,
    };

    let footer_style = if is_finished {
        Style::default().bg(Color::Green).fg(Color::Black).add_modifier(Modifier::BOLD)
    } else {
        Style::default().bg(Color::DarkGray).fg(Color::White)
    };

    f.render_widget(Paragraph::new(footer_text).style(footer_style).alignment(Alignment::Center), chunks[2]);
}

fn render_mission_01(f: &mut Frame, state: &Mission01State, area: Rect) {
    let chunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3), Constraint::Min(0)]).split(area);
    
    // Format Runtime
    let runtime_str = match state.last_runtime {
        Some(d) => format!("{:.2?}", d),
        None => "--".to_string(),
    };

    let hud_text = format!(" STATUS: {} | DISTANCE: {} | TIME: {}", 
        if state.is_gps_compiled { "ONLINE" } else { "OFFLINE" }, 
        state.gps_output,
        runtime_str
    );

    f.render_widget(Paragraph::new(hud_text).block(Block::default().borders(Borders::ALL)), chunks[0]);

    let mut lines = Vec::new();
    for y in 0..state.grid_height {
        let mut row = Vec::new();
        for x in 0..state.grid_width {
            if x == state.player_x && y == state.player_y {
                row.push(Span::styled("@", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
            } else if x == state.target_x && y == state.target_y && state.player_x == x && state.player_y == y {
                row.push(Span::styled("H", Style::default().fg(Color::Green)));
            } else {
                row.push(Span::styled(".", Style::default().fg(Color::DarkGray)));
            }
            row.push(Span::raw(" "));
        }
        lines.push(Line::from(row));
    }
    f.render_widget(Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" SECTOR MAP ")).alignment(Alignment::Center), chunks[1]);
}

fn render_mission_02(f: &mut Frame, state: &Mission02State, area: Rect) {
    let chunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3), Constraint::Min(0)]).split(area);

    let status_color = if state.is_finished { Color::Green } else { Color::Red };
    f.render_widget(Paragraph::new(format!(" RADIO STATUS: {}", state.frequency_output))
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(status_color))), chunks[0]);

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
    f.render_widget(List::new(styled_art).block(Block::default().borders(Borders::ALL).title(" VISUAL FEED ")), chunks[1]);
}

fn render_logs(f: &mut Frame, app: &App, area: Rect) {
    let content = app.get_log_content();
    let p = Paragraph::new(content).block(Block::default().borders(Borders::ALL).title(" TERMINAL "))
        .style(Style::default().fg(Color::Yellow)).wrap(Wrap { trim: true }).scroll((app.vertical_scroll, 0));
    f.render_widget(p, area);
    f.render_stateful_widget(Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight), area.inner(Margin{vertical:1,horizontal:0}), &mut app.scroll_state.clone());
}

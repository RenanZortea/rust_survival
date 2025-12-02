use crate::app::App;
use crate::gameplay::GameState;
use crate::ui::mission_01;
use crate::ui::mission_02;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Tabs, Wrap},
    Frame,
};

const RUST_ORANGE: Color = Color::Rgb(183, 65, 14);

pub fn render_gameplay_shell(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // 1. Header / Tabs
    let size_info = match app.active_mission.binary_size {
        Some(bytes) => format!(" [BIN: {} B]", bytes),
        None => "".to_string(),
    };
    let tabs = Tabs::new(vec![" [1] MISSION ", " [2] LOGS "])
        .block(Block::default().borders(Borders::ALL).title(format!(
            " MISSION: {}{} ",
            app.active_mission.title, size_info
        )))
        .select(app.current_tab)
        .highlight_style(
            Style::default()
                .fg(RUST_ORANGE)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, chunks[0]);

    // 2. Main Content (Mission or Logs)
    match app.current_tab {
        0 => match &app.state {
            GameState::Mission01(s) => mission_01::render(f, s, chunks[1]),
            GameState::Mission02(s) => mission_02::render(f, s, chunks[1]),
            _ => {}
        },
        1 => render_logs(f, app, chunks[1]),
        _ => {}
    }

    // 3. Dynamic Footer
    let (footer_text, footer_style) = get_footer_status(app);
    f.render_widget(
        Paragraph::new(footer_text)
            .style(footer_style)
            .alignment(Alignment::Center),
        chunks[2],
    );
}

fn render_logs(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let content = app.get_log_content();
    f.render_widget(
        Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" TERMINAL OUTPUT "),
            )
            .style(Style::default().fg(Color::Yellow))
            .wrap(Wrap { trim: true })
            .scroll((app.vertical_scroll, 0)),
        area,
    );
    f.render_stateful_widget(
        Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight),
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.scroll_state.clone(),
    );
}

fn get_footer_status(app: &App) -> (&'static str, Style) {
    let is_finished = match &app.state {
        GameState::Mission01(s) => s.is_finished,
        GameState::Mission02(s) => s.is_finished,
        _ => false,
    };

    if is_finished {
        (
            " MISSION COMPLETE. PRESS [ENTER] TO CONTINUE. ",
            Style::default()
                .bg(Color::Green)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
    } else if app.current_tab == 0 {
        (
            " [Arrows] Move | [C] Compile Code | [TAB] View Logs ",
            Style::default().bg(Color::DarkGray).fg(Color::White),
        )
    } else {
        (
            " [Up/Down] Scroll Logs | [C] Re-Compile ",
            Style::default().bg(Color::DarkGray).fg(Color::White),
        )
    }
}

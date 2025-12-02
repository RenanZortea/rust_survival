use crate::levels::mission_01::{Mission01State, TileType};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Widget},
    Frame,
};

// --- 🎨 TERRAIN PALETTE ---
const C_GROUND_BG: Color = Color::Rgb(15, 23, 15);
const C_TREE_BG: Color = Color::Rgb(10, 30, 10);
const C_ROCK_BG: Color = Color::Rgb(25, 25, 30);
const C_RUIN_BG: Color = Color::Rgb(40, 35, 30);
const C_PLAYER: Color = Color::Cyan;
const C_TARGET: Color = Color::Green;

pub fn render(f: &mut Frame, state: &Mission01State, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(32), Constraint::Length(30)])
        .split(area);

    // Render the Map using our new efficient Widget
    f.render_widget(MissionMapWidget { state }, chunks[0]);

    // Render the Sidebar (standard widgets)
    render_sidebar(f, state, chunks[1]);
}

// --- 🚀 HIGH PERFORMANCE WIDGET ---
struct MissionMapWidget<'a> {
    state: &'a Mission01State,
}

impl<'a> Widget for MissionMapWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = self.state;

        // 1. Calculate Viewport Size
        // We subtract 2 for borders. max(1) prevents panic on tiny screens.
        // We divide width by 2 because tiles are 2 chars wide.
        let view_width = ((area.width as i32 - 2) / 2).max(1);
        let view_height = (area.height as i32 - 2).max(1);

        // 2. Camera Logic (Centered on Player)
        let max_cam_x = (state.grid_width - view_width).max(0);
        let max_cam_y = (state.grid_height - view_height).max(0);

        let cam_x = (state.player_x - view_width / 2).clamp(0, max_cam_x);
        let cam_y = (state.player_y - view_height / 2).clamp(0, max_cam_y);

        // 3. Render The Container Block first
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .title(format!(" SECTOR MAP [CAM: {},{}] ", cam_x, cam_y));

        let inner_area = block.inner(area);
        block.render(area, buf);

        // 4. Render The Map Logic (Direct Buffer Write)
        // We iterate over the *screen* coordinates we want to fill
        for y in 0..view_height {
            for x in 0..view_width {
                // Calculate World Coordinates
                let map_x = cam_x + x;
                let map_y = cam_y + y;

                // Bounds check (render empty if outside map)
                if map_x >= state.grid_width || map_y >= state.grid_height {
                    continue;
                }

                // Logic Extraction
                let is_player = map_x == state.player_x && map_y == state.player_y;
                let is_target = map_x == state.target_x && map_y == state.target_y;
                let show_house = is_target && (state.is_finished || is_player);
                let tile = state.terrain[map_y as usize][map_x as usize];

                let (symbol, style) = if is_player {
                    (
                        "🤖",
                        Style::default()
                            .bg(C_GROUND_BG)
                            .fg(C_PLAYER)
                            .add_modifier(Modifier::BOLD),
                    )
                } else if show_house {
                    (
                        " H",
                        Style::default()
                            .bg(C_GROUND_BG)
                            .fg(C_TARGET)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    match tile {
                        TileType::Tree => ("🌲", Style::default().bg(C_TREE_BG)),
                        TileType::Rock => ("  ", Style::default().bg(C_ROCK_BG)),
                        TileType::Ruin => ("  ", Style::default().bg(C_RUIN_BG)),
                        TileType::Ground => ("  ", Style::default().bg(C_GROUND_BG)),
                    }
                };

                // 5. Draw to Buffer
                // Calculate screen position:
                // inner_area.x + (x * 2) -> Because every tile is 2 chars wide
                // inner_area.y + y       -> Standard height
                let screen_x = inner_area.x + (x as u16 * 2);
                let screen_y = inner_area.y + y as u16;

                // Safety: Ensure we don't write outside the inner area
                if screen_x + 1 < inner_area.right() && screen_y < inner_area.bottom() {
                    buf.set_string(screen_x, screen_y, symbol, style);
                }
            }
        }
    }
}

// --- SIDEBAR (Standard Implementation) ---
fn render_sidebar(f: &mut Frame, state: &Mission01State, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let coord_text = format!(" POS: [X:{:03} Y:{:03}]", state.player_x, state.player_y);
    f.render_widget(
        Paragraph::new(coord_text)
            .block(Block::default().borders(Borders::ALL).title(" GPS MODULE "))
            .style(Style::default().fg(Color::Cyan)),
        chunks[0],
    );

    let dist = ((state.player_x - state.target_x).pow(2) as f64
        + (state.player_y - state.target_y).pow(2) as f64)
        .sqrt();
    let max_dist = 60.0;
    let signal_strength = (1.0 - (dist / max_dist)).clamp(0.0, 1.0);

    let gauge_color = if signal_strength > 0.8 {
        Color::Green
    } else if signal_strength > 0.4 {
        Color::Yellow
    } else {
        Color::Red
    };

    let label = if state.is_gps_compiled {
        format!("{:.0}% STRENGTH", signal_strength * 100.0)
    } else {
        "NO DRIVER".to_string()
    };

    f.render_widget(
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" SIGNAL "))
            .gauge_style(Style::default().fg(gauge_color).bg(Color::DarkGray))
            .ratio(if state.is_gps_compiled {
                signal_strength
            } else {
                0.0
            })
            .label(label),
        chunks[1],
    );

    let runtime_str = match state.last_runtime {
        Some(d) => format!("{:.2?}", d),
        None => "--".to_string(),
    };
    f.render_widget(
        Paragraph::new(format!(" LATENCY: {}", runtime_str))
            .block(Block::default().borders(Borders::ALL).title(" BENCHMARK ")),
        chunks[2],
    );

    f.render_widget(
        Paragraph::new(format!("> {}", state.gps_output))
            .block(Block::default().borders(Borders::ALL).title(" STDOUT "))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .style(Style::default().fg(Color::DarkGray)),
        chunks[3],
    );
}

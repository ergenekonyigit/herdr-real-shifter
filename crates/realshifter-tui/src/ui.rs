use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame,
};
use realshifter_core::{gear_color, CliProfile, GearPosition};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Profile Bar Tabs
            Constraint::Min(10),   // Gear Grid Table
            Constraint::Length(4), // Status Bar & Help
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_profile_bar(f, app, chunks[1]);
    draw_gear_grid(f, app, chunks[2]);
    draw_status_bar(f, app, chunks[3]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_text = vec![Line::from(vec![
        Span::styled(
            " RealShifter ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "v0.1.0 ",
            Style::default().fg(Color::DarkGray),
        ),
        Span::raw(" | Herdr Plugin | Active Profile: "),
        Span::styled(
            format!("{} {}", app.config.active_profile.icon_symbol(), app.config.active_profile.display_name()),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | Current Gear: "),
        Span::styled(
            format!(" [{}] ", app.state.current_gear.display_name()),
            Style::default()
                .fg(gear_color(app.state.current_gear))
                .add_modifier(Modifier::BOLD),
        ),
    ])];

    let header = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" RealShifter Dashboard "),
    );

    f.render_widget(header, area);
}

fn draw_profile_bar(f: &mut Frame, app: &App, area: Rect) {
    let profiles = CliProfile::all();
    let titles: Vec<Line> = profiles
        .iter()
        .map(|p| {
            Line::from(format!("{} {}", p.icon_symbol(), p.display_name()))
        })
        .collect();

    let selected_index = profiles
        .iter()
        .position(|p| *p == app.config.active_profile)
        .unwrap_or(0);

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" CLI Profile Preset [Press 'p' to cycle] "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )
        .select(selected_index);

    f.render_widget(tabs, area);
}

fn draw_gear_grid(f: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["Gear", "Action Type", "Label", "Command", "Shifts", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let active_profile = app.config.active_profile;

    let rows = GearPosition::all().iter().map(|gear| {
        let is_current = *gear == app.state.current_gear;
        let mapping = app.config.get_mapping(active_profile, *gear);

        let gear_cell = Cell::from(format!("  {}  ", gear.display_name()))
            .style(Style::default().fg(gear_color(*gear)).add_modifier(Modifier::BOLD));

        let action_cell = match mapping {
            Some(ref m) => Cell::from(format!("{} {}", m.action_type.icon_symbol(), m.action_type.display_name())),
            None => Cell::from("—"),
        };

        let label_cell = match mapping {
            Some(ref m) => Cell::from(m.display_label()),
            None => Cell::from("—"),
        };

        let command_cell = match mapping {
            Some(ref m) => Cell::from(m.effective_command()),
            None => Cell::from("—"),
        };

        let count = app.state.shift_counts.get(gear).copied().unwrap_or(0);
        let count_cell = Cell::from(count.to_string());

        let status_cell = if is_current {
            Cell::from(" 🟢 ENGAGED ").style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        } else {
            Cell::from("  idle  ").style(Style::default().fg(Color::DarkGray))
        };

        let row_style = if is_current {
            Style::default().bg(Color::Rgb(20, 40, 30))
        } else {
            Style::default()
        };

        Row::new(vec![
            gear_cell,
            action_cell,
            label_cell,
            command_cell,
            count_cell,
            status_cell,
        ])
        .style(row_style)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Length(18),
            Constraint::Length(24),
            Constraint::Min(25),
            Constraint::Length(8),
            Constraint::Length(14),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Gear Mapping Status Grid "),
    );

    f.render_widget(table, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let last_action_text = app
        .state
        .last_action
        .as_deref()
        .unwrap_or("None");

    let text = vec![
        Line::from(vec![
            Span::raw("Last Action: "),
            Span::styled(last_action_text, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" | Total Shifts: "),
            Span::styled(app.state.total_shifts.to_string(), Style::default().fg(Color::Yellow)),
            Span::raw(" | Status: "),
            Span::styled(&app.status_message, Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Controls: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("[1-6]: Shift Gear 1-6  |  [r]: Reverse  |  [n]: Neutral  |  [p]: Cycle Profile  |  [q/Esc]: Exit"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Status & Controls "),
    );

    f.render_widget(paragraph, area);
}

use crate::app::{App, EditField};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs},
};
use realshifter_core::{CliProfile, GearPosition, Theme};

pub fn draw(f: &mut Frame, app: &mut App) {
    let theme = app.theme();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Profile Bar Tabs
            Constraint::Min(10),   // Gear Grid Table
            Constraint::Length(4), // Status Bar & Controls
        ])
        .split(f.area());

    draw_header(f, app, &theme, chunks[0]);
    draw_profile_bar(f, app, &theme, chunks[1]);
    draw_gear_grid(f, app, &theme, chunks[2]);
    draw_status_bar(f, app, &theme, chunks[3]);

    if app.show_models_modal {
        draw_models_modal(f, app, &theme);
    } else if app.show_help_modal {
        draw_help_modal(f, app, &theme);
    } else if app.edit_state.is_some() {
        draw_edit_modal(f, app, &theme);
    }
}

fn draw_header(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let is_same_active = app.view_profile == app.active_profile;
    let active_status_span = if is_same_active {
        Span::styled(
            format!(" 🟢 ACTIVE: {} ", app.active_profile.display_name()),
            theme.active_badge,
        )
    } else {
        Span::styled(
            format!(
                " 👁️ VIEWING: {} (Active: {}) ",
                app.view_profile.display_name(),
                app.active_profile.display_name()
            ),
            theme.viewing_badge,
        )
    };

    let theme_badge = format!(" [Theme: {}] ", app.config.theme_mode.display_name());

    let header_text = vec![Line::from(vec![
        Span::styled(" RealShifter ", theme.title),
        Span::styled("v0.1.0 ", theme.secondary_text),
        Span::styled(theme_badge, theme.secondary_text),
        Span::raw(" | "),
        active_status_span,
        Span::raw(" | Current Gear: "),
        Span::styled(
            format!(" [{}] ", app.state.current_gear.display_name()),
            Style::default()
                .fg(theme.gear_color(app.state.current_gear))
                .add_modifier(Modifier::BOLD),
        ),
    ])];

    let header = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.header_border))
            .title(" RealShifter Dashboard "),
    );

    f.render_widget(header, area);
}

fn draw_profile_bar(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let profiles = CliProfile::all();
    let titles: Vec<Line> = profiles
        .iter()
        .map(|p| {
            let active_mark = if *p == app.active_profile {
                "🟢 "
            } else {
                ""
            };
            Line::from(format!(
                "{}{} {}",
                active_mark,
                p.icon_symbol(),
                p.display_name()
            ))
        })
        .collect();

    let selected_index = profiles
        .iter()
        .position(|p| *p == app.view_profile)
        .unwrap_or(0);

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border))
                .title(" Profile View [Press 'h/l' or 'p' to switch | 'Space' to set Active] "),
        )
        .highlight_style(theme.tab_highlight)
        .select(selected_index);

    f.render_widget(tabs, area);
}

fn draw_gear_grid(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let header_cells = [
        "",
        "Gear",
        "Action Type",
        "Label",
        "Command / Flag",
        "Shifts",
        "Status",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(theme.header_cell));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let active_profile = app.view_profile;

    let rows = GearPosition::all().iter().enumerate().map(|(idx, gear)| {
        let is_selected = idx == app.selected_gear_index;
        let is_current = *gear == app.state.current_gear;
        let mapping = app.config.get_mapping(active_profile, *gear);

        let pointer_cell = if is_selected {
            Cell::from("▶").style(theme.title)
        } else {
            Cell::from(" ")
        };

        let gear_cell = Cell::from(format!(" {} ", gear.display_name())).style(
            Style::default()
                .fg(theme.gear_color(*gear))
                .add_modifier(Modifier::BOLD),
        );

        let action_cell = match mapping {
            Some(ref m) => Cell::from(format!(
                "{} {}",
                m.action_type.icon_symbol(),
                m.action_type.display_name()
            )),
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
            Cell::from(" 🟢 ENGAGED ").style(theme.active_badge)
        } else {
            Cell::from("  idle  ").style(theme.idle_badge)
        };

        let row_style = if is_selected {
            theme.selected_row
        } else if is_current {
            theme.current_row
        } else {
            theme.normal_row
        };

        Row::new(vec![
            pointer_cell,
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
            Constraint::Length(2),
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
            .border_style(Style::default().fg(theme.border))
            .title(" Gear Mapping Status Grid [Use 'j/k' to select, 'Enter/e' to edit] "),
    );

    f.render_widget(table, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let last_action_text = app.state.last_action.as_deref().unwrap_or("None");

    let text = vec![
        Line::from(vec![
            Span::raw("Last Action: "),
            Span::styled(last_action_text, theme.accent_cyan),
            Span::raw(" | Total Shifts: "),
            Span::styled(app.state.total_shifts.to_string(), theme.title),
            Span::raw(" | Status: "),
            Span::styled(&app.status_message, theme.accent_green),
        ]),
        Line::from(vec![
            Span::styled("Controls: ", theme.title),
            Span::styled("[j/k]:", theme.accent_cyan),
            Span::raw(" Nav | "),
            Span::styled("[h/l/p]:", theme.accent_cyan),
            Span::raw(" Tab | "),
            Span::styled("[Space]:", theme.accent_cyan),
            Span::raw(" Active | "),
            Span::styled("[e/Enter]:", theme.accent_cyan),
            Span::raw(" Edit | "),
            Span::styled("[t]:", theme.accent_cyan),
            Span::raw(" Theme | "),
            Span::styled("[1-6/r/n]:", theme.accent_cyan),
            Span::raw(" Shift | "),
            Span::styled("[m]:", theme.accent_cyan),
            Span::raw(" Models | "),
            Span::styled("[?]:", theme.accent_cyan),
            Span::raw(" Help | "),
            Span::styled("[q]:", theme.accent_cyan),
            Span::raw(" Exit"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title(" Status & Controls "),
    );

    f.render_widget(paragraph, area);
}

fn draw_edit_modal(f: &mut Frame, app: &App, theme: &Theme) {
    let es = match app.edit_state.as_ref() {
        Some(s) => s,
        None => return,
    };

    let area = f.area();
    let modal_width = 72;
    let modal_height = 18;

    let popup_area = Rect {
        x: area.width.saturating_sub(modal_width) / 2,
        y: area.height.saturating_sub(modal_height) / 2,
        width: modal_width.min(area.width),
        height: modal_height.min(area.height),
    };

    f.render_widget(Clear, popup_area);

    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.modal_border))
        .style(theme.normal_row)
        .title(format!(
            " Edit Mapping: {} ({}) ",
            es.gear.full_name(),
            app.view_profile.display_name()
        ));

    f.render_widget(main_block, popup_area);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // Action Type
            Constraint::Length(2), // Model ID
            Constraint::Length(2), // Effort Level
            Constraint::Length(2), // Custom Command / Flag
            Constraint::Length(2), // Label
            Constraint::Length(3), // Save / Cancel buttons
        ])
        .split(popup_area);

    let field_style = |field: EditField| {
        if es.focused_field == field {
            theme.modal_field_focused
        } else {
            theme.modal_field_unfocused
        }
    };

    // 1. Action Type
    let action_str = format!(
        " {} {}",
        es.action_type.icon_symbol(),
        es.action_type.display_name()
    );
    let p_action = Paragraph::new(Line::from(vec![
        Span::styled("Action Type: ", field_style(EditField::ActionType)),
        Span::styled(action_str, theme.accent_cyan),
        Span::styled(" (Use Left/Right to cycle)", theme.secondary_text),
    ]));
    f.render_widget(p_action, inner_chunks[0]);

    // 2. Model ID
    let model_display = if es.selected_model_id.is_empty() {
        " [None / Default] ".to_string()
    } else {
        format!(" {} ", es.selected_model_id)
    };
    let p_model = Paragraph::new(Line::from(vec![
        Span::styled("Model Target: ", field_style(EditField::Model)),
        Span::styled(model_display, theme.accent_green),
        Span::styled(
            " (Use Left/Right to cycle from snapshot)",
            theme.secondary_text,
        ),
    ]));
    f.render_widget(p_model, inner_chunks[1]);

    // 3. Effort Level
    let effort_display = if let Some(eff) = es.selected_effort {
        format!(" {} ", eff.display_name())
    } else {
        " [N/A or None] ".to_string()
    };
    let p_effort = Paragraph::new(Line::from(vec![
        Span::styled("Reasoning Effort: ", field_style(EditField::Effort)),
        Span::styled(effort_display, theme.accent_magenta),
        Span::styled(" (Low / Medium / High)", theme.secondary_text),
    ]));
    f.render_widget(p_effort, inner_chunks[2]);

    // 4. Custom Command
    let cmd_display = if es.custom_command.is_empty() {
        " <None - Use action default> ".to_string()
    } else {
        es.custom_command.clone()
    };
    let p_cmd = Paragraph::new(Line::from(vec![
        Span::styled("Custom Command: ", field_style(EditField::CustomCommand)),
        Span::styled(cmd_display, theme.primary_text),
    ]));
    f.render_widget(p_cmd, inner_chunks[3]);

    // 5. Label
    let label_display = if es.label.is_empty() {
        " <Auto-generated label> ".to_string()
    } else {
        es.label.clone()
    };
    let p_label = Paragraph::new(Line::from(vec![
        Span::styled("Display Label: ", field_style(EditField::Label)),
        Span::styled(label_display, theme.accent_cyan),
    ]));
    f.render_widget(p_label, inner_chunks[4]);

    // 6. Buttons
    let save_style = if es.focused_field == EditField::Save {
        theme.modal_save_focused
    } else {
        theme.modal_save_unfocused
    };
    let cancel_style = if es.focused_field == EditField::Cancel {
        theme.modal_cancel_focused
    } else {
        theme.modal_cancel_unfocused
    };

    let btn_line = Line::from(vec![
        Span::styled(" [ SAVE (Enter) ] ", save_style),
        Span::raw("    "),
        Span::styled(" [ CANCEL (Esc) ] ", cancel_style),
        Span::styled(
            "     (Use Tab/Up/Down to navigate fields)",
            theme.secondary_text,
        ),
    ]);
    let p_btn = Paragraph::new(btn_line);
    f.render_widget(p_btn, inner_chunks[5]);
}

fn draw_help_modal(f: &mut Frame, _app: &App, theme: &Theme) {
    let area = f.area();
    let modal_width = 75;
    let modal_height = 19;

    let popup_area = Rect {
        x: area.width.saturating_sub(modal_width) / 2,
        y: area.height.saturating_sub(modal_height) / 2,
        width: modal_width.min(area.width),
        height: modal_height.min(area.height),
    };

    f.render_widget(Clear, popup_area);

    let rows = vec![
        Row::new(vec![
            Cell::from("j / k / Up / Down").style(theme.accent_cyan),
            Cell::from("Select gear row in mapping grid").style(theme.primary_text),
        ]),
        Row::new(vec![
            Cell::from("h / l / p / Left / Right").style(theme.accent_cyan),
            Cell::from("Cycle profile view tabs (AGY, Claude, Codex, OpenCode, Custom)")
                .style(theme.primary_text),
        ]),
        Row::new(vec![
            Cell::from("Space / a").style(theme.accent_cyan),
            Cell::from("Set currently viewed profile as Global Active Profile")
                .style(theme.primary_text),
        ]),
        Row::new(vec![
            Cell::from("e / Enter").style(theme.accent_cyan),
            Cell::from("Edit mapping for selected gear (opens Interactive Mapping Editor)")
                .style(theme.primary_text),
        ]),
        Row::new(vec![
            Cell::from("t / T").style(theme.accent_cyan),
            Cell::from("Toggle Theme (Auto / Dark / Light)").style(theme.primary_text),
        ]),
        Row::new(vec![
            Cell::from("1 - 6, r, n").style(theme.accent_cyan),
            Cell::from("Simulate hardware gear shift (Gear 1-6, Reverse, Neutral)")
                .style(theme.primary_text),
        ]),
        Row::new(vec![
            Cell::from("m").style(theme.accent_cyan),
            Cell::from("Toggle available LLM models snapshot modal").style(theme.primary_text),
        ]),
        Row::new(vec![
            Cell::from("?").style(theme.accent_cyan),
            Cell::from("Toggle this Keybindings & Help modal").style(theme.primary_text),
        ]),
        Row::new(vec![
            Cell::from("q / Esc").style(theme.accent_cyan),
            Cell::from("Close modal or Quit RealShifter TUI").style(theme.primary_text),
        ]),
    ];

    let header_cells = ["Keybinding", "Action Description"]
        .iter()
        .map(|h| Cell::from(*h).style(theme.header_cell));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let table = Table::new(
        rows,
        [Constraint::Length(28), Constraint::Min(40)],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.modal_border))
            .style(theme.normal_row)
            .title(" RealShifter TUI Keybindings & Ergonomics Guide (Press '?' or 'Esc' to close) "),
    );

    f.render_widget(table, popup_area);
}

fn draw_models_modal(f: &mut Frame, app: &App, theme: &Theme) {
    let area = f.area();
    let modal_width = 75;
    let modal_height = 16;

    let popup_area = Rect {
        x: area.width.saturating_sub(modal_width) / 2,
        y: area.height.saturating_sub(modal_height) / 2,
        width: modal_width.min(area.width),
        height: modal_height.min(area.height),
    };

    f.render_widget(Clear, popup_area);

    let meta_date = app
        .config
        .profiles
        .get(&app.view_profile)
        .and_then(|p| p.metadata.as_ref())
        .map(|m| m.generated_at.as_str())
        .unwrap_or("2026-08-19");
    let models = app.config.available_models(app.view_profile);

    let header_cells = ["Model ID", "Model Name", "Supported Effort Levels"]
        .iter()
        .map(|h| Cell::from(*h).style(theme.header_cell));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = models.iter().map(|m| {
        let id_cell = Cell::from(m.id.as_str()).style(theme.accent_green);
        let name_cell = Cell::from(m.name.as_str()).style(theme.primary_text);
        let effort_str = if m.effort_levels.is_empty() {
            "N/A".to_string()
        } else {
            m.effort_levels.join(", ")
        };
        let effort_cell = Cell::from(effort_str).style(theme.accent_cyan);

        Row::new(vec![id_cell, name_cell, effort_cell])
    });

    let title_str = format!(
        " Available LLM Models Snapshot ({}) - Press 'Esc' or 'm' to close ",
        meta_date
    );

    let table = Table::new(
        rows,
        [
            Constraint::Length(25),
            Constraint::Length(28),
            Constraint::Min(16),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.modal_border))
            .style(theme.normal_row)
            .title(title_str),
    );

    f.render_widget(table, popup_area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use realshifter_core::ThemeMode;

    #[test]
    fn test_ui_draw_all_views_dark_and_light() {
        for mode in [ThemeMode::Dark, ThemeMode::Light, ThemeMode::Auto] {
            let backend = TestBackend::new(120, 40);
            let mut terminal = Terminal::new(backend).unwrap();

            let mut app = App::new();
            app.config.theme_mode = mode;
            terminal.draw(|f| draw(f, &mut app)).unwrap();

            app.show_models_modal = true;
            terminal.draw(|f| draw(f, &mut app)).unwrap();

            app.show_models_modal = false;
            app.show_help_modal = true;
            terminal.draw(|f| draw(f, &mut app)).unwrap();

            app.show_help_modal = false;
            app.start_editing_selected_gear();
            terminal.draw(|f| draw(f, &mut app)).unwrap();

            app.toggle_theme();
            terminal.draw(|f| draw(f, &mut app)).unwrap();
        }
    }
}

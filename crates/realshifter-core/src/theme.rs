use crate::GearPosition;
use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ThemeMode {
    #[default]
    Auto,
    Dark,
    Light,
}

impl ThemeMode {
    pub fn next(&self) -> Self {
        match self {
            ThemeMode::Auto => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Auto,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ThemeMode::Auto => "Auto",
            ThemeMode::Dark => "Dark",
            ThemeMode::Light => "Light",
        }
    }

    pub fn is_light(&self) -> bool {
        match self {
            ThemeMode::Light => true,
            ThemeMode::Dark => false,
            ThemeMode::Auto => detect_light_terminal(),
        }
    }
}

pub fn detect_light_terminal() -> bool {
    // 1. Check COLORFGBG environment variable (e.g. "0;15" => bg is 15/white)
    if let Ok(val) = std::env::var("COLORFGBG") {
        let parts: Vec<&str> = val.split(';').collect();
        if let Some(bg_str) = parts.last() {
            if let Ok(bg_num) = bg_str.trim().parse::<u8>() {
                if bg_num == 7 || bg_num == 15 || (bg_num >= 8 && bg_num != 8) {
                    return true;
                }
            } else if bg_str.eq_ignore_ascii_case("white") || bg_str.eq_ignore_ascii_case("light") {
                return true;
            }
        }
    }

    // 2. Check TERM_BACKGROUND
    if let Ok(val) = std::env::var("TERM_BACKGROUND") {
        if val.eq_ignore_ascii_case("light") {
            return true;
        }
    }

    // 3. Check THEME environment variable
    if let Ok(theme) = std::env::var("THEME") {
        if theme.to_lowercase().contains("light") {
            return true;
        }
    }

    false
}

pub fn gear_color(gear: GearPosition) -> Color {
    gear_color_for_theme(gear, false)
}

pub fn gear_color_for_theme(gear: GearPosition, is_light: bool) -> Color {
    if is_light {
        match gear {
            GearPosition::Neutral => Color::Rgb(100, 100, 100),
            GearPosition::Gear1 => Color::Rgb(0, 90, 220), // Deep Blue
            GearPosition::Gear2 => Color::Rgb(0, 135, 160), // Dark Teal
            GearPosition::Gear3 => Color::Rgb(0, 135, 45), // Forest Green
            GearPosition::Gear4 => Color::Rgb(140, 20, 150), // Deep Magenta
            GearPosition::Gear5 => Color::Rgb(175, 95, 0), // Amber Brown
            GearPosition::Gear6 => Color::Rgb(190, 35, 120), // Ruby Pink
            GearPosition::Reverse => Color::Rgb(200, 20, 20), // Crimson Red
        }
    } else {
        match gear {
            GearPosition::Neutral => Color::DarkGray,
            GearPosition::Gear1 => Color::Blue,
            GearPosition::Gear2 => Color::Cyan,
            GearPosition::Gear3 => Color::Green,
            GearPosition::Gear4 => Color::Magenta,
            GearPosition::Gear5 => Color::Yellow,
            GearPosition::Gear6 => Color::Rgb(255, 105, 180), // Pink
            GearPosition::Reverse => Color::Red,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub is_light: bool,
    pub border: Color,
    pub header_border: Color,
    pub modal_border: Color,
    pub title: Style,
    pub header_cell: Style,
    pub active_badge: Style,
    pub viewing_badge: Style,
    pub idle_badge: Style,
    pub selected_row: Style,
    pub current_row: Style,
    pub normal_row: Style,
    pub primary_text: Style,
    pub secondary_text: Style,
    pub accent_cyan: Style,
    pub accent_green: Style,
    pub accent_magenta: Style,
    pub modal_field_focused: Style,
    pub modal_field_unfocused: Style,
    pub modal_save_focused: Style,
    pub modal_save_unfocused: Style,
    pub modal_cancel_focused: Style,
    pub modal_cancel_unfocused: Style,
    pub tab_highlight: Style,
}

impl Theme {
    pub fn from_mode(mode: ThemeMode) -> Self {
        if mode.is_light() {
            Self::light()
        } else {
            Self::dark()
        }
    }

    pub fn dark() -> Self {
        Self {
            is_light: false,
            border: Color::Cyan,
            header_border: Color::Cyan,
            modal_border: Color::Yellow,
            title: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            header_cell: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            active_badge: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            viewing_badge: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            idle_badge: Style::default().fg(Color::DarkGray),
            selected_row: Style::default().bg(Color::Rgb(40, 50, 70)),
            current_row: Style::default().bg(Color::Rgb(20, 45, 30)),
            normal_row: Style::default(),
            primary_text: Style::default().fg(Color::White),
            secondary_text: Style::default().fg(Color::DarkGray),
            accent_cyan: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            accent_green: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            accent_magenta: Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            modal_field_focused: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            modal_field_unfocused: Style::default().fg(Color::White),
            modal_save_focused: Style::default()
                .bg(Color::Green)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
            modal_save_unfocused: Style::default().fg(Color::Green),
            modal_cancel_focused: Style::default()
                .bg(Color::Red)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            modal_cancel_unfocused: Style::default().fg(Color::Red),
            tab_highlight: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        }
    }

    pub fn light() -> Self {
        Self {
            is_light: true,
            border: Color::Rgb(0, 100, 160),
            header_border: Color::Rgb(0, 100, 160),
            modal_border: Color::Rgb(160, 90, 0),
            title: Style::default()
                .fg(Color::Rgb(160, 90, 0))
                .add_modifier(Modifier::BOLD),
            header_cell: Style::default()
                .fg(Color::Rgb(0, 80, 150))
                .add_modifier(Modifier::BOLD),
            active_badge: Style::default()
                .fg(Color::Rgb(0, 125, 35))
                .add_modifier(Modifier::BOLD),
            viewing_badge: Style::default()
                .fg(Color::Rgb(160, 90, 0))
                .add_modifier(Modifier::BOLD),
            idle_badge: Style::default().fg(Color::Rgb(120, 120, 120)),
            selected_row: Style::default()
                .bg(Color::Rgb(215, 230, 255))
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
            current_row: Style::default()
                .bg(Color::Rgb(220, 245, 225))
                .fg(Color::Rgb(0, 120, 30))
                .add_modifier(Modifier::BOLD),
            normal_row: Style::default().fg(Color::Black),
            primary_text: Style::default().fg(Color::Black),
            secondary_text: Style::default().fg(Color::Rgb(100, 100, 100)),
            accent_cyan: Style::default()
                .fg(Color::Rgb(0, 120, 150))
                .add_modifier(Modifier::BOLD),
            accent_green: Style::default()
                .fg(Color::Rgb(0, 125, 35))
                .add_modifier(Modifier::BOLD),
            accent_magenta: Style::default()
                .fg(Color::Rgb(130, 20, 140))
                .add_modifier(Modifier::BOLD),
            modal_field_focused: Style::default()
                .fg(Color::Rgb(160, 90, 0))
                .add_modifier(Modifier::BOLD),
            modal_field_unfocused: Style::default().fg(Color::Black),
            modal_save_focused: Style::default()
                .bg(Color::Rgb(0, 140, 40))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            modal_save_unfocused: Style::default()
                .fg(Color::Rgb(0, 125, 35))
                .add_modifier(Modifier::BOLD),
            modal_cancel_focused: Style::default()
                .bg(Color::Rgb(190, 30, 30))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            modal_cancel_unfocused: Style::default().fg(Color::Rgb(180, 20, 20)),
            tab_highlight: Style::default()
                .fg(Color::Rgb(0, 120, 30))
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        }
    }

    pub fn gear_color(&self, gear: GearPosition) -> Color {
        gear_color_for_theme(gear, self.is_light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_colors_dark_and_light() {
        for gear in GearPosition::all() {
            let _c_dark = gear_color_for_theme(*gear, false);
            let _c_light = gear_color_for_theme(*gear, true);
        }
        assert_eq!(
            gear_color_for_theme(GearPosition::Neutral, false),
            Color::DarkGray
        );
        assert_eq!(
            gear_color_for_theme(GearPosition::Gear1, false),
            Color::Blue
        );
        assert_eq!(
            gear_color_for_theme(GearPosition::Reverse, false),
            Color::Red
        );

        assert_eq!(
            gear_color_for_theme(GearPosition::Neutral, true),
            Color::Rgb(100, 100, 100)
        );
        assert_eq!(
            gear_color_for_theme(GearPosition::Gear1, true),
            Color::Rgb(0, 90, 220)
        );
        assert_eq!(
            gear_color_for_theme(GearPosition::Reverse, true),
            Color::Rgb(200, 20, 20)
        );
    }

    #[test]
    fn test_theme_mode_cycle() {
        let mut mode = ThemeMode::Auto;
        assert_eq!(mode.display_name(), "Auto");
        mode = mode.next();
        assert_eq!(mode, ThemeMode::Dark);
        assert_eq!(mode.display_name(), "Dark");
        mode = mode.next();
        assert_eq!(mode, ThemeMode::Light);
        assert_eq!(mode.display_name(), "Light");
        mode = mode.next();
        assert_eq!(mode, ThemeMode::Auto);
    }

    #[test]
    fn test_theme_builders() {
        let dark = Theme::dark();
        assert!(!dark.is_light);
        let light = Theme::light();
        assert!(light.is_light);
    }
}

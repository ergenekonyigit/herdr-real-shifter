use crate::GearPosition;
use ratatui::style::Color;

pub fn gear_color(gear: GearPosition) -> Color {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_colors() {
        assert_eq!(gear_color(GearPosition::Gear1), Color::Blue);
        assert_eq!(gear_color(GearPosition::Reverse), Color::Red);
    }
}

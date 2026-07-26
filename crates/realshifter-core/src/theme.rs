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
        for gear in GearPosition::all() {
            let _c = gear_color(*gear);
        }
        assert_eq!(gear_color(GearPosition::Neutral), Color::DarkGray);
        assert_eq!(gear_color(GearPosition::Gear1), Color::Blue);
        assert_eq!(gear_color(GearPosition::Gear2), Color::Cyan);
        assert_eq!(gear_color(GearPosition::Gear3), Color::Green);
        assert_eq!(gear_color(GearPosition::Gear4), Color::Magenta);
        assert_eq!(gear_color(GearPosition::Gear5), Color::Yellow);
        assert_eq!(gear_color(GearPosition::Gear6), Color::Rgb(255, 105, 180));
        assert_eq!(gear_color(GearPosition::Reverse), Color::Red);
    }
}

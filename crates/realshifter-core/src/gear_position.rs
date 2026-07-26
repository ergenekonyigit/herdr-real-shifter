use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum GearPosition {
    Neutral = 0,
    Gear1 = 1,
    Gear2 = 2,
    Gear3 = 3,
    Gear4 = 4,
    Gear5 = 5,
    Gear6 = 6,
    Reverse = 7,
}

impl GearPosition {
    pub fn is_driving(&self) -> bool {
        !matches!(self, GearPosition::Neutral)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            GearPosition::Neutral => "N",
            GearPosition::Gear1 => "1",
            GearPosition::Gear2 => "2",
            GearPosition::Gear3 => "3",
            GearPosition::Gear4 => "4",
            GearPosition::Gear5 => "5",
            GearPosition::Gear6 => "6",
            GearPosition::Reverse => "R",
        }
    }

    pub fn full_name(&self) -> &'static str {
        match self {
            GearPosition::Neutral => "Neutral",
            GearPosition::Gear1 => "Gear 1",
            GearPosition::Gear2 => "Gear 2",
            GearPosition::Gear3 => "Gear 3",
            GearPosition::Gear4 => "Gear 4",
            GearPosition::Gear5 => "Gear 5",
            GearPosition::Gear6 => "Gear 6",
            GearPosition::Reverse => "Reverse",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            GearPosition::Neutral => "⚪",
            GearPosition::Gear1 => "1️⃣",
            GearPosition::Gear2 => "2️⃣",
            GearPosition::Gear3 => "3️⃣",
            GearPosition::Gear4 => "4️⃣",
            GearPosition::Gear5 => "5️⃣",
            GearPosition::Gear6 => "6️⃣",
            GearPosition::Reverse => "🔴",
        }
    }

    pub fn hid_button_index(&self) -> Option<u8> {
        match self {
            GearPosition::Neutral => None,
            GearPosition::Gear1 => Some(0),
            GearPosition::Gear2 => Some(1),
            GearPosition::Gear3 => Some(2),
            GearPosition::Gear4 => Some(3),
            GearPosition::Gear5 => Some(4),
            GearPosition::Gear6 => Some(5),
            GearPosition::Reverse => Some(6),
        }
    }

    pub fn from_hid_button(button: u8) -> Self {
        match button {
            0 => GearPosition::Gear1,
            1 => GearPosition::Gear2,
            2 => GearPosition::Gear3,
            3 => GearPosition::Gear4,
            4 => GearPosition::Gear5,
            5 => GearPosition::Gear6,
            6 => GearPosition::Reverse,
            _ => GearPosition::Neutral,
        }
    }

    pub fn all_driving() -> &'static [GearPosition] {
        &[
            GearPosition::Gear1,
            GearPosition::Gear2,
            GearPosition::Gear3,
            GearPosition::Gear4,
            GearPosition::Gear5,
            GearPosition::Gear6,
            GearPosition::Reverse,
        ]
    }

    pub fn all() -> &'static [GearPosition] {
        &[
            GearPosition::Neutral,
            GearPosition::Gear1,
            GearPosition::Gear2,
            GearPosition::Gear3,
            GearPosition::Gear4,
            GearPosition::Gear5,
            GearPosition::Gear6,
            GearPosition::Reverse,
        ]
    }
}

impl fmt::Display for GearPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl TryFrom<u8> for GearPosition {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GearPosition::Neutral),
            1 => Ok(GearPosition::Gear1),
            2 => Ok(GearPosition::Gear2),
            3 => Ok(GearPosition::Gear3),
            4 => Ok(GearPosition::Gear4),
            5 => Ok(GearPosition::Gear5),
            6 => Ok(GearPosition::Gear6),
            7 => Ok(GearPosition::Reverse),
            _ => Err(format!("Invalid gear raw value: {value}")),
        }
    }
}

impl FromStr for GearPosition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "0" | "n" | "neutral" => Ok(GearPosition::Neutral),
            "1" | "gear1" | "gear 1" => Ok(GearPosition::Gear1),
            "2" | "gear2" | "gear 2" => Ok(GearPosition::Gear2),
            "3" | "gear3" | "gear 3" => Ok(GearPosition::Gear3),
            "4" | "gear4" | "gear 4" => Ok(GearPosition::Gear4),
            "5" | "gear5" | "gear 5" => Ok(GearPosition::Gear5),
            "6" | "gear6" | "gear 6" => Ok(GearPosition::Gear6),
            "7" | "r" | "reverse" => Ok(GearPosition::Reverse),
            _ => Err(format!("Unknown gear position: '{s}'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_position_display_and_parse() {
        assert_eq!(GearPosition::Gear1.display_name(), "1");
        assert_eq!(GearPosition::Reverse.display_name(), "R");
        assert_eq!(GearPosition::Neutral.display_name(), "N");

        assert_eq!("1".parse::<GearPosition>().unwrap(), GearPosition::Gear1);
        assert_eq!("r".parse::<GearPosition>().unwrap(), GearPosition::Reverse);
        assert_eq!("neutral".parse::<GearPosition>().unwrap(), GearPosition::Neutral);
        assert_eq!("gear 3".parse::<GearPosition>().unwrap(), GearPosition::Gear3);
        assert!("invalid".parse::<GearPosition>().is_err());
    }

    #[test]
    fn test_hid_button_mapping() {
        assert_eq!(GearPosition::from_hid_button(0), GearPosition::Gear1);
        assert_eq!(GearPosition::from_hid_button(1), GearPosition::Gear2);
        assert_eq!(GearPosition::from_hid_button(2), GearPosition::Gear3);
        assert_eq!(GearPosition::from_hid_button(3), GearPosition::Gear4);
        assert_eq!(GearPosition::from_hid_button(4), GearPosition::Gear5);
        assert_eq!(GearPosition::from_hid_button(5), GearPosition::Gear6);
        assert_eq!(GearPosition::from_hid_button(6), GearPosition::Reverse);
        assert_eq!(GearPosition::from_hid_button(99), GearPosition::Neutral);

        assert_eq!(GearPosition::Gear1.hid_button_index(), Some(0));
        assert_eq!(GearPosition::Gear2.hid_button_index(), Some(1));
        assert_eq!(GearPosition::Gear3.hid_button_index(), Some(2));
        assert_eq!(GearPosition::Gear4.hid_button_index(), Some(3));
        assert_eq!(GearPosition::Gear5.hid_button_index(), Some(4));
        assert_eq!(GearPosition::Gear6.hid_button_index(), Some(5));
        assert_eq!(GearPosition::Reverse.hid_button_index(), Some(6));
        assert_eq!(GearPosition::Neutral.hid_button_index(), None);
    }

    #[test]
    fn test_gear_position_names_emojis_and_driving() {
        assert!(!GearPosition::Neutral.is_driving());
        assert!(GearPosition::Gear1.is_driving());
        assert!(GearPosition::Reverse.is_driving());

        assert_eq!(GearPosition::Neutral.full_name(), "Neutral");
        assert_eq!(GearPosition::Gear1.full_name(), "Gear 1");
        assert_eq!(GearPosition::Gear2.full_name(), "Gear 2");
        assert_eq!(GearPosition::Gear3.full_name(), "Gear 3");
        assert_eq!(GearPosition::Gear4.full_name(), "Gear 4");
        assert_eq!(GearPosition::Gear5.full_name(), "Gear 5");
        assert_eq!(GearPosition::Gear6.full_name(), "Gear 6");
        assert_eq!(GearPosition::Reverse.full_name(), "Reverse");

        assert_eq!(GearPosition::Neutral.emoji(), "⚪");
        assert_eq!(GearPosition::Gear1.emoji(), "1️⃣");
        assert_eq!(GearPosition::Reverse.emoji(), "🔴");

        assert_eq!(GearPosition::all_driving().len(), 7);
        assert_eq!(GearPosition::all().len(), 8);

        assert_eq!(format!("{}", GearPosition::Gear1), "1");
    }

    #[test]
    fn test_try_from_u8() {
        for i in 0..=7u8 {
            assert!(GearPosition::try_from(i).is_ok());
        }
        assert!(GearPosition::try_from(8u8).is_err());
    }
}

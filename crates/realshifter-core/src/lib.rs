pub mod cli_profile;
pub mod config;
pub mod gear_action_type;
pub mod gear_mapping;
pub mod gear_position;
pub mod state;
pub mod theme;

pub use cli_profile::CliProfile;
pub use config::Config;
pub use gear_action_type::GearActionType;
pub use gear_mapping::GearMapping;
pub use gear_position::GearPosition;
pub use state::SessionState;
pub use theme::gear_color;
use std::path::PathBuf;

pub fn action_binary_path() -> PathBuf {
    if let Ok(path) = std::env::var("HERDR_ACTION_BIN") {
        return PathBuf::from(path);
    }
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("realshifter-action")))
        .unwrap_or_else(|| "realshifter-action".into())
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

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

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

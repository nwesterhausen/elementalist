/// Various information about the game gathered at build time
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// The name of the game
pub const GAME_NAME: &str = "Elementalist";

/// The name and version of the game
pub fn game_name_and_version() -> String {
    format!("{} v{}", GAME_NAME, built_info::PKG_VERSION)
}

/// A specific version of the game
pub fn game_descriptor() -> String {
    format!(
        "Build v{} ({})",
        built_info::PKG_VERSION,
        built_info::GIT_COMMIT_HASH_SHORT.unwrap_or("unknown")
    )
}

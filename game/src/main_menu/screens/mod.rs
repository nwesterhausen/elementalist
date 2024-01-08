mod audio;
mod controls;
mod gameplay;
mod main_screen;
mod settings;
mod video;

pub use audio::audio_settings_setup as audio_settings;
pub use controls::controls_settings_setup as controls_settings;
pub use gameplay::gameplay_settings_setup as gameplay_settings;
pub use main_screen::main_menu_setup as main_menu;
pub use settings::settings_setup as settings;
pub use video::video_settings_setup as video_settings;

use bevy::prelude::*;
use game_library::{font_resource::FontResource, settings::AccessibilitySettings};

/// Set the initial fonts.
pub fn set_initial_fonts(
    mut font_resource: ResMut<FontResource>,
    asset_loader: Res<AssetServer>,
    accessibility_settings: Res<AccessibilitySettings>,
) {
    font_resource.display_font_handle = asset_loader.load("ui/fonts/AlmendraDisplay-Regular.ttf");
    font_resource.fancy_font_handle = asset_loader.load("ui/fonts/Almendra-Regular.ttf");
    font_resource.dyslexic_font_handle = asset_loader.load("ui/fonts/OpenDyslexic-Regular.otf");
    font_resource.sans_serif_font_handle = asset_loader.load("ui/fonts/RedHatDisplay-Regular.ttf");
    font_resource.monospace_font_handle = asset_loader.load("ui/fonts/SyneMono-Regular.ttf");

    // Set the initial fonts
    font_resource.display_font = font_resource.display_font_handle.clone();
    font_resource.console_font = font_resource.monospace_font_handle.clone();

    font_resource.interface_font =
        font_resource.get_font_handle(accessibility_settings.interface_font_family);
    font_resource.main_font =
        font_resource.get_font_handle(accessibility_settings.main_font_family);
}

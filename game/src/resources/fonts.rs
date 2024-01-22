use bevy::prelude::*;
use game_library::font_resource::FontResource;

/// Set the initial fonts.
pub fn set_initial_fonts(mut font_resource: ResMut<FontResource>, asset_loader: Res<AssetServer>) {
    font_resource.display_font = asset_loader.load("ui/fonts/AlmendraDisplay-Regular.ttf");
    font_resource.interface_font = asset_loader.load("ui/fonts/Almendra-Regular.ttf");
    font_resource.main_font = asset_loader.load("ui/fonts/RedHatDisplay-Regular.ttf");
    font_resource.console_font = asset_loader.load("ui/fonts/NanumGothicCoding-Regular.ttf");
}

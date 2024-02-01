//! Colors used in the game. Some which are specifically named here due to their use in the game.
//!
//! The colors are named from <https://chir.ag/projects/name-that-color/>.
//!
//! The colors come from the Comfy52 color palette <https://lospec.com/palette-list/comfy52>

use bevy::prelude::*;

/// The dark color used for the background.
pub const BACKGROUND_COLOR: Color = DARKER_THUNDER;
/// The dark color used for the background at 50% opacity.
pub const BACKGROUND_COLOR_50: Color = Color::rgba(0.169, 0.133, 0.165, 0.5);

/// The clear color used for the game.
pub const CLEAR_COLOR: Color = MIRAGE;

/// The color used for text.
pub const TEXT_COLOR: Color = Color::rgba(1., 0.658, 0.27, 1.);

/// The color used for selected text.
pub const SELECTED_TEXT_COLOR: Color = KANGAROO;

/// The color used for hovered text.
pub const HOVERED_TEXT_COLOR: Color = MANZ;

/// The color used for hovered text (alternate).
pub const HOVERED_TEXT_COLOR_ALTERNATE: Color = MULE_FAWN;

/// The color used for the health bar.
pub const HEALTH_BAR_COLOR: Color = MOJO;

/// Health bar Ok
pub const HEALTH_BAR_COLOR_OK: Color = COSTA_DEL_SOL;

/// Health bar Moderate
pub const HEALTH_BAR_COLOR_MODERATE: Color = MULE_FAWN;

/// Health bar Critical
pub const HEALTH_BAR_COLOR_CRITICAL: Color = MOJO;

/// The color used for the mana bar.
pub const MANA_BAR_COLOR: Color = JACARTA;

/// The color used for the skill track bar.
pub const SKILL_TRACK_BAR_COLOR: Color = COPPER;

/// <div style="background-color:rgb(25%, 25%, 16%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #3f4328
pub const RANGITOTO: Color = Color::rgba(0.247, 0.262, 0.157, 1.);
/// <div style="background-color:rgb(37%, 44%, 19%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #5f7132
pub const COSTA_DEL_SOL: Color = Color::rgba(0.373, 0.443, 0.196, 1.);
/// <div style="background-color:rgb(58%, 68%, 22%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #94ad39
pub const SUSHI: Color = Color::rgba(0.58, 0.678, 0.224, 1.);
/// <div style="background-color:rgb(76%, 84%, 31%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #c2d64f
pub const CONIFER: Color = Color::rgba(0.761, 0.839, 0.31, 1.);
/// <div style="background-color:rgb(94%, 95%, 49%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #eff37c
pub const MANZ: Color = Color::rgba(0.937, 0.953, 0.486, 1.);
/// <div style="background-color:rgb(89%, 90%, 68%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #e3e6ac
pub const HAMPTON: Color = Color::rgba(0.89, 0.902, 0.675, 1.);
/// <div style="background-color:rgb(65%, 78%, 49%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #a5c67c
pub const OLIVINE: Color = Color::rgba(0.647, 0.776, 0.486, 1.);
/// <div style="background-color:rgb(45%, 60%, 44%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #739a70
pub const LAUREL: Color = Color::rgba(0.451, 0.604, 0.439, 1.);
/// <div style="background-color:rgb(30%, 40%, 35%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #4d6659
pub const NANDOR: Color = Color::rgba(0.302, 0.4, 0.349, 1.);
/// <div style="background-color:rgb(20%, 25%, 26%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #343f41
pub const OUTER_SPACE: Color = Color::rgba(0.204, 0.247, 0.255, 1.);
/// <div style="background-color:rgb(16%, 18%, 23%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #282e3b
pub const EBONY_CLAY: Color = Color::rgba(0.157, 0.18, 0.231, 1.);
/// <div style="background-color:rgb(10%, 12%, 18%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #1a1f2e
pub const MIRAGE: Color = Color::rgba(0.102, 0.122, 0.18, 1.);
/// <div style="background-color:rgb(17%, 19%, 29%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #1e314b
pub const CLOUD_BURST: Color = Color::rgba(0.118, 0.192, 0.294, 1.);
/// <div style="background-color:rgb(18%, 30%, 42%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #2f4c6c
pub const SAN_JUAN: Color = Color::rgba(0.184, 0.298, 0.424, 1.);
/// <div style="background-color:rgb(24%, 50%, 64%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #3d80a3
pub const STEEL_BLUE: Color = Color::rgba(0.239, 0.502, 0.639, 1.);
/// <div style="background-color:rgb(39%, 77%, 80%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #63c4cc
pub const DOWNY: Color = Color::rgba(0.388, 0.769, 0.8, 1.);
/// <div style="background-color:rgb(50%, 85%, 85%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #9ae5d5
pub const WATER_LEAF: Color = Color::rgba(0.604, 0.898, 0.835, 1.);
/// <div style="background-color:rgb(84%, 94%, 94%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #e5efef
pub const MYSTIC: Color = Color::rgba(0.835, 0.937, 0.937, 1.);
/// <div style="background-color:rgb(73%, 79%, 80%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #bac9cd
pub const LOBLOLLY: Color = Color::rgba(0.729, 0.788, 0.804, 1.);
/// <div style="background-color:rgb(55%, 60%, 64%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #8d99a4
pub const REGENT_GRAY: Color = Color::rgba(0.553, 0.6, 0.643, 1.);
/// <div style="background-color:rgb(41%, 44%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #696f80
pub const PALE_SKY: Color = Color::rgba(0.412, 0.435, 0.502, 1.);
/// <div style="background-color:rgb(25%, 26%, 33%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #414453
pub const BRIGHT_GRAY: Color = Color::rgba(0.254, 0.267, 0.325, 1.);
/// <div style="background-color:rgb(72%, 63%, 76%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #b8a1c2
pub const LONDON_HUE: Color = Color::rgba(0.722, 0.631, 0.761, 1.);
/// <div style="background-color:rgb(50%, 41%, 36%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #7e695b
pub const SAND_DUNE: Color = Color::rgba(0.494, 0.412, 0.357, 1.);
/// <div style="background-color:rgb(36%, 23%, 44%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #5c3a6f
pub const BOSSANOVA: Color = Color::rgba(0.361, 0.227, 0.435, 1.);
/// <div style="background-color:rgb(22%, 15%, 37%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #39275e
pub const JACARTA: Color = Color::rgba(0.224, 0.153, 0.369, 1.);
/// <div style="background-color:rgb(18%, 10%, 24%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #2f193e
pub const REVOLVER: Color = Color::rgba(0.184, 0.098, 0.243, 1.);
/// <div style="background-color:rgb(30%, 10%, 29%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #4e1a49
pub const WINE_BERRY: Color = Color::rgba(0.306, 0.102, 0.286, 1.);
/// <div style="background-color:rgb(40%, 16%, 29%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #7b234c
pub const PURPLE_ALTERNATE_DARK: Color = Color::rgba(0.482, 0.141, 0.298, 1.);
/// <div style="background-color:rgb(48%, 14%, 30%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #b23657
pub const TAWNY_PORT: Color = Color::rgba(0.482, 0.137, 0.298, 1.);
/// <div style="background-color:rgb(82%, 41%, 46%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #d16974
pub const CHESTNUT_ROSE: Color = Color::rgba(0.82, 0.412, 0.455, 1.);
/// <div style="background-color:rgb(93%, 67%, 64%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #edaaa3
pub const SEA_PINK: Color = Color::rgba(0.929, 0.667, 0.639, 1.);
/// <div style="background-color:rgb(93%, 80%, 57%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #eecb90
pub const CHALKY: Color = Color::rgba(0.933, 0.796, 0.565, 1.);
/// <div style="background-color:rgb(89%, 66%, 27%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #e1a845
pub const ANZAC: Color = Color::rgba(0.882, 0.659, 0.271, 1.);
/// <div style="background-color:rgb(77%, 47%, 21%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #c57835
pub const COPPER: Color = Color::rgba(0.773, 0.471, 0.208, 1.);
/// <div style="background-color:rgb(55%, 28%, 19%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #8d4830
pub const MULE_FAWN: Color = Color::rgba(0.553, 0.282, 0.188, 1.);
/// <div style="background-color:rgb(89%, 45%, 35%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #e47259
pub const TERRACOTTA: Color = Color::rgba(0.894, 0.447, 0.349, 1.);
/// <div style="background-color:rgb(77%, 23%, 25%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #c33c40
pub const MOJO: Color = Color::rgba(0.765, 0.235, 0.251, 1.);
/// <div style="background-color:rgb(55%, 21%, 29%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #8d3649
pub const SOLID_PINK: Color = Color::rgba(0.553, 0.212, 0.286, 1.);
/// <div style="background-color:rgb(36%, 17%, 20%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #5c2b34
pub const BUCCANEER: Color = Color::rgba(0.361, 0.169, 0.204, 1.);
/// <div style="background-color:rgb(24%, 15%, 17%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #3c252b
pub const COCOA_BROWN: Color = Color::rgba(0.235, 0.145, 0.169, 1.);
/// <div style="background-color:rgb(41%, 25%, 22%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #684039
pub const MILLBROOK: Color = Color::rgba(0.408, 0.251, 0.224, 1.);
/// <div style="background-color:rgb(51%, 34%, 28%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #825646
pub const SPICY_MIX: Color = Color::rgba(0.51, 0.337, 0.275, 1.);
/// <div style="background-color:rgb(72%, 47%, 38%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #b77862
pub const SANTA_FE: Color = Color::rgba(0.718, 0.471, 0.384, 1.);
/// <div style="background-color:rgb(49%, 35%, 37%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #7d595d
pub const RUSSETT: Color = Color::rgba(0.49, 0.349, 0.365, 1.);
/// <div style="background-color:rgb(33%, 23%, 26%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #533b41
pub const MATTERHORN: Color = Color::rgba(0.325, 0.231, 0.255, 1.);
/// <div style="background-color:rgb(25%, 20%, 23%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #3f333b
pub const THUNDER: Color = Color::rgba(0.247, 0.2, 0.231, 1.);
/// <div style="background-color:rgb(17%, 13%, 17%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #2b222a
///
/// (This one also comes up as "Thunder" but is a different color.)
pub const DARKER_THUNDER: Color = Color::rgba(0.169, 0.133, 0.165, 1.);
/// <div style="background-color:rgb(43%, 30%, 29%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #6d4e4b
pub const FERRA: Color = Color::rgba(0.427, 0.306, 0.294, 1.);
/// <div style="background-color:rgb(53%, 44%, 40%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #867066
pub const LIGHTER_SAND_DUNE: Color = Color::rgba(0.525, 0.439, 0.4, 1.);
/// <div style="background-color:rgb(70%, 61%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #b49d7e
pub const MONGOOSE: Color = Color::rgba(0.706, 0.616, 0.494, 1.);
/// <div style="background-color:rgb(77%, 78%, 72%); width: 10px; padding: 10px; border: 1px solid;"></div>
/// Hex code #c4c6b8
pub const KANGAROO: Color = Color::rgba(0.769, 0.776, 0.722, 1.);

/// Color of the Comfy52 palette to be able to use in the game.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub enum PaletteColor {
    Rangitoto,
    CostaDelSol,
    Sushi,
    Conifer,
    Manz,
    Hampton,
    Olivine,
    Laurel,
    Nandor,
    OuterSpace,
    EbonyClay,
    Mirage,
    CloudBurst,
    SanJuan,
    SteelBlue,
    Downy,
    WaterLeaf,
    Mystic,
    Loblolly,
    RegentGray,
    PaleSky,
    BrightGray,
    LondonHue,
    SandDune,
    #[default]
    Bossanova,
    Jacarta,
    Revolver,
    WineBerry,
    PurpleAlternateDark,
    TawnyPort,
    ChestnutRose,
    SeaPink,
    Chalky,
    Anzac,
    Copper,
    MuleFawn,
    Terracotta,
    Mojo,
    SolidPink,
    Buccaneer,
    CocoaBrown,
    Millbrook,
    SpicyMix,
    SantaFe,
    Russett,
    Matterhorn,
    Thunder,
    DarkerThunder,
    Ferra,
    LighterSandDune,
    Mongoose,
    Kangaroo,
}

impl PaletteColor {
    pub fn to_color(&self) -> Color {
        match self {
            Self::Rangitoto => RANGITOTO,
            Self::CostaDelSol => COSTA_DEL_SOL,
            Self::Sushi => SUSHI,
            Self::Conifer => CONIFER,
            Self::Manz => MANZ,
            Self::Hampton => HAMPTON,
            Self::Olivine => OLIVINE,
            Self::Laurel => LAUREL,
            Self::Nandor => NANDOR,
            Self::OuterSpace => OUTER_SPACE,
            Self::EbonyClay => EBONY_CLAY,
            Self::Mirage => MIRAGE,
            Self::CloudBurst => CLOUD_BURST,
            Self::SanJuan => SAN_JUAN,
            Self::SteelBlue => STEEL_BLUE,
            Self::Downy => DOWNY,
            Self::WaterLeaf => WATER_LEAF,
            Self::Mystic => MYSTIC,
            Self::Loblolly => LOBLOLLY,
            Self::RegentGray => REGENT_GRAY,
            Self::PaleSky => PALE_SKY,
            Self::BrightGray => BRIGHT_GRAY,
            Self::LondonHue => LONDON_HUE,
            Self::SandDune => SAND_DUNE,
            Self::Bossanova => BOSSANOVA,
            Self::Jacarta => JACARTA,
            Self::Revolver => REVOLVER,
            Self::WineBerry => WINE_BERRY,
            Self::PurpleAlternateDark => PURPLE_ALTERNATE_DARK,
            Self::TawnyPort => TAWNY_PORT,
            Self::ChestnutRose => CHESTNUT_ROSE,
            Self::SeaPink => SEA_PINK,
            Self::Chalky => CHALKY,
            Self::Anzac => ANZAC,
            Self::Copper => COPPER,
            Self::MuleFawn => MULE_FAWN,
            Self::Terracotta => TERRACOTTA,
            Self::Mojo => MOJO,
            Self::SolidPink => SOLID_PINK,
            Self::Buccaneer => BUCCANEER,
            Self::CocoaBrown => COCOA_BROWN,
            Self::Millbrook => MILLBROOK,
            Self::SpicyMix => SPICY_MIX,
            Self::SantaFe => SANTA_FE,
            Self::Russett => RUSSETT,
            Self::Matterhorn => MATTERHORN,
            Self::Thunder => THUNDER,
            Self::DarkerThunder => DARKER_THUNDER,
            Self::Ferra => FERRA,
            Self::LighterSandDune => LIGHTER_SAND_DUNE,
            Self::Mongoose => MONGOOSE,
            Self::Kangaroo => KANGAROO,
        }
    }
}

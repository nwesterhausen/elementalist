//! Struct definitions for the save file data.

use std::path::Path;

use crate::{enums::Skill, spells::talisman::SpellTalisman, Skills, Xp};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};

/// Struct for the save file data. This is serialized and deserialized to and from a file.
/// (This the data that is saved and loaded). It should be everything to bring a player
/// back to the state they were in when they saved. What it doesn't support is coming back
/// to an exact position in a realm or dungeon, and we don't support that.
///
/// The save file data is stored in a directory provided by the `platform_dirs` crate.
///
/// During the game, the save file data is stored in a `SaveFile` resource. New relevant
/// data is added to the `SaveFile` resource as the player progresses through the game.
/// The only data not constantly added is stat tracking, which is tracked in its own resource,
/// and then added to the `SaveFile` resource when the game is saved.
///
/// Player skills are part of this file, and they can be referenced directly from this resource
/// while in the game.
#[derive(Serialize, Deserialize, Debug, Reflect, Clone, Resource)]
pub struct SaveFile {
    /// Name of the save file
    pub name: String,
    /// Created talismans
    created_talismans: Vec<SpellTalisman>,
    /// Researched talismans
    researched_talismans: Vec<SpellTalisman>,
    /// Player skills
    player_skills: Skills,
    /// Unlocked skill perks for each skill (not implemented yet)
    unlocked_skill_perks: String,
    /// Inventory (essences, items, etc.) (not implemented yet)
    inventory: String,
    /// Stat tracking (not implemented yet)
    stat_tracking: String,
}

impl Default for SaveFile {
    fn default() -> Self {
        Self {
            name: String::from("New Game"),
            created_talismans: Vec::new(),
            researched_talismans: Vec::new(),
            player_skills: Skills {
                tracks: HashMap::new(),
            },
            unlocked_skill_perks: String::new(),
            inventory: String::new(),
            stat_tracking: String::new(),
        }
    }
}

impl SaveFile {
    /// Create a new save file with specified name
    ///
    /// # Parameters
    ///
    /// * `name` - Name of the save file
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Self::default()
        }
    }

    /// Load a save file by name from a path.
    ///
    /// The save files currently are just a YAML formatted file with the '.sav' extension.
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the save file
    #[must_use]
    pub fn load<P: AsRef<Path>>(path: &P) -> Option<Self> {
        let path = path.as_ref();
        if path.exists() {
            let Ok(file) = std::fs::File::open(path) else {
                error!("Unable to open save file: {}", path.display());
                return None;
            };
            let reader = std::io::BufReader::new(file);
            let Ok(save_file) = serde_yaml::from_reader(reader) else {
                error!("Unable to load save file: {}", path.display());
                return None;
            };
            Some(save_file)
        } else {
            None
        }
    }

    /// Save the save file in a directory. This will use the name of the save file to create a '.sav' file.
    ///
    /// # Parameters
    ///
    /// * `directory` - Directory to save the file in
    pub fn save<P: AsRef<Path>>(&self, directory: &P) {
        let path = directory.as_ref().join(format!("{}.sav", self.name));
        let Ok(file) = std::fs::File::create(&path) else {
            error!("Unable to create save file: {}", self.name);
            return;
        };
        let writer = std::io::BufWriter::new(file);
        match serde_yaml::to_writer(writer, self) {
            Ok(()) => {
                info!("Save file saved: {}", self.name);
                debug!("Saved to: {}", &path.display());
            }
            Err(e) => error!("Unable to save save file: {}", e),
        };
    }

    /// Add a created talisman to the save file
    ///
    /// # Parameters
    ///
    /// * `talisman` - The created talisman
    pub fn add_created_talisman(&mut self, talisman: SpellTalisman) {
        self.created_talismans.push(talisman);
    }

    /// Add a researched talisman to the save file
    ///
    /// # Parameters
    ///
    /// * `talisman` - The researched talisman
    pub fn add_researched_talisman(&mut self, talisman: SpellTalisman) {
        self.researched_talismans.push(talisman);
    }

    /// Get all skill data (useful for broad operations on all skills)
    #[must_use]
    pub fn get_skills(&self) -> Skills {
        self.player_skills.clone()
    }

    /// Get xp for a skill
    ///
    /// # Parameters
    ///
    /// * `skill` - The skill to get xp for
    #[must_use]
    pub fn get_xp(&self, skill: Skill) -> Option<&Xp> {
        self.player_skills.get_xp(skill)
    }

    /// Get the level for a skill
    ///
    /// # Parameters
    ///
    /// * `skill` - The skill to get the level for
    #[must_use]
    pub fn get_level(&self, skill: Skill) -> Option<u32> {
        self.player_skills.get_level(skill)
    }

    /// Get the percentage to the next level for a skill
    ///
    /// # Parameters
    ///
    /// * `skill` - The skill to get the percentage to the next level for
    #[must_use]
    pub fn get_percentage_to_next_level(&self, skill: Skill) -> Option<f32> {
        self.player_skills.get_percentage_to_next_level(skill)
    }

    /// Add xp to a skill
    ///
    /// # Parameters
    ///
    /// * `skill` - The skill to add xp to
    /// * `xp` - The amount of xp to add
    pub fn add_xp(&mut self, skill: Skill, xp: u32) {
        self.player_skills.add_xp(skill, xp);
    }

    /// Level-up a skill
    ///
    /// # Parameters
    ///
    /// * `skill` - The skill to level up
    pub fn level_up(&mut self, skill: Skill) {
        self.player_skills.level_up(skill);
    }

    /// Get all created talismans
    #[must_use]
    pub fn get_created_talismans(&self) -> Vec<SpellTalisman> {
        self.created_talismans.clone()
    }

    /// Get all researched talismans
    #[must_use]
    pub fn get_researched_talismans(&self) -> Vec<SpellTalisman> {
        self.researched_talismans.clone()
    }
}

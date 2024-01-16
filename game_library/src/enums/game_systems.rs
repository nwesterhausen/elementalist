use serde::{Deserialize, Serialize};

/// The game systems which can be defined using data files.
/// These are not every system in the game, but any system determined to be "modular"
#[derive(Debug, Serialize, Deserialize)]
pub enum GameSystem {
    /// The spell system (i.e. the data file is a spell data file)
    Spell,
}

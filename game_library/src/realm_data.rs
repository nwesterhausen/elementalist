//! Defines a data structure for the realm.
//!
//! This allows custom realms to be defined and used in game. Realms will be loaded from game data and can be set to generate the realm.
//!
//! Currently realms are just a collection of biomes and a primary element. This will likely be expanded into more details about the biomes
//! or possible other elements that can be included in the realm. Since the realm data is what is used to generate the "Elemental Realm" that
//! the player is able to visit, it will be important to have a good amount of detail about the realm.
//!
//! Additional details we will need are possibly special monsters and any pre-designed structures that can be found in the realm. This will
//! allow for a more unique experience when visiting the realm. The realm will also need to have a unique name and description to give it some
//! flavor and make it feel like a unique place to visit.
//!
//! One thing that would be neat is for the player to be able to choose one or maybe two elements, and a random realm matching those elements
//! will be chosen from the loaded realms. This will allow for a bit of randomness in the game and give the player a unique experience each time
//! they visit a realm. This will also allow for the player to have a bit of control over what they are going to face in the realm, as they can
//! choose the elements that they are strongest against (or that they need to gather resources from).
use bevy::prelude::*;
use std::any::Any;
use std::hash::Hash;

use crate::{
    data_loader::DataFile,
    enums::{GameSystem, MagicType},
    BiomeData, InternalId,
};

/// Details about a realm.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct Realm {
    /// The internal ID of the Realm.
    pub internal_id: Option<String>,
    /// Name of the realm.
    pub name: String,
    /// Short description of the realm.
    pub description: String,
    /// Biomes for the realm.
    pub biomes: Vec<BiomeData>,
    /// The primary element of the realm. (for now limited as magic school but should be its own.)
    pub primary_element: MagicType,
}

impl InternalId for Realm {
    /// Update the realms's internal ID.
    fn update_internal_id(&mut self) {
        self.internal_id = Some(self.get_internal_id());
    }
    /// Get the realms's internal ID.
    #[must_use]
    fn get_internal_id(&self) -> String {
        if self.internal_id.is_some() {
            let id = self.internal_id.clone().unwrap_or_default();
            if !id.is_empty() {
                return id;
            }
        }

        format!(
            "{}{}{}",
            self.name.replace(' ', ""),
            self.biomes.len(),
            self.primary_element
        )
    }
}

impl Default for Realm {
    fn default() -> Self {
        Self {
            internal_id: None,
            name: "Unknown Realm".to_string(),
            description: "A realm that was loaded incorrectly!".to_string(),
            biomes: Vec::new(),
            primary_element: MagicType::Arcane,
        }
    }
}

impl<D: Hash + InternalId + 'static> TryInto<Realm> for DataFile<D> {
    type Error = ();

    fn try_into(self) -> Result<Realm, Self::Error> {
        if self.header.system != GameSystem::Realm {
            return Err(());
        }

        (&self.data as &dyn Any)
            .downcast_ref::<Realm>()
            .cloned()
            .ok_or(())
    }
}

impl<D: Hash + InternalId + 'static> TryFrom<&DataFile<D>> for Realm {
    type Error = ();

    fn try_from(data_file: &DataFile<D>) -> Result<Self, Self::Error> {
        if data_file.header.system != GameSystem::Realm {
            return Err(());
        }

        (&data_file.data as &dyn Any)
            .downcast_ref::<Self>()
            .cloned()
            .ok_or(())
    }
}

impl Hash for Realm {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.description.hash(state);
        self.primary_element.hash(state);
    }
}

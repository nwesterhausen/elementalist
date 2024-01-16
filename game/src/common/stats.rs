use bevy::{prelude::*, reflect::Reflect, utils::hashbrown::HashMap};
use bevy_inspector_egui::prelude::*;
use game_library::{Attribute, Stat, StatEnum};

#[derive(Component, Default, Debug, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Health {
    pub value: Attribute,
}

impl Health {
    pub fn new(value: u32) -> Self {
        Self {
            value: Attribute::new(value),
        }
    }
}

#[derive(Component, Default, Debug, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Mana {
    pub value: Attribute,
}

impl Mana {
    pub fn new(value: u32) -> Self {
        Self {
            value: Attribute::new(value),
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct StatBundle {
    pub stats: HashMap<StatEnum, Stat>,
}

impl StatBundle {
    /// Creates a new stats bundle with the given stats.
    pub fn new(stats: Vec<(StatEnum, f32)>) -> Self {
        let mut stats_map = HashMap::new();

        for stat in stats {
            stats_map.insert(stat.0, Stat::new(stat.1));
        }

        Self { stats: stats_map }
    }
    /// Add a new stat to the stats bundle. If the stat already exists, it will be overwritten.
    pub fn add_stat(&mut self, stat: StatEnum) {
        self.stats.insert(stat, Stat::default());
    }
    /// Get a specific stat from the stats bundle.
    ///
    /// If the stat does not exist, it will return None.
    pub fn get_stat(&self, stat: StatEnum) -> Option<&Stat> {
        self.stats.get(&stat)
    }
    /// Update the value of a specific stat.
    ///
    /// If the stat does not exist, it will be added.
    pub fn update_stat(&mut self, stat: StatEnum, value: f32) {
        if let Some(stat) = self.stats.get_mut(&stat) {
            stat.set_base_value(value);
        } else {
            self.stats.insert(stat, Stat::new(value));
        }
    }
    /// Update the bonus of a specific stat.
    ///
    /// If the stat does not exist, it will be added.
    pub fn update_stat_bonus(&mut self, stat: StatEnum, value: f32) {
        if let Some(stat) = self.stats.get_mut(&stat) {
            stat.set_bonus_value(value);
        } else {
            self.stats.insert(stat.clone(), Stat::new(0.0));
            self.update_stat_bonus(stat, value);
        }
    }
}

//! Contains the `StatBundle` struct, which is a bundle of all stats an entity might have.

use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{enums::StatEnum, Stat};

/// Bundle that contains data for all stats an entity might have.
#[derive(Component, Default, Debug)]
pub struct StatBundle {
    /// A mapping of stat to stat value.
    pub stats: HashMap<StatEnum, Stat>,
}

impl StatBundle {
    /// Creates a new stats bundle with the given stats.
    #[must_use]
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
    #[must_use]
    pub fn get_stat(&self, stat: &StatEnum) -> Option<&Stat> {
        self.stats.get(stat)
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

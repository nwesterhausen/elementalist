use bevy::prelude::*;
use game_library::{Attribute, Stat};

#[derive(Component)]
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

#[derive(Component)]
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

#[derive(Component)]
pub struct Speed {
    pub value: Stat,
}

impl Speed {
    pub fn new(value: f32) -> Self {
        Self {
            value: Stat::new(value),
        }
    }
}

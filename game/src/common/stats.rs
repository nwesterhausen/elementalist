use bevy::{prelude::*, utils::HashMap};
use game_library::{Attribute, Stat};

use crate::spells::components::Spell;

#[derive(Component, Default, Debug)]
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

#[derive(Component, Default, Debug)]
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
pub struct MoveSpeed {
    pub value: Stat,
}

impl MoveSpeed {
    pub fn new(value: f32) -> Self {
        Self {
            value: Stat::new(value),
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct SpellSpeed {
    pub value: Stat,
}

impl SpellSpeed {
    pub fn new(value: f32) -> Self {
        Self {
            value: Stat::new(value),
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct SpellLifetime {
    pub value: Stat,
}

impl SpellLifetime {
    pub fn new(value: f32) -> Self {
        Self {
            value: Stat::new(value),
        }
    }
}

#[derive(Component, Debug)]
pub struct SpellDamage {
    pub value: HashMap<Spell, Stat>,
}

impl std::default::Default for SpellDamage {
    fn default() -> Self {
        let mut value = HashMap::default();
        for spell in Spell::variants() {
            value.insert(spell, Stat::default());
        }
        Self { value }
    }
}

impl SpellDamage {
    pub fn set(&mut self, spell: Spell, value: f32) {
        self.value.insert(spell, Stat::new(value));
    }

    pub fn get(&self, spell: Spell) -> Option<&Stat> {
        self.value.get(&spell)
    }
}

#[derive(Component, Debug, Default)]
pub struct CooldownModifier {
    pub value: Stat,
}

impl CooldownModifier {
    pub fn new(value: f32) -> Self {
        Self {
            value: Stat::new(value),
        }
    }
}

use bevy::prelude::*;

use crate::{berserker::BerserkerHealEvent, constants::*};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HpChangeEvent>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    Hero,
    SwordMan,
    Dummy,
}

#[derive(Component)]
pub struct Unit {
    pub name: String,
    pub unit_type: UnitType,
    pub hp: f32,
    pub max_hp: f32,
    pub speed: f32,
}

#[derive(Event)]
pub struct HpChangeEvent {
    pub entity: Entity,
    pub source: Entity,
    pub old_hp: f32,
    pub new_hp: f32,
    pub max_hp: f32,
    pub change_type: HpChangeType,
}

#[derive(Debug, Clone, Copy)]
pub enum HpChangeType {
    Damage,
    Heal,
    SetValue,
}

impl Unit {
    // Builder pattern approach with defaults
    pub fn builder() -> UnitBuilder {
        UnitBuilder::default()
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }

    pub fn damage(
        &mut self,
        amount: f32,
        entity: Entity,
        source: Entity,
        event_writer: &mut EventWriter<HpChangeEvent>,
    ) {
        let old_hp = self.hp;
        self.hp = (self.hp - amount).max(0.0);
        event_writer.write(HpChangeEvent {
            entity,
            source,
            old_hp,
            new_hp: self.hp,
            max_hp: self.max_hp,
            change_type: HpChangeType::Damage,
        });
    }

    pub fn berserker_heal(
        &mut self,
        amount: f32,
        entity: Entity,
        source: Entity,
        event_writer: &mut EventWriter<BerserkerHealEvent>,
    ) {
        let old_hp = self.hp;
        self.hp = (self.hp + amount).min(self.max_hp);
        event_writer.write(BerserkerHealEvent {
            entity,
            source,
            old_hp,
            new_hp: self.hp,
            max_hp: self.max_hp,
        });
    }

    pub fn heal(
        &mut self,
        amount: f32,
        entity: Entity,
        source: Entity,
        event_writer: &mut EventWriter<HpChangeEvent>,
    ) {
        let old_hp = self.hp;
        self.hp = (self.hp + amount).min(self.max_hp);
        event_writer.write(HpChangeEvent {
            entity,
            source,
            old_hp,
            new_hp: self.hp,
            max_hp: self.max_hp,
            change_type: HpChangeType::Heal,
        });
    }

    pub fn set_hp(
        &mut self,
        new_hp: f32,
        entity: Entity,
        event_writer: &mut EventWriter<HpChangeEvent>,
    ) {
        let old_hp = self.hp;
        self.hp = new_hp.clamp(0.0, self.max_hp);
        event_writer.write(HpChangeEvent {
            entity,
            source: todo!(),
            old_hp,
            new_hp: self.hp,
            max_hp: self.max_hp,
            change_type: HpChangeType::SetValue,
        });
    }

    pub fn hp_percentage(&self) -> f32 {
        self.hp / self.max_hp
    }
}

// Builder pattern for more readable construction with defaults
pub struct UnitBuilder {
    name: String,
    hp: f32,
    max_hp: f32,
    unit_type: UnitType,
    speed: f32,
}

impl Default for UnitBuilder {
    fn default() -> Self {
        Self {
            name: "Unnamed Unit".to_string(),
            hp: DEFAULT_MAX_HP,
            max_hp: DEFAULT_MAX_HP,
            speed: DEFAULT_SPEED,
            unit_type: UnitType::Dummy,
        }
    }
}

impl UnitBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn hp(mut self, hp: f32) -> Self {
        self.hp = hp;
        self
    }

    pub fn unitType(mut self, unit_type: UnitType) -> Self {
        self.unit_type = unit_type;
        self
    }

    pub fn max_hp(mut self, max_hp: f32) -> Self {
        self.max_hp = max_hp;
        // If hp is still the default and we're setting max_hp, update hp to match
        if self.hp == DEFAULT_MAX_HP {
            self.hp = max_hp;
        }
        self
    }

    pub fn build(self) -> Unit {
        Unit {
            name: self.name,
            hp: self.hp,
            max_hp: self.max_hp,
            speed: self.speed,
            unit_type: self.unit_type,
        }
    }
}

use bevy::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HpChangeEvent>();
    }
}

#[derive(Component)]
pub struct Unit {
    pub name: String,
    pub hp: f32,
    pub max_hp: f32,
}

#[derive(Event)]
pub struct HpChangeEvent {
    pub entity: Entity,
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
    pub fn new(name: impl Into<String>, hp: f32, max_hp: f32) -> Self {
        Self {
            name: name.into(),
            hp,
            max_hp,
        }
    }

    // Builder pattern approach with defaults
    pub fn builder() -> UnitBuilder {
        UnitBuilder::default()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }

    pub fn damage(
        &mut self,
        amount: f32,
        entity: Entity,
        event_writer: &mut EventWriter<HpChangeEvent>,
    ) {
        let old_hp = self.hp;
        self.hp = (self.hp - amount).max(0.0);
        event_writer.send(HpChangeEvent {
            entity,
            old_hp,
            new_hp: self.hp,
            max_hp: self.max_hp,
            change_type: HpChangeType::Damage,
        });
    }

    pub fn heal(
        &mut self,
        amount: f32,
        entity: Entity,
        event_writer: &mut EventWriter<HpChangeEvent>,
    ) {
        let old_hp = self.hp;
        self.hp = (self.hp + amount).min(self.max_hp);
        event_writer.send(HpChangeEvent {
            entity,
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
        event_writer.send(HpChangeEvent {
            entity,
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
}

impl Default for UnitBuilder {
    fn default() -> Self {
        Self {
            name: "Unnamed Unit".to_string(),
            hp: 100.0,
            max_hp: 100.0,
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

    pub fn max_hp(mut self, max_hp: f32) -> Self {
        self.max_hp = max_hp;
        // If hp is still the default and we're setting max_hp, update hp to match
        if self.hp == 100.0 {
            self.hp = max_hp;
        }
        self
    }

    pub fn build(self) -> Unit {
        Unit {
            name: self.name,
            hp: self.hp,
            max_hp: self.max_hp,
        }
    }
}
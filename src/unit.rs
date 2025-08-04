use bevy::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HpChangeEvent>();
    }
}

#[derive(Component)]
pub struct Hp {
    pub hp: f32,
    pub max_hp: f32,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
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

impl Hp {
    pub fn new(hp: f32, max_hp: f32) -> Self {
        Self { hp: max_hp, max_hp }
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

        event_writer.write(HpChangeEvent {
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

        event_writer.write(HpChangeEvent {
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

        event_writer.write(HpChangeEvent {
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

pub struct HpPlugin;

impl Plugin for HpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HpChangeEvent>();
    }
}
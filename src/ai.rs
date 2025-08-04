use bevy::prelude::*;

use crate::weapon::GearSet;

#[derive(Component)]
pub struct AIBrain {
    pub gear_set: GearSet,
    pub target: u32,
    pub alert_range: f32,
}

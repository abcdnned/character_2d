use bevy::prelude::*;

/// Component that indicates an entity can deal damage
#[derive(Component, Debug, Clone)]
pub struct Damage {
    pub amount: f32,
    pub damage_type: DamageType,
    pub source: Entity,
}

impl Damage {
    /// Create a new damage component with specified amount and type
    pub fn new(amount: f32, damage_type: DamageType, source: Entity) -> Self {
        Self {
            amount,
            damage_type,
            source,
        }
    }
    
    /// Create a physical damage component
    pub fn physical(amount: f32, entity: Entity) -> Self {
        Self::new(amount, DamageType::Physical, entity)
    }
    
    /// Get the damage amount
    pub fn get_amount(&self) -> f32 {
        self.amount
    }
    
    /// Set the damage amount
    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount;
    }
    
    /// Get the damage type
    pub fn get_type(&self) -> DamageType {
        self.damage_type
    }
    
    /// Multiply damage by a factor (for critical hits, weakness, etc.)
    pub fn multiply(&mut self, factor: f32) {
        self.amount *= factor;
    }
}

/// Types of damage that can be dealt
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Physical,
    Magical,
    Fire,
    Ice,
    Lightning,
    Poison,
    Holy,
    Dark,
}

impl DamageType {
    /// Get the name of the damage type as a string
    pub fn name(&self) -> &'static str {
        match self {
            DamageType::Physical => "Physical",
            DamageType::Magical => "Magical",
            DamageType::Fire => "Fire",
            DamageType::Ice => "Ice",
            DamageType::Lightning => "Lightning",
            DamageType::Poison => "Poison",
            DamageType::Holy => "Holy",
            DamageType::Dark => "Dark",
        }
    }
}
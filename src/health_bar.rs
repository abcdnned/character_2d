use bevy::{
    app::Plugin, color::palettes::css::*, prelude::*, reflect::TypePath, render::render_resource::*,
};
// Import your HpChangeEvent from unit.rs
use crate::unit::HpChangeEvent;

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/health_bar_material.wgsl";

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<HealthBarMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (update_health_bar, update_enemy_health_bar));
    }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct EnemyHealthBar;

#[derive(Resource, Default)]
pub struct LastHitEnemy {
    pub entity: Option<Entity>,
}

fn setup(mut commands: Commands, mut ui_materials: ResMut<Assets<HealthBarMaterial>>) {
    // Initialize the LastHitEnemy resource
    commands.insert_resource(LastHitEnemy::default());

    // Create the main UI container
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::SpaceBetween, // Space between left and right
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            // Player health bar (top left)
            parent.spawn((
                Node {
                    width: Val::Px(600.0),
                    height: Val::Px(25.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                MaterialNode(ui_materials.add(HealthBarMaterial {
                    fill_ratio: Vec4::new(1.0, 0.0, 0.0, 0.0), // Start at full health
                    health_color: LinearRgba::from(RED).to_f32_array().into(), // Red health color
                    border_color: LinearRgba::from(WHITE).to_f32_array().into(), // White border
                })),
                HealthBar,
            ));

            // Enemy health bar (top right)
            parent.spawn((
                Node {
                    width: Val::Px(600.0),
                    height: Val::Px(25.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                MaterialNode(ui_materials.add(HealthBarMaterial {
                    fill_ratio: Vec4::new(0.0, 0.0, 0.0, 0.0), // Start empty (no enemy selected)
                    health_color: LinearRgba::from(GREEN).to_f32_array().into(), // Green health color
                    border_color: LinearRgba::from(WHITE).to_f32_array().into(), // White border
                })),
                EnemyHealthBar,
            ));
        });
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct HealthBarMaterial {
    /// Represents how much of the health bar is filled (0.0 to 1.0)
    #[uniform(0)]
    fill_ratio: Vec4,
    /// Color of the health bar
    #[uniform(1)]
    health_color: Vec4,
    /// Color of the border
    #[uniform(2)]
    border_color: Vec4,
}

impl UiMaterial for HealthBarMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

// Update player health bar when HpChangeEvent is received
fn update_health_bar(
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    mut hp_events: EventReader<HpChangeEvent>,
    health_bar_query: Query<&MaterialNode<HealthBarMaterial>, With<HealthBar>>,
    player_query: Query<(Entity, &crate::unit::Unit), With<crate::Player>>,
) {
    for event in hp_events.read() {
        debug!("Received HpChangeEvent for entity: {:?}", event.entity);
        
        // Check if the event is for a player entity
        if let Ok((player_entity, hp)) = player_query.single() {
            debug!("Player entity: {:?}, HP: {}/{}", player_entity, hp.hp, hp.max_hp);
            
            if event.entity == player_entity {
                debug!("Event is for player entity, updating health bar");
                
                // Update all health bars
                for material_handle in health_bar_query.iter() {
                    if let Some(material) = materials.get_mut(material_handle) {
                        // Calculate fill ratio based on current health from the Hp component
                        let fill_ratio = if hp.max_hp > 0.0 {
                            (hp.hp / hp.max_hp).clamp(0.0, 1.0)
                        } else {
                            0.0
                        };
                        
                        debug!("Updating health bar: fill_ratio = {:.2}", fill_ratio);
                        material.fill_ratio.x = fill_ratio;
                    }
                }
            } else {
                debug!("Event is not for player entity, ignoring");
            }
        } else {
            warn!("Failed to get player entity or HP component");
        }
    }
}

// Update enemy health bar when player damages an enemy
fn update_enemy_health_bar(
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    mut hp_events: EventReader<HpChangeEvent>,
    mut last_hit_enemy: ResMut<LastHitEnemy>,
    enemy_health_bar_query: Query<&MaterialNode<HealthBarMaterial>, With<EnemyHealthBar>>,
    player_query: Query<Entity, With<crate::Player>>,
    unit_query: Query<&crate::unit::Unit>,
) {
    for event in hp_events.read() {
        debug!("Checking HpChangeEvent for enemy health bar update");
        
        // Check if the damage source is a player
        if let Ok(player_entity) = player_query.single() {
            if event.source == player_entity {
                debug!("Player damaged entity: {:?}", event.entity);
                
                // Update the last hit enemy
                last_hit_enemy.entity = Some(event.entity);
                
                // Get the current HP of the damaged entity
                if let Ok(enemy_unit) = unit_query.get(event.entity) {
                    debug!("Enemy HP: {}/{}", enemy_unit.hp, enemy_unit.max_hp);
                    
                    // Update the enemy health bar
                    for material_handle in enemy_health_bar_query.iter() {
                        if let Some(material) = materials.get_mut(material_handle) {
                            let fill_ratio = if enemy_unit.max_hp > 0.0 {
                                (enemy_unit.hp / enemy_unit.max_hp).clamp(0.0, 1.0)
                            } else {
                                0.0
                            };
                            
                            debug!("Updating enemy health bar: fill_ratio = {:.2}", fill_ratio);
                            material.fill_ratio.x = fill_ratio;
                        }
                    }
                } else {
                    warn!("Failed to get Unit component for damaged entity: {:?}", event.entity);
                }
            }
        }
    }
}
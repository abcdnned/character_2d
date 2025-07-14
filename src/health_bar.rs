// ui_material.rs

use bevy::{
    app::Plugin,
    color::palettes::css::{RED, DARK_RED}, 
    prelude::*, 
    reflect::TypePath, 
    render::render_resource::*,
};

// Import your HpChangeEvent from unit.rs
use crate::unit::HpChangeEvent;

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/custom_ui_material.wgsl";

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<HealthBarMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, update_health_bar);
    }
}

#[derive(Component)]
struct HealthBar;

fn setup(
    mut commands: Commands,
    mut ui_materials: ResMut<Assets<HealthBarMaterial>>,
) {
    // Create the health bar UI at the bottom of the screen
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End, // Align to bottom
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(20.0)), // Add some padding from screen edge
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(400.0), // Health bar width
                    height: Val::Px(30.0), // Health bar height
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                MaterialNode(ui_materials.add(HealthBarMaterial {
                    fill_ratio: Vec4::new(1.0, 0.0, 0.0, 0.0), // Start at full health
                    health_color: LinearRgba::from(RED).to_f32_array().into(),
                    border_color: LinearRgba::from(DARK_RED).to_f32_array().into(),
                })),
                BorderRadius::all(Val::Px(5.0)),
                HealthBar,
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

// Update health bar when HpChangeEvent is received
fn update_health_bar(
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    mut hp_events: EventReader<HpChangeEvent>,
    health_bar_query: Query<&MaterialNode<HealthBarMaterial>, With<HealthBar>>,
    player_query: Query<Entity, With<crate::Player>>,
) {
    for event in hp_events.read() {
        // Check if the event is for a player entity
        if let Ok(player_entity) = player_query.get_single() {
            if event.entity == player_entity {
                // Update all health bars
                for material_handle in health_bar_query.iter() {
                    if let Some(material) = materials.get_mut(material_handle) {
                        // Calculate fill ratio based on current health
                        let fill_ratio = if event.max_hp > 0.0 {
                            (event.new_hp / event.max_hp).clamp(0.0, 1.0)
                        } else {
                            0.0
                        };
                        
                        material.fill_ratio.x = fill_ratio;
                        
                        // Optional: Change color based on health percentage
                        if fill_ratio > 0.6 {
                            material.health_color = LinearRgba::from(Color::srgb(0.0, 1.0, 0.0)).to_f32_array().into(); // Green
                        } else if fill_ratio > 0.3 {
                            material.health_color = LinearRgba::from(Color::srgb(1.0, 1.0, 0.0)).to_f32_array().into(); // Yellow
                        } else {
                            material.health_color = LinearRgba::from(RED).to_f32_array().into(); // Red
                        }
                    }
                }
            }
        }
    }
}
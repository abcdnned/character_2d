// ui_material.rs
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
            .add_systems(Update, update_health_bar);
    }
}

#[derive(Component)]
pub struct HealthBar;

fn setup(mut commands: Commands, mut ui_materials: ResMut<Assets<HealthBarMaterial>>) {
    // Create the health bar UI at the top left of the screen
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start, // Align to top
            justify_content: JustifyContent::Start, // Align to left
            padding: UiRect::all(Val::Px(20.0)), // Add some padding from screen edge
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(600.0), // Bigger health bar width
                    height: Val::Px(25.0), // Bigger health bar height
                    border: UiRect::all(Val::Px(1.0)), // Thin border
                    ..default()
                },
                MaterialNode(ui_materials.add(HealthBarMaterial {
                    fill_ratio: Vec4::new(1.0, 0.0, 0.0, 0.0), // Start at full health
                    health_color: LinearRgba::from(RED).to_f32_array().into(), // Red health color
                    border_color: LinearRgba::from(WHITE).to_f32_array().into(), // White border
                })),
                // Remove BorderRadius to make it a standard rectangle (no rounded corners)
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
    player_query: Query<(Entity, &crate::unit::Unit), With<crate::Player>>,
) {
    for event in hp_events.read() {
        // Check if the event is for a player entity
        if let Ok((player_entity, hp)) = player_query.single() {
            if event.entity == player_entity {
                // Update all health bars
                for material_handle in health_bar_query.iter() {
                    if let Some(material) = materials.get_mut(material_handle) {
                        // Calculate fill ratio based on current health from the Hp component
                        let fill_ratio = if hp.max_hp > 0.0 {
                            (hp.hp / hp.max_hp).clamp(0.0, 1.0)
                        } else {
                            0.0
                        };
                        material.fill_ratio.x = fill_ratio;
                    }
                }
            }
        }
    }
}